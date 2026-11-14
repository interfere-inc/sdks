/* oxlint-disable typescript/no-unsafe-assignment */
import { EventType } from "@rrweb/types";
import { record } from "rrweb";
import { startReplayHarness } from "../../../../test/fixtures/replay.test-utils.js";
import type { ReplayHarness } from "../../../../test/fixtures/replay.test-utils.js";
import { replayUploads } from "../../../../test/fixtures/replay-uploads.js";
import replayPlugin from "../index.js";
import { beforeEach, expect, test, vi } from "vitest";

let replay: ReplayHarness;

const { uploads, requests } = replayUploads;

beforeEach(async () => {
  replay = await startReplayHarness();
});

test("stops rrweb and flushes the buffered tail on ordinary cleanup", async () => {
  record.addCustomEvent("cleanup-tail", "recorded");

  replay.cleanup?.();

  replay.cleanup = undefined;

  expect(() => record.addCustomEvent("after-cleanup", true)).toThrow(
    "please add custom event after start recording",
  );

  await replayUploads.uploadsReached(1);

  expect(uploads).toHaveLength(1);

  expect(uploads[0]?.events).toContainEqual({
    type: EventType.Custom,
    data: { tag: "cleanup-tail", payload: "recorded" },
    timestamp: expect.any(Number),
  });
});

test("does not restart recording when cleanup happens during recorder startup", async () => {
  replay.cleanup?.("consent-revoked");

  replay.cleanup = replayPlugin.setup(replay.kernel);

  replay.cleanup?.("consent-revoked");

  await vi.dynamicImportSettled();

  expect(() => record.addCustomEvent("after-early-cleanup", true)).toThrow(
    "please add custom event after start recording",
  );

  await vi.advanceTimersByTimeAsync(10_000);

  expect(requests).toHaveLength(0);
});

test("discards revoked consent data and never includes it after reactivation", async () => {
  record.addCustomEvent("baseline", true);

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(1);

  expect(uploads).toHaveLength(1);

  record.addCustomEvent("revoked-tail", true);

  expect(requests).toHaveLength(1);

  replay.cleanup?.("consent-revoked");

  replay.cleanup = undefined;

  expect(() => record.addCustomEvent("stopped", true)).toThrow(
    "please add custom event after start recording",
  );

  await vi.advanceTimersByTimeAsync(10_000);

  expect(requests).toHaveLength(1);

  replay.cleanup = replayPlugin.setup(replay.kernel);

  await vi.waitFor(() => record.addCustomEvent("reactivated", true));

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(2);

  expect(uploads).toHaveLength(2);

  expect(uploads[1]?.text).toContain("reactivated");

  expect(uploads.map(({ text }) => text).join("\n")).not.toContain(
    "revoked-tail",
  );
});
