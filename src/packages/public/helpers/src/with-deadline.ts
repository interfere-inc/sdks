/**
 * Runs `operation` with an AbortSignal that fires a `TimeoutError` after
 * `timeoutMs`, and clears the timer as soon as the operation settles so a
 * finished call leaves no pending timer behind. Prefer this over
 * `AbortSignal.timeout`, whose timer keeps running after the work completes
 * and holds a Worker or Durable Object invocation open until it fires.
 */
export async function withDeadline<T>(
  timeoutMs: number,
  operation: (signal: AbortSignal) => Promise<T>,
  message = "The operation exceeded its deadline",
) {
  const controller = new AbortController();

  const timer = setTimeout(() => {
    controller.abort(new DOMException(message, "TimeoutError"));
  }, timeoutMs);

  try {
    return await operation(controller.signal);
  } finally {
    clearTimeout(timer);
  }
}
