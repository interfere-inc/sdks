import type { Instrumentation } from "@opentelemetry/instrumentation";

/** Workers' fetch does not run on undici, so workerd has no outbound request instrumentation to install. */
export function outboundInstrumentations() {
  return [] satisfies Instrumentation[];
}
