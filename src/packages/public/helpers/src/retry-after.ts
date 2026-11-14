export interface ParseRetryAfterOptions {
  readonly maxMs?: number;
  readonly now?: number;
}

/**
 * Parses a Retry-After header (delay-seconds or HTTP-date) into milliseconds,
 * clamped to `maxMs` when given. Returns undefined for a missing, negative or
 * unparseable value so the caller picks its own fallback.
 */
export function parseRetryAfter(
  header: string | null | undefined,
  options: ParseRetryAfterOptions = {},
) {
  const value = header?.trim();

  if (!value) {
    return;
  }

  let delayMs: number;

  if (/^\d+$/u.test(value)) {
    delayMs = Number(value) * 1000;
  } else if (/^[A-Za-z]{3}, /u.test(value)) {
    delayMs = Date.parse(value) - (options.now ?? Date.now());
  } else {
    return;
  }

  if (!Number.isSafeInteger(delayMs)) {
    return;
  }

  return Math.min(
    options.maxMs ?? Number.POSITIVE_INFINITY,
    Math.max(0, delayMs),
  );
}
