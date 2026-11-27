/* oxlint-disable typescript/no-unsafe-assignment */
import { SpanKind, SpanStatusCode } from "@opentelemetry/api";
import type { ExportResult } from "@opentelemetry/core";
import { resourceFromAttributes } from "@opentelemetry/resources";
import type { ReadableSpan } from "@opentelemetry/sdk-trace";

import { createTraceExporter } from "./exporter.js";
import { beforeEach, describe, expect, test, vi } from "vitest";
import { HttpResponse, http } from "msw";
import type { HttpResponseResolver } from "msw";

import { createCountSignal } from "../../../test/fixtures/upload-signal.js";
import { worker } from "../../../test/msw.js";

const requests: Request[] = [];

const requestSignal = createCountSignal(requests);

function resultCallback() {
  const called = Promise.withResolvers<true>();
  const spy = vi.fn<(result: ExportResult) => void>();

  const callback = (result: ExportResult) => {
    spy(result);

    called.resolve(true);
  };

  return { callback, spy: Object.assign(spy, { called: called.promise }) };
}

function collector(respond: HttpResponseResolver) {
  return http.post("https://in.example.com/drain/v1/:signal", async (info) => {
    requests.push(info.request.clone());

    requestSignal.notify();

    return respond(info);
  });
}

const CLIENT_OPTIONS = {
  baseUrl: "https://in.example.com",
  releaseTokenHeader: { apiKey: "test-key" },
};

function exporterUnderTest() {
  return createTraceExporter({ clientOptions: CLIENT_OPTIONS });
}

function exportSpan(exporter: ReturnType<typeof exporterUnderTest>) {
  const { callback, spy } = resultCallback();

  exporter.export([makeSpan()], callback);

  return spy;
}

function incompressible(length: number) {
  return btoa(
    String.fromCodePoint(...crypto.getRandomValues(new Uint8Array(length))),
  ).slice(0, length);
}

function makeSpan() {
  return {
    attributes: {},
    droppedAttributesCount: 0,
    droppedEventsCount: 0,
    droppedLinksCount: 0,
    duration: [0, 0],
    endTime: [0, 0],
    ended: true,
    events: [],
    instrumentationScope: { name: "test" },
    kind: SpanKind.INTERNAL,
    links: [],
    name: "test-span",
    resource: resourceFromAttributes({}),
    spanContext: () => ({
      traceId: "a".repeat(32),
      spanId: "b".repeat(16),
      traceFlags: 1,
    }),
    startTime: [0, 0],
    status: { code: SpanStatusCode.UNSET },
  } satisfies ReadableSpan;
}

describe(createTraceExporter, () => {
  beforeEach(() => {
    requests.length = 0;

    worker.use(collector(() => new HttpResponse(null)));
  });

  test("sends nothing after a network failure, confirmed without keepalive, until the backoff expires", async () => {
    expect.hasAssertions();

    vi.useFakeTimers({ toFake: ["Date"] });

    onTestFinished(() => {
      vi.useRealTimers();
    });

    worker.use(
      collector(() =>
        requests.length <= 2 ? HttpResponse.error() : new HttpResponse(null),
      ),
    );

    const exporter = exporterUnderTest();

    const blocked = exportSpan(exporter);

    await blocked.called;

    const skipped = exportSpan(exporter);

    await skipped.called;

    expect(skipped).toHaveBeenCalledWith(expect.objectContaining({ code: 1 }));

    expect(requests.map((request) => request.keepalive)).toStrictEqual([
      true,
      false,
    ]);

    vi.setSystemTime(Date.now() + 5000);

    const resumed = exportSpan(exporter);

    await resumed.called;

    expect(resumed).toHaveBeenCalledWith({ code: 0 });
    expect(requests).toHaveLength(3);
  });

  test("reports FAILED when the collector permanently rejects the payload", async () => {
    expect.hasAssertions();

    worker.use(collector(() => new HttpResponse(null, { status: 400 })));

    const callback = exportSpan(exporterUnderTest());

    await callback.called;

    expect(callback).toHaveBeenCalledWith({
      code: 1,
      error: expect.objectContaining({
        statusCode: 400,
      }),
    });
  });

  test("treats the service worker's synthetic 202 Queued as delivered", async () => {
    expect.hasAssertions();

    worker.use(
      collector(
        () => new HttpResponse(null, { status: 202, statusText: "Queued" }),
      ),
    );

    const callback = exportSpan(exporterUnderTest());

    await callback.called;

    expect(callback).toHaveBeenCalledWith({ code: 0 });
  });

  test.for(["forceFlush", "shutdown"] as const)(
    "%s waits for an export that is still in flight",
    async (method) => {
      expect.hasAssertions();

      const response = Promise.withResolvers<null>();

      onTestFinished(() => response.resolve(null));

      worker.use(
        collector(async () => {
          await response.promise;

          return new HttpResponse(null);
        }),
      );

      const exporter = exporterUnderTest();
      const { callback, spy } = resultCallback();

      exporter.export([makeSpan()], callback);

      let settled = false;

      const flushing = (async () => {
        await exporter[method]();

        settled = true;
      })();

      await requestSignal.reached(1);

      expect(settled).toBe(false);

      response.resolve(null);

      await flushing;

      expect(spy).toHaveBeenCalledWith({ code: 0 });
    },
  );

  test("keeps overlapping exports within half the browser keepalive quota and sends the rest without keepalive", async () => {
    expect.hasAssertions();

    const response = Promise.withResolvers<null>();

    onTestFinished(() => response.resolve(null));

    worker.use(
      collector(async () => {
        await response.promise;

        return new HttpResponse(null);
      }),
    );

    const exporter = exporterUnderTest();

    const callbacks = [1, 2, 3].map(() => {
      const { callback, spy } = resultCallback();

      exporter.export(
        [{ ...makeSpan(), attributes: { payload: incompressible(25_000) } }],
        callback,
      );

      return spy;
    });

    await requestSignal.reached(3);

    expect(requests).toHaveLength(3);

    expect(requests.filter((request) => request.keepalive)).toHaveLength(1);

    response.resolve(null);

    await Promise.all(callbacks.map(async (callback) => callback.called));

    for (const callback of callbacks) {
      expect(callback).toHaveBeenCalledWith({ code: 0 });
    }

    const recovered = exportSpan(exporter);

    await requestSignal.reached(4);

    expect(requests).toHaveLength(4);

    expect(requests[3]?.keepalive).toBe(true);

    await recovered.called;

    expect(recovered).toHaveBeenCalledWith({ code: 0 });
  });
});
