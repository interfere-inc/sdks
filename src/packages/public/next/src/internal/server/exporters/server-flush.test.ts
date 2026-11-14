import { context, trace } from "@opentelemetry/api";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import {
  BasicTracerProvider,
  BatchSpanProcessor,
  InMemorySpanExporter,
} from "@opentelemetry/sdk-trace-base";
import { expect, onTestFinished, test, vi } from "vitest";
import { flushServerTelemetry, setServerFlush } from "./server-flush";

test("flushes only after the request span carrying the capture ends", async () => {
  const manager = new AsyncLocalStorageContextManager().enable();

  context.setGlobalContextManager(manager);

  onTestFinished(() => {
    context.disable();
  });

  const pending = platformWaitUntil();
  const delivered: string[] = [];

  const request = new BasicTracerProvider()
    .getTracer("test")
    .startSpan("GET /failure");

  setServerFlush(async () => {
    await Promise.resolve();

    delivered.push(request.isRecording() ? "open" : "ended");
  });

  await context.with(
    trace.setSpan(context.active(), request),
    flushServerTelemetry,
  );

  expect(pending).toHaveLength(1);

  request.end();

  await Promise.all(pending);

  expect(delivered).toStrictEqual(["ended"]);
});

function platformWaitUntil() {
  const pending: Promise<unknown>[] = [];
  const requestContext = Symbol.for("@next/request-context");

  Object.assign(globalThis, {
    [requestContext]: {
      get: () => ({
        waitUntil: (promise: Promise<unknown>) => {
          pending.push(promise);
        },
      }),
    },
  });

  onTestFinished(() => {
    Reflect.deleteProperty(globalThis, requestContext);
  });

  return pending;
}

test("warns when delivery fails", async () => {
  expect.hasAssertions();

  const failure = new Error("export failed");
  const warn = vi.spyOn(console, "warn");
  const pending = platformWaitUntil();

  setServerFlush(async () => {
    await Promise.reject(failure);
  });

  await flushServerTelemetry();

  await Promise.all(pending);

  expect(warn).toHaveBeenCalledWith(expect.any(String), failure);

  warn.mockRestore();
});

test("returns before a hung delivery settles", async () => {
  const hung = Promise.withResolvers<undefined>();

  setServerFlush(async () => hung.promise);

  const outcome = await Promise.race([
    (async () => {
      await flushServerTelemetry();

      return "returned";
    })(),
    new Promise((resolve) => {
      setImmediate(resolve, "held");
    }),
  ]);

  expect(outcome).toBe("returned");

  hung.resolve(undefined);
});

test("hands the pending flush to the platform waitUntil", async () => {
  const pending = platformWaitUntil();
  const delivered: string[] = [];

  setServerFlush(async () => {
    await Promise.resolve();

    delivered.push("sent");
  });

  await flushServerTelemetry();

  expect(pending).toHaveLength(1);

  await Promise.all(pending);

  expect(delivered).toStrictEqual(["sent"]);
});

test("flushes the global provider from a bundle where register() never ran", async () => {
  vi.resetModules();

  const isolated = await import("./server-flush");

  const pending = platformWaitUntil();

  const exporter = new InMemorySpanExporter();

  const provider = new BasicTracerProvider({
    spanProcessors: [
      new BatchSpanProcessor(exporter, { scheduledDelayMillis: 60_000 }),
    ],
  });

  trace.disable();

  trace.setGlobalTracerProvider(provider);

  onTestFinished(async () => {
    trace.disable();

    await provider.shutdown();
  });

  trace.getTracer("proxy").startSpan("middleware GET").end();

  await isolated.flushServerTelemetry();

  await Promise.all(pending);

  expect(exporter.getFinishedSpans().map((span) => span.name)).toStrictEqual([
    "middleware GET",
  ]);
});
