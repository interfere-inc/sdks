import { describe, expect, test } from "vitest";

import {
  EXCEPTION_LIMITS,
  truncateExceptionMessage,
  truncateExceptionStack,
} from "./truncate.ts";

describe("exception truncation helpers", () => {
  const huge = "x".repeat(5 * 1024 * 1024);

  test("truncateExceptionMessage caps long messages and keeps short ones", () => {
    const truncated = truncateExceptionMessage(huge);

    expect(truncated.length).toBeLessThanOrEqual(
      EXCEPTION_LIMITS.messageLength,
    );

    expect(truncated.endsWith("…")).toBe(true);
    expect(truncateExceptionMessage("short")).toBe("short");
  });

  test("truncateExceptionMessage does not split a surrogate pair", () => {
    const emoji = "😀".repeat(EXCEPTION_LIMITS.messageLength);
    const truncated = truncateExceptionMessage(emoji);

    expect(truncated.at(-2)).not.toMatch(/[\uD800-\uDBFF]/u);
    expect(truncated.isWellFormed()).toBe(true);
  });

  test("truncateExceptionStack bounds the message line, frame count and length", () => {
    const frames = Array.from(
      { length: 500 },
      (_, index) =>
        `    at fn${index} (https://shop.example/app.js:${index}:1)`,
    );

    const stack = truncateExceptionStack(
      [`Error: ${huge}`, ...frames].join("\n"),
      huge,
    );

    expect(stack.length).toBeLessThanOrEqual(EXCEPTION_LIMITS.stackLength);
    expect(stack.startsWith(`Error: ${"x".repeat(100)}`)).toBe(true);
    expect(stack).toContain("    at fn0 (https://shop.example/app.js:0:1)");

    expect(
      stack.split("\n").filter((line: string) => line.includes(" at ")),
    ).toHaveLength(EXCEPTION_LIMITS.stackFrames);
  });
});
