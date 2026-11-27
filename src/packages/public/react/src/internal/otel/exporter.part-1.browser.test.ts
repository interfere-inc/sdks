/* oxlint-disable typescript/no-unsafe-assignment */
import { SpanKind, SpanStatusCode } from "@opentelemetry/api";
import type { ExportResult } from "@opentelemetry/core";
import { resourceFromAttributes } from "@opentelemetry/resources";
import type { ResourceMetrics } from "@opentelemetry/sdk-metrics";
import type { ReadableSpan } from "@opentelemetry/sdk-trace";

import { PRODUCER_VERSION } from "../version.js";
import { createMetricExporter, createTraceExporter } from "./exporter.js";
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

  test("dispatches via a keepalive fetch that never attaches credentials", async () => {
    expect.hasAssertions();

    const callback = exportSpan(exporterUnderTest());

    await requestSignal.reached(1);

    expect(requests).toHaveLength(1);

    const [request] = requests;

    if (!request) {
      throw new Error("Expected an exported request");
    }

    const { url } = request;

    expect(url).toContain("/drain/v1/traces");
    expect(new URL(url).searchParams.has("_pv")).toBe(false);

    expect(request.headers.get("x-interfere-producer-version")).toBe(
      PRODUCER_VERSION,
    );

    expect(request.credentials).toBe("omit");
    expect(request.keepalive).toBe(true);
    expect(request.method).toBe("POST");

    await callback.called;

    expect(callback).toHaveBeenCalledWith({ code: 0 });
  });

  test.each([429, 503])(
    "retries %s without reporting success before acceptance",
    async (status) => {
      expect.hasAssertions();

      worker.use(
        collector(() =>
          requests.length === 1
            ? new HttpResponse(null, {
                status,
                headers: { "Retry-After": "0" },
              })
            : new HttpResponse(null),
        ),
      );

      const callback = exportSpan(exporterUnderTest());

      await callback.called;

      expect(callback).toHaveBeenCalledWith({ code: 0 });

      expect(requests).toHaveLength(2);
      expect(callback).toHaveBeenCalledTimes(1);

      const second = await requests[1]?.text();

      await expect(requests[0]?.text()).resolves.toBe(second);
    },
  );

  test("stamps each attempt with the device clock at dispatch", async () => {
    expect.hasAssertions();

    vi.useFakeTimers({ toFake: ["Date"] });

    onTestFinished(() => {
      vi.useRealTimers();
    });

    const firstAttemptAt = Date.UTC(2026, 10, 4, 12);
    const retryAt = firstAttemptAt + 7000;

    vi.setSystemTime(firstAttemptAt);

    worker.use(
      collector(() => {
        if (requests.length > 1) {
          return new HttpResponse(null);
        }

        vi.setSystemTime(retryAt);

        return new HttpResponse(null, {
          status: 503,
          headers: { "Retry-After": "0" },
        });
      }),
    );

    const callback = exportSpan(exporterUnderTest());

    await callback.called;

    expect(callback).toHaveBeenCalledWith({ code: 0 });

    expect(
      requests.map((request) => request.headers.get("x-interfere-sent-at")),
    ).toStrictEqual([String(firstAttemptAt), String(retryAt)]);
  });

  test("stamps metric exports with the device clock", async () => {
    expect.hasAssertions();

    vi.useFakeTimers({ toFake: ["Date"] });

    onTestFinished(() => {
      vi.useRealTimers();
    });

    const sentAt = Date.UTC(2026, 10, 4, 12);

    vi.setSystemTime(sentAt);

    const { callback, spy } = resultCallback();

    createMetricExporter({ clientOptions: CLIENT_OPTIONS }).export(
      {
        resource: resourceFromAttributes({}),
        scopeMetrics: [],
      } satisfies ResourceMetrics,
      callback,
    );

    await spy.called;

    expect(requests[0]?.url).toContain("/drain/v1/metrics");

    expect(requests[0]?.headers.get("x-interfere-sent-at")).toBe(
      String(sentAt),
    );
  });

  test("short-circuits empty span batches without dispatching", () => {
    const callback = vi.fn<() => void>();

    exporterUnderTest().export([], callback);

    expect(requests).toHaveLength(0);
    expect(callback).toHaveBeenCalledWith({ code: 0 });
  });

  test("does not report a result until the request settles", async () => {
    expect.hasAssertions();

    const response = Promise.withResolvers<Response>();

    onTestFinished(() => response.resolve(new Response(null)));

    worker.use(collector(async () => response.promise));

    const callback = exportSpan(exporterUnderTest());

    await requestSignal.reached(1);

    expect(requests).toHaveLength(1);

    expect(callback).not.toHaveBeenCalled();

    response.resolve(new Response(null));

    await callback.called;

    expect(callback).toHaveBeenCalledExactlyOnceWith({ code: 0 });
  });

  test("reports FAILED with the cause when the fetch rejects", async () => {
    expect.hasAssertions();

    worker.use(collector(() => HttpResponse.error()));

    const callback = exportSpan(exporterUnderTest());

    await callback.called;

    expect(callback).toHaveBeenCalledWith({
      code: 1,
      error: expect.objectContaining({ cause: expect.any(TypeError) }),
    });
  });
});
