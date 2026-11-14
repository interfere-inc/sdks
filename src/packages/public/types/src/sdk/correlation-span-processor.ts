import { acceptsCorrelationBaggageEntry } from "#data/correlation-baggage";

import { propagation } from "@opentelemetry/api";
import type { SpanProcessor } from "@opentelemetry/sdk-trace-base";

/** Creates a processor that copies valid Interfere correlation baggage onto host spans. */
export function correlationSpanProcessor() {
  return { ...correlationBaggageSpanProcessor } satisfies SpanProcessor;
}

export const correlationBaggageSpanProcessor = {
  async forceFlush() {
    await Promise.resolve();
  },
  // oxlint-disable-next-line eslint/no-empty-function
  onEnd() {},
  onStart(span, parentContext) {
    for (const [key, entry] of propagation
      .getBaggage(parentContext)
      ?.getAllEntries() ?? []) {
      if (acceptsCorrelationBaggageEntry(key, entry.value)) {
        span.setAttribute(key, entry.value);
      }
    }
  },
  async shutdown() {
    await Promise.resolve();
  },
} satisfies SpanProcessor;
