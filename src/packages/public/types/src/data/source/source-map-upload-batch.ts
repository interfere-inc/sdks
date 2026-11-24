import { z } from "zod";

export const SOURCE_MAP_UPLOAD_BYTES = 8 * 1024 * 1024;
/** Batch size advertised to released SDKs, which reject larger capabilities. */
export const SOURCE_MAP_BATCH_BYTES = 512 * 1024;
export const SOURCE_MAP_BATCH_FILES = 32;

export const sourceMapBatchCapability = z
  .strictObject({
    version: z.literal(1),
    compression: z.literal("gzip").optional(),
    maxBodyBytes: z.number().int().positive().max(SOURCE_MAP_BATCH_BYTES),
    maxFiles: z.number().int().positive().max(SOURCE_MAP_BATCH_FILES),
  })
  .meta({ id: "SourceMapBatchCapability" });

const base64 = /^[A-Za-z0-9+/]*={0,2}$/u;

export const sourceMapUploadBatch = z
  .strictObject({
    chunks: z
      .array(
        z.strictObject({
          path: z.string().min(1),
          offset: z.number().int().nonnegative(),
          bytes: z
            .string()
            .min(1)
            .refine(
              (value) =>
                value.length % 4 === 0 &&
                base64.test(value) &&
                btoa(atob(value)) === value,
              "Expected canonical base64 bytes",
            ),
        }),
      )
      .min(1)
      .max(SOURCE_MAP_BATCH_FILES),
  })
  .refine(
    ({ chunks }) =>
      new Set(chunks.map(({ path }) => path)).size === chunks.length,
    "Each batch path must be unique",
  )
  .refine(
    (request) =>
      new TextEncoder().encode(JSON.stringify(request)).byteLength <=
      SOURCE_MAP_UPLOAD_BYTES,
    "Source-map batch exceeds 8 MiB",
  )
  .meta({ id: "SourceMapUploadBatch" });

export type SourceMapUploadBatch = z.infer<typeof sourceMapUploadBatch>;

export const sourceMapUploadBatchResponse = z
  .object({ ok: z.literal(true), fileCount: z.number().int().nonnegative() })
  .meta({ id: "SourceMapUploadBatchResponse" });
