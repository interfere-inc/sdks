import { EventType } from "@rrweb/types";
import { HttpResponse, http } from "msw";
import { record } from "rrweb";
import { startReplayHarness } from "../../../../test/fixtures/replay.test-utils.js";
import {
  replayUploadUrl,
  replayUploads,
} from "../../../../test/fixtures/replay-uploads.js";
import { createCountSignal } from "../../../../test/fixtures/upload-signal.js";
import { fetchesSettled, worker } from "../../../../test/msw.js";
import { describe, expect, test, vi } from "vitest";

const { uploads, requests } = replayUploads;

const PAUSED_SNAPSHOT_TAG = "interfere.paused-exception-snapshot";

interface ChunkMeta {
  segmentId: number;
  windowId: string;
}

function metas() {
  return uploads.map(({ metadata }) => metadata as ChunkMeta);
}

function uploadSessionIds() {
  return uploads.map(({ request }) =>
    decodeURIComponent(new URL(request.url).pathname.split("/").at(-1) ?? ""),
  );
}

function hide() {
  globalThis.dispatchEvent(
    new PageTransitionEvent("pagehide", { persisted: true }),
  );
}

function show() {
  globalThis.dispatchEvent(
    new PageTransitionEvent("pageshow", { persisted: true }),
  );
}

function failNetwork() {
  const failed: Request[] = [];
  const signal = createCountSignal(failed);

  worker.use(
    http.post(replayUploadUrl, ({ request }) => {
      failed.push(request);

      signal.notify();

      return HttpResponse.error();
    }),
  );

  return signal.reached;
}

describe("C-L1 a hidden then visible tab continues the same recording", () => {
  test("keeps the session and window ids, continues segment ids and opens the resumed segment with a full snapshot", async () => {
    const harness = await startReplayHarness();

    await vi.advanceTimersByTimeAsync(2000);

    await replayUploads.uploadsReached(1);

    record.addCustomEvent("before-hide", true);

    hide();

    await replayUploads.uploadsReached(2);

    show();

    record.addCustomEvent("after-show", true);

    await vi.advanceTimersByTimeAsync(2000);

    await replayUploads.uploadsReached(3);

    expect(new Set(uploadSessionIds())).toStrictEqual(
      new Set([harness.kernel.getSessionId()]),
    );

    expect(
      metas().map(({ segmentId, windowId }) => [segmentId, windowId]),
    ).toStrictEqual(
      [0, 1, 2].map((segmentId) => [segmentId, harness.kernel.getWindowId()]),
    );

    expect(uploads[1]?.request.keepalive).toBe(true);

    expect(uploads[2]?.events.slice(0, 2)).toMatchObject([
      { type: EventType.Meta },
      { type: EventType.FullSnapshot },
    ]);

    expect(uploads[2]?.text).toContain("after-show");
  });
});

describe("C-L2 a failed request while paused snapshots once per window", () => {
  test("snapshots on the request-failure exception span path while hidden", async () => {
    const harness = await startReplayHarness();

    hide();

    await replayUploads.uploadsReached(1);

    harness.kernel.noteException();

    await replayUploads.uploadsReached(2);

    expect(uploads[1]?.events.at(-1)).toMatchObject({
      type: EventType.Custom,
      data: { tag: PAUSED_SNAPSHOT_TAG, payload: { reason: "hidden" } },
    });

    show();
  });

  test("a burst of failures while paused produces one snapshot", async () => {
    const harness = await startReplayHarness({ fakeMonotonicClock: true });

    hide();

    await replayUploads.uploadsReached(1);

    for (let index = 0; index < 25; index += 1) {
      harness.kernel.noteException();
    }

    await replayUploads.uploadsReached(2);

    await vi.advanceTimersByTimeAsync(60_000);

    expect(requests).toHaveLength(2);

    show();
  });

  test("a resume resets the throttle so the next pause can snapshot again", async () => {
    const harness = await startReplayHarness({ fakeMonotonicClock: true });

    hide();

    await replayUploads.uploadsReached(1);

    harness.kernel.noteException();

    await replayUploads.uploadsReached(2);

    show();

    await vi.advanceTimersByTimeAsync(2000);

    await replayUploads.uploadsReached(3);

    record.addCustomEvent("before-second-hide", true);

    hide();

    await replayUploads.uploadsReached(4);

    harness.kernel.noteException();

    await replayUploads.uploadsReached(5);

    expect(uploads[4]?.events.at(-1)).toMatchObject({
      type: EventType.Custom,
      data: { tag: PAUSED_SNAPSHOT_TAG },
    });

    show();
  });
});

describe("C-L3 chunks after a network recovery carry the same session", () => {
  test("retries a failed chunk with its original segment id and uploads subsequent data", async () => {
    expect.hasAssertions();

    const harness = await startReplayHarness();

    const failuresReached = failNetwork();

    record.addCustomEvent("lost-while-offline", true);

    await vi.advanceTimersByTimeAsync(2000);

    await failuresReached(1);

    await fetchesSettled();

    worker.resetHandlers();

    record.addCustomEvent("after-reconnect", true);

    await vi.advanceTimersByTimeAsync(10_000);

    await replayUploads.uploadsReached(1);

    await vi.advanceTimersByTimeAsync(10_000);

    await replayUploads.uploadsReached(2);

    expect(new Set(uploadSessionIds())).toStrictEqual(
      new Set([harness.kernel.getSessionId()]),
    );

    expect(
      metas()
        .map(({ segmentId }) => segmentId)
        .toSorted((left, right) => left - right),
    ).toStrictEqual([0, 1]);

    expect(uploads.map(({ text }) => text).join("")).toContain(
      "lost-while-offline",
    );

    expect(uploads.map(({ text }) => text).join("")).toContain(
      "after-reconnect",
    );
  });

  test("recording and queued chunks recover after three network failures", async () => {
    expect.hasAssertions();

    await startReplayHarness();

    const failuresReached = failNetwork();

    for (let index = 0; index < 3; index += 1) {
      record.addCustomEvent("offline", index);

      // oxlint-disable-next-line no-await-in-loop -- Each flush must observe the previous step before the clock moves on.
      await vi.advanceTimersByTimeAsync(10_000);
    }

    await failuresReached(3);

    await fetchesSettled();

    record.addCustomEvent("back-online", true);

    worker.resetHandlers();

    await vi.advanceTimersByTimeAsync(60_000);

    await replayUploads.uploadsReached(4);

    expect(uploads.map(({ text }) => text).join("")).toContain("back-online");
    expect(new Set(uploadSessionIds()).size).toBe(1);

    expect(
      metas()
        .map(({ segmentId }) => segmentId)
        .toSorted((left, right) => left - right),
    ).toStrictEqual([0, 1, 2, 3]);
  });
});

describe("C-L4 a remote config flip to disabled stops uploads", () => {
  test("uploads nothing after remote config disables replay", async () => {
    const harness = await startReplayHarness();

    await vi.advanceTimersByTimeAsync(2000);

    await replayUploads.uploadsReached(1);

    harness.runtime.applyRemoteConfig({ replay: false });

    record.addCustomEvent("after-disable", true);

    await vi.advanceTimersByTimeAsync(30_000);

    expect(requests).toHaveLength(1);
  });
});

describe("C-L5 a four-hour tab keeps one coherent recording", () => {
  test("keeps one session and window with contiguous segment ids across four hours of periodic activity", async () => {
    const harness = await startReplayHarness({ fakeMonotonicClock: true });

    const steps = 60;

    for (let step = 0; step < steps; step += 1) {
      globalThis.dispatchEvent(new PointerEvent("pointerdown"));

      record.addCustomEvent("tick", step);

      // oxlint-disable-next-line no-await-in-loop -- Each flush must observe the previous step before the clock moves on.
      await vi.advanceTimersByTimeAsync(4 * 60 * 1000);
    }

    await replayUploads.uploadsReached(steps);

    const sessionId = harness.kernel.getSessionId();

    expect(new Set(uploadSessionIds())).toStrictEqual(new Set([sessionId]));

    expect(new Set(metas().map(({ windowId }) => windowId))).toStrictEqual(
      new Set([harness.kernel.getWindowId()]),
    );

    expect(metas().map(({ segmentId }) => segmentId)).toStrictEqual(
      metas().map((_, index) => index),
    );

    expect(uploads.at(-1)?.text).toContain(`"payload":${steps - 1}`);
  });
});

describe("C-L1b a recording that pauses says why", () => {
  test("ends the chunk flushed when the tab goes to the background with a hidden stop marker", async () => {
    await startReplayHarness();

    await vi.advanceTimersByTimeAsync(2000);

    await replayUploads.uploadsReached(1);

    record.addCustomEvent("before-background", true);

    const visibility = vi.spyOn(document, "visibilityState", "get");

    // oxlint-disable-next-line landmines/no-test-doubles -- A single-tab test browser cannot background its own document; the SDK still reacts to a real visibilitychange event.
    visibility.mockReturnValue("hidden");

    document.dispatchEvent(new Event("visibilitychange", { bubbles: true }));

    await replayUploads.uploadsReached(2);

    expect(uploads[1]?.text).toContain("before-background");

    expect(uploads[1]?.events.at(-1)).toMatchObject({
      type: EventType.Custom,
      data: {
        tag: "interfere.replay-stopped",
        payload: { reason: "hidden" },
      },
    });

    // oxlint-disable-next-line landmines/no-test-doubles -- Restores the visible state the single-tab browser reports.
    visibility.mockReturnValue("visible");

    document.dispatchEvent(new Event("visibilitychange", { bubbles: true }));
  });

  test("ends the chunk flushed when the page is left with an unload stop marker", async () => {
    await startReplayHarness();

    await vi.advanceTimersByTimeAsync(2000);

    await replayUploads.uploadsReached(1);

    record.addCustomEvent("before-pagehide", true);

    hide();

    await replayUploads.uploadsReached(2);

    expect(uploads[1]?.text).toContain("before-pagehide");

    expect(uploads[1]?.events.at(-1)).toMatchObject({
      type: EventType.Custom,
      data: {
        tag: "interfere.replay-stopped",
        payload: { reason: "unload" },
      },
    });

    show();
  });

  test("ends the chunk flushed on an idle pause with an idle stop marker", async () => {
    await startReplayHarness({ fakeMonotonicClock: true });

    await vi.advanceTimersByTimeAsync(2000);

    await replayUploads.uploadsReached(1);

    record.addCustomEvent("before-idle", true);

    await vi.advanceTimersByTimeAsync(6 * 60 * 1000);

    await replayUploads.uploadsReached(2);

    expect(uploads.map(({ text }) => text).join("")).toContain("before-idle");

    expect(uploads.at(-1)?.events.at(-1)).toMatchObject({
      type: EventType.Custom,
      data: {
        tag: "interfere.replay-stopped",
        payload: { reason: "idle" },
      },
    });
  });
});
