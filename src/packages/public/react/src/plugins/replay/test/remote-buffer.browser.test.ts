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
  replay = await startReplayHarness({ remoteConfig: "pending" });
});

test("buffers pending config and delivers the original marker when enabled", async () => {
  record.addCustomEvent("pending-config-marker", true);

  await vi.advanceTimersByTimeAsync(10_000);

  expect(requests).toStrictEqual([]);
  expect(replay.runtime.getPluginConfigState("replay")).toBe("pending");

  replay.runtime.applyRemoteConfigDefaults();

  await replayUploads.uploadsReached(1);

  expect(uploads).toHaveLength(1);

  expect(uploads[0]?.events).toContainEqual({
    type: EventType.Custom,
    data: { tag: "pending-config-marker", payload: true },
    timestamp: expect.any(Number),
  });
});

test("discards pending config data on disabled cleanup", async () => {
  record.addCustomEvent("disabled-tail", true);

  expect(requests).toStrictEqual([]);

  replay.runtime.applyRemoteConfig({ replay: false });

  replay.cleanup?.();

  replay.cleanup = undefined;

  expect(() => record.addCustomEvent("stopped", true)).toThrow(
    "please add custom event after start recording",
  );

  await vi.advanceTimersByTimeAsync(10_000);

  expect(requests).toStrictEqual([]);

  replay.runtime.applyRemoteConfigDefaults();

  replay.runtime.applyRemoteConfig({ replay: true });

  replay.cleanup = replayPlugin.setup(replay.kernel);

  await vi.waitFor(() => record.addCustomEvent("enabled-later", true));

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(1);

  expect(uploads).toHaveLength(1);

  expect(uploads[0]?.text).toContain("enabled-later");
  expect(uploads[0]?.text).not.toContain("disabled-tail");
});
