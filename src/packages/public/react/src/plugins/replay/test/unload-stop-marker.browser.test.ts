import { EventType } from "@rrweb/types";
import { record } from "rrweb";
import { startReplayHarness } from "../../../../test/fixtures/replay.test-utils.js";
import { replayUploads } from "../../../../test/fixtures/replay-uploads.js";
import { expect, test, vi } from "vitest";

const { uploads } = replayUploads;

test("ends the chunk flushed before navigating away with one unload stop marker", async () => {
  await startReplayHarness();

  await vi.advanceTimersByTimeAsync(2000);

  await replayUploads.uploadsReached(1);

  record.addCustomEvent("before-unload", true);

  globalThis.dispatchEvent(new Event("beforeunload"));

  await replayUploads.uploadsReached(2);

  expect(uploads[1]?.text).toContain("before-unload");

  expect(uploads[1]?.events.at(-1)).toMatchObject({
    type: EventType.Custom,
    data: {
      tag: "interfere.replay-stopped",
      payload: { reason: "unload" },
    },
  });

  globalThis.dispatchEvent(
    new PageTransitionEvent("pagehide", { persisted: true }),
  );

  await vi.advanceTimersByTimeAsync(10_000);

  expect(
    uploads.flatMap(
      ({ text }) => text.match(/interfere\.replay-stopped/gu) ?? [],
    ),
  ).toHaveLength(1);

  globalThis.dispatchEvent(
    new PageTransitionEvent("pageshow", { persisted: true }),
  );
});
