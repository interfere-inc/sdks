import { BaggageSpanProcessor } from "@opentelemetry/baggage-span-processor";

/** Creates a processor that copies Interfere correlation baggage (interfere.* keys) onto host spans. */
export function correlationSpanProcessor() {
  return new BaggageSpanProcessor((key) => key.startsWith("interfere."));
}
