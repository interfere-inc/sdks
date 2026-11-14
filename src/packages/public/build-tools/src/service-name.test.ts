import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, beforeEach, expect, test, vi } from "vitest";
import { resolveBuildServiceName } from "./service-name.ts";

let root: string;

beforeEach(() => {
  root = mkdtempSync(join(tmpdir(), "service-name-"));
});

afterEach(() => {
  rmSync(root, { recursive: true, force: true });

  vi.unstubAllEnvs();
});

test.each([
  {
    option: undefined,
    otel: undefined,
    legacy: undefined,
    expected: "platform",
  },
  {
    option: undefined,
    otel: undefined,
    legacy: "legacy-name",
    expected: "platform",
  },
  {
    option: undefined,
    otel: "otel-name",
    legacy: "legacy-name",
    expected: "otel-name",
  },
  {
    option: "custom-name",
    otel: "otel-name",
    legacy: "legacy-name",
    expected: "custom-name",
  },
  { option: " ", otel: " ", legacy: " ", expected: "platform" },
  {
    option: " @customer/custom ",
    otel: "otel-name",
    legacy: undefined,
    expected: "@customer/custom",
  },
])(
  "resolves shared build identity to $expected",
  ({ option, otel, legacy, expected }) => {
    writeFileSync(
      join(root, "package.json"),
      JSON.stringify({ name: " @acme/platform " }),
    );

    expect(
      resolveBuildServiceName({
        root,
        option,
        env: { OTEL_SERVICE_NAME: otel, VITE_INTERFERE_SERVICE_NAME: legacy },
      }),
    ).toBe(expected);
  },
);

test.each([
  undefined,
  "invalid json",
  "null",
  JSON.stringify({}),
  JSON.stringify({ name: 42 }),
  JSON.stringify({ name: " " }),
  JSON.stringify({ name: "@acme/" }),
])("leaves an unusable manifest unnamed: %s", (manifest) => {
  if (manifest !== undefined) {
    writeFileSync(join(root, "package.json"), manifest);
  }

  expect(resolveBuildServiceName({ root, env: {} })).toBeUndefined();
});

test("reads OTEL_SERVICE_NAME from the build process when no environment is supplied", () => {
  vi.stubEnv("OTEL_SERVICE_NAME", " process-name ");

  expect(resolveBuildServiceName({ root })).toBe("process-name");
});

test("does not use framework-specific variables when resolving the app name", () => {
  writeFileSync(
    join(root, "package.json"),
    JSON.stringify({ name: "platform" }),
  );

  expect(
    resolveBuildServiceName({
      root,
      env: { VITE_INTERFERE_SERVICE_NAME: "vite-name" },
    }),
  ).toBe("platform");
});
