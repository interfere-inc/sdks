import type { Span } from "@opentelemetry/api";
import { isPlainRecord, isString } from "@interfere/types/sdk/guards/json";

/** Reads a string attribute from the SDK span's `attributes` record, which the `Span` interface does not expose. */
export function spanStringAttribute(span: Span, key: string) {
  const attributes = "attributes" in span ? span.attributes : null;

  const value = isPlainRecord(attributes) ? attributes[key] : null;

  return isString(value) ? value : null;
}
