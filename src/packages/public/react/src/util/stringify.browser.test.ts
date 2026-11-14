import { safeStringify } from "./stringify.js";
import { describe, expect, test } from "vitest";

describe(safeStringify, () => {
  test.each([
    { value: "", expected: "" },
    { value: 'a "quoted" string', expected: 'a "quoted" string' },
    { value: null, expected: "null" },
    { value: undefined, expected: "undefined" },
    { value: true, expected: "true" },
    { value: 42, expected: "42" },
    { value: Number.NaN, expected: "null" },
    { value: 42n, expected: "42" },
    { value: Symbol("context"), expected: "Symbol(context)" },
    { value: { name: "context" }, expected: '{"name":"context"}' },
    { value: ["context", 42], expected: '["context",42]' },
  ])("serializes $value as $expected", ({ value, expected }) => {
    expect(safeStringify(value)).toBe(expected);
  });

  test("uses string conversion for callable values", () => {
    const callback = () => 42;

    expect(safeStringify(callback)).toBe(String(callback));
  });

  test("falls back to string conversion for cycles", () => {
    const value = { entries: new Set() };

    Object.assign(value, { self: value });

    expect(safeStringify(value)).toBe("[object Object]");
  });

  test("falls back when a custom JSON serializer throws", () => {
    const value = {
      toJSON() {
        throw new Error("not serializable");
      },
      toString() {
        return "fallback";
      },
    };

    expect(safeStringify(value)).toBe("fallback");
  });
});
