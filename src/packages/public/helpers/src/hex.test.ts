import { describe, expect, test } from "vitest";

import { bytesToHex, sha256Hex } from "./hex.ts";

describe("hex encoding", () => {
  test("encodes byte values as two lowercase hex characters", () => {
    expect(bytesToHex(Uint8Array.of(0, 1, 15, 16, 171, 255))).toBe(
      "00010f10abff",
    );
  });

  test("accepts an ArrayBuffer and an empty input", () => {
    expect(bytesToHex(Uint8Array.of(0, 15, 255).buffer)).toBe("000fff");
    expect(bytesToHex(new Uint8Array())).toBe("");
  });
});

describe("sha-256 digest", () => {
  const abc =
    "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

  test("hashes strings as UTF-8", async () => {
    await expect(sha256Hex("abc")).resolves.toBe(abc);
  });

  test("hashes bytes identically to the equivalent string", async () => {
    await expect(sha256Hex(new TextEncoder().encode("abc"))).resolves.toBe(abc);
  });

  test("hashes an ArrayBuffer identically", async () => {
    await expect(
      sha256Hex(new TextEncoder().encode("abc").buffer),
    ).resolves.toBe(abc);
  });
});
