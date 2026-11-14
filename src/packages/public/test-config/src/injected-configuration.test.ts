import { expect, onTestFinished, test } from "vitest";
import { setInjectedConfiguration } from "./injected-configuration";

const name = "__INTERFERE_TEST_CONFIGURATION__";

test("restores an absent configuration slot after repeated assignments", () => {
  expect.hasAssertions();

  expect(Object.hasOwn(globalThis, name)).toBe(false);

  onTestFinished(() => {
    expect(Object.hasOwn(globalThis, name)).toBe(false);
  });

  setInjectedConfiguration(name, "first");

  setInjectedConfiguration(name, "second");

  expect(Object.getOwnPropertyDescriptor(globalThis, name)?.value).toBe(
    "second",
  );
});

test("restores the original configuration descriptor", () => {
  expect.hasAssertions();

  Object.defineProperty(globalThis, name, {
    configurable: true,
    enumerable: false,
    writable: false,
    value: "original",
  });

  const original = Object.getOwnPropertyDescriptor(globalThis, name);

  onTestFinished(() => {
    try {
      expect(Object.getOwnPropertyDescriptor(globalThis, name)).toStrictEqual(
        original,
      );
    } finally {
      Reflect.deleteProperty(globalThis, name);
    }
  });

  setInjectedConfiguration(name, "updated");

  expect(Object.getOwnPropertyDescriptor(globalThis, name)?.value).toBe(
    "updated",
  );
});
