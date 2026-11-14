import { INVALID_SPAN_CONTEXT, trace } from "@opentelemetry/api";
import type { Attributes, Span } from "@opentelemetry/api";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import { WebTracerProvider } from "@opentelemetry/sdk-trace-web";

import {
  applyFetchSpanAttrs,
  applyXhrSpanAttrs,
  createFailureGate,
} from "./request-outcome.js";
import { resolveFailureRanges } from "./failure-policy.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

let provider: WebTracerProvider;

let exporter: InMemorySpanExporter;

const PAGE_STAYS = {
  dispose: () => undefined,
  leftSince: () => false,
};

beforeEach(() => {
  vi.useFakeTimers();

  exporter = new InMemorySpanExporter();

  provider = new WebTracerProvider({
    spanProcessors: [new SimpleSpanProcessor(exporter)],
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

function captureNetwork(
  result: Error,
  request: Parameters<
    typeof applyFetchSpanAttrs
  >[1] = "https://app.test/api/items",
  seedAttributes: Attributes = {},
) {
  const span = provider.getTracer("test").startSpan("fetch", {
    attributes: seedAttributes,
  });

  void applyFetchSpanAttrs(
    span,
    request,
    result,
    [[400, 599]],
    PAGE_STAYS,
    () => true,
    undefined,
  );

  return finishSpan(span);
}

function applyXhr(
  response: { responseURL?: string; status?: number | undefined },
  {
    failureRanges = [[400, 599]],
    gate,
    tracked = true,
  }: {
    failureRanges?: readonly (number | readonly [number, number])[] | null;
    gate?: (key: string) => boolean;
    tracked?: boolean;
  } = {},
) {
  const span = provider.getTracer("test").startSpan("xhr");
  const xhr = new XMLHttpRequest();

  Object.defineProperties(xhr, {
    responseURL: { value: response.responseURL },
    status: { value: response.status },
  });

  void applyXhrSpanAttrs(
    span,
    xhr,
    failureRanges,
    PAGE_STAYS,
    () => tracked,
    gate,
  );

  return finishSpan(span).events;
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

describe("resolveFailureRanges (default-on 4xx + 5xx)", () => {
  test("captures 4xx + 5xx by default when unset", () => {
    expect(resolveFailureRanges(undefined)).toStrictEqual([[400, 599]]);
  });

  test("captures 4xx + 5xx when true", () => {
    expect(resolveFailureRanges(true)).toStrictEqual([[400, 599]]);
  });

  test("disables when false (escape hatch)", () => {
    expect(resolveFailureRanges(false)).toBeNull();
  });

  test("honours explicit statusCodes", () => {
    expect(
      resolveFailureRanges({ statusCodes: [404, [500, 599]] }),
    ).toStrictEqual([404, [500, 599]]);
  });

  test("falls back to the default for empty statusCodes", () => {
    expect(resolveFailureRanges({ statusCodes: [] })).toStrictEqual([
      [400, 599],
    ]);
  });
});

describe("failed-request gate (dedupe + rate-limit)", () => {
  test("dedupes the same method+status+path", () => {
    const gate = createFailureGate();

    expect(gate("GET 500 /api/x")).toBe(true);
    expect(gate("GET 500 /api/x")).toBe(false);
  });

  test("treats distinct keys independently", () => {
    const gate = createFailureGate();

    expect(gate("GET 500 /api/x")).toBe(true);
    expect(gate("GET 404 /api/x")).toBe(true);
    expect(gate("POST 500 /api/x")).toBe(true);
  });

  test("rate-limits a flood of distinct failures", () => {
    const gate = createFailureGate();
    let allowed = 0;

    for (let i = 0; i < 50; i += 1) {
      if (gate(`GET 500 /api/${i}`)) {
        allowed += 1;
      }
    }

    expect(allowed).toBe(20);
  });
});

describe("network transport-failure capture", () => {
  test("records a network_error event carrying the browser error for a fetch rejection", () => {
    const { events } = captureNetwork(new TypeError("Failed to fetch"));

    expect(events[0]?.attributes).toMatchObject({
      "exception.message": "Network network GET https://app.test/api/items",
      "interfere.exception.mechanism": "auto.browser.fetch.network_error",
      "interfere.exception.subtype": "network",
    });

    expect(
      JSON.parse(String(events[0]?.attributes?.["interfere.exception.chain"])),
    ).toStrictEqual([
      {
        kind: "error",
        mechanism: {
          handled: false,
          type: "auto.browser.fetch.network_error",
        },
        stack: "",
        type: "NetworkError",
        value: "Network network GET https://app.test/api/items",
      },
      { kind: "error", stack: "", type: "TypeError", value: "Failed to fetch" },
    ]);
  });

  test("resolves the url from the span when a string-fetch rejection carries none", () => {
    const { events } = captureNetwork(
      new TypeError("Load failed"),
      {},
      {
        "url.full": "https://app.test/api/slow",
      },
    );

    expect(events[0]?.attributes).toMatchObject({
      "exception.message": "Network network GET https://app.test/api/slow",
      "interfere.exception.mechanism": "auto.browser.fetch.network_error",
      "interfere.exception.subtype": "network",
    });
  });
});

describe("xHR failure capture", () => {
  const TRACKED_URL = "https://app.test/api/items";

  test("records an http_error event for a matched status", () => {
    const events = applyXhr({ responseURL: TRACKED_URL, status: 503 });

    expect(events).toHaveLength(1);

    expect(events[0]?.attributes).toMatchObject({
      "exception.message": `HTTP 503 XHR ${TRACKED_URL}`,
      "exception.type": "HttpResponseError",
      "interfere.exception.mechanism": "auto.browser.xhr.failed_response",
    });
  });

  test("records a network_error event when status is 0", () => {
    const events = applyXhr({ responseURL: TRACKED_URL, status: 0 });

    expect(events).toHaveLength(1);

    expect(events[0]?.attributes).toMatchObject({
      "exception.type": "NetworkError",
      "interfere.exception.mechanism": "auto.browser.xhr.network_error",
      "interfere.exception.subtype": "network",
    });
  });

  test("skips when capture is disabled (null ranges)", () => {
    expect(
      applyXhr(
        { responseURL: TRACKED_URL, status: 503 },
        {
          failureRanges: null,
        },
      ),
    ).toHaveLength(0);
  });

  test("skips an untracked url", () => {
    expect(
      applyXhr({ responseURL: TRACKED_URL, status: 503 }, { tracked: false }),
    ).toHaveLength(0);
  });

  test("skips when responseURL is empty", () => {
    expect(applyXhr({ responseURL: "", status: 503 })).toHaveLength(0);
  });

  test("skips when status is not a number", () => {
    expect(
      applyXhr({ responseURL: TRACKED_URL, status: undefined }),
    ).toHaveLength(0);
  });

  test("skips a status outside the configured ranges", () => {
    expect(applyXhr({ responseURL: TRACKED_URL, status: 204 })).toHaveLength(0);
  });

  test("is gated: a deduped matched status is captured once", () => {
    const gate = createFailureGate();

    expect(
      applyXhr({ responseURL: TRACKED_URL, status: 500 }, { gate }),
    ).toHaveLength(1);

    expect(
      applyXhr({ responseURL: TRACKED_URL, status: 500 }, { gate }),
    ).toHaveLength(0);
  });

  test("is gated: a deduped network failure is captured once", () => {
    const gate = createFailureGate();

    expect(
      applyXhr({ responseURL: TRACKED_URL, status: 0 }, { gate }),
    ).toHaveLength(1);

    expect(
      applyXhr({ responseURL: TRACKED_URL, status: 0 }, { gate }),
    ).toHaveLength(0);
  });
});

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
