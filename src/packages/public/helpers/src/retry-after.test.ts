import { parseRetryAfter } from "./retry-after.js";
import { describe, expect, test } from "vitest";

const now = Date.parse("Fri, 30 Oct 2026 12:00:00 GMT");

describe(parseRetryAfter, () => {
  test("reads delay-seconds as milliseconds", () => {
    expect(parseRetryAfter("7")).toBe(7000);
    expect(parseRetryAfter(" 0 ")).toBe(0);
  });

  test("reads an HTTP-date relative to now", () => {
    expect(parseRetryAfter("Fri, 30 Oct 2026 12:00:09 GMT", { now })).toBe(
      9000,
    );
  });

  test("clamps a past HTTP-date to zero", () => {
    expect(parseRetryAfter("Fri, 30 Oct 2026 11:00:00 GMT", { now })).toBe(0);
  });

  test("rejects negative, garbage, empty and missing values", () => {
    expect(parseRetryAfter("-5")).toBeUndefined();
    expect(parseRetryAfter("soon")).toBeUndefined();
    expect(parseRetryAfter("1.5")).toBeUndefined();
    expect(parseRetryAfter("Fri, nonsense", { now })).toBeUndefined();
    expect(parseRetryAfter("")).toBeUndefined();
    expect(parseRetryAfter(null)).toBeUndefined();
    expect(parseRetryAfter(undefined)).toBeUndefined();
    expect(parseRetryAfter("9".repeat(30))).toBeUndefined();
  });

  test("caps seconds and dates at maxMs", () => {
    expect(parseRetryAfter("120", { maxMs: 30_000 })).toBe(30_000);

    expect(
      parseRetryAfter("Fri, 30 Oct 2026 13:00:00 GMT", { maxMs: 30_000, now }),
    ).toBe(30_000);

    expect(parseRetryAfter("5", { maxMs: 30_000 })).toBe(5000);
  });
});
