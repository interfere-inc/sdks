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

test("does not send an empty chunk after a positively observed flush", async () => {
  record.addCustomEvent("initial-buffer", true);

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(1);

  expect(uploads).toHaveLength(1);

  expect(uploads[0]?.text).toContain("initial-buffer");

  await vi.advanceTimersByTimeAsync(30_000);

  expect(requests).toHaveLength(1);
  expect(uploads).toHaveLength(1);

  record.addCustomEvent("later-buffer", true);

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(2);

  expect(uploads).toHaveLength(2);

  expect(uploads[1]?.events).toStrictEqual([
    {
      type: EventType.Custom,
      data: { tag: "later-buffer", payload: true },
      timestamp: expect.any(Number),
    },
  ]);
});
