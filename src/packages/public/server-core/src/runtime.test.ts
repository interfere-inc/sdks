import { once } from "node:events";
import { createServer } from "node:http";
import type { AddressInfo } from "node:net";
import { attribute, exportedSpans } from "@interfere/test-utils/vitest/otlp";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import { toException } from "@interfere/types/sdk/errors";
import {
  ROOT_CONTEXT,
  SpanKind,
  context,
  propagation,
  trace,
} from "@opentelemetry/api";
import { logs } from "@opentelemetry/api-logs";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import { W3CTraceContextPropagator } from "@opentelemetry/core";
import {
  BasicTracerProvider,
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import {
  InMemoryLogRecordExporter,
  LoggerProvider,
  SimpleLogRecordProcessor,
} from "@opentelemetry/sdk-logs";
import {
  afterEach,
  beforeEach,
  expect,
  onTestFinished,
  test,
  vi,
} from "vitest";

import { startNodeRuntime } from "./node.js";
import type { NodeRuntimeInput } from "./node.js";
import { registeredTracerProvider } from "./ownership.js";
import { ServerRuntime } from "./runtime.js";

let collector: Awaited<ReturnType<typeof receiver>>;
let runtime: ServerRuntime;
let input: NodeRuntimeInput;
let host: BasicTracerProvider | undefined;
let manager: AsyncLocalStorageContextManager | undefined;
let hostLogger: LoggerProvider | undefined;

beforeEach(async () => {
  collector = await receiver();

  runtime = new ServerRuntime("core-contract");

  input = {
    name: "core-contract",
    version: "1.0.0",
    apiUrl: collector.url,
    publicKey: "core-contract",
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

test("registration and flush share startup; shutdown permits a fresh owned pipeline", async () => {
  const gate = Promise.withResolvers<undefined>();

  let starts = 0;

  const start = async () => {
    starts += 1;

    await gate.promise;

    return startNodeRuntime(input);
  };

  const registrations = [runtime.register(start), runtime.register(start)];
  let flushed = false;

  const flushing = (async () => {
    await runtime.flush();

    flushed = true;
  })();

  expect(starts).toBe(1);
  expect(flushed).toBe(false);

  gate.resolve(undefined);

  await Promise.all([...registrations, flushing]);

  expect(flushed).toBe(true);
  expect(runtime.mode).toBe("managed");

  const first = registeredTracerProvider();

  expect(first).not.toBeNull();

  await runtime.register(start);

  expect(starts).toBe(1);

  await runtime.shutdown();

  expect(registeredTracerProvider()).toBeNull();

  await runtime.register(start);

  expect(starts).toBe(2);
  expect(registeredTracerProvider()).not.toBe(first);
});

test("startup failure releases owned globals and can register again", async () => {
  const warn = vi.spyOn(console, "warn");

  await runtime.register(async () =>
    startNodeRuntime({
      ...input,
      instrument: () => {
        throw new Error("instrumentation startup failed");
      },
    }),
  );

  expect(registeredTracerProvider()).toBeNull();

  expect(warn).toHaveBeenCalledWith(
    expect.stringContaining("register()"),
    expect.any(Error),
  );

  await runtime.register(async () => startNodeRuntime(input));

  runtime.recordException(new Error("recovered capture"), {
    type: "manual.api.capture",
    handled: true,
  });

  await runtime.flush();

  expect(exportedSpans(collector.requests)).toHaveLength(1);
});

test.each([
  ["interfere", 1],
  ["host", 0],
] as const)(
  "application ownership survives capture delivery through %s",
  async (errors, copies) => {
    const exporter = new InMemorySpanExporter();

    host = new BasicTracerProvider({
      spanProcessors: [new SimpleSpanProcessor(exporter)],
    });

    trace.setGlobalTracerProvider(host);

    manager = new AsyncLocalStorageContextManager().enable();

    context.setGlobalContextManager(manager);

    const propagator = new W3CTraceContextPropagator();

    propagation.setGlobalPropagator(propagator);

    await runtime.register(async () =>
      startNodeRuntime({ ...input, options: { consoleBridge: false, errors } }),
    );

    expect(runtime.mode).toBe("application");

    host.getTracer("host").startActiveSpan("request", (span) => {
      runtime.recordException(
        new Error("failed https://service.example/?token=private-secret"),
        { type: "manual.api.capture", handled: false },
      );

      span.end();
    });

    await runtime.flush();

    const hostSpans = exporter.getFinishedSpans();

    expect(hostSpans).toHaveLength(1);
    expect(hostSpans[0]?.events).toHaveLength(1);
    expect(hostSpans[0]?.status.code).toBe(2);

    expect(JSON.stringify(hostSpans.map((span) => span.events))).not.toContain(
      "private-secret",
    );

    const delivered = exportedSpans(collector.requests);

    expect(delivered).toHaveLength(copies);

    expect(delivered.map((span) => span.links[0]?.spanId)).toStrictEqual(
      Array.from({ length: copies }, () => hostSpans[0]?.spanContext().spanId),
    );

    await runtime.shutdown();

    expect(registeredTracerProvider()).toBe(host);
    expect(propagation.fields()).toStrictEqual(propagator.fields());
  },
);

test.each([
  [undefined, "undefined"],
  [401, "401"],
  [{ code: 401 }, '{"code":401}'],
] as const)(
  "preserves normalized non-error payload %j",
  async (value, serialized) => {
    const pipeline = await startNodeRuntime(input);

    runtime.configureCapture(pipeline.provider, pipeline.capture);

    await runtime.register(async () => {
      await pipeline.flush();

      return pipeline;
    });

    runtime.recordException(toException(value), {
      type: "auto.node.global_handlers.unhandled_rejection",
      handled: false,
    });

    await runtime.flush();

    const spans = exportedSpans(collector.requests);

    expect(spans).toHaveLength(1);
    expect(spans[0]?.status?.code).toBe(2);

    expect(
      attribute(
        spans[0]?.events[0]?.attributes ?? [],
        "interfere.exception.serialized",
      ),
    ).toBe(serialized);
  },
);

test("disabled registration does not consume the initialization slot", async () => {
  await runtime.register(async () => {
    await Promise.resolve();

    return null;
  });

  await runtime.register(async () => startNodeRuntime(input));

  runtime.recordException(new Error("enabled later"), {
    type: "manual.api.capture",
    handled: true,
  });

  await runtime.flush();

  expect(exportedSpans(collector.requests)).toHaveLength(1);
});

test("platform drain keeps its provider alive after SDK shutdown", async () => {
  await runtime.register(async () =>
    startNodeRuntime({
      ...input,
      drain: true,
      logDrain: true,
      vercel: true,
      registerPlatform: async ({ spanProcessors }) => {
        host = new BasicTracerProvider({ spanProcessors });

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

  host = new BasicTracerProvider({
    spanProcessors: [new SimpleSpanProcessor(exporter)],
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
  host = new BasicTracerProvider();

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

test("a multi-megabyte exception is truncated so its batch stays under the gateway cap with its siblings", async () => {
  await runtime.register(async () => startNodeRuntime(input));

  const tracer = trace.getTracer("server");

  tracer.startActiveSpan("GET /route", (request) => {
    tracer.startSpan("sibling", { attributes: { ok: true } }).end();

    runtime.recordException(new Error("x".repeat(9 * 1024 * 1024)), {
      type: "auto.node.request_error",
      handled: false,
    });

    request.end();
  });

  await runtime.flush();

  const traces = collector.requests.filter((request) =>
    request.url.startsWith("/drain/v1/traces"),
  );

  expect(traces.every((request) => request.body.length < 8 * 1024 * 1024)).toBe(
    true,
  );

  const spans = exportedSpans(collector.requests);

  expect(spans.map((span) => span.name).toSorted()).toStrictEqual([
    "GET /route",
    "sibling",
  ]);

  const message = attribute(
    spans.find((span) => span.name === "GET /route")?.events[0]?.attributes ??
      [],
    "exception.message",
  );

  expect(String(message)).toMatch(/^x{1000,65536}$/u);
});

test("a 429 with Retry-After is retried and the batch is delivered exactly once", async () => {
  const statuses: number[] = [];

  const bodies: string[] = [];

  const server = createServer((request, response) => {
    const chunks: Buffer[] = [];

    request.on("data", (chunk: Buffer) => chunks.push(chunk));

    request.on("end", () => {
      const status = statuses.length === 0 ? 429 : 200;

      statuses.push(status);

      bodies.push(Buffer.concat(chunks).toString());

      response.writeHead(
        status,
        status === 429
          ? { "content-type": "application/json", "retry-after": "1" }
          : { "content-type": "application/json" },
      );

      response.end("{}");
    });
  });

  server.listen(0, "127.0.0.1");

  await once(server, "listening");

  onTestFinished(async () => {
    server.closeAllConnections();

    await new Promise((resolve) => {
      server.close(resolve);
    });
  });

  const { port } = server.address() as AddressInfo;

  await runtime.register(async () =>
    startNodeRuntime({
      ...input,
      apiUrl: `http://127.0.0.1:${port}`,
    }),
  );

  runtime.recordException(new Error("throttled once"), {
    type: "manual.api.capture",
    handled: true,
  });

  await runtime.flush();

  expect(statuses).toStrictEqual([429, 200]);

  expect(bodies.filter((body) => body.includes("throttled once"))).toHaveLength(
    2,
  );
});

test("a flush waits for an export an earlier flush already has in flight", async () => {
  const held: (() => void)[] = [];

  const bodies: string[] = [];

  const server = createServer((request, response) => {
    const chunks: Buffer[] = [];

    request.on("data", (chunk: Buffer) => chunks.push(chunk));

    request.on("end", () => {
      bodies.push(Buffer.concat(chunks).toString());

      held.push(() => {
        response.writeHead(200, { "content-type": "application/json" });

        response.end("{}");
      });
    });
  });

  server.listen(0, "127.0.0.1");

  await once(server, "listening");

  onTestFinished(async () => {
    server.closeAllConnections();

    await new Promise((resolve) => {
      server.close(resolve);
    });
  });

  const { port } = server.address() as AddressInfo;

  await runtime.register(async () =>
    startNodeRuntime({
      ...input,
      apiUrl: `http://127.0.0.1:${port}`,
    }),
  );

  runtime.recordException(new Error("delivered in the background"), {
    type: "manual.api.capture",
    handled: true,
  });

  let backgroundDone = false;

  const background = (async () => {
    await runtime.flush();

    backgroundDone = true;
  })();

  await vi.waitUntil(() => held.length > 0);

  const explicit = runtime.flush();

  for (const release of held) {
    release();
  }

  await explicit;

  expect(backgroundDone).toBe(true);

  await background;

  expect(
    bodies.filter((body) => body.includes("delivered in the background")),
  ).toHaveLength(1);
});
