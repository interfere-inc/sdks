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

test("does not take another snapshot for a redundant visible notification", async () => {
  record.addCustomEvent("initial-snapshot", true);

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(1);

  expect(uploads).toHaveLength(1);

  expect(uploads[0]?.events).toContainEqual(
    expect.objectContaining({ type: EventType.FullSnapshot }),
  );

  expect(document.visibilityState).toBe("visible");

  document.dispatchEvent(new Event("visibilitychange", { bubbles: true }));

  record.addCustomEvent("still-visible", true);

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(2);

  expect(uploads).toHaveLength(2);

  expect(uploads[1]?.events).toStrictEqual([
    {
      type: EventType.Custom,
      data: { tag: "still-visible", payload: true },
      timestamp: expect.any(Number),
    },
  ]);
});

test("keeps consecutive checkout headers together in separate chunks", async () => {
  expect.hasAssertions();

  record.addCustomEvent("before-checkout", true);

  record.takeFullSnapshot(true);

  record.takeFullSnapshot(true);

  await replayUploads.uploadsReached(2);

  expect(uploads).toHaveLength(2);

  record.addCustomEvent("after-checkout", true);

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(3);

  expect(uploads).toHaveLength(3);

  expect(uploads[0]?.text).toContain("before-checkout");

  for (const upload of uploads.slice(1)) {
    expect(upload.events.slice(0, 2)).toMatchObject([
      { type: EventType.Meta },
      { type: EventType.FullSnapshot },
    ]);

    expect(upload.events.slice(2)).not.toContainEqual(
      expect.objectContaining({ type: EventType.FullSnapshot }),
    );

    expect(upload.text).not.toContain("before-checkout");
  }

  expect(uploads[2]?.text).toContain("after-checkout");
});
