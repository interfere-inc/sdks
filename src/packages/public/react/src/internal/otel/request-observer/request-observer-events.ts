import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { isSpanContextValid, trace } from "@opentelemetry/api";
import type { Context, SpanContext } from "@opentelemetry/api";
import { SeverityNumber } from "@opentelemetry/api-logs";
import type { LoggerProvider } from "@opentelemetry/sdk-logs";

import { ATTR_URL_FULL, ATTR_URL_PATH } from "../url-attrs.js";

export const REQUEST_OBSERVER_LOGGER_NAME = "@interfere/react/request-observer";

export const REQUEST_OBSERVATION_EVENT_NAME = "interfere.request";

/** The trace id of the `traceparent` the request left with, whichever tool wrote it. */
export const ATTR_REQUEST_TRACE_ID = "interfere.request.trace_id";

export interface RequestObservation {
  errorType?: string;
  correlation?: boolean;
  timestamp?: number;
  kind: "fetch" | "xhr";
  method: string;
  status?: number;
  /** The span context the request left with, when it carried a `traceparent`. */
  traceContext?: SpanContext;
  url: string;
}

function hostContext(ctx: Context) {
  const spanContext = trace.getSpanContext(ctx);

  return spanContext && isSpanContextValid(spanContext) ? ctx : undefined;
}

/** Records request observations as `interfere.request` log events linked to the host's active span, else to the trace the request left with. */
export function requestObservationLogger(
  loggerProvider: LoggerProvider,
  redactUrl: (url: string) => string,
) {
  const logger = loggerProvider.getLogger(REQUEST_OBSERVER_LOGGER_NAME);

  return (observation: RequestObservation, ctx: Context) => {
    let pathname: string | undefined;

    try {
      ({ pathname } = new URL(observation.url));
    } catch {
      pathname = undefined;
    }

    logger.emit(
      omitUndefined({
        attributes: omitUndefined({
          [ATTR_URL_FULL]: redactUrl(observation.url),
          [ATTR_URL_PATH]: pathname,
          "http.request.method": observation.method,
          "http.response.status_code": observation.status,
          "error.type": observation.errorType,
          "interfere.request.kind": observation.kind,
          [ATTR_REQUEST_TRACE_ID]: observation.traceContext?.traceId,
        }),
        context:
          hostContext(ctx) ??
          (observation.traceContext
            ? trace.setSpanContext(ctx, observation.traceContext)
            : ctx),
        eventName: observation.correlation
          ? "interfere.request.correlation"
          : REQUEST_OBSERVATION_EVENT_NAME,
        timestamp: observation.timestamp,
        severityNumber: SeverityNumber.UNSPECIFIED,
      }),
    );
  };
}
