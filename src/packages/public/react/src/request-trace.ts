import { correlateResponseTrace } from "./internal/otel/request-observer/request-trace-correlation.js";

/** Associates an observed response with the final W3C trace headers its request sent. */
export function correlateRequestTrace(
  response: Response,
  headers: HeadersInit,
) {
  try {
    correlateResponseTrace(response, new Headers(headers));
  } catch {
    return;
  }
}
