import {
  inferRuntime,
  normalizeEnv,
  readProcessEnv,
  resolveEnvironment,
} from "./runtime.ts";
import { describe, expect, test } from "vitest";

describe(resolveEnvironment, () => {
  test("reports a production build as production when no variable is set", () => {
    expect(resolveEnvironment({ buildMode: "production", env: {} })).toBe(
      "production",
    );
  });

  test("reports the dev server's mode as development", () => {
    expect(resolveEnvironment({ buildMode: "development", env: {} })).toBe(
      "development",
    );
  });

  test("falls back to development without a variable or a build mode", () => {
    expect(resolveEnvironment({ env: {} })).toBe("development");
  });

  test("prefers the explicit value and every variable over the build mode", () => {
    expect(
      resolveEnvironment({ buildMode: "production", explicit: "preview" }),
    ).toBe("preview");

    expect(
      resolveEnvironment({
        buildMode: "production",
        env: { INTERFERE_ENVIRONMENT: "staging", NODE_ENV: "production" },
      }),
    ).toBe("staging");

    expect(
      resolveEnvironment({
        buildMode: "production",
        env: { NODE_ENV: "test" },
      }),
    ).toBe("test");
  });
});

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
