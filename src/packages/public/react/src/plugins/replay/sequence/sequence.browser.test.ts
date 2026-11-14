import { browserStorage } from "@interfere/test-config/browser-storage";
import { createSegmentSequence } from "./sequence.js";
import { describe, expect, test } from "vitest";

describe("plugins/replay/sequence", () => {
  test("hands out monotonically increasing ids from zero", () => {
    const seq = createSegmentSequence("win_a");

    expect(seq.take()).toBe(0);
    expect(seq.take()).toBe(1);
    expect(seq.take()).toBe(2);
  });

  test("resumes from the persisted value after a reload of the same window", () => {
    const before = createSegmentSequence("win_b");

    before.take();

    before.take();

    const afterReload = createSegmentSequence("win_b");

    expect(afterReload.take()).toBe(2);
    expect(afterReload.take()).toBe(3);
  });

  test("keeps independent counters per window", () => {
    const a = createSegmentSequence("win_c");
    const b = createSegmentSequence("win_d");

    expect(a.take()).toBe(0);
    expect(a.take()).toBe(1);
    expect(b.take()).toBe(0);
  });

  test("allocates distinct ids when recorder instances overlap", () => {
    const first = createSegmentSequence("win_overlap");
    const second = createSegmentSequence("win_overlap");

    expect(first.take()).toBe(0);
    expect(second.take()).toBe(1);
    expect(first.take()).toBe(2);

    const replacement = createSegmentSequence("win_overlap");

    expect(second.take()).toBe(3);
    expect(replacement.take()).toBe(4);
    expect(first.take()).toBe(5);
  });

  test.each([
    ["a non-numeric value", "not-a-number"],
    ["a negative value", "-5"],
  ])("resets to zero when the persisted value is %s", (_label, stored) => {
    browserStorage.session.set("interfere:replay_seq:win_corrupt", stored);

    const seq = createSegmentSequence("win_corrupt");

    expect(seq.take()).toBe(0);
    expect(seq.take()).toBe(1);
  });
});
