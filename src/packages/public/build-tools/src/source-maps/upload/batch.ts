import type { Interfere } from "@interfere/javascript";
import { sourceMapBatchCapability } from "@interfere/types/data/source-map-upload-batch";

import { Buffer } from "node:buffer";

export function sourceMapUploadJobs(
  files: readonly (Interfere.CompleteSourceMapFile & { content: string })[],
  capability: Interfere.SourceMapUploadSessionResponse["batchUpload"],
) {
  const limits =
    capability === undefined
      ? undefined
      : sourceMapBatchCapability.parse(capability);

  return (function* jobs() {
    let chunks: Interfere.SourceMapUploadBatch.Chunks.Item[] = [];
    let rawBytes = 0;

    for (const file of files) {
      if (!limits) {
        yield { file };

        continue;
      }

      const size = Buffer.byteLength(file.content);

      if (size === 0 || 4 * Math.ceil(size / 3) > limits.maxBodyBytes) {
        if (chunks.length > 0) {
          yield { chunks, rawBytes };

          chunks = [];

          rawBytes = 0;
        }

        yield { file };

        continue;
      }

      const chunk = {
        path: file.path,
        offset: 0,
        bytes: Buffer.from(file.content).toString("base64"),
      };

      if (
        Buffer.byteLength(JSON.stringify({ chunks: [chunk] })) >
        limits.maxBodyBytes
      ) {
        if (chunks.length > 0) {
          yield { chunks, rawBytes };

          chunks = [];

          rawBytes = 0;
        }

        yield { file };

        continue;
      }

      if (
        chunks.length >= limits.maxFiles ||
        Buffer.byteLength(JSON.stringify({ chunks: [...chunks, chunk] })) >
          limits.maxBodyBytes
      ) {
        yield { chunks, rawBytes };

        chunks = [];

        rawBytes = 0;
      }

      chunks.push(chunk);

      rawBytes += size;
    }

    if (chunks.length > 0) {
      yield { chunks, rawBytes };
    }
  })();
}
