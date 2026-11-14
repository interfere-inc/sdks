import { inferRuntime, readProcessEnv } from "./runtime";
import { expect, test } from "vitest";

test("detects the native browser", () => {
  expect(inferRuntime()).toBe("browser");
  expect(readProcessEnv()).toBeUndefined();
});
