import { SpanKind, SpanStatusCode } from "@opentelemetry/api";
import type { ExportResult } from "@opentelemetry/core";
import { resourceFromAttributes } from "@opentelemetry/resources";
import type { ReadableSpan } from "@opentelemetry/sdk-trace";

import { createTraceExporter } from "./exporter.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";
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

function bulkySpans(count: number) {
  return Array.from({ length: count }, () => ({
    ...makeSpan(),
    attributes: { payload: "x".repeat(2000) },
  }));
}

describe("trace export while the page is hidden", () => {
  beforeEach(() => {
    requests.length = 0;

    worker.use(collector(() => new HttpResponse(null)));
  });

  afterEach(() => {
    globalThis.dispatchEvent(
      new PageTransitionEvent("pageshow", { persisted: true }),
    );
  });

  test("sends a large batch as one request while the page is visible", async () => {
    expect.hasAssertions();

    const { callback, spy } = resultCallback();

    exporterUnderTest().export(bulkySpans(40), callback);

    await spy.called;

    expect(requests).toHaveLength(1);
    expect(spy).toHaveBeenCalledWith({ code: 0 });
  });

  test("gzips a visible page's export and marks it so the collector can decode it", async () => {
    expect.hasAssertions();

    const { callback, spy } = resultCallback();

    exporterUnderTest().export(bulkySpans(40), callback);

    await spy.called;

    expect(requests).toHaveLength(1);

    const [request] = requests;

    expect(request?.headers.get("content-encoding")).toBe("gzip");

    const compressed = await request?.clone().arrayBuffer();

    const decoded = await new Response(
      new Blob([compressed ?? new ArrayBuffer(0)])
        .stream()
        .pipeThrough(new DecompressionStream("gzip")),
    ).text();

    const exported = JSON.parse(decoded) as {
      resourceSpans: { scopeSpans: { spans: unknown[] }[] }[];
    };

    expect(
      exported.resourceSpans
        .flatMap((resourceSpans) => resourceSpans.scopeSpans)
        .reduce((total, scope) => total + scope.spans.length, 0),
    ).toBe(40);

    expect(compressed?.byteLength).toBeLessThan(
      new TextEncoder().encode(decoded).byteLength / 2,
    );
  });

  test("leaves a small export uncompressed", async () => {
    expect.hasAssertions();

    const { callback, spy } = resultCallback();

    exporterUnderTest().export([makeSpan()], callback);

    await spy.called;

    expect(requests[0]?.headers.get("content-encoding")).toBeNull();
  });

  test("sends a hidden page's export uncompressed so it can leave with the page", async () => {
    expect.hasAssertions();

    globalThis.dispatchEvent(
      new PageTransitionEvent("pagehide", { persisted: true }),
    );

    const { callback, spy } = resultCallback();

    exporterUnderTest().export(bulkySpans(40), callback);

    await spy.called;

    expect(
      requests.map((request) => request.headers.get("content-encoding")),
    ).toStrictEqual(requests.map(() => null));
  });

  test("tags a retained hidden export with its carryover id and leaves a visible export untagged", async () => {
    expect.hasAssertions();

    const exporter = exporterUnderTest();
    const visible = resultCallback();

    exporter.export([makeSpan()], visible.callback);

    await visible.spy.called;

    globalThis.dispatchEvent(
      new PageTransitionEvent("pagehide", { persisted: true }),
    );

    const hidden = resultCallback();

    exporter.export([makeSpan()], hidden.callback);

    await hidden.spy.called;

    expect(hidden.spy).toHaveBeenCalledWith({ code: 0 });

    expect(
      requests.map((request) => request.headers.get("x-interfere-carryover")),
    ).toStrictEqual([null, expect.stringMatching(/^[0-9a-f-]{36}$/u)]);
  });

  test("splits a beforeunload flush into keepalive-sized requests, sends only keepalive ones and carries the rest to the next shown page", async () => {
    expect.hasAssertions();

    globalThis.dispatchEvent(new Event("beforeunload"));

    const { callback, spy } = resultCallback();

    exporterUnderTest().export(bulkySpans(40), callback);

    await requestSignal.reached(1);

    const leaving = requests.length;

    expect([
      leaving > 0,
      requests.every((request) => request.keepalive),
    ]).toStrictEqual([true, true]);

    globalThis.dispatchEvent(
      new PageTransitionEvent("pagehide", { persisted: true }),
    );

    globalThis.dispatchEvent(
      new PageTransitionEvent("pageshow", { persisted: true }),
    );

    await spy.called;

    expect(spy).toHaveBeenCalledWith({ code: 0 });

    const delivered = async () => {
      const bodies = await Promise.all(
        requests.map(
          async (request) =>
            (await request.clone().json()) as {
              resourceSpans: { scopeSpans: { spans: unknown[] }[] }[];
            },
        ),
      );

      return bodies
        .flatMap((body) => body.resourceSpans)
        .flatMap((resourceSpans) => resourceSpans.scopeSpans)
        .reduce((total, scope) => total + scope.spans.length, 0);
    };

    let total = await delivered();

    while (total < 40) {
      // oxlint-disable-next-line no-await-in-loop -- Each wait is for the next request the carried-over drain sends; the spy resolves once exports are parked, before the drain has sent them all.
      await requestSignal.reached(requests.length + 1);

      // oxlint-disable-next-line no-await-in-loop -- The count depends on the request just recorded.
      total = await delivered();
    }

    expect(total).toBe(40);
  });
});
