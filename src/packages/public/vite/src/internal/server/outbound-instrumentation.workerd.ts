import type { Instrumentation } from "@interfere/server-core/otel/instrumentation";

/** Workers' fetch does not run on undici, so workerd has no outbound request instrumentation to install. */
export function outboundInstrumentations() {
  return [] satisfies Instrumentation[];
}
