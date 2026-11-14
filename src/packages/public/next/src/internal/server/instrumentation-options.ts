import type { ServerOptions } from "@interfere/server-core/node";

/** Next-specific hooks on the shared server telemetry contract. */
export interface ServerInstrumentationOptions extends ServerOptions {
  /**
   * Install a `process.on("uncaughtException")` listener that captures the
   * error and flushes pending telemetry. When no other listener handles the
   * exception it then exits with code 1, as Node would have; when the host
   * keeps its own listener (as `next start` does) the server keeps serving.
   * Default `true`. Disable to run your own handler.
   */
  captureUncaughtException?: boolean;
  /**
   * Install a `process.on("unhandledRejection")` listener that captures the
   * rejection and flushes pending telemetry. Never forces an exit. Default
   * `true`. Disable to run your own handler.
   */
  captureUnhandledRejection?: boolean;
  /** Additional exact HTTPS origins for server fetch propagation beyond configured backend hosts in managed Vercel drain mode. Direct export and application-owned instrumentation are unchanged. */
  propagateTo?: string[];
  /**
   * Application service name. Defaults to `OTEL_SERVICE_NAME`, then the
   * build-inferred app name, then OpenTelemetry's unknown-service fallback.
   */
  serviceName?: string;
  /** Optional application namespace for grouping related services. */
  serviceNamespace?: string;
}
