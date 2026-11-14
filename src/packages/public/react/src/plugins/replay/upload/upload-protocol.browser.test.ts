/* oxlint-disable typescript/no-unsafe-assignment */
import { EventType } from "@rrweb/types";
import { record } from "rrweb";
import { startReplayHarness } from "../../../../test/fixtures/replay.test-utils.js";
import type { ReplayHarness } from "../../../../test/fixtures/replay.test-utils.js";
import { replayUploads } from "../../../../test/fixtures/replay-uploads.js";
import { beforeEach, expect, test, vi } from "vitest";
import { z } from "zod";

let replay: ReplayHarness;

const { uploads } = replayUploads;

beforeEach(async () => {
  replay = await startReplayHarness({
    serviceIdentity: { serviceName: "web", serviceNamespace: "shop" },
  });
});

test("posts real recorded NDJSON to the current session upload endpoint", async () => {
  record.addCustomEvent("protocol-marker", "recorded");

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(1);

  expect(uploads).toHaveLength(1);

  expect(uploads[0]?.request.url).toBe(
    `https://replay.invalid/v3/replay/upload/${replay.kernel.getSessionId()}`,
  );

  expect(uploads[0]?.request.headers.get("x-interfere-service")).toBe("web");

  expect(uploads[0]?.request.headers.get("x-interfere-service-namespace")).toBe(
    "shop",
  );

  expect(uploads[0]?.request.method).toBe("POST");
  expect(uploads[0]?.request.credentials).toBe("omit");

  expect(uploads[0]?.request.headers.get("content-type")).toBe(
    "application/octet-stream",
  );

  expect(uploads[0]?.text.endsWith("\n")).toBe(true);

  expect(uploads[0]?.events).toContainEqual({
    type: EventType.Custom,
    data: { tag: "protocol-marker", payload: "recorded" },
    timestamp: expect.any(Number),
  });

  expect(uploads[0]?.metadata).toStrictEqual({
    activityCount: 0,
    eventCount: uploads[0]?.events.length,
    firstTs: expect.any(Number),
    lastTs: expect.any(Number),
    hasFullSnapshot: true,
    segmentId: 0,
    windowId: replay.kernel.getWindowId(),
  });
});

test("stamps every upload with the device clock at the moment it is sent", async () => {
  expect.hasAssertions();

  const before = Date.now();

  record.addCustomEvent("segment-zero", 0);

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(1);

  record.addCustomEvent("segment-one", 1);

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(2);

  const after = Date.now();

  const stamps = uploads.map(({ request, metadata }) => ({
    sentAt: Number(request.headers.get("x-interfere-sent-at")),
    lastTs: z.object({ lastTs: z.number() }).parse(metadata).lastTs,
  }));

  expect(stamps).toHaveLength(2);

  for (const { sentAt, lastTs } of stamps) {
    expect(sentAt).toBeGreaterThanOrEqual(Math.max(before, lastTs));
    expect(sentAt).toBeLessThanOrEqual(after);
  }

  expect(stamps[1]?.sentAt).toBeGreaterThanOrEqual(stamps[0]?.sentAt ?? 0);
});

test("increments native window segment metadata and preserves chunk order", async () => {
  record.addCustomEvent("segment-zero", 0);

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(1);

  expect(uploads).toHaveLength(1);

  record.addCustomEvent("segment-one", 1);

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(2);

  expect(uploads).toHaveLength(2);

  expect(uploads.map(({ metadata }) => metadata)).toStrictEqual([
    expect.objectContaining({
      windowId: replay.kernel.getWindowId(),
      segmentId: 0,
    }),
    expect.objectContaining({
      windowId: replay.kernel.getWindowId(),
      segmentId: 1,
      eventCount: 1,
      hasFullSnapshot: false,
    }),
  ]);

  expect(uploads[0]?.text).toContain("segment-zero");
  expect(uploads[0]?.text).not.toContain("segment-one");

  expect(uploads[1]?.events).toStrictEqual([
    {
      type: EventType.Custom,
      data: { tag: "segment-one", payload: 1 },
      timestamp: expect.any(Number),
    },
  ]);
});
