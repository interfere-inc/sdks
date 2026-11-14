import { expect, test } from "vitest";
import config from "../tsdown.config";

test("passes glob entries to rolldown in a stable order", () => {
  const entries = {
    provider: "/src/provider.tsx",
    "internal/otel/exporter": "/src/internal/otel/exporter.ts",
    api: "/src/api.ts",
    "internal/kernel": "/src/internal/kernel.ts",
  };

  const order = (input: Record<string, string>) =>
    Object.keys(config.inputOptions({ input }).input);

  const expected = [
    "api",
    "internal/kernel",
    "internal/otel/exporter",
    "provider",
  ];

  expect(order(entries)).toStrictEqual(expected);

  expect(
    order(Object.fromEntries(Object.entries(entries).toReversed())),
  ).toStrictEqual(expected);
});
