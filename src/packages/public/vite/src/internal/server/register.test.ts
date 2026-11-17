import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import { attribute, exportedSpans } from "@interfere/test-utils/vitest/otlp";
import { omitUndefined } from "@interfere/helpers/omit-undefined";

import { context, metrics, propagation, trace } from "@opentelemetry/api";
import { logs } from "@opentelemetry/api-logs";
import { UndiciInstrumentation } from "@opentelemetry/instrumentation-undici";
import { LoggerProvider } from "@opentelemetry/sdk-logs";
import { MeterProvider } from "@opentelemetry/sdk-metrics";
import { NodeTracerProvider } from "@opentelemetry/sdk-trace-node";
import {
  afterAll,
  afterEach,
  assert,
  beforeAll,
  beforeEach,
  expect,
  test,
  vi,
} from "vitest";
import type { MockInstance } from "vitest";

let collector: Awaited<ReturnType<typeof receiver>>;

let register: (typeof import("./register"))["register"];

let registerProvider: MockInstance<
  typeof NodeTracerProvider.prototype.register
>;

let registerLogger: MockInstance<typeof logs.setGlobalLoggerProvider>;

let registerMeter: MockInstance<typeof metrics.setGlobalMeterProvider>;

let registerInstrumentation: MockInstance<
  typeof UndiciInstrumentation.prototype.setTracerProvider
>;

beforeAll(async () => {
  collector = await receiver();
});

beforeEach(async () => {
  vi.resetModules();

  vi.clearAllMocks();

  registerProvider = vi.spyOn(NodeTracerProvider.prototype, "register");

  registerLogger = vi.spyOn(logs, "setGlobalLoggerProvider");

  registerMeter = vi.spyOn(metrics, "setGlobalMeterProvider");

  registerInstrumentation = vi.spyOn(
    UndiciInstrumentation.prototype,
    "setTracerProvider",
  );

  collector.reset();

  vi.stubEnv("OTEL_SERVICE_NAME", undefined);

  vi.stubEnv("VITE_INTERFERE_SERVICE_NAME", undefined);

  vi.stubEnv("INTERFERE_API_KEY", "key+value");

  vi.stubEnv("INTERFERE_API_URL", collector.url);

  vi.stubEnv("INTERFERE_SOURCE_ID", "a".repeat(40));

  ({ register } = await import("./register"));
});

afterEach(async () => {
  for (const instrumentation of registerInstrumentation.mock.contexts) {
    if (instrumentation instanceof UndiciInstrumentation) {
      instrumentation.disable();
    }
  }

  await Promise.all([
    ...registerProvider.mock.contexts.map(async (provider) =>
      provider instanceof NodeTracerProvider ? provider.shutdown() : undefined,
    ),
    ...registerLogger.mock.calls.map(async ([provider]) =>
      provider instanceof LoggerProvider ? provider.shutdown() : undefined,
    ),
    ...registerMeter.mock.calls.map(async ([provider]) =>
      provider instanceof MeterProvider ? provider.shutdown() : undefined,
    ),
  ]);

  trace.disable();

  metrics.disable();

  logs.disable();

  context.disable();

  propagation.disable();

  vi.unstubAllEnvs();
});

afterAll(async () => {
  await collector.close();

  vi.restoreAllMocks();
});

test.each<{
  tracing: "auto" | "managed" | undefined;
  ignoreUrls: RegExp[] | undefined;
}>([
  { tracing: undefined, ignoreUrls: undefined },
  { tracing: "auto", ignoreUrls: [/\/custom$/u] },
  { tracing: "managed", ignoreUrls: undefined },
])(
  "tracing $tracing routes each signal and excludes its own requests with ignores $ignoreUrls",
  async ({ tracing, ignoreUrls }) => {
    expect.assertions(12);

    await register(
      omitUndefined({ consoleBridge: false, ignoreUrls, tracing }),
    );

    const [tracerProvider] = registerProvider.mock.contexts;
    const loggerProvider = registerLogger.mock.calls[0]?.[0];
    const meterProvider = registerMeter.mock.calls[0]?.[0];
    const [instrumentation] = registerInstrumentation.mock.contexts;

    if (
      !(tracerProvider instanceof NodeTracerProvider) ||
      !(loggerProvider instanceof LoggerProvider) ||
      !(meterProvider instanceof MeterProvider) ||
      !(instrumentation instanceof UndiciInstrumentation)
    ) {
      throw new TypeError(
        "Expected native telemetry providers and instrumentation",
      );
    }

    tracerProvider.getTracer("registration-test").startSpan("operation").end();

    loggerProvider.getLogger("registration-test").emit({ body: "operation" });

    meterProvider
      .getMeter("registration-test")
      .createCounter("operations")
      .add(1);

    await Promise.all([
      tracerProvider.forceFlush(),
      loggerProvider.forceFlush(),
      meterProvider.forceFlush(),
    ]);

    const requestUrls = collector.requests.map(
      (request) => new URL(request.url, collector.url).href,
    );

    for (const signal of ["logs", "metrics", "traces"]) {
      expect(requestUrls).toContain(`${collector.url}/drain/v1/${signal}`);
    }

    expect(
      new Set(
        collector.requests
          .filter((request) => request.url.startsWith("/drain/"))
          .map((request) => request.headers.get("authorization")),
      ),
    ).toStrictEqual(new Set(["Bearer key+value"]));

    const { ignoreRequestHook } = instrumentation.getConfig();

    if (!ignoreRequestHook) {
      throw new Error("Expected outbound request filtering");
    }

    for (const [url, ignored] of [
      [`${collector.url}/drain/v1/traces`, true],
      [`${collector.url}/drain/v1/logs`, true],
      [`${collector.url}/drain/v1/metrics`, true],
      ["https://customer.example/orders", false],
      ["https://customer.example/custom", Boolean(ignoreUrls)],
    ] as const) {
      const requestUrl = new URL(url);

      expect(
        ignoreRequestHook({
          origin: requestUrl.origin,
          path: requestUrl.pathname + requestUrl.search,
          method: "GET",
          headers: [],
          addHeader: vi.fn<(name: string, value: string) => void>(),
          throwOnError: false,
          completed: false,
          aborted: false,
          idempotent: true,
          contentLength: null,
          contentType: null,
          body: null,
        }),
      ).toBe(ignored);
    }

    await register();

    expect(registerProvider).toHaveBeenCalledTimes(1);
    expect(registerLogger).toHaveBeenCalledTimes(1);
    expect(registerMeter).toHaveBeenCalledTimes(1);
  },
);

test("honors baked drain modes without exporting traces or logs", async () => {
  vi.stubEnv("VITE_INTERFERE_SERVER_TRACING", "drain");

  vi.stubEnv("VITE_INTERFERE_LOG_SOURCING", "drain");

  await register({ consoleBridge: false });

  const [provider] = registerProvider.mock.contexts;

  if (!(provider instanceof NodeTracerProvider)) {
    throw new TypeError("Expected a native tracer provider");
  }

  provider.getTracer("registration-test").startSpan("operation").end();

  await provider.forceFlush();

  expect(registerLogger).not.toHaveBeenCalled();
  expect(registerInstrumentation).not.toHaveBeenCalled();

  expect(
    collector.requests.filter((request) => request.method === "POST"),
  ).toStrictEqual([]);
});

test.each([
  { platform: "vercel", vercel: undefined, capped: true, landed: 0 },
  { platform: undefined, vercel: "1", capped: true, landed: 0 },
  { platform: "", vercel: "1", capped: false, landed: 1 },
  { platform: "cloudflare", vercel: "1", capped: false, landed: 1 },
])(
  "baked platform $platform with VERCEL=$vercel caps the direct trace flush: $capped",
  async ({ platform, vercel, landed }) => {
    vi.stubEnv("VITE_INTERFERE_PLATFORM", platform);

    vi.stubEnv("VERCEL", vercel);

    collector.stall("/drain/v1/traces", 3500);

    await register({ consoleBridge: false });

    const [provider] = registerProvider.mock.contexts;

    if (!(provider instanceof NodeTracerProvider)) {
      throw new TypeError("Expected a native tracer provider");
    }

    provider.getTracer("registration-test").startSpan("operation").end();

    await provider.forceFlush().catch(() => undefined);

    expect(
      collector.requests.filter((request) =>
        request.url.startsWith("/drain/v1/traces"),
      ),
    ).toHaveLength(landed);
  },
);

test.each([
  {
    serviceName: undefined,
    environment: undefined,
    inferred: undefined,
    expected: `unknown_service:${process.argv0}`,
    expectedNamespace: undefined,
  },
  {
    serviceName: undefined,
    environment: "env-api",
    inferred: "platform",
    expected: "env-api",
    expectedNamespace: undefined,
  },
  {
    serviceName: "checkout",
    environment: "env-api",
    inferred: "platform",
    expected: "checkout",
    expectedNamespace: "shop",
  },
  {
    serviceName: undefined,
    environment: undefined,
    inferred: "platform",
    expected: "platform",
    expectedNamespace: undefined,
  },
])(
  "resolves service $serviceName with environment $environment to $expected",
  async ({
    serviceName,
    environment,
    inferred,
    expected,
    expectedNamespace,
  }) => {
    vi.stubEnv("OTEL_SERVICE_NAME", environment);

    vi.stubEnv("VITE_INTERFERE_SERVICE_NAME", inferred);

    await register(
      omitUndefined({
        consoleBridge: false,
        serviceName,
        serviceNamespace: serviceName ? "shop" : undefined,
      }),
    );

    trace.getTracer("test").startSpan("identity").end();

    const [provider] = registerProvider.mock.contexts;

    assert(provider instanceof NodeTracerProvider);

    await provider.forceFlush();

    const spans = exportedSpans(collector.requests).filter(
      (span) => span.name === "identity",
    );

    expect(spans).toHaveLength(1);

    const [span] = spans;

    assert(span);

    expect(attribute(span.resource, "service.name")).toBe(expected);

    expect(attribute(span.resource, "service.namespace")).toBe(
      expectedNamespace,
    );
  },
);
