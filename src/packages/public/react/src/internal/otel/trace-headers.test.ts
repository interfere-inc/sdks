import {
  ROOT_CONTEXT,
  TraceFlags,
  propagation,
  trace,
} from "@opentelemetry/api";
import {
  TraceState,
  W3CBaggagePropagator,
  W3CTraceContextPropagator,
  suppressTracing,
} from "@opentelemetry/core";
import { expect, test } from "vitest";

import type { HeaderSet } from "./instrumentations/header-policy.js";
import { createTraceHeaderWriter } from "./trace-headers.js";

const TRACEPARENT = /^00-(?<trace>[\da-f]{32})-(?<span>[\da-f]{16})-01$/u;

const FOREIGN_TRACE_ID = "12345678901234567890123456789012";

const FOREIGN = `00-${FOREIGN_TRACE_ID}-1234567890123456-01`;

const HOST = {
  spanId: "00f067aa0ba902b7",
  traceFlags: TraceFlags.NONE,
  traceId: "4bf92f3577b34da6a3ce929d0e0e4736",
  traceState: new TraceState("vendor=abc"),
};

const ALL: HeaderSet = new Set(["traceparent", "tracestate", "baggage"]);

const write = createTraceHeaderWriter({
  baggage: null,
  traceContext: new W3CTraceContextPropagator(),
});

test("a request with no active span leaves with a fresh sampled root and reports it as the context it left with", () => {
  expect.hasAssertions();

  const first = new Headers();
  const second = new Headers();

  const left = write(ROOT_CONTEXT, first, ALL);

  const traceparent = first.get("traceparent") ?? "";

  const groups = TRACEPARENT.exec(traceparent)?.groups;

  expect(traceparent).toMatch(TRACEPARENT);
  expect(first.get("tracestate")).toBeNull();

  expect([left?.traceId, left?.spanId]).toStrictEqual([
    groups?.["trace"],
    groups?.["span"],
  ]);

  write(ROOT_CONTEXT, second, ALL);

  expect(second.get("traceparent")).toMatch(TRACEPARENT);
  expect(second.get("traceparent")).not.toBe(traceparent);
});

test("the active span's context is written unchanged, unsampled flags and tracestate included", () => {
  expect.hasAssertions();

  const headers = new Headers();

  const left = write(trace.setSpanContext(ROOT_CONTEXT, HOST), headers, ALL);

  expect([headers.get("traceparent"), headers.get("tracestate")]).toStrictEqual(
    [`00-${HOST.traceId}-${HOST.spanId}-00`, "vendor=abc"],
  );

  expect(left?.traceId).toBe(HOST.traceId);
});

test("a traceparent the request already carries is kept and reported", () => {
  expect.hasAssertions();

  const headers = new Headers({ traceparent: FOREIGN });

  const left = write(ROOT_CONTEXT, headers, ALL);

  expect(headers.get("traceparent")).toBe(FOREIGN);
  expect(left?.traceId).toBe(FOREIGN_TRACE_ID);
});

test("suppressed tracing writes no trace context and reports none", () => {
  expect.hasAssertions();

  const headers = new Headers();

  expect(write(suppressTracing(ROOT_CONTEXT), headers, ALL)).toBeUndefined();
  expect(headers.get("traceparent")).toBeNull();
});

test("the allowed set selects the headers: traceparent without tracestate, baggage alone, or nothing", () => {
  expect.hasAssertions();

  const withBaggage = createTraceHeaderWriter({
    baggage: new W3CBaggagePropagator(),
    traceContext: new W3CTraceContextPropagator(),
  });

  const ctx = propagation.setBaggage(
    trace.setSpanContext(ROOT_CONTEXT, HOST),
    propagation.createBaggage({ "interfere.session.id": { value: "s1" } }),
  );

  const traceOnly = new Headers();
  const baggageOnly = new Headers();
  const nothing = new Headers({ traceparent: FOREIGN });

  withBaggage(ctx, traceOnly, new Set(["traceparent"]));

  withBaggage(ctx, baggageOnly, new Set(["baggage"]));

  const left = withBaggage(ctx, nothing, new Set());

  expect(
    [traceOnly, baggageOnly, nothing].map((headers) => [
      headers.get("traceparent"),
      headers.get("tracestate"),
      headers.get("baggage"),
    ]),
  ).toStrictEqual([
    [`00-${HOST.traceId}-${HOST.spanId}-00`, null, null],
    [null, null, "interfere.session.id=s1"],
    [FOREIGN, null, null],
  ]);

  expect(left?.traceId).toBe(FOREIGN_TRACE_ID);
});
