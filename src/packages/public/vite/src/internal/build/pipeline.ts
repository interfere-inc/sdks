import { sourceMapBatchDigest } from "@interfere/types/data/source-map-batch";
import { cleanupSourceMaps } from "@interfere/build/cleanup-source-maps";
import { resolveReleaseRequest } from "@interfere/build/release-request";
import type { Interfere, InterfereClient } from "@interfere/javascript";
import type { ManifestBundler } from "@interfere/types/data/source-maps";
import { createReleaseResponse } from "@interfere/types/releases/definition";
import type { ReleaseSlug } from "@interfere/types/releases/slug";

import type { DiscoveryResult } from "./source-maps";
import { transferSourceMaps } from "./transfer-source-maps";
import { readPublicationStatus } from "./read-publication-status";
import { RELEASE_REQUEST_BUDGET_MS } from "./request-budget";

export interface ReleasePipelineInput {
  client: InterfereClient;
  buildId: string;
  bundler: ManifestBundler;
  discovery: Pick<DiscoveryResult, "files" | "sourceFileCount">;
  environment: string | null;
  producerVersion: string;
  releaseSlug: ReleaseSlug;
  releasesConfig: Interfere.ReleasesConfigResponse;
  capturePublication?: boolean;
  uploadSourceMaps: boolean;
}

export async function runReleasePipeline(input: ReleasePipelineInput) {
  const { client, discovery, releasesConfig } = input;
  const { files } = discovery;

  const releaseRequest = resolveReleaseRequest({
    ...input,
    config: releasesConfig,
  });

  if (!releaseRequest.ok) {
    return {
      failure: { config: releasesConfig, kind: releaseRequest.reason },
      ok: false as const,
    };
  }

  if (input.uploadSourceMaps && files.length === 0) {
    throw new Error("Source maps are required to prepare a release");
  }

  const start = performance.now();

  const release = createReleaseResponse.parse(
    await client.releases.createRelease(releaseRequest.request, {
      abortSignal: AbortSignal.timeout(RELEASE_REQUEST_BUDGET_MS),
    }),
  );

  const createdAt = performance.now();
  const releaseSlug = release.destination.slug;

  if (!input.uploadSourceMaps) {
    return {
      data: {
        cleanupFailures: [],
        fileCount: 0,
        releaseSlug,
        publication: undefined,
        timing: undefined,
      },
      ok: true as const,
    };
  }

  const transfer = await transferSourceMaps({ ...input, releaseSlug });

  const { completion, session, acceptedAt } = transfer;

  const publication = input.capturePublication
    ? {
        releaseSlug,
        buildId: release.build.hash,
        environment: input.environment,
        sessionId:
          session?.releaseSlug === releaseSlug &&
          session.environment === input.environment
            ? session.sessionId
            : null,
        batchDigest: await sourceMapBatchDigest(completion),
        status: await readPublicationStatus({ ...input, releaseSlug }),
      }
    : undefined;

  const publicationReadAt = performance.now();

  const cleanupFailures = await cleanupSourceMaps(files);

  return {
    data: {
      cleanupFailures,
      fileCount: files.length,
      releaseSlug,
      publication,
      timing: {
        createRelease: Math.round(createdAt - start),
        ...transfer.timing,
        receipt: Math.round(publicationReadAt - acceptedAt),
        cleanup: Math.round(performance.now() - publicationReadAt),
        total: Math.round(performance.now() - start),
      },
    },
    ok: true as const,
  };
}
