const keepaliveReleasedHandlers = new Set<() => void>();

// WARNING: Chromium rejects keepalive requests once their combined in-flight bodies exceed 64 KiB; every keepalive sender on the page shares this budget.
const KEEPALIVE_BODY_BUDGET_BYTES = 60_000;

let keepaliveBodyBytes = 0;

/** Subscribes to capacity released by a completed keepalive request. */
export function onKeepaliveBodyReleased(handler: () => void) {
  keepaliveReleasedHandlers.add(handler);

  return () => {
    keepaliveReleasedHandlers.delete(handler);
  };
}

/**
 * Reserves keepalive budget for one request body. Returns the release callback when the page's in-flight keepalive bodies stay within `limit` (the shared budget by default), or null when the request must go out without keepalive.
 */
export function reserveKeepaliveBody(
  bytes: number,
  limit = KEEPALIVE_BODY_BUDGET_BYTES,
) {
  if (
    keepaliveBodyBytes + bytes >
    Math.min(limit, KEEPALIVE_BODY_BUDGET_BYTES)
  ) {
    return null;
  }

  keepaliveBodyBytes += bytes;

  let released = false;

  return (() => {
    if (released) {
      return;
    }

    released = true;

    keepaliveBodyBytes -= bytes;

    for (const handler of keepaliveReleasedHandlers) {
      handler();
    }
  }) satisfies () => void;
}
