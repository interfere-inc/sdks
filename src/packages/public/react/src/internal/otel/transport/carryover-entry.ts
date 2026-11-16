import type { ExportResult } from "@opentelemetry/core";
import { InterfereError } from "@interfere/javascript";
import { z } from "zod";

export const entrySchema = z.object({
  body: z.string(),
  created: z.number(),
  dispatched: z.number().optional(),
  scope: z.string(),
});

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
    const entry = entrySchema.safeParse(
      JSON.parse(localStorage.getItem(key) ?? "null"),
    );

    if (entry.success) {
      localStorage.setItem(key, JSON.stringify({ ...entry.data, dispatched }));
    }
  } catch {
    return;
  }
}
