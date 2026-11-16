import { EventType } from "@rrweb/types";
import type { eventWithTime } from "@rrweb/types";

import { classifyReplayEvent, utf8ByteLength } from "./chunking.js";
import type { BufferedReplayEvent } from "./chunking.js";
import { createLogger } from "../../../util/log.js";

const log = createLogger("replay");

const MAX_PENDING_BYTES = 16 * 1024 * 1024;

export interface ReplayBuffer {
  drain: () => BufferedReplayEvent[];
  push: (event: eventWithTime) => void;
  retainFromCheckpoint: (cutoffTs: number) => void;
  readonly totalBytes: number;
}

export function createReplayBuffer(opts: {
  onSoftLimit: () => void;
  softLimitBytes: number;
}) {
  let entries: BufferedReplayEvent[] = [];
  let checkpoints: number[] = [];
  let totalBytes = 0;

  return {
    get totalBytes() {
      return totalBytes;
    },

    drain() {
      const drained = entries;

      entries = [];

      checkpoints = [];

      totalBytes = 0;

      return drained;
    },

    push(event) {
      const json = JSON.stringify(event);
      const bytes = utf8ByteLength(json);
      const { isActivity, isFullSnapshot } = classifyReplayEvent(event);

      if (event.type === EventType.Meta) {
        checkpoints.push(entries.length);
      }

      entries.push({
        json,
        ts: Date.now(),
        bytes,
        isActivity,
        isFullSnapshot,
        eventType: event.type,
      });

      totalBytes += bytes;

      if (totalBytes >= opts.softLimitBytes) {
        opts.onSoftLimit();
      }
    },

    retainFromCheckpoint(cutoffTs) {
      const start = checkpoints.findLast(
        (index) => (entries[index]?.ts ?? Infinity) <= cutoffTs,
      );

      if (start === undefined || start === 0) {
        return;
      }

      entries = entries.slice(start);

      checkpoints = checkpoints
        .filter((index) => index >= start)
        .map((index) => index - start);

      totalBytes = entries.reduce((sum, entry) => sum + entry.bytes, 0);
    },
  } satisfies ReplayBuffer;
}

export function capPendingReplayBuffer(
  buffer: ReplayBuffer,
  softLimitBytes: number,
) {
  if (buffer.totalBytes < softLimitBytes) {
    return false;
  }

  buffer.retainFromCheckpoint(Date.now());

  if (buffer.totalBytes < MAX_PENDING_BYTES) {
    return false;
  }

  log.warn(
    "dropping %d replay bytes buffered while remote config is pending",
    buffer.totalBytes,
  );

  buffer.drain();

  return true;
}
