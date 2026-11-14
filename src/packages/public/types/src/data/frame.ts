import { z } from "zod";

/**
 * One stack frame as it lands at the collector — pre-source-map
 * resolution. Produced by the collector's stack parser from the OTel
 * `exception.stacktrace` string, supplemented by SDK-side parsed
 * frames (React component stack, `window.onerror` fallback).
 *
 * Field names match the post-resolution `EnrichedFrame` shape so
 * consumers reading "the file the frame is in" don't have to remember
 * which lifecycle stage they're at. The wire format is the same
 * across SDK + collector + enrichment + storage.
 */
export const ingestedFrame = z
  .object({
    column: z.number().optional(),
    /** Build-injected source-map debug ID; when absent, resolution falls back to matching the file path. */
    debugId: z.string().optional(),
    file: z.string().optional(),
    fn: z.string().optional(),
    /** Stable payload-local frame ID referenced by EnrichedFrame.ingestedId after resolution. */
    id: z.string().optional(),
    line: z.number().optional(),
    /** Source code line preceding the frame, when captured by the SDK. */
    source: z.string().optional(),
  })
  .meta({
    description: "A single stack frame captured from an error.",
    id: "Frame",
    title: "Frame",
  });

export type IngestedFrame = z.infer<typeof ingestedFrame>;
