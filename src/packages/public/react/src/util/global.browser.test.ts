import { getGlobal, getStringGlobal, setGlobal } from "./global.js";
import { describe, expect, test, vi } from "vitest";
import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";

describe("injected globals", () => {
  const key = "__INTERFERE_GLOBAL_TEST_VALUE__";

  test("reads and updates a global without claiming its runtime type", () => {
    setInjectedConfiguration(key, 42);

    expect(getGlobal(key)).toBe(42);

    setGlobal(key, "release-1");

    expect(getStringGlobal(key)).toBe("release-1");
  });

  test.each([undefined, null, false, 42, {}, []])(
    "rejects a non-string injected value: %j",
    (value) => {
      setInjectedConfiguration(key, value);

      expect(getStringGlobal(key)).toBeUndefined();
    },
  );

  test("preserves an explicitly empty string", () => {
    setInjectedConfiguration(key, "");

    expect(getStringGlobal(key)).toBe("");
  });

  test("returns undefined for an absent property", () => {
    setInjectedConfiguration(key, undefined);

    Reflect.deleteProperty(globalThis, key);

    expect(getGlobal(key)).toBeUndefined();
  });

  test("preserves object and function identity without serialization", () => {
    const value = { callback: () => "value" };

    setInjectedConfiguration(key, null);

    setGlobal(key, value);

    expect(getGlobal(key)).toBe(value);

    setGlobal(key, value.callback);

    expect(getGlobal(key)).toBe(value.callback);
  });

  test("keeps an explicitly undefined property present", () => {
    setInjectedConfiguration(key, null);

    setGlobal(key, undefined);

    expect(Object.hasOwn(globalThis, key)).toBe(true);
    expect(getGlobal(key)).toBeUndefined();
  });

  test("uses host accessors for reads and writes", () => {
    setInjectedConfiguration(key, null);

    const reads = vi.fn<() => void>();

    function getter() {
      reads();

      return "host-value";
    }

    const setter = vi.fn<(value: string) => void>();

    Object.defineProperty(globalThis, key, {
      configurable: true,
      get: getter,
      set: setter,
    });

    expect(getStringGlobal(key)).toBe("host-value");
    expect(reads).toHaveBeenCalledTimes(1);

    setGlobal(key, "next-value");

    expect(setter).toHaveBeenCalledExactlyOnceWith("next-value");
  });
});
