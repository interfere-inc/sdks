import { z } from "zod";

import { sourceMapBatchCapability } from "./source-map-upload-batch.ts";

/**
 * Bundler that produced this release metadata. Reported back in the
 * SDK `bundler` upload metric for diagnostics. Mandatory now —
 * server-side suffix-matching against `chunkUrl` is uniform across
 * bundlers, but knowing which bundler produced the manifest is still
 * useful for diagnostics (different bundlers exhibit different stack
 * shapes; we want that pivot when investigating resolution issues).
 */
export const manifestBundlerSchema = z
  .enum(["webpack", "turbopack", "rollup", "rolldown", "tsc"])
  .meta({ id: "ManifestBundler", title: "ManifestBundler" });

export type ManifestBundler = z.infer<typeof manifestBundlerSchema>;

const sourceMapManifestEntrySchema = z.object({
  chunkUrl: z.string(),
  chunkUrls: z.array(z.string()).optional(),
  debugId: z.string(),
  mapHash: z.string(),
  objectKey: z.string(),
});

export const sourceMapManifestSchema = z.object({
  bundler: manifestBundlerSchema,
  createdAt: z.number(),
  files: z.array(sourceMapManifestEntrySchema),
  organizationId: z.string(),
  releaseSlug: z.string(),
  surfaceId: z.string(),
});

export type SourceMapManifest = z.infer<typeof sourceMapManifestSchema>;

/**
 * Per-entry metadata for the legacy multipart upload endpoint
 * (`POST /v1/releases/:slug/source-maps`). One entry per source-map file
 * in the multipart body, paired by `path` matching the multipart filename.
 *
 * Every entry carries `chunkUrl` (the public URL the runtime loads the
 * chunk from) and `debugId` — both mandatory now. The legacy multipart
 * endpoint accepts the same shape as the new presigned-URL flow.
 */
export const uploadSourceMapEntry = z
  .object({
    /** Build-time relative chunk path, such as _next/static/chunks/<hash>.js. */
    chunkUrl: z.string(),
    chunkUrls: z.array(z.string()).optional(),
    debugId: z.string(),
    hash: z.string(),
    path: z.string().min(1),
  })
  .meta({ id: "UploadSourceMapEntry", title: "UploadSourceMapEntry" });

export type UploadSourceMapEntry = z.infer<typeof uploadSourceMapEntry>;

export const uploadSourceMapsResponse = z
  .object({
    fileCount: z.number().int().nonnegative(),
    message: z.string(),
    ok: z.boolean(),
    processed: z.number().int().nonnegative(),
  })
  .meta({ id: "UploadSourceMapsResponse", title: "UploadSourceMapsResponse" });

export type UploadSourceMapsResponse = z.infer<typeof uploadSourceMapsResponse>;

export const signSourceMapFile = z
  .object({
    hasFile: z.boolean(),
    hasNames: z.boolean(),
    hasSourcesContent: z.boolean(),
    mappingsPresent: z.boolean(),
    path: z.string().min(1),
    sha256: z
      .string()
      .regex(/^[a-f0-9]{64}$/u)
      .optional()
      .meta({
        description:
          "Lowercase hex SHA-256 of the file bytes. When the Company already stores these bytes, the response marks the file present and it needs no upload.",
      }),
    sizeBytes: z.number().int().nonnegative(),
  })
  .meta({ id: "SignSourceMapFile", title: "SignSourceMapFile" });

export const signSourceMapsRequest = z
  .object({
    files: z.array(signSourceMapFile),
  })
  .meta({
    description:
      "Initializes an upload session with the file inventory and declared sizes. Upload each file through the source-map endpoint using ordered byte offsets, then finalize the complete batch before the session expires.",
    id: "SignSourceMapsRequest",
    title: "SignSourceMapsRequest",
  });

export type SignSourceMapsRequest = z.infer<typeof signSourceMapsRequest>;

export const signSourceMapUpload = z
  .object({
    objectKey: z.string(),
    path: z.string(),
    presignedUrl: z.string(),
  })
  .meta({ id: "SignSourceMapUpload", title: "SignSourceMapUpload" });

export const signSourceMapsResponse = z
  .object({
    expiresAt: z.number().int(),
    uploads: z.array(signSourceMapUpload),
  })
  .meta({
    id: "SignSourceMapsResponse",
    title: "SignSourceMapsResponse",
  });

export type SignSourceMapsResponse = z.infer<typeof signSourceMapsResponse>;

export const sourceMapUploadSessionResponse = z
  .object({
    batchUpload: sourceMapBatchCapability.optional(),
    expiresAt: z.number().int(),
    uploadGrant: z.string().optional().meta({
      description:
        "Opaque grant for this upload session. Send it as the X-Interfere-Upload-Grant header on chunk and batch uploads in place of the API key; it expires with the session.",
    }),
    uploads: z.array(
      z.object({
        path: z.string(),
        present: z.boolean().optional().meta({
          description:
            "True when the Company already stores the bytes of the declared sha256. Skip uploading this file and still list it at finalize.",
        }),
      }),
    ),
  })
  .meta({ id: "SourceMapUploadSessionResponse" });

export const completeSourceMapFile = z
  .object({
    /** Build-time relative chunk path; deployment origins are supplied at runtime. */
    chunkUrl: z.string(),
    chunkUrls: z.array(z.string()).optional(),
    debugId: z.string(),
    hash: z.string(),
    path: z.string().min(1),
  })
  .meta({ id: "CompleteSourceMapFile", title: "CompleteSourceMapFile" });

export const completeSourceMapsRequest = z
  .object({
    bundler: manifestBundlerSchema,
    files: z.array(completeSourceMapFile),
    sourceFileCount: z.number().int().nonnegative().optional(),
  })
  .meta({
    description:
      "Finalizes an upload session after all file chunks are durably accepted. Verifies the complete file inventory and schedules publication of the source-map manifest. Safe to retry.",
    id: "CompleteSourceMapsRequest",
    title: "CompleteSourceMapsRequest",
  });

export type CompleteSourceMapsRequest = z.infer<
  typeof completeSourceMapsRequest
>;

export const completeSourceMapsResponse = z
  .object({
    fileCount: z.number().int().nonnegative(),
    ok: z.boolean(),
  })
  .meta({
    id: "CompleteSourceMapsResponse",
    title: "CompleteSourceMapsResponse",
  });

export type CompleteSourceMapsResponse = z.infer<
  typeof completeSourceMapsResponse
>;

const DEBUG_ID_RE = /^[a-fA-F0-9-]+$/u;

/**
 * Self-registering snippet appended to a built JS chunk so the runtime
 * can recover the chunk's `debugId` from a live `Error().stack`. Mirrors
 * Sentry's `debugIdInjectionSnippet` but keys the global as `_debugIds`
 * (the TC39 / turbopack-native name) so the SDK's reader picks up both
 * our injected chunks and bundlers that emit the global natively.
 *
 * Appended (never prepended) so it lands past the source map's mapped
 * region and cannot shift original line positions.
 */
export function debugIdRuntimeSnippet(debugId: string) {
  if (!DEBUG_ID_RE.test(debugId)) {
    throw new Error(`debugId must be hex and dashes only, got: ${debugId}`);
  }

  return `;!function(){try{var e="undefined"!=typeof window?window:"undefined"!=typeof global?global:"undefined"!=typeof globalThis?globalThis:"undefined"!=typeof self?self:{},n=(new e.Error).stack;n&&(e._debugIds=e._debugIds||{},e._debugIds[n]="${debugId}")}catch(e){}}();`;
}

const SOURCEMAPPING_RE = /\/\/[#@]\s*sourceMappingURL=\S+\s*$/u;

const SPEC_LAST_DEBUG_ID_RE =
  /\/\/# debugId=(?<id>[a-fA-F0-9-]+)(?![\s\S]*\/\/# debugId=)/mu;

const RUNTIME_DEBUG_ID_RE =
  /(?<prefix>_debugIds\[[^\]]*\]=")[a-fA-F0-9-]+(?<suffix>")/gu;

/**
 * Reads an existing `//# debugId=…` comment from the bundle source.
 * Returns `null` when no debug ID comment is present — the caller is
 * expected to generate and inject one.
 */
export function readDebugIdFromJs(content: string) {
  const match = SPEC_LAST_DEBUG_ID_RE.exec(content);

  return match?.[1] ?? null;
}

/**
 * Inserts a `//# debugId=…` comment plus the `_debugIds` runtime snippet
 * into a built JS chunk. Idempotent: if a spec-compliant comment is
 * already present (webpack 5.104+ with `SourceMapDevToolPlugin({ debugIds:
 * true })`, turbopack with `experimental.turbopack.debugIds`, or any other
 * tool) it is overwritten with the supplied debugId — and any of our own
 * `_debugIds` runtime carriers are re-synced to the same id so the comment,
 * the source map, and the runtime value can never drift apart.
 *
 * Otherwise the snippet and comment are appended just before the existing
 * `//# sourceMappingURL=…` line so the browser still resolves the map.
 * Shared by every bundler plugin so the injected wire format cannot drift.
 *
 * Throws (via {@link debugIdRuntimeSnippet}) on a non-hex/dashes id in every
 * branch, so an unsanitized id can never reach the constructed output.
 */
export function injectDebugIdIntoJs(content: string, debugId: string) {
  const snippet = debugIdRuntimeSnippet(debugId);
  const debugIdComment = `//# debugId=${debugId}`;

  if (SPEC_LAST_DEBUG_ID_RE.test(content)) {
    return content
      .replace(SPEC_LAST_DEBUG_ID_RE, debugIdComment)
      .replaceAll(RUNTIME_DEBUG_ID_RE, `$1${debugId}$2`);
  }

  const match = SOURCEMAPPING_RE.exec(content);

  if (match) {
    const before = content.slice(0, match.index).trimEnd();

    return `${before}\n${snippet}\n${debugIdComment}\n${match[0]}`;
  }

  return `${content}\n${snippet}\n${debugIdComment}`;
}
