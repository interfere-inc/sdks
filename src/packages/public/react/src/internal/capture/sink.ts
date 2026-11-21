import type { NonErrorException } from "@interfere/types/sdk/errors";
import type { Attributes, Context, LogRecord } from "@opentelemetry/api";

/**
 * Where the kernel's captured exceptions and log records go. `OtelSink`
 * wraps the private providers when tracing is on; `CaptureOnlySink` posts
 * hand-encoded exception spans when tracing is off, without a tracer
 * provider, exporter chain or any instrumentation.
 */
export interface TelemetrySink {
  /** Records an exception as an Interfere-rooted exception span when no recording span is active; `origin` carries the capture time (epoch ms) and context of an exception queued before startup finished. */
  recordException: (
    value: Error | NonErrorException,
    attributes: Attributes,
    handled: boolean,
    origin?: { readonly at: number; readonly context: Context },
  ) => void;
  emitLog: (record: LogRecord) => void;
  flush: () => Promise<void>;
  shutdown: () => Promise<void>;
}
