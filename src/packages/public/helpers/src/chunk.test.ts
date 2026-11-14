import { expect, test } from "vitest";
import { chunk } from "./chunk";

test("preserves order, trailing items and its input", () => {
  const items = Object.freeze([1, 2, 3, 4, 5]);

  expect(chunk(items, 2)).toStrictEqual([[1, 2], [3, 4], [5]]);
  expect(chunk([], 2)).toStrictEqual([]);
  expect(chunk(items, 10)).toStrictEqual([[1, 2, 3, 4, 5]]);
});

test.each([0, -1, 1.5, Infinity, Number.NaN])(
  "rejects invalid chunk size %s",
  (size) => {
    expect(() => chunk([1], size)).toThrow(RangeError);
  },
);
