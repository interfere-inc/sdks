import type { TextMapPropagator } from "@opentelemetry/api";
import { W3CBaggagePropagator } from "@interfere/server-core/otel/core/propagation";

import { CacheSafeTraceContextPropagator } from "./cache-safe-trace-context.js";
import { FreshnessStampingPropagator } from "./freshness-propagator.js";

export function managedPropagators() {
  return [
    new CacheSafeTraceContextPropagator(),
    new FreshnessStampingPropagator(),
    new W3CBaggagePropagator(),
  ] satisfies TextMapPropagator[];
}
