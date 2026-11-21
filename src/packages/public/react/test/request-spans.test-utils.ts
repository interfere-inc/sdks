import type { InMemorySpanExporter } from "@opentelemetry/sdk-trace";
import { expect } from "vitest";

const REQUEST_SCOPES = new Set([
  "@interfere/react/request",
  "@opentelemetry/instrumentation-fetch",
  "@opentelemetry/instrumentation-xml-http-request",
]);

// WARNING: registerBundledInstrumentations always registers LongTaskInstrumentation on the private provider, so a 50 ms main-thread stall in the vitest page lands a longtask span in the same exporter; a loaded CI runner stalls constantly, and a request suite that counts or indexes every finished span reads the wrong span.
export function requestSpans(exporter: InMemorySpanExporter) {
  return exporter
    .getFinishedSpans()
    .filter((span) => REQUEST_SCOPES.has(span.instrumentationScope.name));
}

export async function settledRequestSpans(
  exporter: InMemorySpanExporter,
  count: number,
) {
  await expect
    .poll(() => requestSpans(exporter).length, { timeout: 5000 })
    .toBe(count);

  return requestSpans(exporter);
}
