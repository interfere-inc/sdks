import { inferRuntimeFromEvidence } from "./runtime-inference";
import { expect, test } from "vitest";

const NODE = {
  hasWindow: false,
  hasDocument: false,
  hasNavigator: false,
  nodeVersion: "22.0.0",
};

test("recognizes a complete DOM before process metadata", () => {
  expect(
    inferRuntimeFromEvidence({
      ...NODE,
      hasWindow: true,
      hasDocument: true,
      hasNavigator: true,
    }),
  ).toBe("browser");
});

test("does not identify a browser from window alone", () => {
  expect(inferRuntimeFromEvidence({ ...NODE, hasWindow: true })).toBe("node");
});

test("recognizes the EdgeRuntime marker before process metadata", () => {
  expect(
    inferRuntimeFromEvidence({ ...NODE, edgeRuntime: "edge-runtime" }),
  ).toBe("edge");
});

test.each([undefined, null, false, 42, {}])(
  "ignores a non-string EdgeRuntime marker %j",
  (edgeRuntime) => {
    expect(inferRuntimeFromEvidence({ ...NODE, edgeRuntime })).toBe("node");
  },
);

test.each([undefined, null, false, 42, {}])(
  "does not identify absent or malformed Node version %j as Node",
  (nodeVersion) => {
    expect(inferRuntimeFromEvidence({ ...NODE, nodeVersion })).toBeNull();
  },
);
