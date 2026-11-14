import {
  browserStorage,
  fillStorageQuota,
} from "@interfere/test-config/browser-storage";
import { SessionManager } from "./session.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

const UUID_REGEX =
  /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/u;

const WINDOW_ID_REGEX = /^win_/u;

describe("session storage fallback", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();

    vi.restoreAllMocks();
  });

  test("keeps activity and rotation working when storage fills mid-session", () => {
    const rotations: string[] = [];
    const manager = new SessionManager((id) => rotations.push(id));
    const first = manager.getSessionId();

    browserStorage.local.remove("interfere:last_activity");

    browserStorage.local.remove("interfere:session_id");

    browserStorage.local.remove("interfere:session_started");

    fillStorageQuota(localStorage);

    fillStorageQuota(sessionStorage);

    vi.advanceTimersByTime(60_000);

    manager.markActivity();

    expect(manager.getLastActivityAt()).toBe(Date.now());
    expect(manager.getWindowId()).toMatch(WINDOW_ID_REGEX);

    const second = manager.reset();

    expect(second).not.toBe(first);
    expect(manager.getSessionStartedAt()).toBe(Date.now());
    expect(rotations).toStrictEqual([first, second]);
  });

  test("keeps activity and rotation working when storage access is revoked", () => {
    const manager = new SessionManager();
    const first = manager.getSessionId();

    const denied = () => {
      throw new DOMException("Storage access denied", "SecurityError");
    };

    // oxlint-disable-next-line interfere-testing/no-test-doubles -- Browsers cannot revoke this iframe's storage permission in place; reproduce the platform exception after construction.
    vi.spyOn(Storage.prototype, "getItem").mockImplementation(denied);

    // oxlint-disable-next-line interfere-testing/no-test-doubles -- Exercise the platform write exception after a successful storage probe.
    vi.spyOn(Storage.prototype, "setItem").mockImplementation(denied);

    vi.advanceTimersByTime(60_000);

    manager.markActivity();

    expect(manager.getLastActivityAt()).toBe(Date.now());
    expect(manager.getWindowId()).toMatch(WINDOW_ID_REGEX);
    expect(manager.reset()).not.toBe(first);
    expect(manager.getSessionStartedAt()).toBe(Date.now());
  });

  test("works when local and session storage quotas are exhausted", () => {
    fillStorageQuota(localStorage);

    fillStorageQuota(sessionStorage);

    const writes = vi.spyOn(Storage.prototype, "setItem");

    const mgr = new SessionManager();
    const first = mgr.getSessionId();
    const second = mgr.getSessionId();
    const windowId = mgr.getWindowId();

    expect(first).toMatch(UUID_REGEX);
    expect(second).toBe(first);
    expect(windowId).toMatch(WINDOW_ID_REGEX);

    expect(writes).toHaveBeenCalledWith(expect.any(String), expect.any(String));

    expect(writes.mock.results).toHaveLength(2);

    expect(
      writes.mock.results.every(
        (result) =>
          result.type === "throw" &&
          result.value instanceof DOMException &&
          result.value.name === "QuotaExceededError",
      ),
    ).toBe(true);
  });
});
