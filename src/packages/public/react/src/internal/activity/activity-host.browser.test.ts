import { SessionManager } from "../../tracking/session.js";
import { ActivityMonitor } from "./activity.js";
import { EXPIRE_AFTER_MS, IDLE_AFTER_MS } from "./timing.js";
import { resetPageLifecycleForTests } from "../page-lifecycle/page-lifecycle.js";
import { afterEach, beforeEach, expect, test, vi } from "vitest";

let monitor: ActivityMonitor;

let monitors: ActivityMonitor[];

let session: SessionManager;

let input: Parameters<ActivityMonitor["start"]>[0];

beforeEach(() => {
  vi.useFakeTimers();

  vi.setSystemTime(new Date("2026-09-19T12:00:00.000Z"));

  session = new SessionManager();

  monitor = new ActivityMonitor();

  monitors = [monitor];

  input = {
    markActivity: () => session.markActivity(),
    reset: () => session.reset(),
    startedAt: () => session.getSessionStartedAt(),
  };
});

afterEach(() => {
  for (const owned of monitors) {
    owned.stop();
  }

  resetPageLifecycleForTests();

  vi.restoreAllMocks();

  vi.useRealTimers();
});

test("restarts native monitoring after stopping without retaining the old timer", () => {
  const pause = vi.fn<() => void>();

  monitor.onPause(pause);

  monitor.start(input);

  monitor.stop();

  vi.advanceTimersByTime(IDLE_AFTER_MS + 1000);

  expect(pause).not.toHaveBeenCalled();

  monitor.start(input);

  vi.advanceTimersByTime(IDLE_AFTER_MS + 1000);

  expect(pause).toHaveBeenCalledExactlyOnceWith("idle");
});

test("rotates on input even when the poll timer never fired", () => {
  const intervals = vi.spyOn(globalThis, "setInterval");

  const resume = vi.fn<Parameters<typeof monitor.onResume>[0]>();
  const pause = vi.fn<() => void>();
  const id = session.getSessionId();

  monitor.onResume(resume);

  monitor.onPause(pause);

  monitor.start(input);

  const [interval] = intervals.mock.results;

  if (interval?.type !== "return") {
    throw new Error("Expected an installed polling interval");
  }

  clearInterval(interval.value);

  vi.advanceTimersByTime(EXPIRE_AFTER_MS + 1000);

  globalThis.dispatchEvent(new PointerEvent("pointerdown"));

  expect(session.getSessionId()).not.toBe(id);

  expect(localStorage.getItem("interfere:session_id")).toBe(
    session.getSessionId(),
  );

  expect(resume).toHaveBeenCalledExactlyOnceWith("rotated");
  expect(pause).not.toHaveBeenCalled();
});

test("isolates a throwing pause subscriber", () => {
  const good = vi.fn<() => void>();

  monitor.onPause(() => {
    throw new Error("listener failed");
  });

  monitor.onPause(good);

  monitor.start(input);

  expect(() => vi.advanceTimersByTime(IDLE_AFTER_MS + 1000)).not.toThrow();
  expect(good).toHaveBeenCalledExactlyOnceWith("idle");
});

test("ignores a second start while already monitoring", () => {
  const pause = vi.fn<() => void>();
  const other = new SessionManager();
  const otherId = other.getSessionId();
  const id = session.getSessionId();

  monitor.onPause(pause);

  monitor.start(input);

  monitor.start({
    markActivity: () => other.markActivity(),
    reset: () => other.reset(),
    startedAt: () => other.getSessionStartedAt(),
  });

  vi.advanceTimersByTime(EXPIRE_AFTER_MS + 1000);

  globalThis.dispatchEvent(new PointerEvent("pointerdown"));

  expect(pause).toHaveBeenCalledExactlyOnceWith("idle");
  expect(session.getSessionId()).not.toBe(id);
  expect(other.getSessionId()).toBe(otherId);
});

test("can stop before starting", () => {
  expect(() => monitor.stop()).not.toThrow();
});

test("keeps another monitor running when one stops", () => {
  const other = new ActivityMonitor();
  const pauseThis = vi.fn<() => void>();
  const pauseOther = vi.fn<() => void>();

  monitors.push(other);

  monitor.onPause(pauseThis);

  other.onPause(pauseOther);

  monitor.start(input);

  other.start(input);

  monitor.stop();

  vi.advanceTimersByTime(IDLE_AFTER_MS + 1000);

  expect(pauseThis).not.toHaveBeenCalled();
  expect(pauseOther).toHaveBeenCalledExactlyOnceWith("idle");
});
