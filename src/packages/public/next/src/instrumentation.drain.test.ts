import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { omitUndefined } from "@interfere/helpers/omit-undefined";
/* oxlint-disable typescript/no-deprecated */
import { mswServer } from "@interfere/test-config/msw-server";
import { once } from "node:events";
import { HttpResponse, delay, http } from "msw";
import type { ProxyTracerProvider } from "@opentelemetry/api";
import { context, logs, propagation, trace } from "@opentelemetry/api";
import { TracerProvider } from "@interfere/server-core/otel/sdk/trace/provider";
import { PLATFORM_GLOBAL } from "@interfere/types/integrations/platforms/hosting";
import { LOG_SOURCING_GLOBAL } from "@interfere/types/sdk/log-sourcing";
import { BACKEND_HOSTS_GLOBAL } from "@interfere/types/sdk/remote-config/backend-hosts";
import { SERVER_TRACING_GLOBAL } from "@interfere/types/sdk/server-tracing";
import {
  afterAll,
  afterEach,
  beforeAll,
  beforeEach,
  expect,
  test,
  vi,
} from "vitest";

await import("./instrumentation.js");

await import("@vercel/otel");

const requests: string[] = [];

const hold = { ms: 0 };

const collector = http.post("*", async ({ request }) => {
  const body = await request.text();

  await delay(hold.ms);

  requests.push(body);

  return HttpResponse.json({});
});

const server = mswServer(collector);

let collectorUrl: string;

beforeAll(async () => {
  server.listen(0, "127.0.0.1");

  await once(server, "listening");

  const address = server.address();

  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Node returns a string for Unix sockets; this test requires a TCP listener.
  if (address === null || typeof address === "string") {
    throw new Error("Expected a TCP address");
  }

  collectorUrl = `http://127.0.0.1:${address.port}`;
});

beforeEach(() => {
  vi.resetModules();

  requests.length = 0;

  hold.ms = 0;

  vi.stubEnv("INTERFERE_API_KEY", "interfere_pk_test");

  vi.stubEnv("INTERFERE_API_URL", collectorUrl);

  vi.stubEnv("INTERFERE_SOURCE_ID", "0123456789abcdef0123456789abcdef01234567");

  vi.stubEnv("OTEL_EXPORTER_OTLP_TRACES_ENDPOINT", `${collectorUrl}/v1/traces`);

  vi.stubEnv("OTEL_EXPORTER_OTLP_TRACES_PROTOCOL", "http/json");

  setInjectedConfiguration(SERVER_TRACING_GLOBAL, "drain");
});

afterEach(async () => {
  const provider = (
    trace.getTracerProvider() as ProxyTracerProvider
  ).getDelegate() as Partial<TracerProvider>;

  await provider.shutdown?.();

  trace.disable();

  context.disable();

  propagation.disable();

  logs.disable();

  vi.unstubAllEnvs();

  vi.unstubAllGlobals();

  vi.restoreAllMocks();
});

afterAll(async () => {
  server.closeAllConnections();

  await server[Symbol.asyncDispose]();
});

const options = {
  consoleBridge: false,
  captureUncaughtException: false,
  captureUnhandledRejection: false,
};

async function exportSpan(name: string, flush: () => Promise<void>) {
  const span = trace.getTracer("customer").startSpan(name);

  expect(span.isRecording()).toBe(true);

  span.end();

  await flush();
}

test("respects disabled Vercel tracing without failing startup", async () => {
  vi.stubEnv("OTEL_SDK_DISABLED", "true");

  const { register, flush } = await import("./instrumentation.js");

  await expect(register(options)).resolves.toBeUndefined();

  const span = trace.getTracer("customer").startSpan("disabled-work");

  expect(span.isRecording()).toBe(false);

  span.end();

  await flush();

  expect(requests).toHaveLength(0);
});

test("uses the native exporter fallback in drain mode without a Vercel runtime", async () => {
  const { register, flush } = await import("./instrumentation.js");

  await register(options);

  expect(Object.hasOwn(globalThis, Symbol.for("@vercel/request-context"))).toBe(
    false,
  );

  await exportSpan("application-work", flush);

  expect(requests).toHaveLength(1);
  expect(JSON.stringify(requests)).toContain("application-work");
});

test.each([undefined, "1"])(
  "exports directly when the build baked no trace drain, VERCEL=%s",
  async (vercel) => {
    vi.stubEnv("VERCEL", vercel);

    setInjectedConfiguration(SERVER_TRACING_GLOBAL, undefined);

    const { register, flush } = await import("./instrumentation.js");

    await register(options);

    await exportSpan("default-direct", flush);

    expect(JSON.stringify(requests)).toContain("default-direct");
  },
);

test.each([
  { platform: "vercel", vercel: undefined, capped: true, landed: 0 },
  { platform: undefined, vercel: "1", capped: true, landed: 0 },
  { platform: null, vercel: "1", capped: false, landed: 1 },
  { platform: "cloudflare", vercel: "1", capped: false, landed: 1 },
])(
  "baked platform $platform with VERCEL=$vercel caps the direct flush: $capped",
  async ({ platform, vercel, landed }) => {
    vi.stubEnv("VERCEL", vercel);

    setInjectedConfiguration(SERVER_TRACING_GLOBAL, "full");

    setInjectedConfiguration(PLATFORM_GLOBAL, platform);

    hold.ms = 3500;

    const { register, flush } = await import("./instrumentation.js");

    await register(options);

    await exportSpan("held-direct", async () => flush().catch(() => undefined));

    expect(requests).toHaveLength(landed);

    await vi.waitUntil(() => requests.length === 1, { timeout: 5000 });
  },
);

test.each([
  { mode: "sdk", exported: true },
  { mode: "drain", exported: false },
] as const)(
  "logSourcing $mode exports server logs directly: $exported (N-ST-05)",
  async ({ mode, exported }) => {
    setInjectedConfiguration(SERVER_TRACING_GLOBAL, "full");

    setInjectedConfiguration(LOG_SOURCING_GLOBAL, mode);

    const { register, flush } = await import("./instrumentation.js");

    await register(options);

    logs.getLogger("customer").emit({ body: `log-${mode}` });

    await exportSpan(`span-${mode}`, flush);

    expect(JSON.stringify(requests)).toContain(`span-${mode}`);
    expect(JSON.stringify(requests).includes(`log-${mode}`)).toBe(exported);
  },
);

test.each(["full", "drain"] as const)(
  "coexists with a tracer provider registered before Interfere in %s mode",
  async (mode) => {
    setInjectedConfiguration(SERVER_TRACING_GLOBAL, mode);

    vi.stubEnv("NEXT_PUBLIC_INTERFERE_FORCE_ENABLE", "1");

    const foreignTracer = new TracerProvider().getTracer("foreign");

    trace.setGlobalTracerProvider({ getTracer: () => foreignTracer });

    const warn = vi.spyOn(console, "warn");

    const { register, flush, onRequestError } =
      await import("./instrumentation.js");

    await expect(
      register({ ...options, tracing: "managed" }),
    ).resolves.toBeUndefined();

    expect(
      warn.mock.calls.filter(([message]) =>
        String(message).includes("already registered"),
      ),
    ).toHaveLength(1);

    await trace
      .getTracer("customer")
      .startActiveSpan("foreign-request", async (span) => {
        await onRequestError(
          new Error("boom-under-foreign-provider"),
          { method: "GET", path: "/" },
          { routerKind: "App Router", routePath: "/", routeType: "render" },
        );

        span.end();
      });

    await flush();

    expect(JSON.stringify(requests)).toContain("boom-under-foreign-provider");
  },
);

test.each([
  {
    source: "configured hosts",
    backendHosts: [" API.example.com. ", "api.example.com:8443"],
    propagateTo: undefined,
  },
  {
    source: "configured hosts plus additional origins",
    backendHosts: ["api.example.com", "api.example.com"],
    propagateTo: [
      "https://API.example.com:443",
      "https://api.example.com:8443",
    ],
  },
  {
    source: "additional origins only",
    backendHosts: undefined,
    propagateTo: ["https://api.example.com", "https://api.example.com:8443"],
  },
])(
  "drain $source enable fetch with exact origin boundaries",
  async ({ backendHosts, propagateTo }) => {
    setInjectedConfiguration(BACKEND_HOSTS_GLOBAL, backendHosts);

    const vercel = await import("@vercel/otel");

    const enable = vi.spyOn(vercel.FetchInstrumentation.prototype, "enable");

    const { register } = await import("./instrumentation.js");

    try {
      await register(omitUndefined({ ...options, propagateTo }));

      expect(enable).toHaveBeenCalledTimes(1);

      const instrumentation = enable.mock.contexts.find(
        (instance) => instance instanceof vercel.FetchInstrumentation,
      );

      expect(instrumentation).toBeDefined();

      const matchers = instrumentation?.getConfig().propagateContextUrls ?? [];

      expect(matchers).toHaveLength(2);

      const matches = (url: string) =>
        matchers.some(
          (matcher) => matcher instanceof RegExp && matcher.test(url),
        );

      expect(matches("https://api.example.com/orders")).toBe(true);
      expect(matches("https://api.example.com:8443/orders")).toBe(true);
      expect(matches("https://api.example.com?query=1")).toBe(true);
      expect(matches("https://api.example.com.evil.test/orders")).toBe(false);
      expect(matches("https://api.example.com:8444/orders")).toBe(false);
      expect(matches("https://sibling.example.com/orders")).toBe(false);
      expect(matches("http://api.example.com/orders")).toBe(false);
    } finally {
      for (const instrumentation of enable.mock.contexts) {
        if (instrumentation instanceof vercel.FetchInstrumentation) {
          instrumentation.disable();
        }
      }
    }
  },
);

test.each([undefined, []])(
  "drain without backend targets (%j) does not install fetch",
  async (backendHosts) => {
    setInjectedConfiguration(BACKEND_HOSTS_GLOBAL, backendHosts);

    const vercel = await import("@vercel/otel");

    const enable = vi.spyOn(vercel.FetchInstrumentation.prototype, "enable");

    const { register } = await import("./instrumentation.js");

    await register(options);

    expect(enable).not.toHaveBeenCalled();
  },
);
