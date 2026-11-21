import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { EXCEPTION_DELIVERY_ATTRIBUTE } from "@interfere/types/sdk/exception-delivery";
import { resolveUrlRedaction } from "@interfere/types/sdk/url-redaction";
import { SpanStatusCode, context, trace } from "@opentelemetry/api";
import type { Sampler } from "@opentelemetry/sdk-trace";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import {
  AlwaysOffSampler,
  InMemorySpanExporter,
  SimpleSpanProcessor,
  TracerProvider,
} from "@opentelemetry/sdk-trace";
import { afterEach, beforeEach, expect, test, vi } from "vitest";

type Capture = typeof import("./capture.js");

let capture: Capture;

let host: ReturnType<typeof recordingProvider>;

let interfere: ReturnType<typeof recordingProvider>;

const REQUEST = { method: "GET", path: "/api/orders" };

const ROUTE = {
  routePath: "/api/orders",
  routeType: "route",
  routerKind: "App Router",
} as const;

function recordingProvider(sampler?: Sampler) {
  const exporter = new InMemorySpanExporter();

  const provider = new TracerProvider(
    omitUndefined({
      sampler,
      spanProcessors: [new SimpleSpanProcessor({ exporter })],
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

async function raiseInsideHostSpan(message: string) {
  return trace
    .getTracer("host")
    .startActiveSpan("host-render", async (span) => {
      await capture.onRequestError(
        new Error(message, {
          cause: new Error(
            "upstream https://example.com/api?token=secret-value",
          ),
        }),
        REQUEST,
        ROUTE,
      );

      const spanContext = span.spanContext();

      span.end();

      return spanContext;
    });
}

beforeEach(async () => {
  vi.stubEnv("NEXT_PUBLIC_INTERFERE_FORCE_ENABLE", "1");

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

test("application mode records on the host span and delivers a linked copy through Interfere's provider", async () => {
  hostOwnsOpenTelemetry();

  capture.setIsolatedCaptureProvider(interfere.provider, {
    errors: "interfere",
    recordOnHostSpan: true,
    urlRedaction: resolveUrlRedaction(undefined),
  });

  const hostContext = await raiseInsideHostSpan("boom");

  const [hostSpan] = host.exporter.getFinishedSpans();
  const hostEvent = exceptionEvent(host.exporter);

  expect(hostSpan?.status.code).toBe(SpanStatusCode.ERROR);

  expect(hostEvent?.attributes).toMatchObject({
    "exception.message": "boom",
    "http.route": "/api/orders",
    "interfere.exception.mechanism": "auto.function.nextjs.route_handler",
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
  expect(chain).toContain("REDACTED");
  expect(chain).not.toContain("secret-value");
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

test('errors: "host" keeps the host span as the only copy', async () => {
  hostOwnsOpenTelemetry();

  capture.setIsolatedCaptureProvider(interfere.provider, {
    errors: "host",
    recordOnHostSpan: true,
    urlRedaction: resolveUrlRedaction(undefined),
  });

  await raiseInsideHostSpan("boom");

  expect(exceptionEvent(host.exporter)?.attributes).not.toHaveProperty(
    EXCEPTION_DELIVERY_ATTRIBUTE,
  );

  expect(interfere.exporter.getFinishedSpans()).toStrictEqual([]);
});

test("an unsampled host span gets no record and Interfere's copy still links to it", async () => {
  hostOwnsOpenTelemetry(new AlwaysOffSampler());

  capture.setIsolatedCaptureProvider(interfere.provider, {
    errors: "interfere",
    recordOnHostSpan: true,
    urlRedaction: resolveUrlRedaction(undefined),
  });

  const hostContext = await raiseInsideHostSpan("boom");

  expect(host.exporter.getFinishedSpans()).toStrictEqual([]);

  const [delivered] = interfere.exporter.getFinishedSpans();

  expect(exceptionEvent(interfere.exporter)?.attributes).not.toHaveProperty(
    EXCEPTION_DELIVERY_ATTRIBUTE,
  );

  expect(delivered?.links.map((link) => link.context)).toStrictEqual([
    hostContext,
  ]);
});

test("managed mode next to a foreign provider ignores the recording host span", async () => {
  hostOwnsOpenTelemetry();

  capture.setIsolatedCaptureProvider(interfere.provider, {
    errors: "interfere",
    recordOnHostSpan: false,
    urlRedaction: resolveUrlRedaction(undefined),
  });

  const hostContext = await raiseInsideHostSpan("boom");

  expect(
    host.exporter.getFinishedSpans().flatMap((s) => s.events),
  ).toStrictEqual([]);

  const [delivered] = interfere.exporter.getFinishedSpans();

  expect(delivered?.links.map((link) => link.context)).toStrictEqual([
    hostContext,
  ]);
});
