import { SessionManager } from "../../tracking/session.js";
import {
  ActivityMonitor,
  CHECK_INTERVAL_MS,
  EXPIRE_AFTER_MS,
  IDLE_AFTER_MS,
} from "./activity.js";
import { resetPageLifecycleForTests } from "../page-lifecycle/page-lifecycle.js";
import { afterEach, beforeEach, expect, test, vi } from "vitest";

let monitor: ActivityMonitor;

let session: SessionManager;

beforeEach(() => {
  vi.useFakeTimers();

  vi.setSystemTime(new Date("2026-09-19T12:00:00.000Z"));

  session = new SessionManager();

  monitor = new ActivityMonitor();

  monitor.start({
    markActivity: () => session.markActivity(),
    reset: () => session.reset(),
    startedAt: () => session.getSessionStartedAt(),
  });
});

afterEach(() => {
  monitor.stop();

  resetPageLifecycleForTests();

  vi.useRealTimers();
});

test.each([
  {
    name: "pauses after five minutes",
    elapsed: IDLE_AFTER_MS + 1000,
    pauses: 1,
  },
  {
    name: "does not pause before five minutes",
    elapsed: IDLE_AFTER_MS - 60_000,
    pauses: 0,
  },
  {
    name: "pauses once across idle and expired",
    elapsed: EXPIRE_AFTER_MS + 1000,
    pauses: 1,
  },
  {
    name: "does not repeat pause on later expired checks",
    elapsed: EXPIRE_AFTER_MS + CHECK_INTERVAL_MS * 2,
    pauses: 1,
  },
])("$name", ({ elapsed, pauses }) => {
  const pause = vi.fn<() => void>();
  const id = session.getSessionId();

  monitor.onPause(pause);

  vi.advanceTimersByTime(elapsed);

  expect(pause).toHaveBeenCalledTimes(pauses);
  expect(session.getSessionId()).toBe(id);
  expect(localStorage.getItem("interfere:session_id")).toBe(id);
});

test.each([
  {
    name: "resumes idle without a new session",
    elapsed: IDLE_AFTER_MS + 1000,
    reason: "idle",
  },
  {
    name: "rotates after fifteen minutes",
    elapsed: EXPIRE_AFTER_MS + 1000,
    reason: "rotated",
  },
])("$name", ({ elapsed, reason }) => {
  const resume = vi.fn<Parameters<typeof monitor.onResume>[0]>();
  const id = session.getSessionId();

  monitor.onResume(resume);

  vi.advanceTimersByTime(elapsed);

  globalThis.dispatchEvent(new PointerEvent("pointerdown"));

  expect(resume).toHaveBeenCalledExactlyOnceWith(reason);
  expect(session.getSessionId() === id).toBe(reason === "idle");

  expect(localStorage.getItem("interfere:session_id")).toBe(
    session.getSessionId(),
  );

  expect(localStorage.getItem("interfere:last_activity")).toBe(
    String(Date.now()),
  );
});

test("pauses with reason hidden when the page hides and resumes on show without rotating", () => {
  const pause = vi.fn<Parameters<typeof monitor.onPause>[0]>();
  const resume = vi.fn<Parameters<typeof monitor.onResume>[0]>();
  const id = session.getSessionId();

  monitor.onPause(pause);

  monitor.onResume(resume);

  globalThis.dispatchEvent(
    new PageTransitionEvent("pagehide", { persisted: true }),
  );

  expect(pause).toHaveBeenCalledExactlyOnceWith("hidden");

  vi.advanceTimersByTime(IDLE_AFTER_MS + CHECK_INTERVAL_MS);

  expect(pause).toHaveBeenCalledTimes(1);

  globalThis.dispatchEvent(
    new PageTransitionEvent("pageshow", { persisted: true }),
  );

  expect(resume).toHaveBeenCalledExactlyOnceWith("idle");
  expect(session.getSessionId()).toBe(id);
});

test("reports an exception while paused at once, then at most once per five minutes", () => {
  const exceptionWhilePaused =
    vi.fn<Parameters<typeof monitor.onExceptionWhilePaused>[0]>();

  monitor.onExceptionWhilePaused(exceptionWhilePaused);

  monitor.noteException();

  expect(exceptionWhilePaused).not.toHaveBeenCalled();

  vi.advanceTimersByTime(IDLE_AFTER_MS + 1000);

  monitor.noteException();

  monitor.noteException();

  expect(exceptionWhilePaused).toHaveBeenCalledExactlyOnceWith("idle");

  vi.advanceTimersByTime(4 * 60 * 1000);

  monitor.noteException();

  expect(exceptionWhilePaused).toHaveBeenCalledTimes(1);

  vi.advanceTimersByTime(60_000);

  monitor.noteException();

  expect(exceptionWhilePaused).toHaveBeenCalledTimes(2);
});

test("starts a fresh exception throttle for each pause", () => {
  const exceptionWhilePaused =
    vi.fn<Parameters<typeof monitor.onExceptionWhilePaused>[0]>();

  monitor.onExceptionWhilePaused(exceptionWhilePaused);

  globalThis.dispatchEvent(
    new PageTransitionEvent("pagehide", { persisted: true }),
  );

  monitor.noteException();

  globalThis.dispatchEvent(
    new PageTransitionEvent("pageshow", { persisted: true }),
  );

  monitor.noteException();

  globalThis.dispatchEvent(
    new PageTransitionEvent("pagehide", { persisted: true }),
  );

  monitor.noteException();

  expect(exceptionWhilePaused.mock.calls).toStrictEqual([
    ["hidden"],
    ["hidden"],
  ]);
});
