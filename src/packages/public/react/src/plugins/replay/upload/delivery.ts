import { omitUndefined } from "@interfere/helpers/omit-undefined";

import { createLogger } from "../../../util/log.js";
import type { PluginContext } from "../../lib/types.js";
import type { ReplayAdmission } from "../anonymous-gate.js";
import type { ReplayChunkPayload } from "../chunking/chunking.js";
import type { Compressor } from "../compression/compressor-host.js";
import { gzipForUnload } from "../compression/unload-gzip.js";
import type { createReplayCarryover } from "./carryover.js";
import { uploadReplayChunk } from "./upload.js";
import type { resolveTargets } from "../../../internal/config/config.js";
import { reserveKeepaliveBody } from "../../../internal/page-lifecycle/page-lifecycle.js";

const log = createLogger("replay");

export const REPLAY_TRACER_NAME = "@interfere/react/replay";

const REPLAY_META_HEADER = "x-replay-meta";

const textEncoder = new TextEncoder();

export type DeliverResult = "ok" | "skipped" | "retryable" | "fatal";

export type ChunkTrigger = "exception";

const FATAL_STATUSES = new Set([401, 403, 413]);

const CONFLICT_STATUS = 409;

function classifyRejection(status: number, durable = false) {
  if (status === CONFLICT_STATUS) {
    return "skipped" satisfies DeliverResult;
  }

  if (FATAL_STATUSES.has(status)) {
    return "fatal" satisfies DeliverResult;
  }

  if (
    durable &&
    status >= 400 &&
    status < 500 &&
    status !== 408 &&
    status !== 429
  ) {
    return "skipped" satisfies DeliverResult;
  }

  return "retryable" satisfies DeliverResult;
}

export interface ReplayChunkRef {
  segmentId: number;
  windowId: string;
}

interface PreparedBody {
  body: Uint8Array<ArrayBuffer>;
  encoding: "gzip" | null;
  release: (() => void) | null;
}

async function prepareBody(
  compressor: Compressor,
  rawBytes: Uint8Array<ArrayBuffer>,
) {
  if (!compressor.gzip) {
    return {
      body: rawBytes,
      encoding: null,
      release: null,
    } satisfies PreparedBody;
  }

  try {
    const compressed = await compressor.gzip(rawBytes);

    return {
      body: compressed,
      encoding: "gzip",
      release: null,
    } satisfies PreparedBody;
  } catch (error) {
    log.warn("gzip failed for chunk; sending uncompressed: %o", error);

    return {
      body: rawBytes,
      encoding: null,
      release: null,
    } satisfies PreparedBody;
  }
}

// WARNING: Worker replies never arrive during unload, and Chromium grants keepalive only inside a 64 KiB budget shared by every sender on the page; an oversized body is gzipped synchronously and, when it still does not fit, sent without keepalive and lost if the page is torn down first.
function prepareUnloadBody(rawBytes: Uint8Array<ArrayBuffer>) {
  const rawRelease = reserveKeepaliveBody(rawBytes.byteLength);

  if (rawRelease) {
    return {
      body: rawBytes,
      encoding: null,
      release: rawRelease,
    } satisfies PreparedBody;
  }

  const compressed = gzipForUnload(rawBytes);

  if (!compressed) {
    return {
      body: rawBytes,
      encoding: null,
      release: null,
    } satisfies PreparedBody;
  }

  return {
    body: compressed,
    encoding: "gzip",
    release: reserveKeepaliveBody(compressed.byteLength),
  } satisfies PreparedBody;
}

function encodeReplayMeta(
  chunk: ReplayChunkRef,
  payload: ReplayChunkPayload,
  admission: ReplayAdmission | undefined,
) {
  return btoa(
    JSON.stringify({
      activityCount: payload.activityCount,
      eventCount: payload.eventCount,
      firstTs: payload.firstTs,
      hasFullSnapshot: payload.hasFullSnapshot,
      lastTs: payload.lastTs,
      segmentId: chunk.segmentId,
      trigger: admission,
      windowId: chunk.windowId,
    }),
  );
}

export interface DeliverInput {
  target: ReturnType<typeof resolveTargets>["ingest"];
  carryover?: ReturnType<typeof createReplayCarryover>;
  chunk: ReplayChunkRef;
  sessionId?: string;
  compressor: Compressor;
  ctx: PluginContext;
  payload: ReplayChunkPayload;
  /** Marks a chunk of an anonymous session admitted by an error, which the ingest gate requires on surfaces that record anonymous replay only on error. */
  admission?: ReplayAdmission | undefined;
  /** Why the chunk was cut outside the regular flush cadence, recorded on the `replay.chunk` span. */
  trigger?: ChunkTrigger | undefined;
  /** The page is hiding or unloading: compress synchronously and ask the browser to finish the request after the page unloads while the shared keepalive body budget has room. */
  unload: boolean;
}

export async function deliverChunk({
  admission,
  ctx,
  chunk,
  sessionId = ctx.getSessionId(),
  target,
  carryover,
  compressor,
  payload,
  trigger,
  unload,
}: DeliverInput) {
  if (!sessionId) {
    return "ok";
  }

  const rawBytes = textEncoder.encode(payload.bodyJson);
  const uncompressedBytes = rawBytes.byteLength;

  const prepared: PreparedBody = unload
    ? prepareUnloadBody(rawBytes)
    : await prepareBody(compressor, rawBytes);

  const meta = encodeReplayMeta(chunk, payload, admission);

  // oxlint-disable-next-line anti-slop/no-known-value-widening -- Customer authentication header names remain open while replay headers and optional compression are added.
  const headers: Record<string, string> = {
    [REPLAY_META_HEADER]: meta,
  };

  if (prepared.encoding === "gzip") {
    headers["content-encoding"] = "gzip";
  }

  const keepalive = prepared.release !== null;

  let response: unknown;

  try {
    if (unload && carryover) {
      prepared.release?.();
    }

    const upload =
      unload && carryover
        ? await carryover.send(prepared.body, {
            sessionId,
            meta,
            encoding: prepared.encoding,
          })
        : await uploadReplayChunk(target, sessionId, prepared.body, headers, {
            keepalive,
          });

    if (!upload.ok) {
      await upload.body?.cancel();

      return classifyRejection(
        upload.status,
        unload && carryover !== undefined,
      );
    }

    response = await upload.json();
  } catch (error) {
    log.warn("replay chunk upload failed: %o", error);

    return "retryable" satisfies DeliverResult;
  } finally {
    if (!(unload && carryover)) {
      prepared.release?.();
    }
  }

  const queued =
    response !== null &&
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- The transport response can be primitive JSON; only the internal queued marker denotes durable custody.
    typeof response === "object" &&
    "queued" in response &&
    response.queued === true;

  const objectKey =
    response !== null && // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Successful upload responses may contain primitive JSON without a replay artifact reference.
    typeof response === "object" &&
    "objectKey" in response &&
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Only string object keys can be attached as replay artifact URIs; malformed optional keys must be ignored.
    typeof response.objectKey === "string"
      ? response.objectKey
      : null;

  if (trigger === undefined && !unload) {
    return "ok" satisfies DeliverResult;
  }

  const attributes = omitUndefined({
    "replay.chunk.size_bytes": uncompressedBytes,
    "replay.chunk.event_count": payload.eventCount,
    "replay.chunk.uri": objectKey || undefined,
    "replay.chunk.queued": queued || undefined,
    "replay.chunk.compressed_bytes":
      prepared.encoding === "gzip" ? prepared.body.byteLength : undefined,
    "replay.chunk.first_event_ts": payload.firstTs ?? undefined,
    "replay.chunk.last_event_ts": payload.lastTs ?? undefined,
    "replay.chunk.trigger": trigger,
    "replay.chunk.unload": unload || undefined,
    "replay.chunk.keepalive": unload ? keepalive : undefined,
  });

  const span = ctx
    .getTracer(REPLAY_TRACER_NAME)
    .startSpan("replay.chunk", { attributes });

  span.end();

  return "ok";
}
