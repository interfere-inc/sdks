import { LoggerProvider } from "@opentelemetry/sdk-logs";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import { WebTracerProvider } from "@opentelemetry/sdk-trace-web";
import { HttpResponse, http } from "msw";
import { expect, onTestFinished, test } from "vitest";

import { worker } from "../../../../test/msw.js";
import { w3cTraceHeaders } from "../../../../test/trace-headers.test-utils.js";
import { registerBundledInstrumentations } from "./instrumentations.js";

function countWrites<T extends object, K extends keyof T>(target: T, key: K) {
  const descriptor = Object.getOwnPropertyDescriptor(target, key);
  let current = target[key];
  let count = 0;

  Object.defineProperty(target, key, {
    configurable: true,
    enumerable: descriptor?.enumerable ?? false,
    get: () => current,
    set: (next: T[K]) => {
      count += 1;

      current = next;
    },
  });

  return {
    count: () => count,
    restore: () => {
      Object.defineProperty(target, key, { ...descriptor, value: current });
    },
    value: () => current,
  };
}

test.each([
  { mode: "managed" as const, writer: true },
  { mode: "managed" as const, writer: false },
  { mode: "application" as const, writer: true },
])(
  "$mode (writer $writer): one fetch wrapper and one XHR wrapper, each restored on dispose",
  async ({ mode, writer }) => {
    expect.hasAssertions();

    const writes = [
      countWrites(globalThis, "fetch"),
      countWrites(XMLHttpRequest.prototype, "open"),
      countWrites(XMLHttpRequest.prototype, "setRequestHeader"),
      countWrites(XMLHttpRequest.prototype, "send"),
    ] as const;

    const natives = writes.map((write) => write.value());
    const provider = new WebTracerProvider();

    try {
      const handle = registerBundledInstrumentations({
        captureFailedRequests: {
          applicationErrors: true,
          statusCodes: [[400, 599]],
        },
        ignoreUrls: [],
        loggerProvider: new LoggerProvider(),
        mode,
        traceHeaders: writer ? w3cTraceHeaders() : null,
        tracerProvider: provider,
      });

      handle.setPageEventsEnabled(true);

      expect(writes.map((write) => write.count())).toStrictEqual([1, 1, 1, 1]);

      handle.dispose();

      expect(writes.map((write) => write.count())).toStrictEqual([2, 2, 2, 2]);

      expect(
        writes.map((write, index) => write.value() === natives[index]),
      ).toStrictEqual([true, true, true, true]);
    } finally {
      for (const write of writes) {
        write.restore();
      }

      await provider.shutdown();
    }
  },
);

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

test("a reused XHR keeps no Interfere listener once each request completes", async () => {
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

  const url = `${location.origin}/reused-xhr`;

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

  expect(ledger.live.size).toBe(0);

  expect(
    exporter
      .getFinishedSpans()
      .filter((span) => span.attributes["url.full"] === url),
  ).toHaveLength(3);
});
