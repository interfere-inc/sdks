import type { TimedEvent } from "@opentelemetry/sdk-trace-base";

const CAPTURE_STATE = Symbol.for("@interfere/next/capture-state");

interface RecordedEvent {
  readonly event: TimedEvent;
  readonly events: TimedEvent[];
}

interface CaptureState {
  readonly claimed: WeakSet<object>;
  readonly pendingProxyThrows: unknown[];
  readonly recorded: WeakMap<object, RecordedEvent[]>;
  readonly seen: WeakSet<Error>;
}

type CaptureStateHost = typeof globalThis & {
  [CAPTURE_STATE]?: CaptureState;
};

/** Returns the capture dedupe registries, shared by every bundled copy of the SDK in this realm. */
export function captureState() {
  const host: CaptureStateHost = globalThis;

  // WARNING: Next bundles the proxy apart from instrumentation; module-scoped registries would not see each other's claims.
  host[CAPTURE_STATE] ??= {
    claimed: new WeakSet(),
    pendingProxyThrows: [],
    recorded: new WeakMap(),
    seen: new WeakSet(),
  };

  return host[CAPTURE_STATE];
}
