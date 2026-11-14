import {
  TRACEPARENT_MAX_AGE_SECONDS,
  freshnessStamp,
  isFreshnessStampExpired,
} from "./trace-freshness.js";
import { describe, expect, test } from "vitest";

describe(freshnessStamp, () => {
  test("encodes unix seconds", () => {
    expect(freshnessStamp(1_788_283_623_500)).toBe("1788283623");
  });
});

describe(isFreshnessStampExpired, () => {
  const nowMs = 1_788_283_623_000;

  test("accepts a stamp inside the max age", () => {
    expect(isFreshnessStampExpired(freshnessStamp(nowMs), nowMs)).toBe(false);
  });

  test("rejects a stamp older than the max age", () => {
    const stale = freshnessStamp(
      nowMs - (TRACEPARENT_MAX_AGE_SECONDS + 1) * 1000,
    );

    expect(isFreshnessStampExpired(stale, nowMs)).toBe(true);
  });

  test("accepts a future-dated stamp", () => {
    expect(
      isFreshnessStampExpired(freshnessStamp(nowMs + 300_000), nowMs),
    ).toBe(false);
  });

  test("accepts a malformed stamp", () => {
    expect(isFreshnessStampExpired("not-a-number", nowMs)).toBe(false);
  });
});
