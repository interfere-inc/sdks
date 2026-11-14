import { SessionManager } from "../../tracking/session.js";
import { ActivityMonitor, IDLE_AFTER_MS } from "../activity/activity.js";
import {
  onPageHidden,
  onPageShown,
  resetPageLifecycleForTests,
} from "../page-lifecycle/page-lifecycle.js";
import { afterEach, expect, test, vi } from "vitest";

afterEach(() => {
  resetPageLifecycleForTests();

  vi.restoreAllMocks();

  vi.useRealTimers();
});

test("keeps page subscriptions inert and cleanup repeatable in an actual server host", () => {
  expect(globalThis.addEventListener).toBeUndefined();
  expect(globalThis.removeEventListener).toBeUndefined();

  const hidden = vi.fn<() => void>();
  const shown = vi.fn<() => void>();
  const removeHidden = onPageHidden(hidden);
  const removeShown = onPageShown(shown);

  expect(removeHidden).not.toThrow();
  expect(removeShown).not.toThrow();
  expect(resetPageLifecycleForTests).not.toThrow();

  onPageHidden(hidden);

  onPageShown(shown);

  expect(resetPageLifecycleForTests).not.toThrow();
  expect(hidden).not.toHaveBeenCalled();
  expect(shown).not.toHaveBeenCalled();
});

test("does not start an activity timer or mutate its real session without browser events", () => {
  expect(globalThis.addEventListener).toBeUndefined();

  vi.useFakeTimers();

  const schedule = vi.spyOn(globalThis, "setInterval");
  const session = new SessionManager();
  const id = session.getSessionId();
  const monitor = new ActivityMonitor();
  const pause = vi.fn<() => void>();

  onTestFinished(() => monitor.stop());

  monitor.onPause(pause);

  monitor.start({
    markActivity: session.markActivity.bind(session),
    reset: session.reset.bind(session),
    startedAt: session.getSessionStartedAt.bind(session),
  });

  vi.advanceTimersByTime(IDLE_AFTER_MS + 1000);

  expect(schedule).not.toHaveBeenCalled();
  expect(pause).not.toHaveBeenCalled();
  expect(session.getSessionId()).toBe(id);
  expect(() => monitor.stop()).not.toThrow();
});
