import { INVALID_SPAN_CONTEXT, trace } from "@opentelemetry/api";
import type { Span } from "@opentelemetry/api";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
  TracerProvider,
} from "@opentelemetry/sdk-trace";

import { applyFetchSpanAttrs, createFailureGate } from "./outcome.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

let provider: TracerProvider;

let exporter: InMemorySpanExporter;

const PAGE_STAYS = {
  dispose: () => undefined,
  leftSince: () => false,
};

beforeEach(() => {
  vi.useFakeTimers();

  exporter = new InMemorySpanExporter();

  provider = new TracerProvider({
    spanProcessors: [new SimpleSpanProcessor({ exporter })],
  });
});

afterEach(async () => {
  vi.useRealTimers();

  await provider.shutdown();
});

function finishSpan(span: Span) {
  vi.runAllTimers();

  span.end();

  const finished = exporter.getFinishedSpans().at(-1);

  if (!finished) {
    throw new Error("Expected a finished span");
  }

  return { attrs: finished.attributes, events: finished.events };
}

type Result = Response | { message: string; status?: number };

type Ranges = Parameters<typeof applyFetchSpanAttrs>[3];

function run(
  request: Parameters<typeof applyFetchSpanAttrs>[1],
  result: Result,
  ranges: Ranges,
  { gate }: { gate?: (key: string) => boolean } = {},
) {
  const span = provider.getTracer("test").startSpan("fetch");

  void applyFetchSpanAttrs(
    span,
    request,
    result,
    ranges,
    PAGE_STAYS,
    () => true,
    undefined,
    gate,
  );

  return finishSpan(span);
}

function res(status: number) {
  const r = new Response(null, { status });

  Object.defineProperty(r, "url", { value: "https://app.test/api/x" });

  return r;
}

describe("fetch capture guards", () => {
  test("no-ops a failed response when capture is disabled (null ranges)", () => {
    expect(run("https://app.test/api/x", res(503), null).events).toHaveLength(
      0,
    );
  });

  test("no-ops a transport failure when capture is disabled (null ranges)", () => {
    expect(
      run("https://app.test/api/x", new TypeError("Failed to fetch"), null)
        .events,
    ).toHaveLength(0);
  });

  test("matches a bare-number status range", () => {
    const { events } = run("https://app.test/api/x", res(404), [404]);

    expect(events[0]?.attributes?.["interfere.exception.mechanism"]).toBe(
      "auto.browser.fetch.failed_response",
    );
  });

  test("returns when no url can be resolved", () => {
    const { attrs, events } = run({}, { message: "boom" }, [[400, 599]]);

    expect(events).toHaveLength(0);
    expect(attrs).toStrictEqual({});
  });

  test("returns when the resolved url is unparseable", () => {
    expect(
      run("::::bad", { message: "boom" }, [[400, 599]]).events,
    ).toHaveLength(0);
  });

  test("ignores a rejection that is not an Error", () => {
    expect(
      run("https://app.test/api/x", { message: "" }, [[400, 599]]).events,
    ).toHaveLength(0);
  });

  test("ignores a response whose status is outside the ranges", () => {
    expect(
      run("https://app.test/api/x", res(204), [[400, 599]]).events,
    ).toHaveLength(0);
  });

  test("dedupes a gated failed response", () => {
    const gate = createFailureGate();

    expect(
      run("https://app.test/api/x", res(503), [[400, 599]], { gate }).events,
    ).toHaveLength(1);

    expect(
      run("https://app.test/api/x", res(503), [[400, 599]], { gate }).events,
    ).toHaveLength(0);
  });

  test("dedupes a gated transport failure", () => {
    const gate = createFailureGate();

    expect(
      run(
        "https://app.test/api/x",
        new TypeError("Failed to fetch"),
        [[400, 599]],
        { gate },
      ).events,
    ).toHaveLength(1);

    expect(
      run(
        "https://app.test/api/x",
        new TypeError("Failed to fetch"),
        [[400, 599]],
        { gate },
      ).events,
    ).toHaveLength(0);
  });

  test("returns when the span exposes no attributes to recover a url from", () => {
    const span = trace.wrapSpanContext(INVALID_SPAN_CONTEXT);
    const addEvent = vi.spyOn(span, "addEvent");

    void applyFetchSpanAttrs(
      span,
      {},
      { message: "boom" },
      [[400, 599]],
      PAGE_STAYS,
      () => true,
      undefined,
    );

    expect(addEvent).not.toHaveBeenCalled();
  });
});
