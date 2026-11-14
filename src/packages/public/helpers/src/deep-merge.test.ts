/* oxlint-disable typescript/no-confusing-void-expression */
import { deepMerge, mergeValues } from "./deep-merge";
import { expect, expectTypeOf, test } from "vitest";

test("clones nested objects and arrays without mutating either input", () => {
  const base = { nested: { left: 1 }, list: [{ value: 1 }] };
  const override = { nested: { right: 2 }, list: [{ value: 2 }] };
  const merged = mergeValues(base, override);

  expect(merged).toStrictEqual({
    nested: { left: 1, right: 2 },
    list: [{ value: 1 }, { value: 2 }],
  });

  expect(merged.nested).not.toBe(base.nested);
  expect(merged.list[0]).not.toBe(base.list[0]);
  expect(merged.list[1]).not.toBe(override.list[0]);
  expect(base).toStrictEqual({ nested: { left: 1 }, list: [{ value: 1 }] });

  expect(override).toStrictEqual({
    nested: { right: 2 },
    list: [{ value: 2 }],
  });
});

test("preserves opaque instances and functions as leaf values", () => {
  const timestamp = new Date("2026-01-01T00:00:00Z");
  const handler = () => "result";
  const original = { timestamp, handler };
  const cloned = deepMerge(original);

  expect(cloned).not.toBe(original);
  expect(cloned.timestamp).toBe(timestamp);
  expect(cloned.handler).toBe(handler);
  expect(cloned.handler()).toBe("result");
});

test("replaces unrelated primitive values with the rightmost input", () => {
  expect(mergeValues(1, "next")).toBe("next");
  expect(mergeValues({ nested: true }, null)).toBeNull();
  expect(mergeValues(false, 0)).toBe(0);
});

test("skips undefined overrides while retaining false, zero and null", () => {
  const merged = deepMerge(
    { enabled: true, count: 1, nested: { keep: true } },
    undefined,
    { enabled: false, count: 0, nested: null },
  );

  expect(merged).toStrictEqual({ enabled: false, count: 0, nested: null });

  expect(mergeValues({ keep: 1 }, { keep: undefined })).toStrictEqual({
    keep: 1,
  });
});

test("concatenates and clones array inputs", () => {
  expect(mergeValues([1, 2], [3])).toStrictEqual([1, 2, 3]);
  expect(mergeValues([1], null)).toStrictEqual([1]);
  expect(mergeValues(null, [2])).toStrictEqual([2]);
});

test("folds multiple overrides in order", () => {
  const merged = deepMerge(
    { count: 1 },
    { count: 2, first: true },
    { count: 3, last: "done" },
  );

  expect(merged).toStrictEqual({ count: 3, first: true, last: "done" });

  expectTypeOf(merged.count).toEqualTypeOf<3>();

  expectTypeOf(merged.first).toEqualTypeOf<true>();

  expectTypeOf(merged.last).toEqualTypeOf<"done">();
});

test("preserves primitive values without overrides", () => {
  expect(deepMerge(42)).toBe(42);
  expect(deepMerge(null)).toBeNull();
  expect(deepMerge(undefined)).toBeUndefined();
});
