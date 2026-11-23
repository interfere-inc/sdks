import type { Attributes, Span } from "@opentelemetry/api";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
  TracerProvider,
} from "@opentelemetry/sdk-trace";

import {
  applyFetchSpanAttrs,
  applyXhrSpanAttrs,
  createFailureGate,
} from "./request-outcome.js";
import { resolveFailureRanges } from "./failure-policy.js";
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
  const span = provider.getTracer("test").startSpan("xhr", {
    attributes: response.status === 0 ? { "error.type": "Error" } : {},
  });

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
