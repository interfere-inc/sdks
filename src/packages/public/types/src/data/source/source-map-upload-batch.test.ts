import { describe, expect, test } from "vitest";

import {
  SOURCE_MAP_BATCH_FILES,
  SOURCE_MAP_UPLOAD_BYTES,
  sourceMapUploadBatch,
} from "./source-map-upload-batch";

const chunk = { path: "app.js.map", offset: 0, bytes: "AA==" };

describe("source-map batch", () => {
  test("preserves binary bytes and Unicode paths", () => {
    const request = {
      chunks: [{ ...chunk, path: "日本語.js.map", bytes: "AP8BgA==" }],
    };

    expect(sourceMapUploadBatch.parse(request)).toStrictEqual(request);
  });

  test.each(["", "A", "AA", "AA=", "AB==", "AAB=", "AA==\n", "____", "!!!!"])(
    "rejects noncanonical base64 %j",
    (bytes) => {
      expect(
        sourceMapUploadBatch.safeParse({ chunks: [{ ...chunk, bytes }] })
          .success,
      ).toBe(false);
    },
  );

  test.each([-1, 0.5, Number.MAX_SAFE_INTEGER + 1])(
    "rejects offset %s",
    (offset) => {
      expect(
        sourceMapUploadBatch.safeParse({ chunks: [{ ...chunk, offset }] })
          .success,
      ).toBe(false);
    },
  );

  test("rejects duplicate paths even at different offsets", () => {
    expect(
      sourceMapUploadBatch.safeParse({
        chunks: [chunk, { ...chunk, offset: 1 }],
      }).success,
    ).toBe(false);
  });

  test("bounds file count and rejects undeclared fields", () => {
    expect(sourceMapUploadBatch.safeParse({ chunks: [] }).success).toBe(false);

    const chunks = Array.from({ length: SOURCE_MAP_BATCH_FILES }, (_, i) => ({
      ...chunk,
      path: `${i}.map`,
    }));

    expect(sourceMapUploadBatch.safeParse({ chunks }).success).toBe(true);

    expect(
      sourceMapUploadBatch.safeParse({
        chunks: [...chunks, { ...chunk, path: "extra.map" }],
      }).success,
    ).toBe(false);

    expect(
      sourceMapUploadBatch.safeParse({ chunks: [chunk], extra: true }).success,
    ).toBe(false);

    expect(
      sourceMapUploadBatch.safeParse({ chunks: [{ ...chunk, extra: true }] })
        .success,
    ).toBe(false);
  });

  test("bounds actual UTF-8 JSON bytes including path metadata", () => {
    const overhead = new TextEncoder().encode(
      JSON.stringify({ chunks: [{ ...chunk, path: "" }] }),
    ).length;

    const path = "a".repeat(SOURCE_MAP_UPLOAD_BYTES - overhead);

    expect(
      sourceMapUploadBatch.safeParse({ chunks: [{ ...chunk, path }] }).success,
    ).toBe(true);

    expect(
      sourceMapUploadBatch.safeParse({
        chunks: [{ ...chunk, path: `${path}a` }],
      }).success,
    ).toBe(false);

    expect(
      sourceMapUploadBatch.safeParse({
        chunks: [{ ...chunk, path: `${path.slice(2)}日` }],
      }).success,
    ).toBe(false);
  });
});
