import { reserveKeepaliveBody } from "./page-lifecycle.js";
import { describe, expect, onTestFinished, test } from "vitest";

function reserve(bytes: number, limit?: number) {
  const release = reserveKeepaliveBody(bytes, limit);

  if (release) {
    onTestFinished(release);
  }

  return release;
}

describe("B18 shared keepalive budget", () => {
  test("every SDK keepalive sender together stays under 60,000 bytes, below the 65,536-byte browser quota (B18)", () => {
    expect(reserve(30_000)).not.toBeNull();
    expect(reserve(30_000)).not.toBeNull();
    expect(reserve(1)).toBeNull();
  });
});

describe("B19 per-sender limit", () => {
  test("a sender's own limit caps its share without raising the shared budget (B19)", () => {
    expect(reserve(20_000, 32_000)).not.toBeNull();
    expect(reserve(20_000, 32_000)).toBeNull();
    expect(reserve(20_000)).not.toBeNull();
    expect(reserve(25_000, 100_000)).toBeNull();
  });
});

describe("B20 idempotent release", () => {
  test("releasing one reservation twice frees its bytes once (B20)", () => {
    const first = reserveKeepaliveBody(30_000);

    expect(first).not.toBeNull();

    expect(reserve(30_000)).not.toBeNull();

    first?.();

    first?.();

    expect(reserve(30_000)).not.toBeNull();
    expect(reserve(1)).toBeNull();
  });
});
