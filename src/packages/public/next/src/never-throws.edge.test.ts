import { HttpResponse, http } from "msw";
import { delivered, network } from "../test/edge-network.fixture";
import { trace } from "@opentelemetry/api";
import { TracerProvider } from "@opentelemetry/sdk-trace";
import type { MockInstance } from "vitest";
import { afterEach, beforeAll, beforeEach, expect, test, vi } from "vitest";

const ROUTE = {
  routerKind: "App Router",
  routePath: "/broken",
  routeType: "route",
  renderSource: undefined,
  revalidateReason: undefined,
  renderType: undefined,
} as const;

let registration: MockInstance<typeof trace.setGlobalTracerProvider>;

beforeAll(async () => {
  await import("./instrumentation.edge.js");
});

beforeEach(() => {
  vi.resetModules();

  trace.disable();

  registration = vi.spyOn(trace, "setGlobalTracerProvider");

  vi.stubEnv("INTERFERE_API_KEY", "interfere_pk_test");

  vi.stubEnv("INTERFERE_API_URL", "https://edge.invalid");

  vi.stubEnv("NEXT_PUBLIC_INTERFERE_FORCE_ENABLE", "1");

  vi.spyOn(console, "warn");
});

afterEach(async () => {
  const providers = registration.mock.calls.flatMap(([provider]) =>
    provider instanceof TracerProvider ? [provider] : [],
  );

  await Promise.all(providers.map(async (provider) => provider.shutdown()));

  trace.disable();

  vi.unstubAllEnvs();

  vi.restoreAllMocks();
});

test("delivery resolves when the collector is unreachable", async () => {
  network.use(
    http.post("https://edge.invalid/drain/v1/traces", () =>
      HttpResponse.error(),
    ),
  );

  const { onRequestError, register } =
    await import("./instrumentation.edge.js");

  await expect(register()).resolves.toBeUndefined();

  await expect(
    onRequestError(new Error("customer failure"), undefined, ROUTE),
  ).resolves.toBeUndefined();

  await expect(delivered()).resolves.toBeUndefined();
});

test("register and capture resolve beside a foreign provider whose tracer throws", async () => {
  network.use(
    http.post(
      "https://edge.invalid/drain/v1/traces",
      () => new HttpResponse(null, { status: 204 }),
    ),
  );

  trace.setGlobalTracerProvider({
    getTracer() {
      throw new Error("injected tracer failure");
    },
  });

  const { onRequestError, register } =
    await import("./instrumentation.edge.js");

  await expect(register()).resolves.toBeUndefined();

  await expect(
    onRequestError(new Error("customer failure"), undefined, ROUTE),
  ).resolves.toBeUndefined();

  await expect(delivered()).resolves.toBeUndefined();
});

test("register and capture resolve with a malformed API URL", async () => {
  vi.stubEnv("INTERFERE_API_URL", "not a url");

  const { onRequestError, register } =
    await import("./instrumentation.edge.js");

  await expect(register()).resolves.toBeUndefined();

  await expect(
    onRequestError(new Error("customer failure"), undefined, ROUTE),
  ).resolves.toBeUndefined();

  await expect(delivered()).resolves.toBeUndefined();
});
