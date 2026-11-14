import { EventType } from "@rrweb/types";
import { record } from "rrweb";
import { startReplayHarness } from "../../../../test/fixtures/replay.test-utils.js";
import { replayUploads } from "../../../../test/fixtures/replay-uploads.js";
import { beforeEach, expect, test, vi } from "vitest";

const { uploads, requests } = replayUploads;

function pageTransition(type: "pagehide" | "pageshow") {
  return new PageTransitionEvent(type, { persisted: true });
}

beforeEach(async () => {
  await startReplayHarness();
});

test("uploads the first chunk two seconds after recording starts and then every ten seconds", async () => {
  await vi.advanceTimersByTimeAsync(2000);

  await replayUploads.uploadsReached(1);

  expect(uploads[0]?.events).toContainEqual(
    expect.objectContaining({ type: EventType.FullSnapshot }),
  );

  record.addCustomEvent("after-first-flush", true);

  await vi.advanceTimersByTimeAsync(9000);

  expect(uploads).toHaveLength(1);

  await vi.advanceTimersByTimeAsync(1000);

  await replayUploads.uploadsReached(2);

  expect(uploads[1]?.text).toContain("after-first-flush");
});

test("flushes the tail with keepalive on pagehide once and never resends it from the timer", async () => {
  record.addCustomEvent("before-hide", true);

  globalThis.dispatchEvent(pageTransition("pagehide"));

  await replayUploads.uploadsReached(1);

  expect(uploads[0]?.request.keepalive).toBe(true);

  expect(uploads[0]?.text).toContain("before-hide");

  expect(() => record.addCustomEvent("while-hidden", true)).toThrow(
    "please add custom event after start recording",
  );

  await vi.advanceTimersByTimeAsync(20_000);

  expect(requests).toHaveLength(1);

  globalThis.dispatchEvent(pageTransition("pageshow"));

  record.addCustomEvent("after-show", true);

  await vi.advanceTimersByTimeAsync(2000);

  await replayUploads.uploadsReached(2);

  expect(uploads[1]?.events).toContainEqual(
    expect.objectContaining({ type: EventType.FullSnapshot }),
  );

  expect(uploads[1]?.text).toContain("after-show");

  expect(uploads[1]?.text).not.toContain("before-hide");
});
