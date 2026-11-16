import { utf8ByteLength } from "../chunking/chunking.js";
import type { ReplayChunkPayload } from "../chunking/chunking.js";
import type { ReplayAdmission } from "../anonymous-gate.js";
import type { ChunkTrigger, ReplayChunkRef } from "./delivery.js";
import { createLogger } from "../../../util/log.js";

const MAX_RETRY_BYTES = 16 * 1024 * 1024;
const log = createLogger("replay");

export interface PendingReplayChunk {
  payload: ReplayChunkPayload;
  chunk: ReplayChunkRef;
  sessionId: string;
  admission?: ReplayAdmission | undefined;
  trigger: ChunkTrigger | undefined;
}

export function createReplayRetryQueue(opts: { onKeyframeLost: () => void }) {
  let pending: { chunk: PendingReplayChunk; bytes: number }[] = [];
  let totalBytes = 0;

  return {
    push(chunk: PendingReplayChunk) {
      const bytes = utf8ByteLength(chunk.payload.bodyJson);

      pending.push({ chunk, bytes });

      totalBytes += bytes;

      if (totalBytes <= MAX_RETRY_BYTES) {
        return;
      }

      while (
        totalBytes > MAX_RETRY_BYTES ||
        pending[0]?.chunk.payload.hasFullSnapshot === false
      ) {
        const dropped = pending.shift();

        if (!dropped) {
          break;
        }

        totalBytes -= dropped.bytes;

        log.warn(
          "replay retry buffer full; dropping oldest chunk of %d bytes",
          dropped.bytes,
        );
      }

      if (pending.length === 0) {
        opts.onKeyframeLost();
      }
    },
    drain() {
      const chunks = pending.map(({ chunk }) => chunk);

      pending = [];

      totalBytes = 0;

      return chunks;
    },
    clear() {
      pending = [];

      totalBytes = 0;
    },
  };
}
