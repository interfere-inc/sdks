import { network } from "@interfere/test-config/browser-network";
import { HttpResponse, http } from "msw";
import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { page } from "vitest/browser";
import { renderHook } from "vitest-browser-react";
import { close, init } from "./instrument-client.js";
import { InterfereProvider, useInterfere, useSession } from "./provider.js";
import { afterEach, assert, beforeEach, expect, test, vi } from "vitest";

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
    http.all("https://next-provider.invalid/*", ({ request }) => {
      requests.push(request);

      unexpected.push(`${request.method} ${request.url}`);

      return HttpResponse.error();
    }),
  );

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

    if (unexpected.length > 0) {
      throw new Error(unexpected.join("\n"));
    }
  } finally {
    vi.unstubAllGlobals();
  }
});

test("renders children and exposes the initialized kernel's actual accessors", async ({
  onTestFinished,
}) => {
  expect.hasAssertions();

  const kernel = await init({
    apiHost: "https://next-provider.invalid",
    enabled: true,
    tracing: false,
  });

  const view = await renderHook(
    () => ({ context: useInterfere(), session: useSession() }),
    {
      wrapper: ({ children }) => (
        <InterfereProvider plugins={plugins}>
          <div>provider children</div>
          {children}
        </InterfereProvider>
      ),
    },
  );

  onTestFinished(async () => view.unmount());

  expect(kernel).not.toBeNull();

  await expect.element(page.getByText("provider children")).toBeVisible();

  expect(view.result.current.context.kernel).toBe(kernel);

  assert.exists(kernel);

  expect(view.result.current.context.consent).toBe(kernel.consent);

  assert.exists(kernel);

  expect(view.result.current.context.device).toBe(kernel.device);

  assert.exists(kernel);

  expect(view.result.current.context.identity).toBe(kernel.identity);

  assert.exists(kernel);

  expect(view.result.current.context.session).toBe(kernel.session);

  assert.exists(kernel);

  expect(view.result.current.session).toBe(kernel.session.getId());
  expect(view.result.current.session).toStrictEqual(expect.any(String));

  view.result.current.context.consent.set({ analytics: true, replay: false });

  assert.exists(kernel);

  expect(kernel.consent.get()).toStrictEqual({
    analytics: true,
    replay: false,
  });

  view.result.current.context.consent.set({ analytics: false, replay: true });

  assert.exists(kernel);

  expect(kernel.consent.get()).toStrictEqual({
    analytics: false,
    replay: true,
  });

  await vi.waitFor(() => {
    expect(
      requests.map((request) => [request.method, request.url]),
    ).toStrictEqual([
      ["GET", "https://next-provider.invalid/v3/config"],
      ["POST", "https://next-provider.invalid/v3/session"],
    ]);
  });
});
