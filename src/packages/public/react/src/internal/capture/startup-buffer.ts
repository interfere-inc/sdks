import type { NonErrorException } from "@interfere/types/sdk/errors";

import type { Attributes, Context } from "@opentelemetry/api";

const STARTUP_EXCEPTION_LIMIT = 50;

const CONSENT_GRACE_MS = 5000;

/** An exception captured while startup buffering holds delivery. */
export interface PendingException {
  readonly attrs: Attributes;
  readonly handled: boolean;
  readonly origin: { readonly at: number; readonly context: Context };
  readonly value: Error | NonErrorException;
}

/** Holds exceptions captured during startup and delivers them once, in capture order, when released. */
export class StartupExceptionBuffer {
  private readonly deliver: (pending: readonly PendingException[]) => void;
  private pending: PendingException[] | null = null;
  private timer: ReturnType<typeof setTimeout> | undefined;

  constructor(deliver: (pending: readonly PendingException[]) => void) {
    this.deliver = deliver;
  }

  get holding() {
    return this.pending !== null;
  }

  hold() {
    this.pending = [];
  }

  /** Buffers one exception; a full buffer whose release is only deferred delivers first, so a storm loses device attributes rather than exceptions. Returns false when nothing is held. */
  push(entry: PendingException) {
    if (this.pending && this.pending.length >= STARTUP_EXCEPTION_LIMIT) {
      this.releaseDeferred();
    }

    if (!this.pending) {
      return false;
    }

    if (this.pending.length < STARTUP_EXCEPTION_LIMIT) {
      this.pending.push(entry);
    }

    return true;
  }

  /** Delivers now, or with `defer` waits for `releaseDeferred` or a short grace period. */
  release(defer: boolean) {
    if (this.pending && defer) {
      this.timer ??= setTimeout(() => {
        this.drain();
      }, CONSENT_GRACE_MS);

      return;
    }

    this.drain();
  }

  /** Delivers a release that is waiting on its grace period; does nothing otherwise. */
  releaseDeferred() {
    if (this.timer !== undefined) {
      this.drain();
    }
  }

  discard() {
    clearTimeout(this.timer);

    this.timer = undefined;
    this.pending = null;
  }

  private drain() {
    const pending = this.pending ?? [];

    this.discard();

    this.deliver(pending);
  }
}
