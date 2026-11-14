import { gunzipSync } from "fflate";

import { createCompressor } from "./compressor-host.js";
import { describe, expect, test } from "vitest";

describe("plugins/replay/compressor-host", () => {
  test("gzips bytes that round-trip through gunzip", async () => {
    const compressor = createCompressor();

    if (!compressor.gzip) {
      throw new Error("test invariant: expected non-null gzip");
    }

    const payload = new TextEncoder().encode(
      JSON.stringify({ hello: "world" }),
    );

    const compressed = await compressor.gzip(payload);

    expect(compressed).toBeInstanceOf(Uint8Array);
    expect(compressed.byteLength).toBeGreaterThan(0);

    expect(new TextDecoder().decode(gunzipSync(compressed))).toBe(
      JSON.stringify({ hello: "world" }),
    );

    compressor.dispose();
  });

  test("compresses concurrent calls independently", async () => {
    const compressor = createCompressor();

    if (!compressor.gzip) {
      throw new Error("test invariant: expected non-null gzip");
    }

    const { gzip } = compressor;
    const a = new TextEncoder().encode("alpha");
    const b = new TextEncoder().encode("bravo bravo bravo");
    const c = new TextEncoder().encode("charlie charlie charlie charlie");

    const [outA, outB, outC] = await Promise.all([gzip(a), gzip(b), gzip(c)]);

    expect(new TextDecoder().decode(gunzipSync(outA))).toBe("alpha");

    expect(new TextDecoder().decode(gunzipSync(outB))).toBe(
      "bravo bravo bravo",
    );

    expect(new TextDecoder().decode(gunzipSync(outC))).toBe(
      "charlie charlie charlie charlie",
    );

    compressor.dispose();
  });
});
