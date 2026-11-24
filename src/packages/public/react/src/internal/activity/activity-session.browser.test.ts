import { browserStorage } from "@interfere/test-config/browser-storage";
import { SessionManager } from "../../tracking/session.js";
import { ActivityMonitor } from "./activity.js";
import { CHECK_INTERVAL_MS, MAX_AGE_MS } from "./timing.js";
import { resetPageLifecycleForTests } from "../page-lifecycle/page-lifecycle.js";
import { afterEach, beforeEach, expect, test, vi } from "vitest";

let monitor: ActivityMonitor;

let session: SessionManager;

beforeEach(() => {
  vi.useFakeTimers();

  vi.setSystemTime(new Date("2026-09-19T12:00:00.000Z"));

  session = new SessionManager();

  monitor = new ActivityMonitor();
});

afterEach(() => {
  monitor.stop();

  resetPageLifecycleForTests();

  vi.useRealTimers();
});

test("pauses an over-age session and rotates on the next input", () => {
  const pause = vi.fn<() => void>();
  const resume = vi.fn<Parameters<typeof monitor.onResume>[0]>();
  const id = session.getSessionId();

  vi.setSystemTime(Date.now() + MAX_AGE_MS + 1000);

  monitor.onPause(pause);

  monitor.onResume(resume);

  monitor.start({
    markActivity: () => session.markActivity(),
    reset: () => session.reset(),
    startedAt: () => session.getSessionStartedAt(),
  });

  vi.advanceTimersByTime(CHECK_INTERVAL_MS);

  expect(pause).toHaveBeenCalledExactlyOnceWith("idle");
  expect(session.getSessionId()).toBe(id);

  globalThis.dispatchEvent(new PointerEvent("pointerdown"));

  expect(session.getSessionId()).not.toBe(id);
  expect(session.getSessionStartedAt()).toBe(Date.now());

  expect(localStorage.getItem("interfere:session_id")).toBe(
    session.getSessionId(),
  );

  expect(resume).toHaveBeenCalledExactlyOnceWith("rotated");
});

test("persists activity updates outside the keepalive window", () => {
  monitor.start({
    markActivity: () => session.markActivity(),
    reset: () => session.reset(),
    startedAt: () => session.getSessionStartedAt(),
  });

  vi.advanceTimersByTime(500);

  globalThis.dispatchEvent(new PointerEvent("pointerdown"));

  expect(localStorage.getItem("interfere:last_activity")).toBe(
    String(Date.now()),
  );

  vi.advanceTimersByTime(2000);

  globalThis.dispatchEvent(new PointerEvent("pointerdown"));

  expect(localStorage.getItem("interfere:last_activity")).toBe(
    String(Date.now()),
  );
});

test("does not persist back-to-back activity inside the keepalive window", () => {
  monitor.start({
    markActivity: () => session.markActivity(),
    reset: () => session.reset(),
    startedAt: () => session.getSessionStartedAt(),
  });

  vi.advanceTimersByTime(500);

  globalThis.dispatchEvent(new PointerEvent("pointerdown"));

  const firstActivity = String(Date.now());

  vi.advanceTimersByTime(1);

  globalThis.dispatchEvent(new PointerEvent("pointerdown"));

  expect(localStorage.getItem("interfere:last_activity")).toBe(firstActivity);
});

test("keeps a restored session whose start time is unknown", () => {
  const id = session.getSessionId();
  const pause = vi.fn<() => void>();

  browserStorage.local.remove("interfere:session_started");

  session = new SessionManager();

  expect(session.getSessionStartedAt()).toBe(0);

  monitor.onPause(pause);

  monitor.start({
    markActivity: () => session.markActivity(),
    reset: () => session.reset(),
    startedAt: () => session.getSessionStartedAt(),
  });

  vi.advanceTimersByTime(CHECK_INTERVAL_MS);

  expect(pause).not.toHaveBeenCalled();
  expect(session.getSessionId()).toBe(id);
});
