import { isNonErrorException } from "@interfere/types/sdk/errors";
import type { NonErrorException } from "@interfere/types/sdk/errors";

const REPEAT_DEDUP_CAP = 100;

const REPEAT_RATE_MAX = 5;

const REPEAT_RATE_WINDOW_MS = 60_000;

export function exceptionKey(exception: Error | NonErrorException) {
  return isNonErrorException(exception)
    ? `${exception.type}|${exception.value}`
    : `${exception.name}|${exception.message}`;
}

export function createRepeatGate() {
  const counts = new Map<string, number>();
  let windowStart = 0;

  return ((key: string) => {
    const now = Date.now();

    if (now - windowStart > REPEAT_RATE_WINDOW_MS) {
      windowStart = now;

      counts.clear();
    }

    const count = counts.get(key) ?? 0;

    if (count >= REPEAT_RATE_MAX) {
      return false;
    }

    if (counts.size >= REPEAT_DEDUP_CAP) {
      counts.clear();
    }

    counts.set(key, count + 1);

    return true;
  }) satisfies (key: string) => boolean;
}
