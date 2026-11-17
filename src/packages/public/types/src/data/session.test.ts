import { describe, expect, test } from "vitest";
import {
  pageSessionIdSchema,
  sessionIdSchema,
  storedSessionIdSchema,
} from "./session";
import {
  isSessionId,
  sessionIdWirePattern,
  toPageSessionId,
  toSessionId,
} from "./session-id";

describe("session identity validation", () => {
  test.each([
    "019f4cac-2178-768b-ac0a-5b98b34d4c58",
    "server_019f4cac-2178-768b-ac0a-5b98b34d4c58",
  ])("accepts wire identity %s", (value) => {
    expect(sessionIdSchema.parse(value)).toBe(value);
    expect(storedSessionIdSchema.parse(value)).toBe(value);
  });

  test("preserves legacy storage identities without accepting them on the wire", () => {
    expect(storedSessionIdSchema.parse("legacy-session")).toBe(
      "legacy-session",
    );

    expect(sessionIdSchema.safeParse("legacy-session").success).toBe(false);
    expect(storedSessionIdSchema.safeParse("").success).toBe(false);
  });
});

function mints(mint: (value: string) => string, value: string) {
  try {
    return mint(value) === value;
  } catch {
    return false;
  }
}

const candidates = [
  "019f4cac-2178-768b-ac0a-5b98b34d4c58",
  "019F4CAC-2178-768B-AC0A-5B98B34D4C58",
  "server_019f4cac-2178-768b-ac0a-5b98b34d4c58",
  "page_019f4cac-2178-768b-ac0a-5b98b34d4c58",
  "page_9d1e3f2a-8c4b-4f6e-9a7d-1b2c3d4e5f60",
  "9d1e3f2a-8c4b-4f6e-9a7d-1b2c3d4e5f60",
  "server_9d1e3f2a-8c4b-4f6e-9a7d-1b2c3d4e5f60",
  "019f4cac-2178-768b-ac0a-5b98b34d4c58\n",
  "legacy-session",
  "",
];

test.each(candidates)(
  "sessionIdWirePattern agrees with sessionIdSchema for %j",
  (value) => {
    expect(new RegExp(sessionIdWirePattern, "u").test(value)).toBe(
      sessionIdSchema.safeParse(value).success,
    );
  },
);

test.each(candidates)(
  "isSessionId agrees with sessionIdSchema for %j",
  (value) => {
    expect(isSessionId(value)).toBe(sessionIdSchema.safeParse(value).success);
  },
);

test.each(candidates)(
  "toPageSessionId agrees with pageSessionIdSchema for %j",
  (value) => {
    expect(mints(toPageSessionId, value)).toBe(
      pageSessionIdSchema.safeParse(value).success,
    );
  },
);

test("isSessionId rejects a missing stored id", () => {
  expect(isSessionId(null)).toBe(false);
  expect(isSessionId(undefined)).toBe(false);
});

test("toSessionId and toPageSessionId reject malformed ids", () => {
  expect(toSessionId("019f4cac-2178-768b-ac0a-5b98b34d4c58")).toBe(
    "019f4cac-2178-768b-ac0a-5b98b34d4c58",
  );

  expect(() => toSessionId("legacy-session")).toThrow(TypeError);

  expect(() => toPageSessionId("019f4cac-2178-768b-ac0a-5b98b34d4c58")).toThrow(
    TypeError,
  );
});
