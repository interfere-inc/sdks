import { recordExceptionOnSpan, startExceptionSpan } from "./exception-span.js";
import type { TelemetrySink } from "./sink.js";
import type { OtelProviderHandle } from "../otel/provider.js";
import { failedRequestContext } from "../otel/request-observer/failed-requests.js";

const EXCEPTION_TRACER_NAME = "@interfere/react/exception";

const LOGGER_NAME = "@interfere/react";

/** The sink for pages with tracing on: exceptions become spans on the private tracer provider, a failed fetch's rejection joining its request span's trace, and log records go to the private logger provider. */
export function createOtelSink(handle: OtelProviderHandle) {
  const tracer = handle.tracerProvider.getTracer(EXCEPTION_TRACER_NAME);
  const logger = handle.loggerProvider.getLogger(LOGGER_NAME);

  return {
    recordException: (value, attributes, handled, origin) => {
      const request = failedRequestContext(value);

      const span = startExceptionSpan(
        tracer,
        value,
        request ? { at: origin?.at ?? Date.now(), context: request } : origin,
      );

      recordExceptionOnSpan(span, value, attributes, handled, origin?.at);

      span.end(origin?.at);
    },
    emitLog: (record) => {
      logger.emit(record);
    },
    flush: async () => handle.flush(),
    shutdown: async () => handle.shutdown(),
  } satisfies TelemetrySink;
}
