import { isUrlIgnored } from "@opentelemetry/core";

import { INGEST_IGNORE_RE } from "./exporter.js";
import { describe, expect, test } from "vitest";

describe("INGEST_IGNORE_RE contract", () => {
  test.each([
    ["traces", "https://api.interfere.com"],
    ["logs", "https://api.interfere.com/"],
    ["metrics", "https://api.interfere.com"],
    ["traces", "https://in.example.com/api/interfere"],
  ] as const)(
    "ignores the %s URL the exporter actually posts to",
    (signal, base) => {
      expect(
        isUrlIgnored(`${base.replace(/\/$/u, "")}/drain/v1/${signal}`, [
          INGEST_IGNORE_RE,
        ]),
      ).toBe(true);
    },
  );

  test("leaves customer traffic instrumented", () => {
    expect(isUrlIgnored("https://app.test/api/items", [INGEST_IGNORE_RE])).toBe(
      false,
    );
  });
});
