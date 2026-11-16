import { isRetryableStatus } from "@interfere/helpers/retry-request";
import type { InterfereError } from "@interfere/javascript";

import { IngestBackoffError } from "../internal/ingest-backoff/ingest-backoff.js";

const RETRY_DELAYS_MS = [5000, 15_000] as const;

export function syncKey(
  sessionId: string,
  deviceId: string | null,
  fpHash: string | null,
) {
  return `${sessionId}|${deviceId ?? ""}|${fpHash ?? ""}`;
}

export class SessionRetry {
  private timer: ReturnType<typeof setTimeout> | null = null;
  private attempts = 0;

  cancel() {
    if (this.timer !== null) {
      clearTimeout(this.timer);

      this.timer = null;
    }
  }

  reset() {
    this.cancel();

    this.attempts = 0;
  }

  schedule(error: InterfereError, retry: () => void) {
    const status = error.statusCode;

    const unreachable =
      error.cause instanceof TypeError ||
      error.cause instanceof IngestBackoffError;

    const transient =
      !unreachable && (status === undefined || isRetryableStatus(status));

    const delay = RETRY_DELAYS_MS[this.attempts];

    if (!transient || delay === undefined || this.timer !== null) {
      return;
    }

    this.attempts += 1;

    this.timer = setTimeout(() => {
      this.timer = null;

      retry();
    }, delay);
  }
}
