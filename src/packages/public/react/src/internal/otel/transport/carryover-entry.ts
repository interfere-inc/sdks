import type { ExportResult } from "@opentelemetry/core";
import { InterfereError } from "@interfere/javascript";
import { isPlainRecord } from "@interfere/types/sdk/guards/json";

/** A retained export as it is stored under its carryover key. */
export interface CarryoverEntry {
  body: string;
  created: number;
  dispatched?: number;
  scope: string;
}

/** Whether a value read back from storage is a carryover entry. */
export function isCarryoverEntry(value: unknown): value is CarryoverEntry {
  return (
    isPlainRecord(value) &&
    typeof value["body"] === "string" &&
    typeof value["created"] === "number" &&
    (value["dispatched"] === undefined ||
      typeof value["dispatched"] === "number") &&
    typeof value["scope"] === "string"
  );
}

export function isTerminal(result: ExportResult) {
  return (
    result.error instanceof InterfereError &&
    result.error.statusCode !== undefined &&
    result.error.statusCode >= 400 &&
    result.error.statusCode < 500 &&
    result.error.statusCode !== 408 &&
    result.error.statusCode !== 429
  );
}

/** Identifies a retained export and reports keepalive dispatch or capacity exhaustion to its drain. */
export interface RetainedSend {
  id: string;
  onKeepalive: () => void;
  onBudgetExhausted: () => void;
}

export function markDispatched(key: string, dispatched: number | undefined) {
  try {
    const entry: unknown = JSON.parse(localStorage.getItem(key) ?? "null");

    if (isCarryoverEntry(entry)) {
      localStorage.setItem(key, JSON.stringify({ ...entry, dispatched }));
    }
  } catch {
    return;
  }
}
