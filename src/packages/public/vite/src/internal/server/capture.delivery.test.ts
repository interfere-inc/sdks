import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { EXCEPTION_DELIVERY_ATTRIBUTE } from "@interfere/types/sdk/exception-delivery";
import { SpanStatusCode, context, trace } from "@opentelemetry/api";
import type { Sampler } from "@opentelemetry/sdk-trace-base";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import {
  AlwaysOffSampler,
  BasicTracerProvider,
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import { afterEach, beforeEach, expect, test, vi } from "vitest";

type Capture = typeof import("./capture.js");

await import("./capture.js");

let capture: Capture;

let host: ReturnType<typeof recordingProvider>;

let interfere: ReturnType<typeof recordingProvider>;

const REQUEST = { method: "GET", path: "/api/orders" };

function recordingProvider(sampler?: Sampler) {
  const exporter = new InMemorySpanExporter();

  const provider = new BasicTracerProvider(
    omitUndefined({
      sampler,
      spanProcessors: [new SimpleSpanProcessor(exporter)],
    }),
  );

  return { exporter, provider };
}

function hostOwnsOpenTelemetry(sampler?: Sampler) {
  host = recordingProvider(sampler);

  trace.setGlobalTracerProvider(host.provider);
}

function exceptionEvent(exporter: InMemorySpanExporter) {
  const events = exporter
    .getFinishedSpans()
    .flatMap((span) => span.events.filter((e) => e.name === "exception"));

  expect(events).toHaveLength(1);

  return events[0];
}

function raiseInsideHostSpan(message: string) {
  return trace.getTracer("host").startActiveSpan("host-render", (span) => {
    capture.captureError(
      new Error(message, { cause: new Error("upstream failed") }),
      {
        request: REQUEST,
        mechanism: { handled: false, type: "auto.function.vite.request" },
      },
    );

    const spanContext = span.spanContext();

    span.end();

    return spanContext;
  });
}

beforeEach(async () => {
  vi.stubEnv("INTERFERE_API_KEY", "interfere_pk_test_123");

  vi.resetModules();

  capture = await import("./capture.js");

  interfere = recordingProvider();

  trace.disable();

  context.disable();

  context.setGlobalContextManager(
    new AsyncLocalStorageContextManager().enable(),
  );
});

afterEach(async () => {
  context.disable();

  trace.disable();

  await host.provider.shutdown();

  await interfere.provider.shutdown();

  vi.unstubAllEnvs();
});

test("application mode records on the host span and delivers a linked copy through Interfere's provider", () => {
  hostOwnsOpenTelemetry();

  capture.setIsolatedCaptureProvider(interfere.provider, {
    errors: "interfere",
    recordOnHostSpan: true,
  });

  const hostContext = raiseInsideHostSpan("boom");

  const [hostSpan] = host.exporter.getFinishedSpans();
  const hostEvent = exceptionEvent(host.exporter);

  expect(hostSpan?.status.code).toBe(SpanStatusCode.ERROR);

  expect(hostEvent?.attributes).toMatchObject({
    "exception.message": "boom",
    "http.request.method": "GET",
    "url.path": "/api/orders",
    "interfere.exception.mechanism": "auto.function.vite.request",
    [EXCEPTION_DELIVERY_ATTRIBUTE]: "interfere",
  });

  const [delivered] = interfere.exporter.getFinishedSpans();
  const deliveredEvent = exceptionEvent(interfere.exporter);

  const { [EXCEPTION_DELIVERY_ATTRIBUTE]: marker, ...recorded } =
    hostEvent?.attributes ?? {};

  expect(marker).toBe("interfere");
  expect(deliveredEvent?.attributes).toStrictEqual(recorded);

  const chain = String(
    deliveredEvent?.attributes?.["interfere.exception.chain"],
  );

  expect(JSON.parse(chain)).toHaveLength(2);
  expect(chain).toContain("upstream failed");
  expect(deliveredEvent?.time).toStrictEqual(hostEvent?.time);
  expect(delivered?.status.code).toBe(SpanStatusCode.ERROR);
  expect(delivered?.spanContext().traceId).not.toBe(hostContext.traceId);

  expect(delivered?.attributes).toMatchObject({
    "interfere.capture.kind": "exception",
  });

  expect(delivered?.links.map((link) => link.context)).toStrictEqual([
    hostContext,
  ]);
});

test('errors: "host" keeps the host span as the only copy', () => {
  hostOwnsOpenTelemetry();

  capture.setIsolatedCaptureProvider(interfere.provider, {
    errors: "host",
    recordOnHostSpan: true,
  });

  raiseInsideHostSpan("boom");

  expect(exceptionEvent(host.exporter)?.attributes).not.toHaveProperty(
    EXCEPTION_DELIVERY_ATTRIBUTE,
  );

  expect(interfere.exporter.getFinishedSpans()).toStrictEqual([]);
});

test("an unsampled host span gets no record and Interfere's copy still links to it", () => {
  hostOwnsOpenTelemetry(new AlwaysOffSampler());

  capture.setIsolatedCaptureProvider(interfere.provider, {
    errors: "interfere",
    recordOnHostSpan: true,
  });

  const hostContext = raiseInsideHostSpan("boom");

  expect(host.exporter.getFinishedSpans()).toStrictEqual([]);

  const [delivered] = interfere.exporter.getFinishedSpans();

  expect(exceptionEvent(interfere.exporter)?.attributes).not.toHaveProperty(
    EXCEPTION_DELIVERY_ATTRIBUTE,
  );

  expect(delivered?.links.map((link) => link.context)).toStrictEqual([
    hostContext,
  ]);
});

test("managed mode next to a foreign provider ignores the recording host span", () => {
  hostOwnsOpenTelemetry();

  capture.setIsolatedCaptureProvider(interfere.provider, {
    errors: "interfere",
    recordOnHostSpan: false,
  });

  const hostContext = raiseInsideHostSpan("boom");

  expect(
    host.exporter.getFinishedSpans().flatMap((s) => s.events),
  ).toStrictEqual([]);

  const [delivered] = interfere.exporter.getFinishedSpans();

  expect(delivered?.links.map((link) => link.context)).toStrictEqual([
    hostContext,
  ]);
});
