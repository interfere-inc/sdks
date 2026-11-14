import { W3CTraceContextPropagator } from "@opentelemetry/core";

import { createTraceHeaderWriter } from "../src/internal/otel/trace-headers.js";

/** A writer that adds W3C trace context where absent and no baggage. */
export function w3cTraceHeaders() {
  return createTraceHeaderWriter({
    baggage: null,
    traceContext: new W3CTraceContextPropagator(),
  });
}
