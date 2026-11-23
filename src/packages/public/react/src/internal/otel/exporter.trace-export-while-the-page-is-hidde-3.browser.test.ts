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
  publicKeyHeader: { apiKey: "test-key" },
};

function exporterUnderTest() {
  return createTraceExporter({ clientOptions: CLIENT_OPTIONS });
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

function bulkySpans(count: number) {
  return Array.from({ length: count }, () => ({
    ...makeSpan(),
    attributes: { payload: "x".repeat(2000) },
  }));
}

async function wait(ms: number) {
  await new Promise<void>((resolve) => {
    setTimeout(resolve, ms);
  });
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

  test("carries an over-budget export over after beforeunload instead of sending it without keepalive, and delivers it once the page is shown", async () => {
    expect.hasAssertions();

    globalThis.dispatchEvent(new Event("beforeunload"));

    const { callback, spy } = resultCallback();

    exporterUnderTest().export(
      [{ ...makeSpan(), attributes: { payload: incompressible(40_000) } }],
      callback,
    );

    await wait(100);

    expect(requests.filter((request) => !request.keepalive)).toStrictEqual([]);

    globalThis.dispatchEvent(
      new PageTransitionEvent("pagehide", { persisted: true }),
    );

    globalThis.dispatchEvent(
      new PageTransitionEvent("pageshow", { persisted: true }),
    );

    await spy.called;

    await requestSignal.reached(1);

    expect(spy).toHaveBeenCalledWith({ code: 0 });

    expect(
      requests.map((request) => request.headers.get("x-interfere-carryover")),
    ).toStrictEqual([expect.stringMatching(/^[0-9a-f-]{36}$/u)]);
  });

  test("splits a large batch into keepalive-sized requests once hidden", async () => {
    expect.hasAssertions();

    globalThis.dispatchEvent(
      new PageTransitionEvent("pagehide", { persisted: true }),
    );

    const { callback, spy } = resultCallback();

    exporterUnderTest().export(bulkySpans(40), callback);

    await spy.called;

    expect(requests.length).toBeGreaterThan(1);
    expect(spy).toHaveBeenCalledTimes(1);
    expect(spy).toHaveBeenCalledWith({ code: 0 });

    const bodies = await Promise.all(
      requests.map(async (request) => request.clone().arrayBuffer()),
    );

    for (const body of bodies) {
      expect(body.byteLength).toBeLessThanOrEqual(16_000);
    }

    const exported = await Promise.all(
      requests.map(
        async (request) =>
          (await request.json()) as {
            resourceSpans: { scopeSpans: { spans: unknown[] }[] }[];
          },
      ),
    );

    const spanCount = exported
      .flatMap((body) => body.resourceSpans)
      .flatMap((resourceSpans) => resourceSpans.scopeSpans)
      .reduce((total, scope) => total + scope.spans.length, 0);

    expect(spanCount).toBe(40);
  });
});
