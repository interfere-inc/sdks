import { enrichedFrameSchema } from "./error";
import { describe, expect, test } from "vitest";

const frame = {
  column: 20,
  file: "bundle.js",
  filePath: "app.ts",
  fileReference: "app.ts",
  fn: "a",
  inApp: true,
  ingestedId: "frame-0",
  line: 1,
  originalColumn: 2,
  originalLine: 7,
  originalName: "checkout",
  resolved: true,
};

describe("enriched frame source context", () => {
  test("accepts existing resolved frames without excerpts", () => {
    expect(enrichedFrameSchema.parse(frame)).toStrictEqual(frame);
  });

  test("preserves embedded source and its original line numbering", () => {
    const enriched = {
      ...frame,
      sourceContext: { code: "checkout();\nthrow new Error();", startLine: 6 },
    };

    expect(enrichedFrameSchema.parse(enriched)).toStrictEqual(enriched);
  });

  test.each([0, -1, 1.5])(
    "rejects invalid excerpt starting line %i",
    (startLine) => {
      expect(
        enrichedFrameSchema.safeParse({
          ...frame,
          sourceContext: { code: "checkout();", startLine },
        }).success,
      ).toBe(false);
    },
  );
});
