import { sourceMapBatchCapability } from "@interfere/types/data/source-map-upload-batch";
import type { Interfere, InterfereClient } from "@interfere/javascript";

import { sourceMapUploadJobs } from "./upload-source-map-batch.ts";
import { uploadSourceMapRequest } from "./upload-source-map-request.ts";

const CHUNK_BYTES = 8 * 1024 * 1024;
const LEGACY_CHUNK_BYTES = 512 * 1024;

export async function uploadSourceMapChunks(options: {
  client: InterfereClient;
  releaseSlug: string;
  environment?: string | undefined;
  signed: Interfere.SourceMapUploadSessionResponse;
  concurrency?: number | undefined;
  /** Requires sourceMapUploadFetch on the client; defaults to identity. */
  compression?: "gzip";
  files: readonly (Interfere.CompleteSourceMapFile & { content: string })[];
}) {
  const { client, environment, files, releaseSlug, signed } = options;

  const concurrency = options.concurrency ?? 20;

  if (!Number.isSafeInteger(concurrency) || concurrency < 1) {
    throw new Error("Source-map upload concurrency must be a positive integer");
  }

  const capability = sourceMapBatchCapability.safeParse(signed.batchUpload);

  const supportsGzip =
    capability.success && capability.data.compression === "gzip";

  const chunkBytes = supportsGzip ? CHUNK_BYTES : LEGACY_CHUNK_BYTES;

  const headers: Record<string, string> = {};

  if (options.compression === "gzip" && supportsGzip) {
    headers["content-encoding"] = "gzip";
  }

  if (signed.uploadGrant) {
    headers["x-interfere-upload-grant"] = signed.uploadGrant;
  }

  const controller = new AbortController();

  try {
    const sessionDeadlineMs = signed.expiresAt * 1000;

    const assertSessionOpen = () => {
      if (
        !Number.isFinite(sessionDeadlineMs) ||
        Date.now() >= sessionDeadlineMs
      ) {
        throw new Error("Source-map upload session expired");
      }
    };

    assertSessionOpen();

    const abortSignal = AbortSignal.any([
      controller.signal,
      AbortSignal.timeout(sessionDeadlineMs - Date.now()),
    ]);

    const expected = new Set(files.map((file) => file.path));

    if (
      signed.uploads.length !== files.length ||
      new Set(signed.uploads.map((file) => file.path)).size !== files.length ||
      signed.uploads.some((file) => !expected.has(file.path))
    ) {
      throw new Error("Source-map server returned a different batch");
    }

    const jobs = sourceMapUploadJobs(files, signed.batchUpload);
    let totalBytes = 0;

    await Promise.all(
      Array.from({ length: Math.min(concurrency, files.length) }, async () => {
        for (const job of jobs) {
          abortSignal.throwIfAborted();

          assertSessionOpen();

          if (job.chunks) {
            // oxlint-disable-next-line no-await-in-loop -- One worker waits for its durable acknowledgement before taking another job.
            await uploadSourceMapRequest(
              async () =>
                await client.releases.uploadSourceMapBatch(
                  { releaseSlug, environment, chunks: job.chunks },
                  { abortSignal, headers },
                ),
              abortSignal,
            );

            totalBytes += job.rawBytes;

            continue;
          }

          const { file } = job;
          const bytes = new TextEncoder().encode(file.content);

          for (
            let offset = 0;
            offset < bytes.byteLength;
            offset += chunkBytes
          ) {
            assertSessionOpen();

            const chunk = new Blob([
              bytes.subarray(offset, offset + chunkBytes),
            ]);

            // oxlint-disable-next-line no-await-in-loop -- Each file's chunks must be acknowledged in offset order.
            await uploadSourceMapRequest(
              async () =>
                await client.releases.uploadSourceMap(
                  chunk,
                  releaseSlug,
                  { environment, path: file.path, offset },
                  {
                    abortSignal,
                    headers: {
                      ...headers,
                      "x-interfere-source-map-size": String(chunk.size),
                    },
                  },
                ),
              abortSignal,
            );
          }

          totalBytes += bytes.byteLength;
        }
      }),
    );

    assertSessionOpen();

    return totalBytes;
  } finally {
    controller.abort();
  }
}
