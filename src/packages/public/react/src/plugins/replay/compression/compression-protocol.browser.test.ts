/* oxlint-disable typescript/no-unsafe-assignment */
import { EventType } from "@rrweb/types";
import { record } from "rrweb";
import { startReplayHarness } from "../../../../test/fixtures/replay.test-utils.js";
import { replayUploads } from "../../../../test/fixtures/replay-uploads.js";
import { beforeEach, expect, test, vi } from "vitest";

const { uploads } = replayUploads;

beforeEach(async () => {
  await startReplayHarness();
});

test("uploads native gzip that decodes into recorded events", async () => {
  record.addCustomEvent("gzip-marker", true);

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(1);

  expect(uploads).toHaveLength(1);

  expect(uploads[0]?.request.headers.get("content-encoding")).toBe("gzip");

  expect(uploads[0]?.events).toContainEqual({
    type: EventType.Custom,
    data: { tag: "gzip-marker", payload: true },
    timestamp: expect.any(Number),
  });
});

test("compresses consecutive chunks independently with the native host", async () => {
  record.addCustomEvent("first-chunk", true);

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(1);

  expect(uploads).toHaveLength(1);

  expect(uploads[0]?.request.headers.get("content-encoding")).toBe("gzip");
  expect(uploads[0]?.text).toContain("first-chunk");

  record.addCustomEvent("second-chunk", true);

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(2);

  expect(uploads).toHaveLength(2);

  expect(uploads[1]?.request.headers.get("content-encoding")).toBe("gzip");
  expect(uploads[1]?.text).toContain("second-chunk");
  expect(uploads[1]?.text).not.toContain("first-chunk");
});
