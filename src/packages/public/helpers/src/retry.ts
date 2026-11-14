export interface RetryOptions {
  readonly attempts?: number;
  readonly baseDelayMs?: number;
  readonly maxDelayMs?: number;
  // oxlint-disable-next-line anti-slop/no-unknown-parameters -- Operations throw arbitrary values; callers classify them.
  readonly delayMs?: (index: number, error: unknown) => number;
  // oxlint-disable-next-line anti-slop/no-unknown-parameters -- Operations throw arbitrary values; callers classify them.
  readonly retryOn?: (error: unknown) => boolean;
  readonly random?: () => number;
  readonly signal?: AbortSignal;
  readonly sleep?: (ms: number, signal?: AbortSignal) => Promise<void>;
}

/**
 * Exponential backoff with equal jitter for the zero-based retry `index`,
 * capped at `maxDelayMs`.
 */
export function backoffMs(
  index: number,
  baseDelayMs: number,
  maxDelayMs: number,
  random: () => number = Math.random,
) {
  const ceiling = Math.min(maxDelayMs, baseDelayMs * 2 ** index);

  return Math.round(ceiling * (0.5 + random() * 0.5));
}

/** Resolves after `ms`; rejects with the signal's reason if it aborts first. */
export async function sleep(ms: number, signal?: AbortSignal) {
  signal?.throwIfAborted();

  await new Promise<void>((resolve, reject) => {
    const abort = () => {
      clearTimeout(timer);

      // oxlint-disable-next-line typescript/prefer-promise-reject-errors -- An abort reason is caller-supplied and must propagate unchanged.
      reject(signal?.reason);
    };

    const timer = setTimeout(() => {
      signal?.removeEventListener("abort", abort);

      resolve();
    }, ms);

    signal?.addEventListener("abort", abort, { once: true });
  });
}

/**
 * Runs `operation`, retrying thrown failures that `retryOn` accepts (all of
 * them by default). `attempts` counts total tries. The wait before retry `n`
 * is `delayMs(n, error)` when given, else jittered exponential backoff.
 * Rethrows the last error when attempts run out or `retryOn` declines.
 */
export async function retry<T>(
  operation: (attempt: number) => Promise<T>,
  options: RetryOptions = {},
) {
  const attempts = Math.max(1, options.attempts ?? 3);
  const baseDelayMs = options.baseDelayMs ?? 250;
  const maxDelayMs = options.maxDelayMs ?? 10_000;
  const wait = options.sleep ?? sleep;

  /* oxlint-disable no-await-in-loop -- Each attempt starts only after the previous one failed and its backoff elapsed. */
  for (let attempt = 0; ; attempt += 1) {
    try {
      return await operation(attempt);
    } catch (error) {
      if (attempt + 1 >= attempts || options.retryOn?.(error) === false) {
        throw error;
      }

      await wait(
        options.delayMs?.(attempt, error) ??
          backoffMs(attempt, baseDelayMs, maxDelayMs, options.random),
        options.signal,
      );
    }
  }
  /* oxlint-enable no-await-in-loop */
}
