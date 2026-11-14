import { createReplayRetryQueue } from "./retry-queue.js";
import type { PendingReplayChunk } from "./retry-queue.js";
import { expect, test } from "vitest";

function chunk(segmentId: number, bodyJson: string) {
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
      hasFullSnapshot: false,
    },
  } satisfies PendingReplayChunk;
}

test("retains the original session, segment and payload until a retry", () => {
  const queue = createReplayRetryQueue();
  const pending = chunk(4, "events");

  queue.push(pending);

  expect(queue.drain()).toStrictEqual([pending]);
  expect(queue.drain()).toStrictEqual([]);
});

test("bounds retained UTF-8 bytes and evicts oldest failed chunks", () => {
  const queue = createReplayRetryQueue();

  queue.push(chunk(0, "a".repeat(8 * 1024 * 1024)));

  const last = chunk(1, "é".repeat(5 * 1024 * 1024));

  queue.push(last);

  expect(queue.drain()).toStrictEqual([last]);
});

test("consent cleanup clears every pending retry", () => {
  const queue = createReplayRetryQueue();

  queue.push(chunk(0, "events"));

  queue.clear();

  expect(queue.drain()).toStrictEqual([]);
});
