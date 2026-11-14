import type { Logger } from "@opentelemetry/api-logs";

/** The part of a logger provider page-view and user-action events need: with tracing on the SDK logger provider, with `tracing: false` the kernel's sink. */
export interface EventLoggerProvider {
  getLogger: (name: string) => Pick<Logger, "emit">;
}
