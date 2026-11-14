import { record } from "rrweb";
import { startReplayHarness } from "../../../../test/fixtures/replay.test-utils.js";
import { replayUploads } from "../../../../test/fixtures/replay-uploads.js";
import { expect, test, vi } from "vitest";

const { uploads } = replayUploads;

test("flushes when the page fires beforeunload", async () => {
  await startReplayHarness();

  await vi.advanceTimersByTimeAsync(2000);

  await replayUploads.uploadsReached(1);

  record.addCustomEvent("before-unload", true);

  globalThis.dispatchEvent(new Event("beforeunload"));

  await replayUploads.uploadsReached(2);

  expect(uploads[1]?.text).toContain("before-unload");
});

test("records and uploads in a fresh harness after a test that fired beforeunload", async () => {
  await startReplayHarness();

  await vi.advanceTimersByTimeAsync(2000);

  await replayUploads.uploadsReached(1);

  expect(uploads[0]?.text).toContain('"type":2');
});
