import { parseEnvValue, readFirstEnvValue } from "./env.ts";
import { describe, expect, test } from "vitest";

describe(parseEnvValue, () => {
  test("returns null for invalid or empty inputs", () => {
    expect(parseEnvValue(undefined)).toBeNull();
    expect(parseEnvValue(null)).toBeNull();
    expect(parseEnvValue(123)).toBeNull();
    expect(parseEnvValue("")).toBeNull();
    expect(parseEnvValue("   ")).toBeNull();
  });

  test("returns trimmed string values", () => {
    expect(parseEnvValue("  value_123  ")).toBe("value_123");
  });
});

describe(readFirstEnvValue, () => {
  test("returns first non-empty key by precedence order", () => {
    const env = {
      FALLBACK: "fallback",
      PRIMARY: "",
      SECONDARY: "  second  ",
    };

    expect(readFirstEnvValue(env, ["PRIMARY", "SECONDARY", "FALLBACK"])).toBe(
      "second",
    );
  });
});
