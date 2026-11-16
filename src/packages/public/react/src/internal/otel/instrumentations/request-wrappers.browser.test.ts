import { LoggerProvider } from "@opentelemetry/sdk-logs";
import {
  InMemorySpanExporter,
  NoopSpanProcessor,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import type { ReadableSpan, Span } from "@opentelemetry/sdk-trace-base";
import { WebTracerProvider } from "@opentelemetry/sdk-trace-web";
import { HttpResponse, http } from "msw";
import { expect, onTestFinished, test } from "vitest";

import { worker } from "../../../../test/msw.js";
import { w3cTraceHeaders } from "../../../../test/trace-headers.test-utils.js";
import { registerBundledInstrumentations } from "./instrumentations.js";

test("one upstream wrapper per request API, inert after dispose, and requests still reach the network", async () => {
  expect.hasAssertions();

  const nativeFetch = globalThis.fetch;
  // oxlint-disable-next-line typescript/unbound-method -- The prototype methods are compared by identity.
  const nativeOpen = XMLHttpRequest.prototype.open;
  // oxlint-disable-next-line typescript/unbound-method -- The prototype methods are compared by identity.
  const nativeSend = XMLHttpRequest.prototype.send;
  // oxlint-disable-next-line typescript/unbound-method -- The prototype methods are compared by identity.
  const nativeSetRequestHeader = XMLHttpRequest.prototype.setRequestHeader;

  const started: string[] = [];

  const provider = new WebTracerProvider({
    spanProcessors: [
      new (class extends NoopSpanProcessor {
        override onStart(span: Span) {
          started.push(span.name);
        }
      })(),
    ],
  });

  onTestFinished(async () => {
    // oxlint-disable-next-line interfere-testing/no-test-doubles -- Restores the native fetch by identity after the inert wrapper.
    globalThis.fetch = nativeFetch;

    XMLHttpRequest.prototype.open = nativeOpen;

    XMLHttpRequest.prototype.send = nativeSend;

    await provider.shutdown();
  });

  const url = `${location.origin}/wrapped`;

  worker.use(http.get(url, () => new HttpResponse(null, { status: 204 })));

  const handle = registerBundledInstrumentations({
    ignoreUrls: [],
    loggerProvider: new LoggerProvider(),
    traceHeaders: w3cTraceHeaders(),
    tracerProvider: provider,
  });

  const wrapped = [
    globalThis.fetch,
    // oxlint-disable-next-line typescript/unbound-method -- The prototype methods are compared by identity.
    XMLHttpRequest.prototype.open,
    // oxlint-disable-next-line typescript/unbound-method -- The prototype methods are compared by identity.
    XMLHttpRequest.prototype.send,
  ];

  expect([
    wrapped[0] === nativeFetch,
    wrapped[1] === nativeOpen,
    wrapped[2] === nativeSend,
    XMLHttpRequest.prototype.setRequestHeader === nativeSetRequestHeader,
  ]).toStrictEqual([false, false, false, true]);

  handle.dispose();

  expect([
    globalThis.fetch,
    // oxlint-disable-next-line typescript/unbound-method -- The prototype methods are compared by identity.
    XMLHttpRequest.prototype.open,
    // oxlint-disable-next-line typescript/unbound-method -- The prototype methods are compared by identity.
    XMLHttpRequest.prototype.send,
  ]).toStrictEqual(wrapped);

  const response = await fetch(url);

  expect([response.status, started]).toStrictEqual([204, []]);
});

function listenerLedger(xhr: XMLHttpRequest) {
  const live = new Set<EventListener>();
  const add = xhr.addEventListener.bind(xhr);
  const remove = xhr.removeEventListener.bind(xhr);

  Object.defineProperty(xhr, "addEventListener", {
    value: (
      type: string,
      listener: EventListener,
      options?: AddEventListenerOptions,
    ) => {
      live.add(listener);

      options?.signal?.addEventListener("abort", () => live.delete(listener));

      add(type, listener, options);
    },
  });

  Object.defineProperty(xhr, "removeEventListener", {
    value: (type: string, listener: EventListener) => {
      live.delete(listener);

      remove(type, listener);
    },
  });

  return { add, live };
}

test("a reused XHR keeps no instrumentation listener once each request span ends", async () => {
  expect.hasAssertions();

  const exporter = new InMemorySpanExporter();
  const ended = Promise.withResolvers<undefined>();
  const url = `${location.origin}/reused-xhr`;

  let endedRequests = 0;

  const provider = new WebTracerProvider({
    spanProcessors: [
      new SimpleSpanProcessor(exporter),
      new (class extends NoopSpanProcessor {
        override onEnd(span: ReadableSpan) {
          if (span.attributes["url.full"] === url) {
            endedRequests += 1;
          }

          if (endedRequests === 3) {
            ended.resolve(undefined);
          }
        }
      })(),
    ],
  });

  const handle = registerBundledInstrumentations({
    ignoreUrls: [],
    traceHeaders: w3cTraceHeaders(),
    tracerProvider: provider,
  });

  onTestFinished(async () => {
    handle.dispose();

    await provider.shutdown();
  });

  worker.use(http.get(url, () => new HttpResponse(null, { status: 204 })));

  const xhr = new XMLHttpRequest();
  const ledger = listenerLedger(xhr);

  for (let send = 0; send < 3; send += 1) {
    // oxlint-disable-next-line eslint/no-await-in-loop -- The same XHR object is reused, so each request must finish before the next open.
    await new Promise<void>((resolve) => {
      ledger.add("loadend", () => resolve(), { once: true });

      xhr.open("GET", url);

      xhr.send();
    });
  }

  await ended.promise;

  expect(ledger.live.size).toBe(0);

  expect(
    exporter
      .getFinishedSpans()
      .filter((span) => span.attributes["url.full"] === url),
  ).toHaveLength(3);
});

test("ending open requests as the page leaves exports a just-completed request span without waiting for upstream's resource-timing delay", async () => {
  expect.hasAssertions();

  const exporter = new InMemorySpanExporter();

  const provider = new WebTracerProvider({
    spanProcessors: [new SimpleSpanProcessor(exporter)],
  });

  const handle = registerBundledInstrumentations({
    ignoreUrls: [],
    traceHeaders: w3cTraceHeaders(),
    tracerProvider: provider,
  });

  onTestFinished(async () => {
    handle.dispose();

    await provider.shutdown();
  });

  const url = `${location.origin}/leaving`;

  worker.use(http.get(url, () => new HttpResponse(null, { status: 204 })));

  const response = await fetch(url);

  const before = exporter.getFinishedSpans().length;

  handle.endOpenRequests();

  expect([
    response.status,
    before,
    exporter.getFinishedSpans().map((span) => span.attributes["url.full"]),
  ]).toStrictEqual([204, 0, [url]]);
});
