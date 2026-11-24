import { afterEach, expect, test, vi } from "vitest";
import { DeviceManager } from "./device.js";
import { getLogLevel, setLogLevel } from "../util/log.js";

afterEach(() => {
  vi.restoreAllMocks();
});

test("reports unavailable fingerprint APIs without logging a customer exception", async () => {
  const previousLevel = getLogLevel();
  const errors = vi.spyOn(console, "error");
  const manager = new DeviceManager();

  setLogLevel("error");

  try {
    manager.init();

    await expect(manager.whenFingerprintReady()).resolves.toBeNull();

    expect(manager.getDeviceId()).toBeTypeOf("string");

    expect(errors).toHaveBeenCalledWith(
      "[Interfere:device]",
      "fingerprint init failed; visitor identity will be cookie-only",
      expect.any(String),
    );

    expect(
      errors.mock.calls.flat().some((value) => value instanceof Error),
    ).toBe(false);
  } finally {
    setLogLevel(previousLevel);
  }
});
