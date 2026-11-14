import type { ServerOptions } from "@interfere/server-core/node";

/** Vite request instrumentation on the shared server telemetry contract. */
export interface ServerInstrumentationOptions extends Omit<
  ServerOptions,
  "_internalAdditionalLogRecordProcessors" | "_internalAdditionalSpanProcessors"
> {
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
  /**
   * URL patterns the undici fetch instrumentation skips (no spans
   * created for these). The collector OTLP endpoint is always
   * skipped automatically — without this the SDK would trace its
   * own export requests in an infinite loop.
   */
  ignoreUrls?: (string | RegExp)[];
  /**
   * URL patterns the undici fetch instrumentation injects W3C
   * `traceparent` + `baggage` headers on for cross-process trace
   * correlation. Same-origin requests aren't a concept on the
   * server the way they are in a browser, so the allowlist is the
   * only propagation control.
   */
  propagateContextUrls?: (string | RegExp)[];
  /**
   * Application service name. Defaults to `OTEL_SERVICE_NAME`, then OTel's
   * `unknown_service:<executable>` or `unknown_service` when unavailable.
   */
  serviceName?: string;
  /** Optional application namespace for grouping related services. */
  serviceNamespace?: string;
}
