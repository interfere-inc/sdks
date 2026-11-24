import type { ResolvedTracingOptions } from "../otel/wire.js";
import type { KernelInternalOptions } from "./options.js";

export function resolveTracingOptions(
  opts: KernelInternalOptions,
  serviceName: string | undefined,
  backendHosts: ResolvedTracingOptions["backendHosts"],
) {
  const tracingOpts =
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- The public tracing option accepts an instrumentation-options object or false; anything else means defaults.
    typeof opts.tracing === "object" ? opts.tracing : undefined;

  const resolvedTracing: ResolvedTracingOptions = { ...tracingOpts };

  if (serviceName) {
    resolvedTracing.serviceName = serviceName;
  }

  if (opts.serviceNamespace !== undefined) {
    resolvedTracing.serviceNamespace = opts.serviceNamespace;
  }

  if (opts._internalAdditionalSpanProcessors) {
    resolvedTracing.additionalSpanProcessors =
      opts._internalAdditionalSpanProcessors;
  }

  if (opts._internalAdditionalLogRecordProcessors) {
    resolvedTracing.additionalLogRecordProcessors =
      opts._internalAdditionalLogRecordProcessors;
  }

  if (opts._internalAdditionalMetricReaders) {
    resolvedTracing.additionalMetricReaders =
      opts._internalAdditionalMetricReaders;
  }

  if (backendHosts) {
    resolvedTracing.backendHosts = backendHosts;
  }

  return resolvedTracing;
}
