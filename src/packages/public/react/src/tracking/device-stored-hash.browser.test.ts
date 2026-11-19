import { browserStorage } from "@interfere/test-config/browser-storage";
/* oxlint-disable typescript/require-await */
import { DeviceManager } from "./device.js";
import type { FingerprintProvider } from "./device.js";
import { afterEach, describe, expect, test, vi } from "vitest";

function fixedFp(value: string | null) {
  return (async () => value) satisfies FingerprintProvider;
}

function countingFp(value: string | null) {
  let calls = 0;

  return {
    calls: () => calls,
    run: (async () => {
      calls += 1;

      return value;
    }) satisfies FingerprintProvider,
  };
}

afterEach(() => {
  vi.restoreAllMocks();
});

describe("stored fpHash", () => {
  test("the first visit computes the hash and stores it next to the device id", async () => {
    const provider = countingFp("hash-first");
    const dm = new DeviceManager(provider.run);

    dm.init();

    await expect(dm.whenFingerprintReady()).resolves.toBe("hash-first");

    expect(provider.calls()).toBe(1);

    expect(localStorage.getItem("interfere:fp_hash")).toBe(
      `${dm.getDeviceId()}:hash-first`,
    );
  });

  test("a later visit reuses the stored hash without running the provider", async () => {
    const first = new DeviceManager(fixedFp("hash-stored"));

    first.init();

    await first.whenFingerprintReady();

    const provider = countingFp("hash-recomputed");
    const later = new DeviceManager(provider.run);

    later.init();

    expect(later.getFpHash()).toBe("hash-stored");

    await expect(later.whenFingerprintReady()).resolves.toBe("hash-stored");

    expect(provider.calls()).toBe(0);
  });

  test("a later visit does not import FingerprintJS", async () => {
    const first = new DeviceManager();

    first.init();

    const hash = await first.whenFingerprintReady();

    const ric = vi.spyOn(globalThis, "requestIdleCallback");
    const later = new DeviceManager();

    later.init();

    await expect(later.whenFingerprintReady()).resolves.toBe(hash);

    expect(ric).not.toHaveBeenCalled();
  });

  test("a cleared device id recomputes the hash", async () => {
    const first = new DeviceManager(fixedFp("hash-old"));

    first.init();

    await first.whenFingerprintReady();

    browserStorage.local.remove("interfere:device_id");

    browserStorage.cookies.remove("interfere_did");

    const provider = countingFp("hash-new");
    const later = new DeviceManager(provider.run);

    later.init();

    await expect(later.whenFingerprintReady()).resolves.toBe("hash-new");

    expect(provider.calls()).toBe(1);
    expect(later.getDeviceId()).not.toBe(first.getDeviceId());

    expect(localStorage.getItem("interfere:fp_hash")).toBe(
      `${later.getDeviceId()}:hash-new`,
    );
  });

  test("a hash stored for another device id is recomputed", async () => {
    browserStorage.local.set("interfere:device_id", "device-b");

    browserStorage.local.set("interfere:fp_hash", "device-a:hash-a");

    const provider = countingFp("hash-b");
    const dm = new DeviceManager(provider.run);

    dm.init();

    await expect(dm.whenFingerprintReady()).resolves.toBe("hash-b");

    expect(provider.calls()).toBe(1);
  });

  test("clear removes the stored hash with the device id", async () => {
    const dm = new DeviceManager(fixedFp("hash-cleared"));

    dm.init();

    await dm.whenFingerprintReady();

    dm.clear();

    expect(localStorage.getItem("interfere:device_id")).toBeNull();
    expect(localStorage.getItem("interfere:fp_hash")).toBeNull();
  });

  test("a clear while the hash is computing stores nothing", async () => {
    const fingerprint = Promise.withResolvers<string | null>();
    const dm = new DeviceManager(async () => fingerprint.promise);

    dm.init();

    const pending = dm.whenFingerprintReady();

    dm.clear();

    fingerprint.resolve("hash-late");

    await expect(pending).resolves.toBeNull();

    expect(localStorage.getItem("interfere:fp_hash")).toBeNull();
  });

  test("reset rebinds the stored hash to the new device id", async () => {
    const dm = new DeviceManager(fixedFp("hash-kept"));

    dm.init();

    await dm.whenFingerprintReady();

    const rotated = dm.reset();

    expect(localStorage.getItem("interfere:fp_hash")).toBe(
      `${rotated}:hash-kept`,
    );
  });
});
