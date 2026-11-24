import { parseRetryAfter } from "./after.js";
import { backoffMs, sleep as defaultSleep } from "../retry.js";

const DEFAULT_ATTEMPTS = 5;

const DEFAULT_BASE_DELAY_MS = 250;

const DEFAULT_MAX_DELAY_MS = 10_000;

const RETRYABLE_STATUSES = new Set([408, 425, 429]);

export interface RetryRequestOptions {
  readonly attempts?: number;
  readonly baseDelayMs?: number;
  readonly maxDelayMs?: number;
  readonly now?: () => number;
  readonly random?: () => number;
  readonly sleep?: (ms: number) => Promise<void>;
}

/**
 * Whether a response status is worth another attempt. Covers the throttling
 * and transient-server range only — a 4xx that is not 408/425/429 means the
 * request itself is wrong and will fail identically forever.
 */
export function isRetryableStatus(status: number) {
  return RETRYABLE_STATUSES.has(status) || status >= 500;
}

/**
 * Sends a request, retrying throttled, transient and network failures with
 * exponential backoff and jitter.
 *
 * Object storage answers a small but non-zero share of requests with 429 or
 * 5xx as a matter of course. A release that uploads thousands of files is
 * therefore near-certain to see one, so a single un-retried attempt turns a
 * routine condition into a failed build.
 *
 * Returns the last response when attempts run out, so the caller still
 * decides what a failure means; rethrows the last error if every attempt
 * threw.
 */
export async function retryRequest(
  send: () => Promise<Response>,
  options: RetryRequestOptions = {},
) {
  const attempts = Math.max(1, options.attempts ?? DEFAULT_ATTEMPTS);
  const baseDelayMs = options.baseDelayMs ?? DEFAULT_BASE_DELAY_MS;
  const maxDelayMs = options.maxDelayMs ?? DEFAULT_MAX_DELAY_MS;
  const random = options.random ?? Math.random;
  const now = options.now ?? Date.now;
  const sleep = options.sleep ?? defaultSleep;

  let lastError: unknown;

  /* oxlint-disable no-await-in-loop -- Retry attempts wait for the previous response, body cleanup, and backoff. */
  for (let attempt = 0; attempt < attempts; attempt += 1) {
    const isFinalAttempt = attempt === attempts - 1;

    try {
      const response = await send();

      if (response.ok || !isRetryableStatus(response.status)) {
        return response;
      }

      if (isFinalAttempt) {
        return response;
      }

      const retryAfterMs = parseRetryAfter(
        response.headers.get("retry-after"),
        {
          now: now(),
        },
      );

      await response.body?.cancel();

      await sleep(
        retryAfterMs ?? backoffMs(attempt, baseDelayMs, maxDelayMs, random),
      );
    } catch (error) {
      lastError = error;

      if (isFinalAttempt) {
        throw error;
      }

      await sleep(backoffMs(attempt, baseDelayMs, maxDelayMs, random));
    }
  }

  /* oxlint-enable no-await-in-loop */
  throw lastError;
}
