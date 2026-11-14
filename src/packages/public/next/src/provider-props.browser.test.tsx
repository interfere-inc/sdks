import { network } from "@interfere/test-config/browser-network";
import { InterfereClient } from "@interfere/javascript";
import { HttpResponse, http } from "msw";
import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { ErrorBoundary } from "@interfere/react/error-boundary";
import { page } from "vitest/browser";
import { render } from "vitest-browser-react";
import { close, init } from "./instrument-client.js";
import { InterfereProvider } from "./provider.js";
import { afterEach, assert, beforeEach, expect, test, vi } from "vitest";
import type { MockInstance } from "vitest";

const plugins = {
  device: false,
  errors: false,
  logs: false,
  pageEvents: false,
  rageClick: false,
  replay: false,
};

const requests: Request[] = [];

const unexpected: string[] = [];

let registrations: MockInstance<InterfereClient["sessions"]["createSession"]>;

beforeEach(async () => {
  await close();

  requests.length = 0;

  unexpected.length = 0;

  network.use(
    http.get("https://next-provider.invalid/v3/config", ({ request }) => {
      requests.push(request);

      return HttpResponse.json({ plugins });
    }),
    http.post("https://next-provider.invalid/v3/session", ({ request }) => {
      requests.push(request);

      return new HttpResponse(null, { status: 204 });
    }),
    http.post(
      "https://next-provider.invalid/drain/v1/traces",
      ({ request }) => {
        requests.push(request);

        return new HttpResponse(null, { status: 204 });
      },
    ),
    http.all("https://next-provider.invalid/*", ({ request }) => {
      requests.push(request);

      unexpected.push(`${request.method} ${request.url}`);

      return HttpResponse.error();
    }),
  );

  const client = new InterfereClient({
    baseUrl: "https://next-provider.invalid",
    publicKeyHeader: { apiKey: "interfere_pk_test" },
  });

  // oxlint-disable-next-line typescript/no-unsafe-assignment
  const sessions: InterfereClient["sessions"] = Object.getPrototypeOf(
    client.sessions,
  );

  registrations = vi.spyOn(sessions, "createSession");

  setInjectedConfiguration("__INTERFERE_BUILD__", true);

  setInjectedConfiguration("__INTERFERE_FORCE_ENABLE__", true);

  setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "interfere_pk_test");

  setInjectedConfiguration("__INTERFERE_PLUGINS__", undefined);

  setInjectedConfiguration("__INTERFERE_INGEST_URL__", undefined);

  setInjectedConfiguration("__INTERFERE_SDK_STACK__", undefined);

  const saved = [localStorage, sessionStorage].flatMap((storage) =>
    [
      "interfere:session_id",
      "interfere:session_started",
      "interfere:last_activity",
      "interfere:window_id",
    ].map((key) => ({ storage, key, value: storage.getItem(key) })),
  );

  onTestFinished(() => {
    for (const { storage, key, value } of saved) {
      if (value === null) {
        storage.removeItem(key);
      } else {
        storage.setItem(key, value);
      }
    }
  });
});

afterEach(async () => {
  try {
    await close();

    await Promise.all(
      registrations.mock.results.flatMap((result) =>
        result.type === "return" ? [result.value] : [],
      ),
    );

    if (unexpected.length > 0) {
      throw new Error(unexpected.join("\n"));
    }
  } finally {
    vi.unstubAllGlobals();
  }
});

test("forwards current options, preserves omitted consent, and honors explicit revocation", async ({
  onTestFinished,
}) => {
  const view = await render(
    <InterfereProvider
      consent={{ analytics: false, replay: false }}
      plugins={{ ...plugins, replay: { maskAllInputs: true } }}
    >
      <div>configured children</div>
    </InterfereProvider>,
  );

  onTestFinished(async () => view.unmount());

  await view.rerender(
    <InterfereProvider
      consent={{ analytics: true, replay: false }}
      plugins={{ ...plugins, replay: { maskAllInputs: true } }}
    >
      <div>configured children</div>
    </InterfereProvider>,
  );

  const kernel = await init({
    apiHost: "https://next-provider.invalid",
    enabled: true,
    tracing: false,
  });

  expect(kernel).not.toBeNull();

  assert.exists(kernel);

  await expect
    .poll(() => kernel.consent.get())
    .toEqual({ analytics: true, replay: false });

  assert.exists(kernel);

  expect(kernel.getReplayConfig()).toStrictEqual({ maskAllInputs: true });

  await view.rerender(
    <InterfereProvider
      consent={{ analytics: false, replay: true }}
      plugins={{ ...plugins, replay: { maskAllInputs: false } }}
    >
      <div>configured children</div>
    </InterfereProvider>,
  );

  assert.exists(kernel);

  expect(kernel.consent.get()).toStrictEqual({
    analytics: false,
    replay: true,
  });

  assert.exists(kernel);

  expect(kernel.getReplayConfig()).toStrictEqual({ maskAllInputs: false });

  await view.rerender(
    <InterfereProvider>
      <div>configured children</div>
    </InterfereProvider>,
  );

  assert.exists(kernel);

  expect(kernel.consent.get()).toStrictEqual({
    analytics: false,
    replay: true,
  });

  expect(kernel.getReplayConfig()).toStrictEqual({});

  await view.rerender(
    <InterfereProvider consent={false}>
      <div>configured children</div>
    </InterfereProvider>,
  );

  expect(kernel.consent.get()).toStrictEqual({
    analytics: false,
    replay: false,
  });

  await expect.element(page.getByText("configured children")).toBeVisible();
});

function ThrowingChild(): never {
  throw new Error("next-provider-render");
}

test.each([
  [{}, true],
  [{ captureRenderErrors: false }, false],
] as const)("forwards render capture %j", async (props, enabled) => {
  const kernel = await init({
    apiHost: "https://next-provider.invalid",
    enabled: true,
    tracing: false,
  });

  assert.exists(kernel);

  vi.spyOn(console, "error");

  const recordSpy = vi.spyOn(kernel, "recordException");

  await render(
    <ErrorBoundary fallback={<div>host fallback</div>}>
      <InterfereProvider {...props}>
        <ThrowingChild />
      </InterfereProvider>
    </ErrorBoundary>,
  );

  await expect.element(page.getByText("host fallback")).toBeVisible();

  expect(
    recordSpy.mock.calls.some(([, options]) => !options.mechanism.handled),
  ).toBe(enabled);
});
