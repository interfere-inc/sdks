import type { CompleteSourceMapsRequest } from "./source-maps.ts";
import { bytesToHex } from "@interfere/helpers/hex";

export async function sourceMapBatchDigest(batch: CompleteSourceMapsRequest) {
  const files = batch.files.toSorted((left, right) =>
    left.path < right.path ? -1 : Number(left.path > right.path),
  );

  const content = JSON.stringify({
    bundler: batch.bundler,
    sourceFileCount: batch.sourceFileCount ?? null,
    files: files.map(({ path, hash, debugId, chunkUrl, chunkUrls }) => {
      const identity: (string | string[])[] = [path, hash, debugId, chunkUrl];

      if (chunkUrls !== undefined) {
        identity.push([...new Set(chunkUrls)].toSorted());
      }

      return identity;
    }),
  });

  const digest = await crypto.subtle.digest(
    "SHA-256",
    new TextEncoder().encode(content),
  );

  return bytesToHex(digest);
}
