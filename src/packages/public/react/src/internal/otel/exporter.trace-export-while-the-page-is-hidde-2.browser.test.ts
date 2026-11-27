import { SpanKind, SpanStatusCode } from "@opentelemetry/api";
import type { ExportResult } from "@opentelemetry/core";
import { resourceFromAttributes } from "@opentelemetry/resources";
import type { ReadableSpan } from "@opentelemetry/sdk-trace";
import { z } from "zod";

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

  test("puts exception spans, then request spans, first in a leave-time export", async () => {
    expect.hasAssertions();

    globalThis.dispatchEvent(new Event("beforeunload"));

    const { callback } = resultCallback();

    exporterUnderTest().export(
      [
        { ...makeSpan(), name: "resource" },
        { ...makeSpan(), kind: SpanKind.CLIENT, name: "GET" },
        {
          ...makeSpan(),
          events: [
            {
              attributes: { "exception.type": "TypeError" },
              droppedAttributesCount: 0,
              name: "exception",
              time: [0, 0],
            },
          ],
          name: "exception",
        },
      ],
      callback,
    );

    await requestSignal.reached(1);

    const [request] = requests;

    const body = z
      .object({
        resourceSpans: z.array(
          z.object({
            scopeSpans: z.array(
              z.object({ spans: z.array(z.object({ name: z.string() })) }),
            ),
          }),
        ),
      })
      .parse(await request?.clone().json());

    expect(
      body.resourceSpans
        .flatMap((resource) => resource.scopeSpans)
        .flatMap((scope) => scope.spans)
        .map((span) => span.name),
    ).toStrictEqual(["exception", "GET", "resource"]);
  });

  test("hands a visible export still in flight without keepalive to the leave path when the page starts leaving, exception first", async () => {
    expect.hasAssertions();

    const navigation = Promise.withResolvers<null>();

    onTestFinished(() => navigation.resolve(null));

    worker.use(
      collector(async ({ request }) => {
        if (!request.keepalive) {
          await navigation.promise;

          return HttpResponse.error();
        }

        return new HttpResponse(null);
      }),
    );

    const { callback, spy } = resultCallback();

    exporterUnderTest().export(
      [
        { ...makeSpan(), attributes: { payload: incompressible(60_000) } },
        {
          ...makeSpan(),
          events: [
            {
              attributes: { "exception.type": "TypeError" },
              droppedAttributesCount: 0,
              name: "exception",
              time: [0, 0],
            },
          ],
          name: "exception",
        },
      ],
      callback,
    );

    await requestSignal.reached(1);

    expect(requests.map((request) => request.keepalive)).toStrictEqual([false]);

    globalThis.dispatchEvent(new Event("beforeunload"));

    await requestSignal.reached(2);

    navigation.resolve(null);

    const [, leaving] = requests;

    const body = z
      .object({
        resourceSpans: z.array(
          z.object({
            scopeSpans: z.array(
              z.object({ spans: z.array(z.object({ name: z.string() })) }),
            ),
          }),
        ),
      })
      .parse(await leaving?.clone().json());

    expect([
      leaving?.keepalive,
      body.resourceSpans
        .flatMap((resource) => resource.scopeSpans)
        .flatMap((scope) => scope.spans)
        .map((span) => span.name),
    ]).toStrictEqual([true, ["exception"]]);

    globalThis.dispatchEvent(
      new PageTransitionEvent("pagehide", { persisted: true }),
    );

    globalThis.dispatchEvent(
      new PageTransitionEvent("pageshow", { persisted: true }),
    );

    await spy.called;

    await wait(100);

    expect(spy.mock.calls).toStrictEqual([[{ code: 0 }]]);
  });

  test("leaves a visible export already sent with keepalive alone when the page starts leaving", async () => {
    expect.hasAssertions();

    const response = Promise.withResolvers<null>();

    onTestFinished(() => response.resolve(null));

    worker.use(
      collector(async () => {
        await response.promise;

        return new HttpResponse(null);
      }),
    );

    const { callback, spy } = resultCallback();

    exporterUnderTest().export([makeSpan()], callback);

    await requestSignal.reached(1);

    globalThis.dispatchEvent(new Event("beforeunload"));

    response.resolve(null);

    await spy.called;

    await wait(100);

    expect([
      requests.map((request) => request.keepalive),
      spy.mock.calls,
    ]).toStrictEqual([[true], [[{ code: 0 }]]]);
  });
});
