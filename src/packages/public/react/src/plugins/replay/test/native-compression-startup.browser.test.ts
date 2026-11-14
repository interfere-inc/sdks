/* oxlint-disable typescript/no-unsafe-assignment */
import { EventType } from "@rrweb/types";
import { record } from "rrweb";
import { createReplayHarness } from "../../../../test/fixtures/replay.test-utils.js";
import type { ReplayHarness } from "../../../../test/fixtures/replay.test-utils.js";
import { replayUploads } from "../../../../test/fixtures/replay-uploads.js";
import replayPlugin from "../index.js";
import { beforeEach, expect, test, vi } from "vitest";

let replay: ReplayHarness;

const { uploads } = replayUploads;

beforeEach(() => {
  replay = createReplayHarness();
});

test("records and delivers native compressed events after recorder startup", async () => {
  replay.cleanup = replayPlugin.setup(replay.kernel);

  await expect
    .poll(() => record.mirror.getId(replay.target))
    .toBeGreaterThan(-1);

  await vi.dynamicImportSettled();

  record.addCustomEvent("native-startup", true);

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(1);

  expect(uploads).toHaveLength(1);

  expect(uploads[0]?.request.headers.get("content-encoding")).toBe("gzip");

  expect(uploads[0]?.events).toContainEqual({
    type: EventType.Custom,
    data: { tag: "native-startup", payload: true },
    timestamp: expect.any(Number),
  });
});
