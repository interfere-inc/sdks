import { attribute, exportedSpans } from "@interfere/test-utils/vitest/otlp";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import { toException } from "@interfere/types/sdk/errors";
import { context, logs, propagation, trace } from "@opentelemetry/api";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import { W3CTraceContextPropagator } from "@opentelemetry/core";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
  TracerProvider,
} from "@opentelemetry/sdk-trace";
import type { LoggerProvider } from "@opentelemetry/sdk-logs";
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

test("trace and log exports carry the workspace API key as a bearer token and no query credential", async () => {
  await runtime.register(async () =>
    startNodeRuntime({
      ...input,
      options: { consoleBridge: { level: "info" } },
    }),
  );

  runtime.recordException(new Error("keyed capture"), {
    type: "manual.api.capture",
    handled: true,
  });

  // oxlint-disable-next-line no-console -- Exercise the real log exporter through the console bridge.
  console.info("keyed-log");

  await runtime.flush();

  const exports = collector.requests.filter((request) =>
    request.url.startsWith("/drain/v1/"),
  );

  expect(exports.map((request) => request.url).toSorted()).toStrictEqual([
    "/drain/v1/logs",
    "/drain/v1/traces",
  ]);

  expect(
    exports.map((request) => request.headers.get("authorization")),
  ).toStrictEqual(["Bearer core-contract", "Bearer core-contract"]);
});

test("a public key alias travels in its header and never in the query", async () => {
  await runtime.register(async () =>
    startNodeRuntime({ ...input, credential: { publicKey: "core-alias" } }),
  );

  runtime.recordException(new Error("alias capture"), {
    type: "manual.api.capture",
    handled: true,
  });

  await runtime.flush();

  const exported = collector.requests.find((request) =>
    request.url.startsWith("/drain/v1/traces"),
  );

  expect(exported?.url).toBe("/drain/v1/traces");
  expect(exported?.headers.get("x-interfere-public-key")).toBe("core-alias");
  expect(exported?.headers.has("authorization")).toBe(false);
});

test.each([
  ["interfere", 1],
  ["host", 0],
] as const)(
  "application ownership survives capture delivery through %s",
  async (errors, copies) => {
    const exporter = new InMemorySpanExporter();

    host = new TracerProvider({
      spanProcessors: [new SimpleSpanProcessor({ exporter })],
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
