import { describe, expect, test } from "vitest";

import { omitUndefined } from "./omit-undefined";

describe(omitUndefined, () => {
  test("removes undefined values and preserves other falsy values", () => {
    expect(
      omitUndefined({
        missing: undefined,
        nil: null,
        empty: "",
        zero: 0,
        disabled: false,
      }),
    ).toStrictEqual({ nil: null, empty: "", zero: 0, disabled: false });
  });

  test("does not mutate its input or clone nested values", () => {
    const nested = { value: 1 };
    const input = Object.freeze({ nested, missing: undefined });

    const result = omitUndefined(input);

    expect(result.nested).toBe(nested);
    expect(result).not.toHaveProperty("missing");
    expect(input).toHaveProperty("missing");
  });

  test("preserves special property names as own data without changing the result prototype", () => {
    const prototypeValue = { injected: true };
    const input = { constructor: "customer constructor", missing: undefined };

    Object.defineProperty(input, "__proto__", {
      enumerable: true,
      value: prototypeValue,
    });

    const result = omitUndefined(input);

    expect(Object.getPrototypeOf(result)).toBe(Object.prototype);

    expect(Object.getOwnPropertyDescriptor(result, "__proto__")).toStrictEqual({
      configurable: true,
      enumerable: true,
      value: prototypeValue,
      writable: true,
    });

    expect(Object.hasOwn(result, "constructor")).toBe(true);
    expect(result.constructor).toBe("customer constructor");
    expect(result).not.toHaveProperty("injected");
  });

  test("copies only own enumerable string keys and evaluates getters once", () => {
    let reads = 0;

    function read() {
      reads += 1;

      return "value";
    }

    const symbol = Symbol("private");
    const input = { [symbol]: "symbol value" };

    Object.defineProperty(input, "visible", { enumerable: true, get: read });

    Object.defineProperty(input, "hidden", { value: "hidden value" });

    expect(omitUndefined(input)).toStrictEqual({ visible: "value" });
    expect(reads).toBe(1);
  });

  test("retains native entry ordering after removing undefined properties", () => {
    const result = omitUndefined({
      z: 1,
      10: "ten",
      2: "two",
      skipped: undefined,
      a: 2,
    });

    expect(Object.keys(result)).toStrictEqual(["2", "10", "z", "a"]);
  });
});
