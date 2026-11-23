/* oxlint-disable unicorn/prefer-number-coercion */
const MILLIS_PER_SECOND = 1000;

export const TRACESTATE_FRESHNESS_KEY = "interfere";

export const TRACEPARENT_MAX_AGE_SECONDS = 120;

export function freshnessStamp(nowMs: number) {
  return Math.floor(nowMs / MILLIS_PER_SECOND).toString(10);
}

export function isFreshnessStampExpired(stamp: string, nowMs: number) {
  const stampedAtSeconds = Number.parseInt(stamp, 10);

  if (!Number.isFinite(stampedAtSeconds)) {
    return false;
  }

  return (
    nowMs / MILLIS_PER_SECOND - stampedAtSeconds > TRACEPARENT_MAX_AGE_SECONDS
  );
}
