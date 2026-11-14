import { ErrorBoundary } from "@interfere/react/error-boundary";
import { network } from "@interfere/test-config/browser-network";
import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { HttpResponse, http } from "msw";
import { afterEach, assert, beforeEach, expect, test, vi } from "vitest";
import { page } from "vitest/browser";
import { render } from "vitest-browser-react";

import { close, init } from "./init";
import { InterfereProvider } from "./provider";

const plugins = {
  device: false,
  errors: false,
  logs: false,
  pageEvents: false,
  rageClick: false,
  replay: false,
};

beforeEach(async () => {
  await close();

  network.use(
    http.get("https://vite-provider.invalid/v3/config", () =>
      HttpResponse.json({ plugins }),
    ),
    http.post(
      "https://vite-provider.invalid/v3/session",
      () => new HttpResponse(null, { status: 204 }),
    ),
    http.post(
      "https://vite-provider.invalid/drain/v1/traces",
      () => new HttpResponse(null, { status: 204 }),
    ),
  );

  setInjectedConfiguration("__INTERFERE_FORCE_ENABLE__", true);

  setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "interfere_pk_test");
});

afterEach(async () => {
  await close();

  vi.restoreAllMocks();
});

function ThrowingChild(): never {
  throw new Error("vite-provider-render");
}

test.each([
  [{}, true],
  [{ captureRenderErrors: false }, false],
  [{ captureRenderErrors: true }, true],
] as const)("forwards render capture %j", async (props, enabled) => {
  const kernel = await init({
    apiHost: "https://vite-provider.invalid",
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
