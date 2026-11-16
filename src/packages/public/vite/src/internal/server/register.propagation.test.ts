import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import { attribute, exportedSpans } from "@interfere/test-utils/vitest/otlp";
import { context, metrics, propagation, trace } from "@opentelemetry/api";
import { logs } from "@opentelemetry/api-logs";
import { UndiciInstrumentation } from "@opentelemetry/instrumentation-undici";
import { LoggerProvider } from "@opentelemetry/sdk-logs";
import { MeterProvider } from "@opentelemetry/sdk-metrics";
import { NodeTracerProvider } from "@opentelemetry/sdk-trace-node";
import { afterEach, beforeEach, expect, test, vi } from "vitest";
import type { MockInstance } from "vitest";

await import("./register");

await import("./request-trace");

let providers: MockInstance<typeof NodeTracerProvider.prototype.register>;
let loggers: MockInstance<typeof logs.setGlobalLoggerProvider>;
let meters: MockInstance<typeof metrics.setGlobalMeterProvider>;

let instrumentations: MockInstance<
  typeof UndiciInstrumentation.prototype.setTracerProvider
>;

let collector: Awaited<ReturnType<typeof receiver>>;
let backend: Awaited<ReturnType<typeof receiver>>;

function pin() {
  return { backend, collector, providers };
}

beforeEach(async () => {
  vi.resetModules();

  vi.clearAllMocks();

  providers = vi.spyOn(NodeTracerProvider.prototype, "register");

  loggers = vi.spyOn(logs, "setGlobalLoggerProvider");

  meters = vi.spyOn(metrics, "setGlobalMeterProvider");

  instrumentations = vi.spyOn(
    UndiciInstrumentation.prototype,
    "setTracerProvider",
  );

  collector = await receiver();

  backend = await receiver();

  vi.stubEnv("VITE_INTERFERE_PUBLIC_KEY", "propagation-test");

  vi.stubEnv("INTERFERE_API_URL", collector.url);

  vi.stubEnv("INTERFERE_SOURCE_ID", "a".repeat(40));
});

afterEach(async () => {
  for (const instrumentation of instrumentations.mock.contexts) {
    if (instrumentation instanceof UndiciInstrumentation) {
      instrumentation.disable();
    }
  }

  await Promise.all([
    ...providers.mock.contexts.map(async (provider) =>
      provider instanceof NodeTracerProvider ? provider.shutdown() : undefined,
    ),
    ...loggers.mock.calls.map(async ([provider]) => {
      if (provider instanceof LoggerProvider) {
        await provider.shutdown();
      }
    }),
    ...meters.mock.calls.map(async ([provider]) => {
      if (provider instanceof MeterProvider) {
        await provider.shutdown();
      }
    }),
  ]);

  trace.disable();

  propagation.disable();

  context.disable();

  metrics.disable();

  logs.disable();

  await Promise.all([collector.close(), backend.close()]);

  vi.unstubAllEnvs();
});

test.each([
  { policy: "empty", urls: () => [], headers: [null, null, null] },
  {
    policy: "string",
    urls: (url: string) => [`${url}/allowed`],
    headers: [
      expect.stringMatching(/^00-/u),
      expect.stringContaining("interfere="),
      expect.stringContaining("interfere.session.id=test-session"),
    ],
  },
  {
    policy: "regexp",
    urls: () => [/\/allowed$/gu],
    headers: [
      expect.stringMatching(/^00-/u),
      expect.stringContaining("interfere="),
      expect.stringContaining("interfere.session.id=test-session"),
    ],
  },
])(
  "enforces the $policy propagation policy on real requests without dropping spans",
  async ({ urls, headers }) => {
    expect.hasAssertions();

    const { backend: target, collector: sink, providers: tracers } = pin();

    const { register } = await import("./register");

    const { traceMeta } = await import("./request-trace");

    await register({
      tracing: "managed",
      consoleBridge: false,
      propagateContextUrls: urls(target.url),
    });

    const ctx = propagation.setBaggage(
      context.active(),
      propagation.createBaggage({
        "interfere.session.id": { value: "test-session" },
      }),
    );

    await context.with(ctx, async () =>
      trace
        .getTracer("propagation-test")
        .startActiveSpan("parent", async (span) => {
          try {
            expect(traceMeta()).toContainEqual(
              expect.objectContaining({ name: "traceparent" }),
            );

            const responses = await Promise.all([
              fetch(`${target.url}/allowed`),
              fetch(`${target.url}/denied`),
              fetch(`${target.url}/allowed`),
              fetch(`${target.url}/denied`, {
                headers: { baggage: "customer=value" },
              }),
            ]);

            await Promise.all(
              responses.map(async (response) => response.text()),
            );
          } finally {
            span.end();
          }
        }),
    );

    const allowed = target.requests.filter(
      (request) => request.url === "/allowed",
    );

    const denied = target.requests.filter(
      (request) => request.url === "/denied",
    );

    expect(allowed).toHaveLength(2);
    expect(denied).toHaveLength(2);

    for (const request of allowed) {
      expect([
        request.headers.get("traceparent"),
        request.headers.get("tracestate"),
        request.headers.get("baggage"),
      ]).toStrictEqual(headers);
    }

    for (const request of denied) {
      expect(request.headers.get("traceparent")).toBeNull();
      expect(request.headers.get("tracestate")).toBeNull();
    }

    expect(
      denied.map((request) => request.headers.get("baggage")),
    ).toStrictEqual(expect.arrayContaining([null, "customer=value"]));

    await Promise.all(
      tracers.mock.contexts.map(async (provider) =>
        provider instanceof NodeTracerProvider
          ? provider.forceFlush()
          : undefined,
      ),
    );

    const spans = exportedSpans(sink.requests).filter((span) =>
      attribute(span.attributes, "url.full")?.toString().startsWith(target.url),
    );

    expect(spans).toHaveLength(4);
  },
);
