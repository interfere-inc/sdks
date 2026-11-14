/* oxlint-disable typescript/no-unsafe-assignment */
import { EventType } from "@rrweb/types";
import { record } from "rrweb";
import { startReplayHarness } from "../../../../test/fixtures/replay.test-utils.js";
import { replayUploads } from "../../../../test/fixtures/replay-uploads.js";
import { beforeEach, expect, test, vi } from "vitest";

const { uploads, requests } = replayUploads;

beforeEach(async () => {
  await startReplayHarness();
});

test.each([
  ["plain", "x".repeat(3 * 1024 * 1024)],
  ["escaped", '"'.repeat(1.5 * 1024 * 1024)],
])(
  "splits %s events using their serialized byte size",
  async (_name, payload) => {
    expect.hasAssertions();

    record.addCustomEvent("large-first", payload);

    record.addCustomEvent("large-second", payload);

    await replayUploads.uploadsReached(2);

    expect(uploads).toHaveLength(2);

    for (const { text } of uploads) {
      expect(new TextEncoder().encode(text).byteLength).toBeLessThanOrEqual(
        8 * 1024 * 1024,
      );
    }

    expect(uploads.flatMap(({ events }) => events)).toStrictEqual(
      expect.arrayContaining([
        {
          type: EventType.Custom,
          data: { tag: "large-first", payload },
          timestamp: expect.any(Number),
        },
        {
          type: EventType.Custom,
          data: { tag: "large-second", payload },
          timestamp: expect.any(Number),
        },
      ]),
    );

    expect(
      uploads.find(({ text }) => text.includes("large-first"))?.metadata,
    ).toStrictEqual(expect.objectContaining({ segmentId: 0 }));

    expect(
      uploads.find(({ text }) => text.includes("large-second"))?.metadata,
    ).toStrictEqual(expect.objectContaining({ segmentId: 1 }));
  },
);

test("drops an individually oversized event while preserving later recorded data", async () => {
  record.addCustomEvent("baseline", true);

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(1);

  expect(uploads).toHaveLength(1);

  record.addCustomEvent("oversized-event", "x".repeat(9 * 1024 * 1024));

  record.addCustomEvent("after-oversized", true);

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(2);

  expect(uploads).toHaveLength(2);

  expect(requests).toHaveLength(2);

  expect(uploads[1]?.events).toStrictEqual([
    {
      type: EventType.Custom,
      data: { tag: "after-oversized", payload: true },
      timestamp: expect.any(Number),
    },
  ]);
});
