/**
 * Inclusive HTTP status range. `[400, 499]` matches every 4xx;
 * a bare number matches that exact code.
 */
export type HttpStatusCodeRange = number | readonly [number, number];

export interface CaptureFailedRequestsOptions {
  /** Inspect bounded HTTP-200 JSON for error-specific codes. Off by default. */
  applicationErrors?: boolean;
  /** Defaults to `[[400, 599]]` (4xx + 5xx) when not specified. */
  statusCodes?: readonly HttpStatusCodeRange[];
}

/**
 * Promote tracked failed fetch/XHR responses to captured exception events.
 * On by default (same-origin 4xx + 5xx, deduped + rate-limited); `false`
 * disables, `{ statusCodes }` tunes non-200 ranges. HTTP-200 inspection requires
 * `{ applicationErrors: true }`. Only explicit cancellation evidence produces
 * cancellation exceptions; unannotated aborts/timeouts/page exits remain suppressed.
 */
export type CaptureFailedRequestsOption =
  | boolean
  | CaptureFailedRequestsOptions;

const DEFAULT_FAILURE_RANGES: readonly HttpStatusCodeRange[] = [[400, 599]];

export function resolveFailureRanges(
  opt: CaptureFailedRequestsOption | undefined,
) {
  if (opt === false) {
    return null;
  }

  if (opt === undefined || opt === true) {
    return DEFAULT_FAILURE_RANGES;
  }

  const codes = opt.statusCodes;

  return (codes && codes.length > 0 ? codes : DEFAULT_FAILURE_RANGES) satisfies
    | readonly HttpStatusCodeRange[]
    | null;
}

export function statusMatches(
  status: number,
  ranges: readonly HttpStatusCodeRange[],
) {
  for (const range of ranges) {
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Status filters accept either one numeric code or an inclusive two-number range.
    if (typeof range === "number") {
      if (status === range) {
        return true;
      }
    } else if (status >= range[0] && status <= range[1]) {
      return true;
    }
  }

  return false;
}

export function capturesApplicationErrors(
  option: CaptureFailedRequestsOption | undefined,
) {
  return (
    option !== undefined &&
    option !== true &&
    option !== false &&
    option.applicationErrors === true
  );
}
