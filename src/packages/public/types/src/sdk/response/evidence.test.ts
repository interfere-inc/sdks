import { describe, expect, test } from "vitest";

import {
  readJsonResponse,
  redactErrorDetail,
  responseErrorDetail,
  responseSkipReason,
} from "./evidence.ts";

describe(responseErrorDetail, () => {
  test.each([
    ["message", { message: "Quota exceeded" }, "Quota exceeded"],
    [
      "detail",
      { detail: "Field amount is required" },
      "Field amount is required",
    ],
    ["title", { title: "Unprocessable Entity" }, "Unprocessable Entity"],
    ["a string error", { error: "invalid_grant" }, "invalid_grant"],
    ["error.message", { error: { message: "Bad key" } }, "Bad key"],
    ["errors[0].message", { errors: [{ message: "Not found" }] }, "Not found"],
  ])("reads %s", (_name, body, detail) => {
    expect(responseErrorDetail(body)).toBe(detail);
  });

  test("prefers message over the other fields and skips blank values", () => {
    expect(
      responseErrorDetail({ message: "  ", detail: "Second", title: "Third" }),
    ).toBe("Second");
  });

  test("returns nothing for bodies without a message-like field", () => {
    expect(responseErrorDetail({ code: "E1", data: null })).toBeNull();
    expect(responseErrorDetail(["message"])).toBeNull();
    expect(responseErrorDetail("message")).toBeNull();
  });

  test("redacts before cutting to 256 characters", () => {
    const detail = responseErrorDetail({
      message: `user jane.doe@example.com ${"x".repeat(400)}`,
    });

    expect(detail).toHaveLength(256);
    expect(detail?.startsWith("user [email] x")).toBe(true);
    expect(detail?.endsWith("…")).toBe(true);
  });
});

describe(redactErrorDetail, () => {
  test("masks emails, card numbers and phone numbers", () => {
    expect(
      redactErrorDetail(
        "jane@example.com paid with 4111 1111 1111 1111, call +14155550123 or (415) 555-0123",
      ),
    ).toBe("[email] paid with [card], call [phone] or [phone]");
  });

  test("keeps dates, decimals, short numbers and identifiers", () => {
    const text =
      "Order 12345 failed on 2026-11-26 for amount 12.50 (code ERR_429, request req_01HZX)";

    expect(redactErrorDetail(text)).toBe(text);
  });
});

describe(responseSkipReason, () => {
  const headers = (values: Record<string, string>) => (name: string) =>
    values[name] ?? null;

  test("reads JSON without a length only when the stream read enforces the bound", () => {
    const chunked = headers({ "content-type": "application/problem+json" });

    expect(responseSkipReason(chunked, { lengthRequired: false })).toBeNull();

    expect(responseSkipReason(chunked, { lengthRequired: true })).toBe(
      "no-content-length",
    );
  });

  test("skips declared bodies over 64 KiB and non-JSON bodies", () => {
    expect(
      responseSkipReason(
        headers({
          "content-length": String(64 * 1024 + 1),
          "content-type": "application/json",
        }),
        { lengthRequired: false },
      ),
    ).toBe("over-64KiB");

    expect(
      responseSkipReason(headers({ "content-type": "text/html" }), {
        lengthRequired: false,
      }),
    ).toBe("non-json");
  });
});

describe(readJsonResponse, () => {
  const cancelFailed = () => {
    throw new Error("clone cancellation failed");
  };

  test("parses a clone and leaves the application's body unread", async () => {
    const response = Response.json(
      { message: "Quota exceeded" },
      { status: 429 },
    );

    await expect(
      readJsonResponse(response, cancelFailed),
    ).resolves.toStrictEqual({
      body: { message: "Quota exceeded" },
    });

    expect(response.bodyUsed).toBe(false);

    await expect(response.json()).resolves.toStrictEqual({
      message: "Quota exceeded",
    });
  });

  test("stops reading past 64 KiB", async () => {
    const response = Response.json({ message: "x".repeat(64 * 1024) });

    await expect(
      readJsonResponse(response, cancelFailed),
    ).resolves.toStrictEqual({
      skipped: "over-64KiB",
    });
  });
});
