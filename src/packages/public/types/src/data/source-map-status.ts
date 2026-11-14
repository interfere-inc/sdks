import { z } from "zod";

import { releaseSlugSchema } from "../releases/slug.ts";
import { sourceMapPublicationDiagnostics } from "./source-map-diagnostics.ts";

export const sourceMapPublicationStatus = z
  .object({
    releaseSlug: releaseSlugSchema,
    environment: z.string().nullable(),
    sessionId: z.uuid().nullable(),
    state: z.enum([
      "absent",
      "uploading",
      "accepted",
      "ready",
      "expiring",
      "expired",
    ]),
    fileCount: z.number().int().nonnegative(),
    publishedFileCount: z.number().int().nonnegative(),
    batchDigest: z
      .string()
      .regex(/^[a-f0-9]{64}$/u)
      .nullable(),
    manifestReady: z.boolean(),
    diagnostics: sourceMapPublicationDiagnostics.optional(),
  })
  .meta({
    id: "SourceMapPublicationStatus",
    title: "Source map publication status",
  });
