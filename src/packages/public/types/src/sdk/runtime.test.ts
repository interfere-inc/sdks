import { inferRuntime, normalizeEnv, readProcessEnv } from "./runtime.ts";
import { describe, expect, test } from "vitest";

describe(inferRuntime, () => {
  test("detects node from process.versions.node", () => {
    expect(inferRuntime()).toBe("node");
  });

  test("detects edge from the nextRuntime option", () => {
    expect(inferRuntime({ nextRuntime: "edge" })).toBe("edge");
  });
});

describe(normalizeEnv, () => {
  test("lowercases and trims string values", () => {
    expect(normalizeEnv("  DRAIN ")).toBe("drain");
  });

  test("normalizes blank and non-string values to null", () => {
    expect(normalizeEnv("   ")).toBeNull();
    expect(normalizeEnv(42)).toBeNull();
    expect(normalizeEnv(undefined)).toBeNull();
  });
});

describe(readProcessEnv, () => {
  test("returns process.env when the runtime provides it", () => {
    expect(readProcessEnv()).toBe(process.env);
  });
});
