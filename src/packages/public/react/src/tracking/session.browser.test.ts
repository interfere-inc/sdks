import { browserStorage } from "@interfere/test-config/browser-storage";
import { SessionManager } from "./session.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

const UUID_REGEX =
  /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/u;

const WINDOW_ID_REGEX = /^win_/u;

const SESSION_ID_KEY = "interfere:session_id";

const LAST_ACTIVITY_KEY = "interfere:last_activity";

const SESSION_STARTED_KEY = "interfere:session_started";

const WINDOW_ID_KEY = "interfere:window_id";

const FIFTEEN_MINUTES = 15 * 60 * 1000;

const TWENTY_FOUR_HOURS = 24 * 60 * 60 * 1000;

describe(SessionManager, () => {
  beforeEach(() => {
    vi.useFakeTimers();

    vi.setSystemTime(new Date("2026-09-19T12:00:00.000Z"));
  });

  afterEach(() => {
    vi.useRealTimers();

    vi.restoreAllMocks();

    vi.unstubAllGlobals();
  });

  test("generates a window id without the secure-context randomUUID API", () => {
    // oxlint-disable-next-line interfere-testing/no-test-doubles -- Secure-context API availability must be removed while retaining real getRandomValues.
    vi.stubGlobal("crypto", {
      getRandomValues: crypto.getRandomValues.bind(crypto),
    });

    const manager = new SessionManager();

    expect(manager.getWindowId().slice(4)).toMatch(UUID_REGEX);
    expect(manager.getWindowId()).toBe(manager.getWindowId());
  });

  test("creates a new session on first instantiation", () => {
    const mgr = new SessionManager();
    const id = mgr.getSessionId();

    expect(id).toMatch(UUID_REGEX);
    expect(localStorage.getItem(SESSION_ID_KEY)).toBe(id);
  });

  test("restores an existing session from localStorage", () => {
    const sessionId = "01955a60-7400-7000-8000-000000000001";

    browserStorage.local.set(SESSION_ID_KEY, sessionId);

    browserStorage.local.set(LAST_ACTIVITY_KEY, String(Date.now()));

    const mgr = new SessionManager();

    expect(mgr.getSessionId()).toBe(sessionId);
  });

  test("replaces a malformed persisted session identifier", () => {
    browserStorage.local.set(SESSION_ID_KEY, "not-a-session-id");

    browserStorage.local.set(LAST_ACTIVITY_KEY, String(Date.now()));

    const manager = new SessionManager();

    expect(manager.getSessionId()).toMatch(UUID_REGEX);
    expect(localStorage.getItem(SESSION_ID_KEY)).toBe(manager.getSessionId());
  });

  test("restores or rotates a valid session at idle timeout offset -1 ms", () => {
    const offset = -1;
    const storedId = "01955a60-7400-7000-8000-000000000001";
    const startedAt = Date.now() - 60 * 60 * 1000;
    const onRotate = vi.fn<(id: string) => void>();

    browserStorage.local.set(SESSION_ID_KEY, storedId);

    browserStorage.local.set(SESSION_STARTED_KEY, String(startedAt));

    browserStorage.local.set(
      LAST_ACTIVITY_KEY,
      String(Date.now() - FIFTEEN_MINUTES - offset),
    );

    const manager = new SessionManager(onRotate);
    const id = manager.getSessionId();

    expect(id).toMatch(UUID_REGEX);
    expect(id).toBe(storedId);
    expect(localStorage.getItem(SESSION_ID_KEY)).toBe(id);

    expect(manager.getSessionStartedAt()).toBe(startedAt);

    expect(localStorage.getItem(LAST_ACTIVITY_KEY)).toBe(String(Date.now()));
    expect(onRotate.mock.calls).toStrictEqual([]);
  });

  test("restores or rotates a valid session at idle timeout offset 0 ms", () => {
    const offset = 0;
    const storedId = "01955a60-7400-7000-8000-000000000001";
    const startedAt = Date.now() - 60 * 60 * 1000;
    const onRotate = vi.fn<(id: string) => void>();

    browserStorage.local.set(SESSION_ID_KEY, storedId);

    browserStorage.local.set(SESSION_STARTED_KEY, String(startedAt));

    browserStorage.local.set(
      LAST_ACTIVITY_KEY,
      String(Date.now() - FIFTEEN_MINUTES - offset),
    );

    const manager = new SessionManager(onRotate);
    const id = manager.getSessionId();

    expect(id).toMatch(UUID_REGEX);
    expect(id).toBe(storedId);
    expect(localStorage.getItem(SESSION_ID_KEY)).toBe(id);

    expect(manager.getSessionStartedAt()).toBe(startedAt);

    expect(localStorage.getItem(LAST_ACTIVITY_KEY)).toBe(String(Date.now()));
    expect(onRotate.mock.calls).toStrictEqual([]);
  });

  test("restores or rotates a valid session at idle timeout offset 1 ms", () => {
    const offset = 1;
    const storedId = "01955a60-7400-7000-8000-000000000001";
    const startedAt = Date.now() - 60 * 60 * 1000;
    const onRotate = vi.fn<(id: string) => void>();

    browserStorage.local.set(SESSION_ID_KEY, storedId);

    browserStorage.local.set(SESSION_STARTED_KEY, String(startedAt));

    browserStorage.local.set(
      LAST_ACTIVITY_KEY,
      String(Date.now() - FIFTEEN_MINUTES - offset),
    );

    const manager = new SessionManager(onRotate);
    const id = manager.getSessionId();

    expect(id).toMatch(UUID_REGEX);
    expect(id).not.toBe(storedId);
    expect(localStorage.getItem(SESSION_ID_KEY)).toBe(id);

    expect(manager.getSessionStartedAt()).toBe(Date.now());

    expect(localStorage.getItem(LAST_ACTIVITY_KEY)).toBe(String(Date.now()));
    expect(onRotate.mock.calls).toStrictEqual([[id]]);
  });

  test("restores or rotates a recently active session at max age offset -1 ms", () => {
    const offset = -1;
    const storedId = "01955a60-7400-7000-8000-000000000002";
    const startedAt = Date.now() - TWENTY_FOUR_HOURS - offset;
    const onRotate = vi.fn<(id: string) => void>();

    browserStorage.local.set(SESSION_ID_KEY, storedId);

    browserStorage.local.set(LAST_ACTIVITY_KEY, String(Date.now()));

    browserStorage.local.set(SESSION_STARTED_KEY, String(startedAt));

    const manager = new SessionManager(onRotate);
    const id = manager.getSessionId();

    expect(id).toMatch(UUID_REGEX);
    expect(id).toBe(storedId);
    expect(localStorage.getItem(SESSION_ID_KEY)).toBe(id);

    expect(manager.getSessionStartedAt()).toBe(startedAt);

    expect(localStorage.getItem(SESSION_STARTED_KEY)).toBe(String(startedAt));

    expect(onRotate.mock.calls).toStrictEqual([]);
  });

  test("restores or rotates a recently active session at max age offset 0 ms", () => {
    const offset = 0;
    const storedId = "01955a60-7400-7000-8000-000000000002";
    const startedAt = Date.now() - TWENTY_FOUR_HOURS - offset;
    const onRotate = vi.fn<(id: string) => void>();

    browserStorage.local.set(SESSION_ID_KEY, storedId);

    browserStorage.local.set(LAST_ACTIVITY_KEY, String(Date.now()));

    browserStorage.local.set(SESSION_STARTED_KEY, String(startedAt));

    const manager = new SessionManager(onRotate);
    const id = manager.getSessionId();

    expect(id).toMatch(UUID_REGEX);
    expect(id).toBe(storedId);
    expect(localStorage.getItem(SESSION_ID_KEY)).toBe(id);

    expect(manager.getSessionStartedAt()).toBe(startedAt);

    expect(localStorage.getItem(SESSION_STARTED_KEY)).toBe(String(startedAt));

    expect(onRotate.mock.calls).toStrictEqual([]);
  });

  test("restores or rotates a recently active session at max age offset 1 ms", () => {
    const offset = 1;
    const storedId = "01955a60-7400-7000-8000-000000000002";
    const startedAt = Date.now() - TWENTY_FOUR_HOURS - offset;
    const onRotate = vi.fn<(id: string) => void>();

    browserStorage.local.set(SESSION_ID_KEY, storedId);

    browserStorage.local.set(LAST_ACTIVITY_KEY, String(Date.now()));

    browserStorage.local.set(SESSION_STARTED_KEY, String(startedAt));

    const manager = new SessionManager(onRotate);
    const id = manager.getSessionId();

    expect(id).toMatch(UUID_REGEX);
    expect(id).not.toBe(storedId);
    expect(localStorage.getItem(SESSION_ID_KEY)).toBe(id);

    expect(manager.getSessionStartedAt()).toBe(Date.now());

    expect(localStorage.getItem(SESSION_STARTED_KEY)).toBe(String(Date.now()));

    expect(onRotate.mock.calls).toStrictEqual([[id]]);
  });

  test("records the session start time on rotation", () => {
    const mgr = new SessionManager();

    mgr.getSessionId();

    expect(mgr.getSessionStartedAt()).toBe(Date.now());
    expect(localStorage.getItem(SESSION_STARTED_KEY)).toBe(String(Date.now()));
  });

  test("does not self-rotate after the timeout (expiry is driven by the activity monitor)", () => {
    const mgr = new SessionManager();
    const firstId = mgr.getSessionId();

    vi.advanceTimersByTime(FIFTEEN_MINUTES + 1);

    expect(mgr.getSessionId()).toBe(firstId);
  });

  test("resets to a new session immediately", () => {
    const onRotate = vi.fn<() => void>();
    const mgr = new SessionManager(onRotate);
    const firstId = mgr.getSessionId();
    const secondId = mgr.reset();

    expect(secondId).toMatch(UUID_REGEX);
    expect(secondId).not.toBe(firstId);
    expect(mgr.getSessionId()).toBe(secondId);
    expect(localStorage.getItem(SESSION_ID_KEY)).toBe(secondId);
    expect(onRotate).toHaveBeenLastCalledWith(secondId);
  });

  test("fires onRotate callback when a new session is created", () => {
    const onRotate = vi.fn<() => void>();
    const mgr = new SessionManager(onRotate);

    expect(onRotate).toHaveBeenCalledTimes(1);
    expect(onRotate).toHaveBeenCalledWith(mgr.getSessionId());
  });

  test("markActivity updates last_activity; getSessionId does not", () => {
    const mgr = new SessionManager();

    mgr.getSessionId();

    vi.advanceTimersByTime(60_000);

    const unchanged = localStorage.getItem(LAST_ACTIVITY_KEY);

    mgr.getSessionId();

    expect(localStorage.getItem(LAST_ACTIVITY_KEY)).toBe(unchanged);

    vi.advanceTimersByTime(60_000);

    mgr.markActivity();

    expect(localStorage.getItem(LAST_ACTIVITY_KEY)).toBe(String(Date.now()));
  });

  describe("window ID", () => {
    test("generates a window ID with win_ prefix", () => {
      const mgr = new SessionManager();
      const winId = mgr.getWindowId();

      expect(winId).toMatch(WINDOW_ID_REGEX);
    });

    test("persists window ID in sessionStorage", () => {
      const mgr = new SessionManager();
      const winId = mgr.getWindowId();

      expect(sessionStorage.getItem(WINDOW_ID_KEY)).toBe(winId);
    });

    test("gives a duplicated tab its own window ID despite copied sessionStorage", () => {
      const original = new SessionManager();
      const originalWindow = original.getWindowId();
      const sessionId = original.getSessionId();

      browserStorage.session.set(WINDOW_ID_KEY, originalWindow);

      const duplicate = new SessionManager();
      const duplicateWindow = duplicate.getWindowId();

      expect(duplicateWindow).toMatch(WINDOW_ID_REGEX);
      expect(duplicateWindow).not.toBe(originalWindow);
      expect(duplicate.getWindowId()).toBe(duplicateWindow);
      expect(duplicate.getSessionId()).toBe(sessionId);
      expect(original.getWindowId()).toBe(originalWindow);
    });

    test("returns the same window ID on subsequent calls", () => {
      const mgr = new SessionManager();

      expect(mgr.getWindowId()).toBe(mgr.getWindowId());
    });
  });
});
