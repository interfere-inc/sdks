import type { CreateReleaseResponse } from "@interfere/types/releases/definition";
import { cleanupSourceMaps } from "@interfere/build/cleanup-source-maps";
import { sourceMapUploadFetch } from "@interfere/build/source-map-upload-fetch";
import { InterfereClient } from "@interfere/javascript";
import type { Interfere } from "@interfere/javascript";
import { PUBLIC_KEY_HEADER } from "@interfere/constants/api";
import type { ReleaseSlug } from "@interfere/types/releases/slug";
import { diagnoseRelease } from "@interfere/types/sdk/build-diagnostics";
import type { ReleaseDiagnostic } from "@interfere/types/sdk/build-diagnostics";

import { existsSync, readdirSync, statSync } from "node:fs";
import path from "node:path";
import type { Logger } from "vite";

import { PRODUCER_VERSION } from "../../version";
import type { createBuildReceipt } from "./build-receipt";
import { runReleasePipeline } from "./pipeline";
import { DIAGNOSTIC_CONTEXT } from "./preflight";
import {
  RELEASE_CLIENT_RETRY,
  RELEASE_REQUEST_BUDGET_MS,
} from "./request-budget";
import { discoverByPairing, pairSourceMapsForCleanup } from "./source-maps";
import type { DiscoveryResult } from "./source-maps";

function logDiagnostic(logger: Logger, diagnostic: ReleaseDiagnostic) {
  for (const line of [diagnostic.title, ...diagnostic.details]) {
    logger[diagnostic.severity](`[interfere] ${line}`);
  }
}

export interface PluginState {
  releaseConfig?: Interfere.ReleasesConfigResponse | undefined;
  receipt?: ReturnType<typeof createBuildReceipt> | undefined;
  apiHost: string | undefined;
  apiKey: string | undefined;
  apiUrl: string;
  buildStartedAt: number;
  commitSha: string | null;
  releaseEnabled: boolean;
  retainSourceMaps: boolean;
  skippedRelease: { title: string; details: readonly string[] } | null;
  uploadSourceMaps: boolean;
  environment: string | null;
  isProductionBuild: boolean;
  libraryBuild: boolean;
  logger: Logger;
  projectDir: string;
  publicKey: string | undefined;
  releaseToken?: boolean;
  registeredRelease?: CreateReleaseResponse | undefined;
  serviceName: string | undefined;
  releaseSlug: ReleaseSlug | null;
  serverBuild: boolean;
  ssrBuild: boolean;
  serverSourcemapsDir: string;
  uploadOutDir: string;
  releaseStarted: boolean;
  uploadComplete: boolean;
  clientOutDirs: readonly string[];
  watch: boolean;
}

type UploadReadyPluginState = PluginState & {
  apiKey: string;
  commitSha: string;
  releaseSlug: ReleaseSlug;
};

function hasFreshServerSourceMaps(root: string, sinceMs: number) {
  if (!existsSync(root)) {
    return false;
  }

  const entries = readdirSync(root, { withFileTypes: true });

  for (const entry of entries) {
    const absolute = path.join(root, entry.name);

    if (entry.isDirectory()) {
      if (hasFreshServerSourceMaps(absolute, sinceMs)) {
        return true;
      }

      continue;
    }

    if (!entry.name.endsWith(".map")) {
      continue;
    }

    if (statSync(absolute).mtimeMs >= sinceMs) {
      return true;
    }
  }

  return false;
}

export function isReleaseReady(
  state: PluginState,
): state is UploadReadyPluginState {
  return Boolean(
    state.releaseEnabled &&
    state.apiKey &&
    state.releaseSlug &&
    state.commitSha &&
    !state.releaseStarted,
  );
}

function shouldWaitForServerOutput(state: UploadReadyPluginState) {
  return (
    state.uploadSourceMaps &&
    state.serverBuild &&
    !state.ssrBuild &&
    !hasFreshServerSourceMaps(state.serverSourcemapsDir, state.buildStartedAt)
  );
}

type ReleasePipelineFailure = Extract<
  Awaited<ReturnType<typeof runReleasePipeline>>,
  { ok: false }
>["failure"];

type ReleasePipelineSuccess = Extract<
  Awaited<ReturnType<typeof runReleasePipeline>>,
  { ok: true }
>;

function logSkippedRelease(logger: Logger, failure: ReleasePipelineFailure) {
  if (failure.kind === "missing_source_provider") {
    logger.warn(
      `[interfere] release skipped: this surface has no version control provider configured. Configure one at https://interfere.com/~/${failure.config.org.slug}/apps/${failure.config.surface.slug}.`,
    );

    return;
  }

  logger.warn(
    `[interfere] release skipped: this surface has no deployment target configured. Configure one at https://interfere.com/~/${failure.config.org.slug}/apps/${failure.config.surface.slug}.`,
  );
}

function logCleanupFailures(
  logger: Logger,
  failures: ReleasePipelineSuccess["data"]["cleanupFailures"],
) {
  for (const failure of failures) {
    logger.warn(
      `[interfere] cleanup left ${failure.absolute} on disk: ${failure.code ?? failure.message}`,
    );
  }
}

async function discoverUploadableSourceMaps(state: UploadReadyPluginState) {
  let discovered: DiscoveryResult;

  try {
    discovered = await discoverByPairing({
      distDir: state.uploadOutDir,
      projectDir: state.projectDir,
      publicPathRewrites: [
        ...state.clientOutDirs.flatMap((outDir) => {
          const from = path
            .relative(state.uploadOutDir, outDir)
            .replaceAll("\\", "/");

          return from === "" || from.startsWith("..") || path.isAbsolute(from)
            ? []
            : [{ from, to: "" }];
        }),
        { from: "public", to: "" },
      ],
      subtrees: ["."],
    });
  } catch (error) {
    logDiagnostic(state.logger, diagnoseRelease(DIAGNOSTIC_CONTEXT, error));

    return null;
  }

  if (discovered.missingSourceMapCount > 0) {
    state.logger.warn(
      `[interfere] ${discovered.missingSourceMapCount} referenced source map(s) not found. Errors in those chunks will not resolve to source.`,
    );
  }

  if (discovered.files.length === 0) {
    state.logger.warn(
      "[interfere] No source maps found; the release remains unready. Enable source-map generation and upload them during the build.",
    );

    return null;
  }

  return discovered;
}

async function publishRelease(
  state: UploadReadyPluginState,
  discovered: DiscoveryResult,
) {
  const { apiKey, apiUrl, releaseSlug, commitSha, logger, publicKey } = state;

  const client = new InterfereClient({
    fetch: sourceMapUploadFetch,
    baseUrl: apiUrl,
    apiKey: { token: apiKey },
    headers: {
      "user-agent": PRODUCER_VERSION,
      [PUBLIC_KEY_HEADER]: publicKey,
      "x-interfere-service": state.serviceName,
    },
    ...RELEASE_CLIENT_RETRY,
  });

  const releasesConfig =
    state.releaseConfig ??
    (await client.releases.getReleasesConfig({
      abortSignal: AbortSignal.timeout(RELEASE_REQUEST_BUDGET_MS),
    }));

  const result = await runReleasePipeline({
    client,
    buildId: commitSha,
    bundler: "rollup",
    discovery: discovered,
    environment: state.environment,
    producerVersion: PRODUCER_VERSION,
    releaseSlug,
    releasesConfig,
    registeredRelease: state.registeredRelease,
    capturePublication: state.receipt !== undefined,
    uploadSourceMaps: state.uploadSourceMaps,
  });

  if (!result.ok) {
    logSkippedRelease(logger, result.failure);

    return;
  }

  logCleanupFailures(logger, result.data.cleanupFailures);

  if (result.data.publication) {
    state.receipt?.recordPublication(result.data.publication);
  }

  if (!result.data.timing) {
    logger.info(
      `[interfere] release ${result.data.releaseSlug} registered without source maps.`,
    );

    return;
  }

  logger.info(
    `[interfere] source-map stages (ms): create=${result.data.timing.createRelease}, sign=${result.data.timing.sign}, session=${result.data.timing.session}, upload=${result.data.timing.upload}, finalize=${result.data.timing.finalize}, receipt=${result.data.timing.receipt}, cleanup=${result.data.timing.cleanup}; bytes=${result.data.timing.totalBytes}.`,
  );

  state.uploadComplete = true;

  logger.info(
    `[interfere] release ${result.data.releaseSlug} confirmed — ${result.data.fileCount} maps, ${result.data.timing.total}ms.`,
  );
}

export async function releaseBuild(state: UploadReadyPluginState) {
  const { logger, releaseSlug } = state;

  logger.info(
    state.uploadSourceMaps
      ? `[interfere] uploading source maps for release ${releaseSlug}…`
      : `[interfere] registering release ${releaseSlug} without source maps…`,
  );

  const discovered = state.uploadSourceMaps
    ? await discoverUploadableSourceMaps(state)
    : { files: [], missingSourceMapCount: 0, sourceFileCount: 0 };

  if (!discovered) {
    logCleanupFailures(
      logger,
      await cleanupSourceMaps(
        await pairSourceMapsForCleanup(state.uploadOutDir),
      ),
    );

    return;
  }

  try {
    await publishRelease(state, discovered);
  } catch (error) {
    logDiagnostic(logger, diagnoseRelease(DIAGNOSTIC_CONTEXT, error));
  }

  if (!state.uploadComplete) {
    logCleanupFailures(logger, await cleanupSourceMaps(discovered.files));
  }
}

export async function releaseBuildWhenReady(state: PluginState) {
  if (!isReleaseReady(state)) {
    return;
  }

  if (shouldWaitForServerOutput(state)) {
    state.logger.info(
      `[interfere] waiting for server build output in ${path.relative(state.projectDir, state.serverSourcemapsDir)} before uploading source maps.`,
    );

    return;
  }

  state.releaseStarted = true;

  await releaseBuild(state);
}
