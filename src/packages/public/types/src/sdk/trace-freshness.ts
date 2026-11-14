/* oxlint-disable unicorn/prefer-number-coercion */
export const TRACESTATE_FRESHNESS_KEY = "interfere";

export const TRACEPARENT_MAX_AGE_SECONDS = 120;

export function freshnessStamp(nowMs: number) {
  return Math.floor(nowMs / 1000).toString(10);
}

export function isFreshnessStampExpired(stamp: string, nowMs: number) {
  const stampedAtSeconds = Number.parseInt(stamp, 10);

  if (!Number.isFinite(stampedAtSeconds)) {
    return false;
  }

  return nowMs / 1000 - stampedAtSeconds > TRACEPARENT_MAX_AGE_SECONDS;
}
