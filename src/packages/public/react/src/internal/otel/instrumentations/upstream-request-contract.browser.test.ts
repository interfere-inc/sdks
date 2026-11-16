import { ROOT_CONTEXT, context, propagation, trace } from "@opentelemetry/api";
import { W3CTraceContextPropagator } from "@opentelemetry/core";
import { registerInstrumentations } from "@opentelemetry/instrumentation";
import { FetchInstrumentation } from "@opentelemetry/instrumentation-fetch";
import { XMLHttpRequestInstrumentation } from "@opentelemetry/instrumentation-xml-http-request";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import {
  StackContextManager,
  WebTracerProvider,
} from "@opentelemetry/sdk-trace-web";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, expect, onTestFinished, test } from "vitest";

import { worker } from "../../../../test/msw.js";

const URL_ECHO = `${location.origin}/upstream-contract`;

const nativeFetch = globalThis.fetch;
// oxlint-disable-next-line typescript/unbound-method -- The native functions are saved to restore them by identity.
const nativeOpen = XMLHttpRequest.prototype.open;
// oxlint-disable-next-line typescript/unbound-method -- The native functions are saved to restore them by identity.
const nativeSend = XMLHttpRequest.prototype.send;

let wire: { baggage: string | null; traceparent: string | null }[];

beforeEach(() => {
  wire = [];

  worker.use(
    http.get(URL_ECHO, ({ request }) => {
      wire.push({
        baggage: request.headers.get("baggage"),
        traceparent: request.headers.get("traceparent"),
      });

      return new HttpResponse(null, { status: 204 });
    }),
  );
});

afterEach(() => {
  context.disable();

  propagation.disable();

  trace.disable();

  // oxlint-disable-next-line interfere-testing/no-test-doubles -- Restores the real native fetch by identity.
  globalThis.fetch = nativeFetch;

  XMLHttpRequest.prototype.open = nativeOpen;

  XMLHttpRequest.prototype.send = nativeSend;
});

function provider() {
  const exporter = new InMemorySpanExporter();

  const tracerProvider = new WebTracerProvider({
    spanProcessors: [new SimpleSpanProcessor(exporter)],
  });

  onTestFinished(async () => tracerProvider.shutdown());

  return { exporter, tracerProvider };
}

async function sendXhr() {
  return new Promise<void>((resolve) => {
    const xhr = new XMLHttpRequest();

    xhr.addEventListener("loadend", () => resolve(), { once: true });

    xhr.open("GET", URL_ECHO);

    xhr.send();
  });
}

async function settle() {
  await new Promise<void>((resolve) => {
    setTimeout(resolve, 350);
  });
}

test("a second fetch instrumentation wraps on top of the first instead of unwrapping it, and each provider records one span per request", async () => {
  const foreign = provider();
  const ours = provider();

  const unregisterForeign = registerInstrumentations({
    instrumentations: [new FetchInstrumentation()],
    tracerProvider: foreign.tracerProvider,
  });

  const foreignWrapper = globalThis.fetch;

  const unregisterOurs = registerInstrumentations({
    instrumentations: [new FetchInstrumentation()],
    tracerProvider: ours.tracerProvider,
  });

  expect(globalThis.fetch).not.toBe(foreignWrapper);

  await fetch(URL_ECHO);

  await settle();

  expect([
    foreign.exporter.getFinishedSpans().length,
    ours.exporter.getFinishedSpans().length,
  ]).toStrictEqual([1, 1]);

  unregisterOurs();

  await fetch(URL_ECHO);

  await settle();

  expect([
    foreign.exporter.getFinishedSpans().length,
    ours.exporter.getFinishedSpans().length,
  ]).toStrictEqual([2, 1]);

  unregisterForeign();
});

test("requestHook runs before injection, and with no global propagator the instrumentation writes nothing over the hook's headers", async () => {
  const ours = provider();

  registerInstrumentations({
    instrumentations: [
      new FetchInstrumentation({
        requestHook: (span, request) => {
          const { traceId, spanId } = span.spanContext();

          if (request instanceof Request) {
            request.headers.set("traceparent", `00-${traceId}-${spanId}-01`);
          } else {
            const headers = new Headers(request.headers);

            headers.set("traceparent", `00-${traceId}-${spanId}-01`);

            headers.append("baggage", "interfere.session.id=s1");

            request.headers = headers;
          }
        },
      }),
    ],
    tracerProvider: ours.tracerProvider,
  });

  await fetch(URL_ECHO);

  await settle();

  const [span] = ours.exporter.getFinishedSpans();

  expect(wire).toStrictEqual([
    {
      baggage: "interfere.session.id=s1",
      traceparent: `00-${span?.spanContext().traceId}-${span?.spanContext().spanId}-01`,
    },
  ]);
});

test("a foreign global W3C propagator keeps the hook's traceparent without a context manager and rewrites it to the instrumentation's own span with one", async () => {
  const ours = provider();

  propagation.setGlobalPropagator(new W3CTraceContextPropagator());

  registerInstrumentations({
    instrumentations: [
      new FetchInstrumentation({
        requestHook: (_span, request) => {
          if (request instanceof Request) {
            return;
          }

          const headers = new Headers(request.headers);

          headers.set(
            "traceparent",
            `00-${"1".repeat(32)}-${"2".repeat(16)}-01`,
          );

          request.headers = headers;
        },
      }),
    ],
    tracerProvider: ours.tracerProvider,
  });

  await fetch(URL_ECHO);

  context.setGlobalContextManager(new StackContextManager().enable());

  await context.with(ROOT_CONTEXT, async () => fetch(URL_ECHO));

  await settle();

  const spans = ours.exporter.getFinishedSpans();

  expect(wire.map((entry) => entry.traceparent?.split("-")[1])).toStrictEqual([
    "1".repeat(32),
    spans[1]?.spanContext().traceId,
  ]);
});

test("the XHR instrumentation has no request hook: without a global propagator it writes no header, so a header policy needs another path", async () => {
  const ours = provider();

  registerInstrumentations({
    instrumentations: [new XMLHttpRequestInstrumentation()],
    tracerProvider: ours.tracerProvider,
  });

  await sendXhr();

  await settle();

  expect([wire, ours.exporter.getFinishedSpans().length]).toStrictEqual([
    [{ baggage: null, traceparent: null }],
    1,
  ]);
});
