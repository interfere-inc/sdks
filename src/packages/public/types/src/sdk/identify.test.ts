import { identifyParamsSchema } from "./identify";
import { describe, expect, test } from "vitest";

describe("identifyParamsSchema contract", () => {
  test("accepts an omitted company", () => {
    expect(
      identifyParamsSchema.parse({
        identifier: "customer-123",
        source: { name: "Internal", type: "custom" },
      }),
    ).not.toHaveProperty("company");
  });

  test.each(["A", "Enterprise", "a".repeat(100)])(
    "trims and accepts a company tier at the allowed lengths: %s",
    (tier) => {
      expect(
        identifyParamsSchema.parse({
          company: { tier: ` ${tier} ` },
          identifier: "customer-123",
          source: { name: "Internal", type: "custom" },
        }).company,
      ).toStrictEqual({ tier });
    },
  );

  test.each([
    {},
    { tier: "" },
    { tier: "   " },
    { tier: "a".repeat(101) },
    { tier: "Enterprise", extra: true },
    { tier: null },
    { tier: 1 },
  ])("rejects an invalid company: %j", (company) => {
    expect(
      identifyParamsSchema.safeParse({
        company,
        identifier: "customer-123",
        source: { name: "Internal", type: "custom" },
      }).success,
    ).toBe(false);
  });

  test("requires Clerk identifiers to use raw Clerk user ids", () => {
    expect(
      identifyParamsSchema.safeParse({
        identifier: "internal-user-id",
        source: { name: "Clerk", type: "clerk" },
      }).success,
    ).toBe(false);

    expect(
      identifyParamsSchema.safeParse({
        identifier: "user_123",
        source: { name: "Clerk", type: "clerk" },
      }).success,
    ).toBe(true);
  });

  test("allows non-Clerk identifiers to remain provider-defined", () => {
    expect(
      identifyParamsSchema.safeParse({
        identifier: "internal-user-id",
        source: { name: "Internal", type: "custom" },
      }).success,
    ).toBe(true);
  });
});

describe("identifyParamsSchema company", () => {
  const base = {
    identifier: "internal-user-id",
    source: { name: "Internal", type: "custom" },
  };

  test("trims the company tier", () => {
    expect(
      identifyParamsSchema.parse({
        ...base,
        company: { tier: " Enterprise " },
      }),
    ).toMatchObject({ company: { tier: "Enterprise" } });
  });

  test.each([
    { tier: " " },
    { tier: "x".repeat(101) },
    { tier: "Enterprise", name: "Acme" },
    {},
  ])("rejects invalid company payloads: %j", (company) => {
    expect(identifyParamsSchema.safeParse({ ...base, company }).success).toBe(
      false,
    );
  });
});
