import type { ExceptionValue } from "./plugins/payload/errors";

import { isNonErrorException, toException, toExceptions } from "./errors.ts";
import { EXCEPTION_LIMITS } from "./errors/truncate.ts";
import { describe, expect, test } from "vitest";

function asErrorVariant(e: ExceptionValue | undefined) {
  if (!e || e.kind === "non-error") {
    throw new Error("expected error variant exception");
  }

  return e;
}

describe("exception payload bounds", () => {
  const huge = "x".repeat(5 * 1024 * 1024);

  test("toExceptions bounds every entry's value and stack", () => {
    expect.hasAssertions();

    const error = new Error(huge, { cause: new Error(huge) });

    const exceptions = toExceptions(error, { handled: true, type: "test" });

    expect(exceptions).toHaveLength(2);

    for (const entry of exceptions) {
      const { stack, value } = asErrorVariant(entry);

      expect(value.length).toBeLessThanOrEqual(EXCEPTION_LIMITS.messageLength);
      expect(stack.length).toBeLessThanOrEqual(EXCEPTION_LIMITS.stackLength);
    }
  });

  test("toException bounds serialized payloads by breadth and length, not only depth", () => {
    const wide = Object.fromEntries(
      Array.from({ length: 10_000 }, (_, index) => [
        `key${index}`,
        huge.slice(0, 500),
      ]),
    );

    const result = toException(wide);

    if (!isNonErrorException(result)) {
      throw new Error("expected a non-error exception");
    }

    expect(JSON.stringify(result.serialized).length).toBeLessThanOrEqual(
      EXCEPTION_LIMITS.serializedLength,
    );

    expect(result.value.length).toBeLessThanOrEqual(
      EXCEPTION_LIMITS.messageLength,
    );

    expect(result.serialized).toHaveProperty("key0");
  });

  test("toException bounds a long thrown string leaf", () => {
    const result = toException({ body: huge });

    if (!isNonErrorException(result)) {
      throw new Error("expected a non-error exception");
    }

    expect(JSON.stringify(result.serialized).length).toBeLessThanOrEqual(
      EXCEPTION_LIMITS.serializedLength,
    );
  });
});
