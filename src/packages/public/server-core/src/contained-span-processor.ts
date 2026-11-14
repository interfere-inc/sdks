import type { SpanProcessor } from "@opentelemetry/sdk-trace-base";

import { contain } from "./contain.js";

const SITE = "Span processor";

export function containSpanProcessor(processor: SpanProcessor) {
  return {
    forceFlush: async () => processor.forceFlush(),
    onEnd: (span) => contain(SITE, () => processor.onEnd(span)),
    onEnding: (span) => contain(SITE, () => processor.onEnding?.(span)),
    onStart: (span, parentContext) =>
      contain(SITE, () => processor.onStart(span, parentContext)),
    shutdown: async () => processor.shutdown(),
  } satisfies SpanProcessor;
}
