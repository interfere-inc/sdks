import { createLogger } from "../../util/log.js";
import { onPageHidden, onPageShown } from "../page-lifecycle/page-lifecycle.js";

const log = createLogger("activity");

/** Input-idle time after which the session pauses (replay stops recording); the session id is kept. */
export const IDLE_AFTER_MS = 5 * 60 * 1000;

/** Input gap after which the next input rotates to a new session id. */
export const EXPIRE_AFTER_MS = 15 * 60 * 1000;

const PAUSED_EXCEPTION_INTERVAL_MS = 5 * 60 * 1000;

export const MAX_AGE_MS = 24 * 60 * 60 * 1000;

export const CHECK_INTERVAL_MS = 30 * 1000;

const KEEPALIVE_THROTTLE_MS = 1000;

const ACTIVITY_EVENTS = [
  "pointerdown",
  "keydown",
  "scroll",
  "touchstart",
  "mousemove",
] as const;

export interface ActivitySession {
  markActivity: () => void;
  lastActivityAt?: () => number;
  reset: () => void;
  /** Wall-clock epoch ms the session began; `0` when unknown. */
  startedAt: () => number;
}

export type SessionResumeReason = "idle" | "rotated";

export type SessionPauseReason = "hidden" | "idle";

type Handler<A> = (arg: A) => void;

function monotonicNow() {
  return typeof performance === "undefined" ? Date.now() : performance.now();
}

function subscribe<A>(set: Set<Handler<A>>, handler: Handler<A>) {
  set.add(handler);

  return (() => {
    set.delete(handler);
  }) satisfies () => void;
}

function fire<A>(handlers: Set<Handler<A>>, arg: A) {
  for (const handler of handlers) {
    try {
      handler(arg);
    } catch (error) {
      log.warn(
        "activity handler threw: %s",
        error instanceof Error ? error.constructor.name : "UnknownError",
      );
    }
  }
}

export class ActivityMonitor {
  private session: ActivitySession | null = null;
  private pausedFor: SessionPauseReason | null = null;
  private lastPausedExceptionMs: number | null = null;
  private lastActivityMs = 0;
  private lastActivityWallMs = 0;
  private keepaliveUntilMs = 0;
  private checkTimer: ReturnType<typeof setInterval> | null = null;
  private unsubscribeShown: (() => void) | null = null;
  private unsubscribeHidden: (() => void) | null = null;
  private readonly pauseHandlers = new Set<Handler<SessionPauseReason>>();
  private readonly resumeHandlers = new Set<Handler<SessionResumeReason>>();
  private readonly exceptionHandlers = new Set<Handler<void>>();
  private readonly pausedExceptionHandlers = new Set<
    Handler<SessionPauseReason>
  >();

  onPause(handler: Handler<SessionPauseReason>) {
    return subscribe(this.pauseHandlers, handler);
  }

  onExceptionWhilePaused(handler: Handler<SessionPauseReason>) {
    return subscribe(this.pausedExceptionHandlers, handler);
  }

  onException(handler: Handler<void>) {
    return subscribe(this.exceptionHandlers, handler);
  }

  /** Reports a recorded exception; fires `exceptionWhilePaused` when the session is paused, at most once per five minutes per pause. */
  noteException() {
    fire(this.exceptionHandlers, undefined);

    if (this.pausedFor === null) {
      return;
    }

    const now = monotonicNow();

    if (
      this.lastPausedExceptionMs !== null &&
      now - this.lastPausedExceptionMs < PAUSED_EXCEPTION_INTERVAL_MS
    ) {
      return;
    }

    this.lastPausedExceptionMs = now;

    fire(this.pausedExceptionHandlers, this.pausedFor);
  }

  onResume(handler: Handler<SessionResumeReason>) {
    return subscribe(this.resumeHandlers, handler);
  }

  start(session: ActivitySession) {
    if (
      this.checkTimer !== null ||
      // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Activity monitoring is inert in non-browser hosts without callable event registration.
      typeof globalThis.addEventListener !== "function"
    ) {
      return;
    }

    this.session = session;
    this.pausedFor = null;
    this.lastPausedExceptionMs = null;
    this.lastActivityMs = monotonicNow();
    this.lastActivityWallMs = Date.now();
    this.keepaliveUntilMs = 0;

    for (const type of ACTIVITY_EVENTS) {
      globalThis.addEventListener(type, this.onInput, {
        capture: true,
        passive: true,
      });
    }

    this.unsubscribeShown = onPageShown(this.onInput);
    this.unsubscribeHidden = onPageHidden(this.onHidden);
    this.checkTimer = setInterval(this.poll, CHECK_INTERVAL_MS);
  }

  stop() {
    if (this.checkTimer === null) {
      return;
    }

    clearInterval(this.checkTimer);

    this.checkTimer = null;

    for (const type of ACTIVITY_EVENTS) {
      globalThis.removeEventListener(type, this.onInput, true);
    }

    this.unsubscribeShown?.();

    this.unsubscribeHidden?.();

    this.unsubscribeShown = null;
    this.unsubscribeHidden = null;
    this.session = null;
    this.pausedFor = null;
    this.lastPausedExceptionMs = null;
    this.lastActivityMs = 0;
    this.keepaliveUntilMs = 0;
  }

  private idleMs() {
    return monotonicNow() - this.lastActivityMs;
  }

  private isOverMaxAge() {
    const startedAt = this.session?.startedAt() ?? 0;

    return startedAt > 0 && Date.now() - startedAt >= MAX_AGE_MS;
  }

  private pause(reason: SessionPauseReason) {
    if (this.pausedFor !== null) {
      return;
    }

    this.pausedFor = reason;
    this.lastPausedExceptionMs = null;

    log.debug("session paused: %s", reason);

    fire(this.pauseHandlers, reason);
  }

  private readonly onHidden = () => {
    this.pause("hidden");
  };

  private readonly poll = () => {
    if (this.pausedFor !== null) {
      return;
    }

    if (this.isOverMaxAge() || this.idleMs() >= IDLE_AFTER_MS) {
      this.pause("idle");
    }
  };

  private readonly onInput = () => {
    const now = monotonicNow();

    const lastActivity =
      this.session?.lastActivityAt?.() ?? this.lastActivityWallMs;

    if (this.isOverMaxAge() || Date.now() - lastActivity >= EXPIRE_AFTER_MS) {
      this.resume(now, "rotated");

      return;
    }

    if (this.pausedFor !== null) {
      this.resume(now, "idle");

      return;
    }

    if (now < this.keepaliveUntilMs) {
      return;
    }

    this.touch(now);

    this.session?.markActivity();
  };

  private resume(now: number, reason: SessionResumeReason) {
    if (reason === "rotated") {
      log.debug("resume after expiry → new session");

      this.session?.reset();
    }

    this.pausedFor = null;
    this.lastPausedExceptionMs = null;

    this.touch(now);

    this.session?.markActivity();

    fire(this.resumeHandlers, reason);
  }

  private touch(now: number) {
    this.lastActivityMs = now;
    this.lastActivityWallMs = Date.now();
    this.keepaliveUntilMs = now + KEEPALIVE_THROTTLE_MS;
  }
}
