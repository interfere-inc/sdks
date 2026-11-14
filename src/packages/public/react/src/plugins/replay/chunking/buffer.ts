import { EventType } from "@rrweb/types";
import type { eventWithTime } from "@rrweb/types";

import { classifyReplayEvent, utf8ByteLength } from "./chunking.js";
import type { BufferedReplayEvent } from "./chunking.js";

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

      entries.push({ json, ts: Date.now(), bytes, isActivity, isFullSnapshot });

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
