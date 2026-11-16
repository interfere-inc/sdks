import {
  browserStorage,
  fillLocalStorageQuota,
} from "@interfere/test-config/browser-storage";
/* oxlint-disable typescript/require-await */
import { DeviceManager } from "./device.js";
import type { FingerprintProvider } from "./device.js";
import { afterEach, describe, expect, test, vi } from "vitest";

function fixedFp(value: string | null) {
  return (async () => value) satisfies FingerprintProvider;
}

function failingFp() {
  return (async () => {
    throw new Error("fp failed");
  }) satisfies FingerprintProvider;
}

const VISITOR_ID_RE = /^[a-f0-9]{16,}$/iu;

describe(DeviceManager, () => {
  afterEach(() => {
    vi.restoreAllMocks();

    vi.unstubAllGlobals();
  });

  test("creates and rotates device ids without the secure-context randomUUID API", async () => {
    // oxlint-disable-next-line interfere-testing/no-test-doubles -- Secure-context API availability must be removed while retaining real getRandomValues.
    vi.stubGlobal("crypto", {
      getRandomValues: crypto.getRandomValues.bind(crypto),
    });

    const manager = new DeviceManager(fixedFp(null));

    manager.init();

    const first = manager.getDeviceId();
    const second = manager.reset();

    expect(first).toMatch(/^[a-f0-9-]{36}$/u);
    expect(second).toMatch(/^[a-f0-9-]{36}$/u);
    expect(second).not.toBe(first);
    expect(localStorage.getItem("interfere:device_id")).toBe(second);

    await manager.whenFingerprintReady();
  });

  test("generates and persists a device id in localStorage", async () => {
    const dm = new DeviceManager();

    dm.init();

    const id = dm.getDeviceId();

    expect(id).toMatch(/\S/u);
    expect(id).toBeTypeOf("string");
    expect(localStorage.getItem("interfere:device_id")).toBe(id);

    await dm.whenFingerprintReady();
  });

  test("restores device id from localStorage on subsequent inits", async () => {
    browserStorage.local.set("interfere:device_id", "existing-device-123");

    const dm = new DeviceManager();

    dm.init();

    expect(dm.getDeviceId()).toBe("existing-device-123");

    await dm.whenFingerprintReady();
  });

  test("writes device id to cookie as fallback", async () => {
    const dm = new DeviceManager();

    dm.init();

    const id = dm.getDeviceId();

    expect(document.cookie).toContain(`interfere_did=${id}`);

    await dm.whenFingerprintReady();
  });

  test("restores the cookie identity when localStorage has reached its quota", async () => {
    const first = new DeviceManager();

    first.init();

    await first.whenFingerprintReady();

    const id = first.getDeviceId();

    browserStorage.local.remove("interfere:device_id");

    fillLocalStorageQuota();

    const restored = new DeviceManager();

    restored.init();

    await restored.whenFingerprintReady();

    expect(id).not.toBeNull();
    expect(restored.getDeviceId()).toBe(id);
    expect(document.cookie).toContain(`interfere_did=${id}`);
  });

  test("computes fpHash asynchronously", async () => {
    const dm = new DeviceManager();

    dm.init();

    const hash = await dm.whenFingerprintReady();

    expect(hash).toMatch(VISITOR_ID_RE);
    expect(dm.getFpHash()).toBe(hash);
  });

  test("handles FP OSS failure gracefully — but a thrown provider rejects", async () => {
    const dm = new DeviceManager(failingFp());

    dm.init();

    await expect(dm.whenFingerprintReady()).rejects.toThrow("fp failed");

    expect(dm.getDeviceId()).toMatch(/\S/u);
    expect(dm.getFpHash()).toBeNull();
  });

  test("returns null hash when provider resolves null", async () => {
    const dm = new DeviceManager(fixedFp(null));

    dm.init();

    await expect(dm.whenFingerprintReady()).resolves.toBeNull();

    expect(dm.getFpHash()).toBeNull();
  });

  test("each instance owns its own state", async () => {
    browserStorage.local.set("interfere:device_id", "seeded-id");

    const a = new DeviceManager(fixedFp("hash-a"));

    a.init();

    await a.whenFingerprintReady();

    const b = new DeviceManager(fixedFp("hash-b"));

    expect(b.getDeviceId()).toBeNull();
    expect(b.getFpHash()).toBeNull();

    b.init();

    await b.whenFingerprintReady();

    expect(a.getDeviceId()).toBe("seeded-id");
    expect(b.getDeviceId()).toBe("seeded-id");
    expect(a.getFpHash()).toBe("hash-a");
    expect(b.getFpHash()).toBe("hash-b");
  });

  test("rotates device id and persistence on reset", async () => {
    const dm = new DeviceManager();

    dm.init();

    const originalId = dm.getDeviceId();

    expect(originalId).toMatch(/\S/u);

    const rotated = dm.reset();

    expect(rotated).not.toBe(originalId);
    expect(dm.getDeviceId()).toBe(rotated);
    expect(localStorage.getItem("interfere:device_id")).toBe(rotated);
    expect(document.cookie).toContain(`interfere_did=${rotated}`);

    await dm.whenFingerprintReady();
  });

  test("preserves fpHash across reset", async () => {
    const dm = new DeviceManager();

    dm.init();

    const hash = await dm.whenFingerprintReady();

    expect(hash).toMatch(VISITOR_ID_RE);

    expect(dm.getFpHash()).toBe(hash);

    dm.reset();

    expect(dm.getFpHash()).toBe(hash);
  });

  describe("default provider", () => {
    test("gates fingerprint work behind requestIdleCallback", async () => {
      const ric = vi.spyOn(globalThis, "requestIdleCallback");
      const dm = new DeviceManager();

      dm.init();

      expect(ric).toHaveBeenCalledExactlyOnceWith(
        expect.any(Function),
        expect.objectContaining({ timeout: 5000 }),
      );

      expect(dm.getFpHash()).toBeNull();

      await dm.whenFingerprintReady();

      expect(dm.getFpHash()).toMatch(VISITOR_ID_RE);
    });
  });

  describe("default provider — live FingerprintJS in chromium", () => {
    test("returns a non-empty visitor id end-to-end", async () => {
      const dm = new DeviceManager();

      dm.init();

      const hash = await dm.whenFingerprintReady();

      expect(hash).toMatch(/\S/u);
      expect(hash).toBeTypeOf("string");
      expect(hash).toMatch(VISITOR_ID_RE);
    });

    test("is stable across DeviceManager instances within the same browser context", async () => {
      const a = new DeviceManager();

      a.init();

      const hashA = await a.whenFingerprintReady();

      const b = new DeviceManager();

      b.init();

      const hashB = await b.whenFingerprintReady();

      expect(hashA).toMatch(/\S/u);
      expect(hashB).toBe(hashA);
    });

    test("skips the font, canvas and WebGL extension probes that each run as a long task", async () => {
      const createElement = vi.spyOn(document, "createElement");
      const toDataURL = vi.spyOn(HTMLCanvasElement.prototype, "toDataURL");

      const extensions = vi.spyOn(
        WebGLRenderingContext.prototype,
        "getSupportedExtensions",
      );

      const dm = new DeviceManager();

      dm.init();

      await expect(dm.whenFingerprintReady()).resolves.toMatch(VISITOR_ID_RE);

      expect(
        createElement.mock.calls.filter(([tag]) => tag === "iframe"),
      ).toStrictEqual([]);

      expect(toDataURL).not.toHaveBeenCalled();
      expect(extensions).not.toHaveBeenCalled();
    });
  });
});
