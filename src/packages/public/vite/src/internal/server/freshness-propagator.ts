import {
  TRACESTATE_FRESHNESS_KEY,
  freshnessStamp,
} from "@interfere/types/sdk/trace-freshness";

import { isSpanContextValid, trace } from "@opentelemetry/api";
import type {
  Context,
  TextMapPropagator,
  TextMapSetter,
} from "@opentelemetry/api";
import { TraceState, isTracingSuppressed } from "@opentelemetry/core";

const TRACE_STATE_HEADER = "tracestate";

/**
 * Stamp-only propagator that writes a `tracestate` freshness entry
 * (`interfere=<unix seconds>`) at inject time, never touching
 * `traceparent`. The SSR trace-meta tags feed the browser SDK's
 * propagation reader, which drops traceparents older than its freshness
 * window — so HTML cached by a CDN can't stitch every visitor in the
 * cache window onto one stale trace. Mirrors `@interfere/next`'s
 * stamper; Vite SSR has no framework-level ISR, so no static-generation
 * gating is needed here.
 */
export class FreshnessStampingPropagator implements TextMapPropagator {
  inject<Carrier>(
    context: Context,
    carrier: Carrier,
    setter: TextMapSetter<Carrier>,
  ) {
    const spanContext = trace.getSpanContext(context);

    if (
      !spanContext ||
      !isSpanContextValid(spanContext) ||
      isTracingSuppressed(context)
    ) {
      return;
    }

    const traceState = (spanContext.traceState ?? new TraceState()).set(
      TRACESTATE_FRESHNESS_KEY,
      freshnessStamp(Date.now()),
    );

    setter.set(carrier, TRACE_STATE_HEADER, traceState.serialize());
  }

  extract(context: Context) {
    return context;
  }

  fields() {
    return [TRACE_STATE_HEADER] satisfies string[];
  }
}
