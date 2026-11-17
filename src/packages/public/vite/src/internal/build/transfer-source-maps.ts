import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { uploadSourceMapChunks } from "@interfere/build/upload-source-map-chunks";
import { completeSourceMapsResponse } from "@interfere/types/data/source-maps";
import { extractRichness } from "@interfere/types/sdk/source-map-richness";

import type { ReleasePipelineInput } from "./pipeline";
import { readPublicationStatus } from "./read-publication-status";
import { RELEASE_REQUEST_BUDGET_MS } from "./request-budget";

export async function transferSourceMaps(
  input: Pick<
    ReleasePipelineInput,
    | "client"
    | "bundler"
    | "discovery"
    | "environment"
    | "releaseSlug"
    | "capturePublication"
  >,
) {
  const { client, bundler, releaseSlug } = input;
  const { files, sourceFileCount } = input.discovery;
  const start = performance.now();

  const signFiles = files.map((file) => ({
    path: file.path,
    sizeBytes: new TextEncoder().encode(file.content).byteLength,
    ...extractRichness(file.content),
  }));

  const environment = input.environment ?? undefined;

  const signed = await client.releases.signSourceMapUploads(
    { releaseSlug, environment, files: signFiles },
    { abortSignal: AbortSignal.timeout(RELEASE_REQUEST_BUDGET_MS) },
  );

  const signedAt = performance.now();

  const session = input.capturePublication
    ? await readPublicationStatus(input)
    : null;

  const sessionReadAt = performance.now();

  const totalBytes = await uploadSourceMapChunks({
    compression: "gzip",
    client,
    environment,
    releaseSlug,
    files,
    signed,
  });

  const uploadedAt = performance.now();

  const completion = {
    bundler,
    sourceFileCount,
    files: files.map(({ path, hash, debugId, chunkUrl, chunkUrls }) =>
      omitUndefined({
        path,
        hash,
        debugId,
        chunkUrl,
        chunkUrls,
      }),
    ),
  };

  const accepted = completeSourceMapsResponse.safeParse(
    await client.releases.finalizeSourceMaps(
      {
        releaseSlug,
        environment,
        ...completion,
      },
      {
        abortSignal: AbortSignal.timeout(
          Math.max(0, signed.expiresAt * 1000 - Date.now()),
        ),
      },
    ),
  );

  if (
    !accepted.success ||
    !accepted.data.ok ||
    accepted.data.fileCount !== files.length
  ) {
    throw new Error("Source-map server did not accept the complete batch");
  }

  const acceptedAt = performance.now();

  return {
    completion,
    session,
    acceptedAt,
    timing: {
      sign: Math.round(signedAt - start),
      session: Math.round(sessionReadAt - signedAt),
      upload: Math.round(uploadedAt - sessionReadAt),
      finalize: Math.round(acceptedAt - uploadedAt),
      totalBytes,
    },
  };
}
