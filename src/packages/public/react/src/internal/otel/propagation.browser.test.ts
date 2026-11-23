import { propagation, trace } from "@opentelemetry/api";
import { W3CTraceContextPropagator } from "@opentelemetry/core";

import { readPropagationFromDocument } from "./propagation.js";
import { afterEach, beforeEach, describe, expect, test } from "vitest";

const MILLIS_PER_SECOND = 1000;

const TRACE_ID = "0af7651916cd43dd8448eb211c80319c";

const SPAN_ID = "b7ad6b7169203331";

const TRACEPARENT = `00-${TRACE_ID}-${SPAN_ID}-01`;

const UNSAMPLED_TRACEPARENT = `00-${TRACE_ID}-${SPAN_ID}-00`;

const ownedMeta: HTMLMetaElement[] = [];

let previousMeta: { element: Element; next: ChildNode | null }[];

function nowSeconds() {
  return Math.floor(Date.now() / MILLIS_PER_SECOND);
}

function freshStamp(extra = "") {
  return `interfere=${nowSeconds()}${extra}`;
}

function setMeta(name: string, content: string) {
  const m = document.createElement("meta");

  m.setAttribute("name", name);

  m.setAttribute("content", content);

  document.head.append(m);

  ownedMeta.push(m);
}

function extract() {
  const ctx = readPropagationFromDocument();

  if (!ctx) {
    throw new Error("expected a non-null context");
  }

  return ctx;
}

describe(readPropagationFromDocument, () => {
  beforeEach(() => {
    propagation.setGlobalPropagator(new W3CTraceContextPropagator());

    previousMeta = Array.from(
      document.head.querySelectorAll(
        'meta[name="traceparent"], meta[name="tracestate"], meta[name="baggage"]',
      ),
      (element) => ({ element, next: element.nextSibling }),
    );

    for (const { element } of previousMeta) {
      element.remove();
    }
  });

  afterEach(() => {
    propagation.disable();

    for (const m of ownedMeta.splice(0)) {
      m.remove();
    }

    for (const { element, next } of previousMeta.toReversed()) {
      document.head.insertBefore(element, next);
    }

    previousMeta = [];
  });

  test("returns null when no traceparent meta is present", () => {
    setMeta("tracestate", freshStamp());

    expect(readPropagationFromDocument()).toBeNull();
  });

  test("keeps a sampled remote parent with its id and flags", () => {
    setMeta("traceparent", TRACEPARENT);

    setMeta("tracestate", freshStamp());

    const spanCtx = trace.getSpanContext(extract());

    expect(spanCtx?.traceId).toBe(TRACE_ID);
    expect(spanCtx?.spanId).toBe(SPAN_ID);
    expect(spanCtx?.traceFlags).toBe(1);
    expect(spanCtx?.isRemote).toBe(true);
  });

  test("keeps an unsampled remote parent with flags 00 and its tracestate intact", () => {
    setMeta("traceparent", UNSAMPLED_TRACEPARENT);

    setMeta("tracestate", `vendor=abc123,${freshStamp()}`);

    const spanCtx = trace.getSpanContext(extract());

    expect(spanCtx?.traceId).toBe(TRACE_ID);
    expect(spanCtx?.spanId).toBe(SPAN_ID);
    expect(spanCtx?.traceFlags).toBe(0);
    expect(spanCtx?.traceState?.get("vendor")).toBe("abc123");
    expect(spanCtx?.traceState?.get("interfere")).toBe(String(nowSeconds()));
  });

  test("keeps foreign tracestate entries alongside the stamp", () => {
    setMeta("traceparent", TRACEPARENT);

    setMeta("tracestate", `${freshStamp()},vendor=abc123`);

    expect(trace.getSpanContext(extract())?.traceState?.get("vendor")).toBe(
      "abc123",
    );
  });

  test("extracts baggage when the meta is present", () => {
    setMeta("traceparent", TRACEPARENT);

    setMeta("tracestate", freshStamp());

    setMeta("baggage", "key1=val1,key2=val2");

    const ctx = extract();

    expect(propagation.getBaggage(ctx)?.getEntry("key1")?.value).toBe("val1");
    expect(propagation.getBaggage(ctx)?.getEntry("key2")?.value).toBe("val2");
  });

  test("returns null when the traceparent is malformed", () => {
    setMeta("traceparent", "not a valid traceparent");

    setMeta("tracestate", freshStamp());

    expect(readPropagationFromDocument()).toBeNull();
  });

  test("returns null when the traceparent encodes the invalid trace id", () => {
    setMeta("traceparent", `00-00000000000000000000000000000000-${SPAN_ID}-01`);

    setMeta("tracestate", freshStamp());

    expect(readPropagationFromDocument()).toBeNull();
  });

  test("returns null when only the span id is invalid", () => {
    setMeta("traceparent", `00-${TRACE_ID}-0000000000000000-01`);

    setMeta("tracestate", freshStamp());

    expect(readPropagationFromDocument()).toBeNull();
  });

  test.each([TRACEPARENT, UNSAMPLED_TRACEPARENT])(
    "returns null for an unstamped traceparent %s",
    (traceparent) => {
      setMeta("traceparent", traceparent);

      expect(readPropagationFromDocument()).toBeNull();
    },
  );

  test("returns null when the tracestate carries only foreign entries", () => {
    setMeta("traceparent", TRACEPARENT);

    setMeta("tracestate", "vendor=abc123");

    expect(readPropagationFromDocument()).toBeNull();
  });

  test("returns null when the interfere stamp is not a unix timestamp", () => {
    setMeta("traceparent", TRACEPARENT);

    setMeta("tracestate", "interfere=not-a-number");

    expect(readPropagationFromDocument()).toBeNull();
  });

  test.each([TRACEPARENT, UNSAMPLED_TRACEPARENT])(
    "returns null when the stamp on %s exceeds the max age",
    (traceparent) => {
      setMeta("traceparent", traceparent);

      setMeta("tracestate", `interfere=${nowSeconds() - 121}`);

      expect(readPropagationFromDocument()).toBeNull();
    },
  );

  test("accepts a stamp at the max-age boundary", () => {
    setMeta("traceparent", TRACEPARENT);

    setMeta("tracestate", `interfere=${nowSeconds() - 119}`);

    expect(readPropagationFromDocument()).not.toBeNull();
  });

  test("accepts a future-dated interfere stamp (client clock behind server)", () => {
    setMeta("traceparent", TRACEPARENT);

    setMeta("tracestate", `interfere=${nowSeconds() + 300}`);

    expect(readPropagationFromDocument()).not.toBeNull();
  });

  test("prefers the stamped tracestate meta when a composed stack emits several", () => {
    setMeta("traceparent", TRACEPARENT);

    setMeta("tracestate", "vendor=abc123");

    setMeta("tracestate", `interfere=${nowSeconds() - 121},vendor=abc123`);

    expect(readPropagationFromDocument()).toBeNull();
  });

  test("extracts the stamped tracestate meta's entries when fresh", () => {
    setMeta("traceparent", TRACEPARENT);

    setMeta("tracestate", "vendor=abc123");

    setMeta("tracestate", `${freshStamp()},vendor=abc123`);

    expect(trace.getSpanContext(extract())?.traceState?.get("vendor")).toBe(
      "abc123",
    );
  });
});
