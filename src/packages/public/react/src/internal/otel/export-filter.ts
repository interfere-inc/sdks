import type { Span } from "@opentelemetry/api";
import { createLogger } from "../../util/log.js";
import { hrTimeToMilliseconds } from "@opentelemetry/core";
import type { ReadableSpan, SpanProcessor } from "@opentelemetry/sdk-trace";

const log = createLogger("export-filter");

const LONG_TASK_SCOPE = "@opentelemetry/instrumentation-long-task";

const LONG_TASK_MIN_EXPORT_MS = 500;

const excluded = new WeakSet<object>();

/** Keeps a span out of export once it ends. */
export function excludeFromExport(span: Span) {
  excluded.add(span);
}

/** Whether a finished span is worth exporting. Excluded spans and long tasks under 500 ms are dropped. */
export function shouldExportSpan(span: ReadableSpan) {
  return !(
    excluded.has(span) ||
    (span.instrumentationScope.name === LONG_TASK_SCOPE &&
      hrTimeToMilliseconds(span.duration) < LONG_TASK_MIN_EXPORT_MS)
  );
}

function carriesException(span: ReadableSpan) {
  return span.events.some((event) => event.name === "exception");
}

/** Forwards only exportable spans to the wrapped export processor, flushes it as soon as a span carrying an exception is queued, and reports that span to `onExceptionSpan`. */
export function createExportFilterProcessor(
  next: SpanProcessor,
  onExceptionSpan?: () => void,
) {
  let flushScheduled = false;

  const flushSoon = () => {
    if (flushScheduled) {
      return;
    }

    flushScheduled = true;

    queueMicrotask(() => {
      flushScheduled = false;

      // oxlint-disable-next-line promise/prefer-await-to-then -- Export failures are reported by the exporter; the flush itself must never surface as an unhandled rejection.
      next.forceFlush().catch(log.debug);
    });
  };

  return {
    onStart: (span, parentContext) => {
      next.onStart(span, parentContext);
    },
    onEnd: (span) => {
      if (!shouldExportSpan(span)) {
        return;
      }

      next.onEnd(span);

      if (carriesException(span)) {
        onExceptionSpan?.();

        flushSoon();
      }
    },
    forceFlush: async () => next.forceFlush(),
    shutdown: async () => next.shutdown(),
  } satisfies SpanProcessor;
}
