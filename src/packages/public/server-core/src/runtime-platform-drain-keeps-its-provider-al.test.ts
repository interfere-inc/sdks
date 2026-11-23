import { attribute, exportedSpans } from "@interfere/test-utils/vitest/otlp";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import {
  ROOT_CONTEXT,
  SpanKind,
  context,
  logs,
  propagation,
  trace,
} from "@opentelemetry/api";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import { W3CTraceContextPropagator } from "@opentelemetry/core";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
  TracerProvider,
} from "@opentelemetry/sdk-trace";
import {
  InMemoryLogRecordExporter,
  LoggerProvider,
  SimpleLogRecordProcessor,
} from "@opentelemetry/sdk-logs";
import { afterEach, beforeEach, expect, test, vi } from "vitest";

import { startNodeRuntime } from "./node.js";
import type { NodeRuntimeInput } from "./node.js";
import { registeredTracerProvider } from "./ownership.js";
import { ServerRuntime } from "./runtime.js";

let collector: Awaited<ReturnType<typeof receiver>>;
let runtime: ServerRuntime;
let input: NodeRuntimeInput;
let host: TracerProvider | undefined;
let manager: AsyncLocalStorageContextManager | undefined;
let hostLogger: LoggerProvider | undefined;

beforeEach(async () => {
  collector = await receiver();

  runtime = new ServerRuntime("core-contract");

  input = {
    name: "core-contract",
    version: "1.0.0",
    apiUrl: collector.url,
    credential: { apiKey: "core-contract" },
    resourceAttributes: {},
    options: { consoleBridge: false },
    drain: false,
    logDrain: false,
    vercel: false,
    propagator: new W3CTraceContextPropagator(),
  };
});

afterEach(async () => {
  await runtime.shutdown();

  await host?.shutdown();

  await hostLogger?.shutdown();

  hostLogger = undefined;

  host = undefined;

  manager?.disable();

  manager = undefined;

  trace.disable();

  context.disable();

  propagation.disable();

  logs.disable();

  await collector.close();

  vi.restoreAllMocks();

  vi.unstubAllEnvs();
});

test("platform drain keeps its provider alive after SDK shutdown", async () => {
  await runtime.register(async () =>
    startNodeRuntime({
      ...input,
      drain: true,
      logDrain: true,
      vercel: true,
      registerPlatform: async ({ spanProcessors }) => {
        host = new TracerProvider({ spanProcessors });

        trace.setGlobalTracerProvider(host);

        await host.forceFlush();

        return host;
      },
    }),
  );

  runtime.recordException(new Error("platform capture"), {
    type: "manual.api.capture",
    handled: true,
  });

  await runtime.flush();

  expect(exportedSpans(collector.requests)).toStrictEqual([]);

  await runtime.shutdown();

  expect(registeredTracerProvider()).toBe(host);

  const span = host?.getTracer("platform").startSpan("after SDK shutdown");

  expect(span?.isRecording()).toBe(true);

  span?.end();
});

test("explicit managed mode isolates errors from an existing host", async () => {
  const exporter = new InMemorySpanExporter();

  host = new TracerProvider({
    spanProcessors: [new SimpleSpanProcessor({ exporter })],
  });

  trace.setGlobalTracerProvider(host);

  const warn = vi.spyOn(console, "warn");

  await runtime.register(async () =>
    startNodeRuntime({
      ...input,
      options: { consoleBridge: false, tracing: "managed" },
    }),
  );

  runtime.recordException(new Error("isolated capture"), {
    type: "manual.api.capture",
    handled: true,
  });

  await runtime.flush();

  expect(exportedSpans(collector.requests)).toHaveLength(1);
  expect(exporter.getFinishedSpans()).toStrictEqual([]);

  expect(warn).toHaveBeenCalledWith(
    expect.stringContaining("Another OpenTelemetry tracer provider"),
  );

  await runtime.shutdown();

  expect(registeredTracerProvider()).toBe(host);
});

test("console level and compression apply while preserving a host propagator and context", async () => {
  // oxlint-disable-next-line no-console -- Retain the host method to verify bridge teardown.
  const originalInfo = console.info;

  manager = new AsyncLocalStorageContextManager().enable();

  context.setGlobalContextManager(manager);

  const propagator = new W3CTraceContextPropagator();

  propagation.setGlobalPropagator(propagator);

  await runtime.register(async () =>
    startNodeRuntime({
      ...input,
      options: { compression: "gzip", consoleBridge: { level: "info" } },
    }),
  );

  // oxlint-disable-next-line no-console -- Exercise the configured console bridge and its real exporter.
  console.info("core-console-once");

  // oxlint-disable-next-line no-console -- The configured info threshold must suppress debug telemetry.
  console.debug("core-console-filtered");

  await runtime.flush();

  const requests = collector.requests.filter((request) =>
    request.url.includes("logs"),
  );

  expect(requests).toHaveLength(1);
  expect(requests[0]?.headers.get("content-encoding")).toBe("gzip");
  expect(requests[0]?.body.match(/core-console-once/gu)).toHaveLength(1);
  expect(requests[0]?.body).not.toContain("core-console-filtered");

  await runtime.shutdown();

  // oxlint-disable-next-line no-console -- Verify shutdown restored the actual host method.
  expect(console.info).toBe(originalInfo);
  expect(propagation.fields()).toStrictEqual(propagator.fields());
});

test("application logger remains usable after the SDK releases its console bridge", async () => {
  host = new TracerProvider();

  trace.setGlobalTracerProvider(host);

  const exporter = new InMemoryLogRecordExporter();

  hostLogger = new LoggerProvider({
    processors: [new SimpleLogRecordProcessor({ exporter })],
  });

  logs.setGlobalLoggerProvider(hostLogger);

  await runtime.register(async () =>
    startNodeRuntime({ ...input, options: { consoleBridge: {} } }),
  );

  // oxlint-disable-next-line no-console -- Exercise the default debug threshold through the actual application logger.
  console.debug("host-owned-before");

  await runtime.flush();

  await hostLogger.forceFlush();

  expect(
    exporter.getFinishedLogRecords().map((record) => record.body),
  ).toStrictEqual(["host-owned-before"]);

  expect(
    collector.requests.filter((request) => request.url.includes("logs")),
  ).toStrictEqual([]);

  await runtime.shutdown();

  expect(logs.getLoggerProvider()).toBe(hostLogger);

  hostLogger.getLogger("host").emit({ body: "host-owned-after" });

  await hostLogger.forceFlush();

  expect(
    exporter.getFinishedLogRecords().map((record) => record.body),
  ).toStrictEqual(["host-owned-before", "host-owned-after"]);
});

test("captures normalized strings literally and errors without a stack", async () => {
  await runtime.register(async () => startNodeRuntime(input));

  runtime.recordException(
    {
      kind: "non-error",
      type: "string",
      value: "literal",
      serialized: "literal",
    },
    { type: "manual.api.capture", handled: true },
  );

  const error = new Error("stackless");

  delete error.stack;

  runtime.recordException(error, { type: "manual.api.capture", handled: true });

  await runtime.flush();

  const spans = exportedSpans(collector.requests);

  expect(spans).toHaveLength(2);

  expect(
    attribute(
      spans[0]?.events[0]?.attributes ?? [],
      "interfere.exception.serialized",
    ),
  ).toBe("literal");

  expect(
    attribute(spans[1]?.events[0]?.attributes ?? [], "exception.message"),
  ).toBe("stackless");

  expect(
    attribute(spans[1]?.events[0]?.attributes ?? [], "exception.stacktrace"),
  ).toBeUndefined();
});

test("an exception under an unsampled inbound parent is delivered without sampling the rest of the trace", async () => {
  await runtime.register(async () => startNodeRuntime(input));

  const traceId = "4bf92f3577b34da6a3ce929d0e0e4736";

  const inbound = propagation.extract(ROOT_CONTEXT, {
    traceparent: `00-${traceId}-00f067aa0ba902b7-00`,
  });

  const tracer = trace.getTracer("server");

  context.with(inbound, () => {
    tracer.startActiveSpan(
      "GET /route",
      { kind: SpanKind.SERVER },
      (request) => {
        tracer.startSpan("healthy child").end();

        runtime.recordException(new Error("unsampled route failure"), {
          type: "auto.node.request_error",
          handled: false,
        });

        request.end();
      },
    );
  });

  await runtime.flush();

  const spans = exportedSpans(collector.requests);

  expect(spans).toHaveLength(1);
  expect(spans[0]?.traceId).toBe(traceId);
  expect(spans[0]?.status?.code).toBe(2);

  expect(
    attribute(spans[0]?.events[0]?.attributes ?? [], "exception.message"),
  ).toBe("unsampled route failure");
});

test.each([
  ["parentbased_always_on", undefined],
  ["parentbased_traceidratio", "0"],
  ["traceidratio", "0"],
  ["always_off", undefined],
])(
  "an exception is delivered under the %s sampler while ordinary spans follow it",
  async (sampler, argument) => {
    vi.stubEnv("OTEL_TRACES_SAMPLER", sampler);

    vi.stubEnv("OTEL_TRACES_SAMPLER_ARG", argument);

    await runtime.register(async () => startNodeRuntime(input));

    const inbound = propagation.extract(ROOT_CONTEXT, {
      traceparent: "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-00",
    });

    const tracer = trace.getTracer("server");

    context.with(inbound, () => {
      tracer.startActiveSpan("GET /route", (request) => {
        runtime.recordException(new Error(`failure under ${sampler}`), {
          type: "auto.node.request_error",
          handled: false,
        });

        request.end();
      });
    });

    await runtime.flush();

    expect(
      exportedSpans(collector.requests).map((span) =>
        attribute(span.events[0]?.attributes ?? [], "exception.message"),
      ),
    ).toStrictEqual([`failure under ${sampler}`]);
  },
);
