import { EventType, IncrementalSource } from "@rrweb/types";

const ACTIVITY_SOURCES = new Set<number>([
  IncrementalSource.MouseMove,
  IncrementalSource.MouseInteraction,
  IncrementalSource.Scroll,
  IncrementalSource.Input,
  IncrementalSource.TouchMove,
  IncrementalSource.Drag,
  IncrementalSource.Selection,
]);

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
