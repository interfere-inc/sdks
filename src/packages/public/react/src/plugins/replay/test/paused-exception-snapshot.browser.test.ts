import { EventType } from "@rrweb/types";
import { record } from "rrweb";
import { startReplayHarness } from "../../../../test/fixtures/replay.test-utils.js";
import { replayUploads } from "../../../../test/fixtures/replay-uploads.js";
import { expect, onTestFinished, test, vi } from "vitest";

const { uploads, requests } = replayUploads;

const mechanism = { handled: false, type: "browser.onerror" } as const;

const PAUSED_SNAPSHOT_TAG = "interfere.paused-exception-snapshot";

function backgroundFailure(message: string) {
  const error = new Error(message);

  error.stack = `Error: ${message}\n    at pollStatus (https://app.example/assets/poll-a1b2.js:1:4821)`;

  return error;
}

function hidePage() {
  globalThis.dispatchEvent(
    new PageTransitionEvent("pagehide", { persisted: true }),
  );

  onTestFinished(() => {
    globalThis.dispatchEvent(
      new PageTransitionEvent("pageshow", { persisted: true }),
    );
  });
}

test("uploads one full snapshot at the exception instant while the page is hidden, then stays paused", async () => {
  const harness = await startReplayHarness();

  hidePage();

  await replayUploads.uploadsReached(1);

  const before = Date.now();

  harness.kernel.recordException(backgroundFailure("poll failed"), {
    mechanism,
  });

  const after = Date.now();

  await replayUploads.uploadsReached(2);

  const [, snapshot] = uploads;

  expect(snapshot?.events).toMatchObject([
    { type: EventType.Meta },
    { type: EventType.FullSnapshot },
    {
      type: EventType.Custom,
      data: { tag: PAUSED_SNAPSHOT_TAG, payload: { reason: "hidden" } },
    },
  ]);

  expect(snapshot?.metadata).toSatisfy(
    (meta: { firstTs: number; lastTs: number }) =>
      meta.firstTs >= before && meta.lastTs <= after,
  );

  expect(snapshot?.request.keepalive).toBe(false);

  expect(() => record.addCustomEvent("still-hidden", true)).toThrow(
    "please add custom event after start recording",
  );

  harness.kernel.recordException(backgroundFailure("poll failed again"), {
    mechanism,
  });

  await vi.advanceTimersByTimeAsync(20_000);

  expect(requests).toHaveLength(2);
});

test("allows the next paused snapshot once the five-minute throttle has passed", async () => {
  const harness = await startReplayHarness({ fakeMonotonicClock: true });

  hidePage();

  await replayUploads.uploadsReached(1);

  harness.kernel.recordException(backgroundFailure("first"), { mechanism });

  await replayUploads.uploadsReached(2);

  await vi.advanceTimersByTimeAsync(4 * 60 * 1000);

  harness.kernel.recordException(backgroundFailure("too soon"), { mechanism });

  await vi.advanceTimersByTimeAsync(60 * 1000);

  expect(requests).toHaveLength(2);

  harness.kernel.recordException(backgroundFailure("later"), { mechanism });

  await replayUploads.uploadsReached(3);

  expect(uploads[2]?.events.slice(0, 2)).toMatchObject([
    { type: EventType.Meta },
    { type: EventType.FullSnapshot },
  ]);
});

test("snapshots with reason idle when the session paused for input idleness", async () => {
  const harness = await startReplayHarness({ fakeMonotonicClock: true });

  await vi.advanceTimersByTimeAsync(6 * 60 * 1000);

  expect(() => record.addCustomEvent("idle", true)).toThrow(
    "please add custom event after start recording",
  );

  harness.kernel.recordException(backgroundFailure("idle failure"), {
    mechanism,
  });

  const snapshot = await replayUploads.uploadMatching(({ text }) =>
    text.includes(PAUSED_SNAPSHOT_TAG),
  );

  expect(snapshot.events.at(-1)).toMatchObject({
    type: EventType.Custom,
    data: { tag: PAUSED_SNAPSHOT_TAG, payload: { reason: "idle" } },
  });
});

test("does not cut an extra chunk for an exception while recording", async () => {
  const harness = await startReplayHarness();

  harness.kernel.recordException(backgroundFailure("foreground failure"), {
    mechanism,
  });

  await vi.advanceTimersByTimeAsync(2000);

  await replayUploads.uploadsReached(1);

  await vi.advanceTimersByTimeAsync(5000);

  expect(requests).toHaveLength(1);
});

test("stops on the session idle pause and records again on the lifecycle resume", async () => {
  await startReplayHarness({ fakeMonotonicClock: true });

  await vi.advanceTimersByTimeAsync(6 * 60 * 1000);

  expect(() => record.addCustomEvent("idle", true)).toThrow(
    "please add custom event after start recording",
  );

  globalThis.dispatchEvent(new PointerEvent("pointerdown"));

  record.addCustomEvent("after-input", true);

  await vi.advanceTimersByTimeAsync(2000);

  const resumed = await replayUploads.uploadMatching(({ text }) =>
    text.includes("after-input"),
  );

  expect(resumed.events.slice(0, 2)).toMatchObject([
    { type: EventType.Meta },
    { type: EventType.FullSnapshot },
  ]);
});
