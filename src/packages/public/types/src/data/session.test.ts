import { describe, expect, test } from "vitest";
import {
  sessionIdSchema,
  sessionIdWirePattern,
  storedSessionIdSchema,
} from "./session";

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

test.each([
  "019f4cac-2178-768b-ac0a-5b98b34d4c58",
  "019F4CAC-2178-768B-AC0A-5B98B34D4C58",
  "server_019f4cac-2178-768b-ac0a-5b98b34d4c58",
  "9d1e3f2a-8c4b-4f6e-9a7d-1b2c3d4e5f60",
  "server_9d1e3f2a-8c4b-4f6e-9a7d-1b2c3d4e5f60",
  "legacy-session",
  "",
])("sessionIdWirePattern agrees with sessionIdSchema for %j", (value) => {
  expect(new RegExp(sessionIdWirePattern, "u").test(value)).toBe(
    sessionIdSchema.safeParse(value).success,
  );
});
