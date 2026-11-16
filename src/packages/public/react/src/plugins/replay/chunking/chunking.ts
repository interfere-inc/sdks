import { EventType, IncrementalSource } from "@rrweb/types";

const UPLOAD_LIMIT_BYTES = 8 * 1024 * 1024;

const TARGET_CHUNK_BYTES = 6 * 1024 * 1024;

const NEWLINE_BYTES = 1;

const textEncoder = new TextEncoder();

const ACTIVITY_SOURCES = new Set<number>([
  IncrementalSource.MouseMove,
  IncrementalSource.MouseInteraction,
  IncrementalSource.Scroll,
  IncrementalSource.Input,
  IncrementalSource.TouchMove,
  IncrementalSource.Drag,
  IncrementalSource.Selection,
]);

export interface BufferedReplayEvent {
  bytes: number;
  isActivity: boolean;
  isFullSnapshot: boolean;
  eventType: number | null;
  json: string;
  ts: number;
}

export interface ReplayChunkPayload {
  activityCount: number;
  bodyJson: string;
  eventCount: number;
  firstTs: number | null;
  hasFullSnapshot: boolean;
  lastTs: number | null;
}

export interface SplitReplayEventsResult {
  awaitingKeyframe: boolean;
  chunks: BufferedReplayEvent[][];
  droppedEventBytes: number[];
  requestKeyframe: boolean;
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Replay classification must tolerate malformed buffered values without discarding their serialized payloads.
export function classifyReplayEvent(event: unknown) {
  if (
    event === null || // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Primitive replay values have no event discriminator.
    typeof event !== "object" ||
    !("type" in event) ||
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- String event codes must not be treated as rrweb numeric event codes.
    typeof event.type !== "number"
  ) {
    return { isActivity: false, isFullSnapshot: false };
  }

  if (event.type === (EventType.FullSnapshot as number)) {
    return { isActivity: false, isFullSnapshot: true };
  }

  if (
    event.type === (EventType.IncrementalSnapshot as number) &&
    "data" in event &&
    event.data !== null && // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Malformed incremental payloads have no activity source to classify.
    typeof event.data === "object" &&
    "source" in event.data
  ) {
    const { source } = event.data;

    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Only numeric rrweb sources participate in the activity source set.
    if (typeof source === "number" && ACTIVITY_SOURCES.has(source)) {
      return { isActivity: true, isFullSnapshot: false };
    }
  }

  return { isActivity: false, isFullSnapshot: false };
}

function countActivityEvents(group: BufferedReplayEvent[]) {
  let count = 0;

  for (const entry of group) {
    if (entry.isActivity) {
      count += 1;
    }
  }

  return count;
}

export function toReplayChunkPayload(group: BufferedReplayEvent[]) {
  const parts: string[] = [];
  let hasFullSnapshot = false;

  for (const entry of group) {
    parts.push(entry.json);

    if (entry.isFullSnapshot) {
      hasFullSnapshot = true;
    }
  }

  const firstTs = group[0]?.ts ?? null;

  return {
    activityCount: countActivityEvents(group),
    bodyJson: parts.length > 0 ? `${parts.join("\n")}\n` : "",
    eventCount: group.length,
    firstTs,
    hasFullSnapshot,
    lastTs: group.at(-1)?.ts ?? null,
  } satisfies ReplayChunkPayload;
}

export function splitReplayEvents(
  buffered: BufferedReplayEvent[],
  awaitingKeyframe = false,
) {
  const chunks: BufferedReplayEvent[][] = [];
  const droppedEventBytes: number[] = [];

  let awaiting = awaitingKeyframe;
  let requestKeyframe = false;

  let current: BufferedReplayEvent[] = [];
  let currentBytes = 0;

  for (const entry of buffered) {
    if (awaiting && entry.eventType === EventType.IncrementalSnapshot) {
      continue;
    }

    if (entry.eventType === EventType.Meta || entry.isFullSnapshot) {
      awaiting = false;
    }

    const singleEventBodyBytes = entry.bytes + NEWLINE_BYTES;

    if (singleEventBodyBytes > UPLOAD_LIMIT_BYTES) {
      droppedEventBytes.push(singleEventBodyBytes);

      if (entry.eventType === EventType.IncrementalSnapshot) {
        requestKeyframe = true;
      }

      awaiting ||=
        entry.isFullSnapshot ||
        entry.eventType === EventType.IncrementalSnapshot;

      continue;
    }

    if (
      current.length > 0 &&
      currentBytes + singleEventBodyBytes > TARGET_CHUNK_BYTES
    ) {
      chunks.push(current);

      current = [];

      currentBytes = 0;
    }

    currentBytes += singleEventBodyBytes;

    current.push(entry);
  }

  if (current.length > 0) {
    chunks.push(current);
  }

  return {
    awaitingKeyframe: awaiting,
    chunks,
    droppedEventBytes,
    requestKeyframe,
  } satisfies SplitReplayEventsResult;
}

export function utf8ByteLength(value: string) {
  return textEncoder.encode(value).length;
}
