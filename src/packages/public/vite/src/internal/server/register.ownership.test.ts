import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import { exceptions, exportedSpans } from "@interfere/test-utils/vitest/otlp";
import { EXCEPTION_DELIVERY_ATTRIBUTE } from "@interfere/types/sdk/exception-delivery";

import { context, metrics, propagation, trace } from "@opentelemetry/api";
import { logs } from "@opentelemetry/api-logs";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import { W3CTraceContextPropagator } from "@opentelemetry/core";
import { UndiciInstrumentation } from "@opentelemetry/instrumentation-undici";
import {
  InMemoryLogRecordExporter,
  LoggerProvider,
  SimpleLogRecordProcessor,
} from "@opentelemetry/sdk-logs";
import {
  BasicTracerProvider,
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import { NodeTracerProvider } from "@opentelemetry/sdk-trace-node";
import { setImmediate } from "node:timers/promises";
import {
  afterAll,
  afterEach,
  beforeAll,
  beforeEach,
  expect,
  test,
  vi,
} from "vitest";
import type { MockInstance } from "vitest";

await import("./register");

await import("./capture");

await import("./runtime");

let collector: Awaited<ReturnType<typeof receiver>>;

let register: (typeof import("./register"))["register"];

let captureError: (typeof import("./capture"))["captureError"];

let registerProvider: MockInstance<
  typeof NodeTracerProvider.prototype.register
>;

let registerLogger: MockInstance<typeof logs.setGlobalLoggerProvider>;

let registerMeter: MockInstance<typeof metrics.setGlobalMeterProvider>;

let registerInstrumentation: MockInstance<
  typeof UndiciInstrumentation.prototype.setTracerProvider
>;

let warn: MockInstance<Console["warn"]>;

const originalConsole = { ...console };

async function load() {
  vi.resetModules();

  ({ register } = await import("./register"));

  ({ captureError } = await import("./capture"));
}

beforeAll(async () => {
  collector = await receiver();
});

beforeEach(async () => {
  vi.clearAllMocks();

  registerProvider = vi.spyOn(NodeTracerProvider.prototype, "register");

  registerLogger = vi.spyOn(logs, "setGlobalLoggerProvider");

  registerMeter = vi.spyOn(metrics, "setGlobalMeterProvider");

  registerInstrumentation = vi.spyOn(
    UndiciInstrumentation.prototype,
    "setTracerProvider",
  );

  warn = vi.spyOn(console, "warn");

  collector.reset();

  vi.stubEnv("INTERFERE_API_KEY", "key+value");

  vi.stubEnv("INTERFERE_API_URL", collector.url);

  vi.stubEnv("INTERFERE_SOURCE_ID", "a".repeat(40));

  vi.stubEnv("OTEL_BSP_SCHEDULE_DELAY", "20");

  await load();
});

afterEach(async () => {
  Object.assign(console, originalConsole);

  await Promise.all(
    registerProvider.mock.contexts.map(async (provider) =>
      provider instanceof NodeTracerProvider ? provider.shutdown() : undefined,
    ),
  );

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

function hostOwnsOpenTelemetry() {
  const exporter = new InMemorySpanExporter();

  const provider = new BasicTracerProvider({
    spanProcessors: [new SimpleSpanProcessor(exporter)],
  });

  trace.setGlobalTracerProvider(provider);

  context.setGlobalContextManager(
    new AsyncLocalStorageContextManager().enable(),
  );

  propagation.setGlobalPropagator(new W3CTraceContextPropagator());

  return { exporter, provider, fields: propagation.fields() };
}

function expectNoGlobalWrites() {
  expect(registerProvider).not.toHaveBeenCalled();
  expect(registerLogger).not.toHaveBeenCalled();
  expect(registerMeter).not.toHaveBeenCalled();
  expect(registerInstrumentation).not.toHaveBeenCalled();
}

test.each([
  {
    tracing: undefined,
    warnings: [
      [
        expect.stringContaining(
          "[interfere] Adopted the OpenTelemetry tracer provider",
        ),
      ],
    ],
  },
  {
    tracing: "auto",
    warnings: [
      [
        expect.stringContaining(
          "[interfere] Adopted the OpenTelemetry tracer provider",
        ),
      ],
    ],
  },
  { tracing: "application", warnings: [] },
] as const)(
  "tracing $tracing adopts the host provider and delivers captured errors through an isolated provider",
  async ({ tracing, warnings }) => {
    const host = hostOwnsOpenTelemetry();
    const loggerBefore = logs.getLoggerProvider();

    await register(omitUndefined({ consoleBridge: false, tracing }));

    await register(omitUndefined({ consoleBridge: false, tracing }));

    expectNoGlobalWrites();

    expect(propagation.fields()).toStrictEqual(host.fields);
    expect(logs.getLoggerProvider()).toBe(loggerBefore);
    expect(warn.mock.calls).toStrictEqual(warnings);

    const { runtime } = await import("./runtime");

    expect(runtime.mode).toBe("application");

    const hostContext = host.provider
      .getTracer("host")
      .startActiveSpan("host-request", (span) => {
        captureError(new Error("boom-in-host"));

        span.end();

        return span.spanContext();
      });

    captureError(new Error("boom-outside"));

    expect(
      host.exporter
        .getFinishedSpans()
        .find((span) => span.name === "host-request")
        ?.events.map((event) => event.attributes?.["exception.message"]),
    ).toStrictEqual(["boom-in-host"]);

    await expect
      .poll(() => exceptions(collector.requests, "boom-outside"), {
        timeout: 5000,
      })
      .toHaveLength(1);

    const [outside] = exceptions(collector.requests, "boom-outside");

    expect(outside?.parentSpanId).toBeUndefined();
    expect(outside?.traceId).not.toBe(hostContext.traceId);

    await expect
      .poll(() => exceptions(collector.requests, "boom-in-host"), {
        timeout: 5000,
      })
      .toHaveLength(1);

    const [inside] = exceptions(collector.requests, "boom-in-host");

    expect(inside?.traceId).not.toBe(hostContext.traceId);

    expect(inside?.links).toMatchObject([
      { traceId: hostContext.traceId, spanId: hostContext.spanId },
    ]);

    expect(
      host.exporter.getFinishedSpans()[0]?.events[0]?.attributes,
    ).toHaveProperty(EXCEPTION_DELIVERY_ATTRIBUTE, "interfere");

    expect(
      exportedSpans(collector.requests).map((span) => span.name),
    ).not.toContain("host-request");
  },
);

test('application mode with errors: "host" keeps the recording host span as the only copy', async () => {
  const host = hostOwnsOpenTelemetry();

  await register({
    consoleBridge: false,
    errors: "host",
    tracing: "application",
  });

  expectNoGlobalWrites();

  host.provider.getTracer("host").startActiveSpan("host-request", (span) => {
    captureError(new Error("host-only"));

    span.end();
  });

  captureError(new Error("outside-host"));

  await expect
    .poll(() => exceptions(collector.requests, "outside-host"), {
      timeout: 5000,
    })
    .toHaveLength(1);

  expect(exceptions(collector.requests, "host-only")).toStrictEqual([]);

  const event = host.exporter.getFinishedSpans()[0]?.events[0];

  expect(event?.attributes).toHaveProperty("exception.message", "host-only");
  expect(event?.attributes).not.toHaveProperty(EXCEPTION_DELIVERY_ATTRIBUTE);
});

test("managed mode next to a registered provider warns once naming application mode and captures through an isolated provider", async () => {
  vi.stubEnv("OTEL_TRACES_SAMPLER", "always_off");

  const host = hostOwnsOpenTelemetry();

  await register({ consoleBridge: false, tracing: "managed" });

  expectNoGlobalWrites();

  expect(warn.mock.calls).toStrictEqual([
    [expect.stringContaining('register({ tracing: "application" })')],
  ]);

  host.provider.getTracer("host").startActiveSpan("host-request", (span) => {
    captureError(new Error("boom-under-foreign"));

    span.end();
  });

  expect(
    host.exporter.getFinishedSpans().flatMap((span) => span.events),
  ).toStrictEqual([]);

  await expect
    .poll(() => exceptions(collector.requests, "boom-under-foreign"), {
      timeout: 5000,
    })
    .toHaveLength(1);
});

test.each([
  {
    consoleBridge: undefined,
    expected: ["debug", "log", "info", "warn", "error"],
  },
  { consoleBridge: true, expected: ["debug", "log", "info", "warn", "error"] },
  { consoleBridge: false, expected: [] },
  { consoleBridge: {}, expected: ["debug", "log", "info", "warn", "error"] },
  {
    consoleBridge: { level: "debug" },
    expected: ["debug", "log", "info", "warn", "error"],
  },
  {
    consoleBridge: { level: "info" },
    expected: ["log", "info", "warn", "error"],
  },
  { consoleBridge: { level: "warn" }, expected: ["warn", "error"] },
  { consoleBridge: { level: "error" }, expected: ["error"] },
] as const)(
  "bridges console into the host logger with $consoleBridge",
  async ({ consoleBridge, expected }) => {
    expect.assertions(8);

    const records = new InMemoryLogRecordExporter();

    const hostLogger = new LoggerProvider({
      processors: [new SimpleLogRecordProcessor({ exporter: records })],
    });

    logs.setGlobalLoggerProvider(hostLogger);

    registerLogger.mockClear();

    try {
      const methods = ["debug", "log", "info", "warn", "error"] as const;
      const originals = { ...console };

      await register(
        omitUndefined({ tracing: "application" as const, consoleBridge }),
      );

      for (const method of methods) {
        expect(console[method] !== originals[method]).toBe(
          expected.some((bridged) => bridged === method),
        );

        console[method](`threshold-${method}`);
      }

      await setImmediate();

      expect(registerLogger).not.toHaveBeenCalled();
      expect(logs.getLoggerProvider()).toBe(hostLogger);

      expect(
        records.getFinishedLogRecords().map((record) => record.body),
      ).toStrictEqual(expected.map((method) => `threshold-${method}`));
    } finally {
      await hostLogger.shutdown();
    }
  },
);

test("a foreign propagator leaves Interfere owning the tracer provider with instrumentation off and one warning", async () => {
  propagation.setGlobalPropagator(new W3CTraceContextPropagator());

  await register({ consoleBridge: false });

  expect(registerProvider).toHaveBeenCalledTimes(1);
  expect(registerMeter).toHaveBeenCalledTimes(1);
  expect(registerInstrumentation).not.toHaveBeenCalled();
  expect(propagation.fields()).toStrictEqual(["traceparent", "tracestate"]);

  expect(warn.mock.calls).toStrictEqual([
    [expect.stringContaining("propagator already owns trace headers")],
  ]);
});
