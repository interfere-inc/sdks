import { createReplayRetryQueue } from "./retry-queue.js";
import type { PendingReplayChunk } from "./retry-queue.js";
import { expect, test, vi } from "vitest";

const MiB = 1024 * 1024;

function chunk(segmentId: number, bodyJson: string, hasFullSnapshot = false) {
  return {
    chunk: { segmentId, windowId: "window" },
    sessionId: "session",
    trigger: undefined,
    payload: {
      bodyJson,
      eventCount: 1,
      activityCount: 0,
      firstTs: 1,
      lastTs: 1,
      hasFullSnapshot,
    },
  } satisfies PendingReplayChunk;
}

function createQueue() {
  const onKeyframeLost = vi.fn<() => void>();

  return { onKeyframeLost, queue: createReplayRetryQueue({ onKeyframeLost }) };
}

test("retains the original session, segment and payload until a retry", () => {
  const { queue } = createQueue();
  const pending = chunk(4, "events");

  queue.push(pending);

  expect(queue.drain()).toStrictEqual([pending]);
  expect(queue.drain()).toStrictEqual([]);
});

test("bounds retained UTF-8 bytes and evicts oldest failed chunks", () => {
  const { onKeyframeLost, queue } = createQueue();

  queue.push(chunk(0, "a".repeat(8 * MiB)));

  const last = chunk(1, "é".repeat(5 * MiB), true);

  queue.push(last);

  expect(queue.drain()).toStrictEqual([last]);
  expect(onKeyframeLost).not.toHaveBeenCalled();
});

test("an eviction also drops the chunks that played on top of the lost keyframe", () => {
  const { onKeyframeLost, queue } = createQueue();

  for (const pending of [
    chunk(0, "a".repeat(8 * MiB), true),
    chunk(1, "incremental"),
    chunk(2, "b".repeat(9 * MiB)),
  ]) {
    queue.push(pending);
  }

  expect(queue.drain()).toStrictEqual([]);
  expect(onKeyframeLost).toHaveBeenCalledTimes(1);
});

test("an eviction keeps everything from the next queued keyframe", () => {
  const { onKeyframeLost, queue } = createQueue();

  const keyframe = chunk(2, "b".repeat(6 * MiB), true);
  const tail = chunk(3, "c".repeat(3 * MiB));

  for (const pending of [
    chunk(0, "a".repeat(8 * MiB), true),
    chunk(1, "incremental"),
    keyframe,
    tail,
  ]) {
    queue.push(pending);
  }

  expect(queue.drain()).toStrictEqual([keyframe, tail]);
  expect(onKeyframeLost).not.toHaveBeenCalled();
});

test("consent cleanup clears every pending retry", () => {
  const { onKeyframeLost, queue } = createQueue();

  queue.push(chunk(0, "events"));

  queue.clear();

  expect(queue.drain()).toStrictEqual([]);
  expect(onKeyframeLost).not.toHaveBeenCalled();
});
