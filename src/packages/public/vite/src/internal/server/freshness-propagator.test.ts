import { omitUndefined } from "@interfere/helpers/omit-undefined";

import {
  ROOT_CONTEXT,
  TraceFlags,
  defaultTextMapSetter,
  trace,
} from "@opentelemetry/api";
import {
  CompositePropagator,
  TraceState,
  W3CTraceContextPropagator,
} from "@opentelemetry/core";

import { FreshnessStampingPropagator } from "./freshness-propagator.js";
import { describe, expect, test } from "vitest";

const TRACE_ID = "0af7651916cd43dd8448eb211c80319c";

const SPAN_ID = "b7ad6b7169203331";

const STAMP_RE = /(?:^|,)interfere=(?<seconds>\d+)(?:,|$)/u;

function contextWithSpan(traceState?: TraceState) {
  return trace.setSpanContext(
    ROOT_CONTEXT,
    omitUndefined({
      spanId: SPAN_ID,
      traceFlags: TraceFlags.SAMPLED,
      traceId: TRACE_ID,
      traceState,
    }),
  );
}

describe(FreshnessStampingPropagator, () => {
  test("stamps tracestate and never writes traceparent", () => {
    const propagator = new FreshnessStampingPropagator();
    const carrier: Record<string, string> = {};

    propagator.inject(contextWithSpan(), carrier, defaultTextMapSetter);

    expect(carrier["traceparent"]).toBeUndefined();
    expect(carrier["tracestate"]).toMatch(STAMP_RE);
  });

  test("preserves existing tracestate entries", () => {
    const propagator = new FreshnessStampingPropagator();
    const carrier: Record<string, string> = {};

    propagator.inject(
      contextWithSpan(new TraceState("vendor=abc123")),
      carrier,
      defaultTextMapSetter,
    );

    expect(carrier["tracestate"]).toMatch(STAMP_RE);
    expect(carrier["tracestate"]).toMatch(/(?:^|,)vendor=abc123(?:,|$)/u);
  });

  test("injects nothing when no span context is present", () => {
    const propagator = new FreshnessStampingPropagator();
    const carrier: Record<string, string> = {};

    propagator.inject(ROOT_CONTEXT, carrier, defaultTextMapSetter);

    expect(carrier["tracestate"]).toBeUndefined();
  });

  test("composes after the stock W3C propagator", () => {
    const composite = new CompositePropagator({
      propagators: [
        new W3CTraceContextPropagator(),
        new FreshnessStampingPropagator(),
      ],
    });

    const carrier: Record<string, string> = {};

    composite.inject(contextWithSpan(), carrier, defaultTextMapSetter);

    expect(carrier["traceparent"]).toBe(`00-${TRACE_ID}-${SPAN_ID}-01`);
    expect(carrier["tracestate"]).toMatch(STAMP_RE);
  });
});
