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

import { isStaticGenerationRender } from "../static-generation.js";

const TRACE_STATE_HEADER = "tracestate";

function carrierHoldsTraceparent(carrier: readonly unknown[]) {
  return carrier.some(
    (entry) =>
      entry !== null &&
      // oxlint-disable-next-line anti-slop/no-runtime-typeof -- OTel array carriers may contain arbitrary entries; only an object with the traceparent key bypasses static-render suppression.
      typeof entry === "object" &&
      "key" in entry &&
      entry.key === "traceparent",
  );
}

export interface FreshnessStampingPropagatorOptions {
  isStaticRender?: () => boolean;
}

/**
 * Stamp-only propagator that writes a `tracestate` freshness entry
 * (`interfere=<unix seconds>`) at inject time. It never touches
 * `traceparent`, so it composes cleanly after any W3C trace-context
 * propagator — the standalone `register()` path pairs it with
 * `CacheSafeTraceContextPropagator`, and composed bootstraps
 * (`@vercel/otel`'s `propagators: ["auto", ...kit.propagators]`,
 * dd-trace, custom providers) pick it up through `kit.propagators`
 * without conflicting with their own trace-context propagator.
 *
 * Why: Next's `clientTraceMetadata` bakes the injected trace context
 * into any HTML it caches — ISR revalidations and CDN full-page caches
 * then serve one render's `traceparent` to every visitor in the cache
 * window, and the browser SDK would stitch all of those sessions onto a
 * single stale trace. The client's propagation reader uses this stamp
 * to drop traceparents older than its freshness window; a re-injected
 * stamp always overwrites an inherited one, so the value reflects this
 * render, not the upstream request that carried it in.
 *
 * Static-generation renders (build prerender, runtime ISR revalidation)
 * feed `clientTraceMetadata` through an array carrier. On those, the
 * stamp is written only when a trace-context propagator ahead of this
 * one already wrote `traceparent` into the carrier — a composed setup
 * whose own propagator this SDK cannot gate, where the stamp is the
 * only staleness defense. In the standalone `register()` path the
 * cache-safe propagator skips `traceparent` first, so no meta is
 * emitted at all, matching Next's build-time prerender output.
 */
export class FreshnessStampingPropagator implements TextMapPropagator {
  private readonly isStaticRender: () => boolean;

  constructor(options: FreshnessStampingPropagatorOptions = {}) {
    this.isStaticRender = options.isStaticRender ?? isStaticGenerationRender;
  }

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

    if (
      Array.isArray(carrier) &&
      this.isStaticRender() &&
      !carrierHoldsTraceparent(carrier)
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
