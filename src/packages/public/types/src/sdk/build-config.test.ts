import { buildConfigSchema } from "./build-config";
import { expect, test } from "vitest";

test("retains valid build settings when another field is invalid", () => {
  expect(
    buildConfigSchema.parse({
      backendHosts: ["backend.customer.test"],
      ingestUrl: "https://ingest.example.com",
      logSourcing: "drain",
      platform: "vercel",
      plugins: { errors: false },
      serverTracing: "unsupported",
    }),
  ).toStrictEqual({
    backendHosts: ["backend.customer.test"],
    ingestUrl: "https://ingest.example.com",
    logSourcing: "drain",
    platform: "vercel",
    plugins: { errors: false },
    serverTracing: undefined,
  });
});

test.each([null, [], "invalid"])(
  "falls back to defaults for an invalid config %j",
  (input) => {
    expect(buildConfigSchema.parse(input)).toStrictEqual({});
  },
);
