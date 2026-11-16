import { EventType } from "@rrweb/types";
import { record } from "rrweb";
import { startReplayHarness } from "../../../../test/fixtures/replay.test-utils.js";
import { replayUploads } from "../../../../test/fixtures/replay-uploads.js";
import { expect, test, vi } from "vitest";
import { z } from "zod";

vi.mock(import("rrweb"), { spy: true });

const { uploads } = replayUploads;

const mechanism = { handled: false, type: "browser.onerror" } as const;

const fullSnapshotSchema = z.looseObject({
  type: z.literal(EventType.FullSnapshot),
});

function checkoutIntervals() {
  return vi
    .mocked(record)
    .mock.calls.map(([options]) => options?.checkoutEveryNms);
}

test("an on_error buffer checks out through rrweb every 30 seconds instead of a timer", async () => {
  expect.hasAssertions();

  vi.mocked(record).mockClear();

  const harness = await startReplayHarness({
    anonymousReplayPolicy: "on_error",
    fakeDate: true,
  });

  expect(checkoutIntervals()).toStrictEqual([30_000]);

  await vi.advanceTimersByTimeAsync(90_000);

  harness.kernel.recordException(new Error("checkout failed"), { mechanism });

  await replayUploads.uploadsReached(1);

  const snapshots = uploads
    .flatMap((upload) => upload.events)
    .filter((event) => fullSnapshotSchema.safeParse(event).success);

  expect(snapshots).toHaveLength(1);

  expect(checkoutIntervals().at(-1)).not.toBe(30_000);
});
