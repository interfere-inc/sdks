import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { sourceMapUploadFetch } from "@interfere/build/source-map-upload-fetch";
import { uploadSourceMapChunks } from "@interfere/build/upload-source-map-chunks";
import { PUBLIC_KEY_HEADER } from "@interfere/constants/api";
import { InterfereClient } from "@interfere/javascript";
import type { Interfere } from "@interfere/javascript";
import { completeSourceMapsResponse } from "@interfere/types/data/source-maps";
import { releaseBuildResultSchema } from "@interfere/types/releases/build-telemetry";
import { createReleaseResponse } from "@interfere/types/releases/definition";
import type { ReleaseSlug } from "@interfere/types/releases/slug";
import { classifyReleasePipelineError } from "@interfere/types/sdk/build-diagnostics";
import { extractRichness } from "@interfere/types/sdk/source-map-richness";

import type {
  ProductionCompileContext,
  ResolvedBuildConfig,
} from "../../config.js";
import { log } from "../logger.js";
import { PRODUCER_VERSION } from "../version.js";
import { readNextMajorVersion } from "../setup-warnings.js";
import { detectActiveBundler } from "./detect-bundler.js";
import { resolveReleaseRequest } from "./release/index.js";
import type { SourceMapFile } from "./source-maps/discover.js";
import { discoverSourceMaps } from "./source-maps/discover.js";
import { cleanupBuildSourceMaps } from "./source-maps/cleanup.js";
import { reportUnmappedChunks } from "./source-maps/guard.js";
import { EMPTY_TELEMETRY } from "./telemetry.js";

export async function runBuildPipeline(
  context: ProductionCompileContext,
  metadata: ResolvedBuildConfig & {
    apiKey: string;
    buildId: string;
    releaseSlug: ReleaseSlug;
  },
) {
  const client = new InterfereClient({
    fetch: sourceMapUploadFetch,
    baseUrl: metadata.apiUrl,
    apiKey: { token: metadata.apiKey },
    headers: {
      [PUBLIC_KEY_HEADER]: metadata.publicKey,
      "x-interfere-producer-version": PRODUCER_VERSION,
      "x-interfere-service": metadata.serviceName,
    },
    maxRetries: 4,
    timeoutInSeconds: 10,
  });

  const start = performance.now();
  const timing = { ...EMPTY_TELEMETRY };
  let { releaseSlug } = metadata;
  let files: SourceMapFile[] = [];

  const cleanupFailures: Awaited<ReturnType<typeof cleanupBuildSourceMaps>> =
    [];

  let reportResult: NonNullable<Interfere.TelemetryRequest["result"]> | null =
    "pipeline_failed";

  try {
    const bundler = detectActiveBundler(
      // oxlint-disable-next-line node/no-process-env
      process.env,
      process.argv,
      readNextMajorVersion(context.projectDir),
    );

    const discovered =
      metadata.uploadSourceMaps || metadata.purgeSourceMaps
        ? await discoverSourceMaps({ bundler, ...context })
        : { files: [], missingSourceMapCount: 0, sourceFileCount: 0 };

    ({ files } = discovered);

    const { sourceFileCount } = discovered;

    if (metadata.uploadSourceMaps) {
      await reportUnmappedChunks({ discovered: files, ...context });
    }

    if (metadata.uploadSourceMaps && discovered.missingSourceMapCount > 0) {
      log.warn("Some source maps are missing", [
        `${discovered.missingSourceMapCount} referenced source map(s) not found. Errors in those chunks will not resolve to source.`,
      ]);
    }

    const config =
      metadata.releaseConfig ??
      (await client.releases.getReleasesConfig({
        abortSignal: AbortSignal.timeout(60_000),
      }));

    timing.discover = Math.round(performance.now() - start);
    timing.fileCount = files.length;

    const request = resolveReleaseRequest(
      metadata.buildId,
      metadata.environment,
      metadata.releaseSlug,
      config,
    );

    if (!request.ok) {
      reportResult = null;

      return {
        accepted: false as const,
        fileCount: 0,
        reason: request.reason,
      };
    }

    if (metadata.uploadSourceMaps && files.length === 0) {
      reportResult = "no_source_maps";

      return {
        accepted: false as const,
        fileCount: 0,
        reason: "no_source_maps" as const,
      };
    }

    const registering = performance.now();

    const release = createReleaseResponse.parse(
      await client.releases.createRelease(request.request, {
        abortSignal: AbortSignal.timeout(60_000),
      }),
    );

    const registered = performance.now();

    releaseSlug = release.destination.slug;

    timing.createRelease = Math.round(registered - registering);

    if (metadata.uploadSourceMaps) {
      const signFiles = files.map((file) => ({
        path: file.path,
        sizeBytes: new TextEncoder().encode(file.content).byteLength,
        sha256: file.hash,
        ...extractRichness(file.content),
      }));

      const environment = metadata.environment ?? undefined;

      const signed = await client.releases.signSourceMapUploads(
        { releaseSlug, environment, files: signFiles },
        { abortSignal: AbortSignal.timeout(60_000) },
      );

      timing.totalBytes = await uploadSourceMapChunks({
        compression: "gzip",
        client,
        environment,
        releaseSlug,
        files,
        signed,
      });

      const accepted = completeSourceMapsResponse.safeParse(
        await client.releases.finalizeSourceMaps(
          {
            releaseSlug,
            environment,
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
    }

    timing.upload = metadata.uploadSourceMaps
      ? Math.round(performance.now() - registered)
      : 0;

    reportResult = "ok";

    return {
      accepted: true as const,
      buildId: release.build.hash,
      cleanupFailures,
      config,
      fileCount: metadata.uploadSourceMaps ? files.length : 0,
      release,
      timing,
    };
  } catch (error) {
    reportResult =
      releaseBuildResultSchema.safeParse(
        classifyReleasePipelineError(error).toLowerCase(),
      ).data ?? "pipeline_failed";

    throw error;
  } finally {
    const cleanupStart = performance.now();

    if (metadata.purgeSourceMaps) {
      try {
        cleanupFailures.push(...(await cleanupBuildSourceMaps(context, files)));
      } catch (error) {
        cleanupFailures.push({
          absolute: context.distDir,
          code: undefined,
          message: error instanceof Error ? error.message : String(error),
        });
      }
    }

    timing.cleanup = Math.round(performance.now() - cleanupStart);

    if (cleanupFailures.length > 0) {
      log.warn(
        "Source-map cleanup left files on disk",
        cleanupFailures.map(
          (failure) => `${failure.absolute}: ${failure.message}`,
        ),
      );
    }

    timing.total = Math.round(performance.now() - start);

    if (reportResult !== null) {
      await client.releases
        .reportTelemetry(
          { ...timing, result: reportResult, releaseSlug },
          { abortSignal: AbortSignal.timeout(10_000) },
        )
        .catch(() => undefined);
    }
  }
}
