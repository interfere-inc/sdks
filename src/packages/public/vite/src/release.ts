import { cleanupSourceMaps } from "@interfere/build/cleanup-source-maps";
import { sourceMapUploadFetch } from "@interfere/build/source-map-upload-fetch";
import { InterfereClient, InterfereError } from "@interfere/javascript";
import { API_URL, PUBLIC_KEY_HEADER } from "@interfere/constants/api";
import { releaseSlugSchema } from "@interfere/types/releases/slug";
import { resolveDeployEnvironment } from "@interfere/types/sdk/deploy-environment";

import { readFile } from "node:fs/promises";
import path from "node:path";
import { z } from "zod";

import { runReleasePipeline } from "./internal/build/pipeline";
import {
  RELEASE_CLIENT_RETRY,
  RELEASE_REQUEST_BUDGET_MS,
} from "./internal/build/request-budget";
import { discoverByPairing } from "./internal/build/source-maps";
import { PRODUCER_VERSION } from "./version";

const receiptSchema = z.object({
  build: z.object({
    buildId: z.string().nullable(),
    commitSha: z.string().nullable(),
    releaseSlug: releaseSlugSchema.nullable(),
    serviceName: z.string().optional(),
  }),
  schemaVersion: z.literal(1),
});

export interface ReleaseLogger {
  error: (message: string) => void;
  info: (message: string) => void;
  warn: (message: string) => void;
}

export interface ReleaseBuildOutputOptions {
  /** Server-side secret key for publishing this build; without it the retained source maps are only deleted. */
  apiKey?: string | undefined;
  /** Interfere API origin; defaults to the hosted API. */
  apiUrl?: string | undefined;
  /** Build output directory holding the chunks and the source maps the build retained. */
  distDir: string;
  /** Deployment environment recorded on the release; defaults to the deploy provider's environment variables. */
  environment?: string | null | undefined;
  logger?: ReleaseLogger | undefined;
  /** Project root the build ran in; defaults to the working directory. */
  projectDir?: string | undefined;
  /** Public workspace or App key sent with the release requests. */
  publicKey?: string | undefined;
  /** The private build receipt the plugin wrote (`interfere({ buildReceipt })`). */
  receipt: string;
}

export type ReleaseBuildOutputResult =
  | { fileCount: number; releaseSlug: string; status: "released" }
  | { reason: string; status: "skipped" }
  | { message: string; status: "failed" };

const releaseUploadConflictSchema = z.object({
  code: z.literal("RELEASE_UPLOAD_CONFLICT"),
});

const RELEASE_ATTEMPTS = 2;

const PERMANENT_STATUS = (status: number) =>
  status < 500 && status !== 408 && status !== 429;

async function readReceipt(projectDir: string, receipt: string) {
  return receiptSchema.parse(
    JSON.parse(await readFile(path.resolve(projectDir, receipt), "utf8")),
  );
}

async function discover(distDir: string, projectDir: string) {
  return discoverByPairing({
    distDir,
    projectDir,
    publicPathRewrites: [{ from: "public", to: "" }],
    subtrees: ["."],
  });
}

/**
 * Registers a release for a build that retained its source maps
 * (`interfere({ sourceMaps: { retain: true } })`) and uploads them, then
 * deletes the maps from the output so they are never served. Run it once per
 * deployment environment on the same build output. API failures are reported,
 * never thrown, and the maps are deleted either way.
 */
export async function releaseBuildOutput(options: ReleaseBuildOutputOptions) {
  const logger = options.logger ?? console;
  const projectDir = options.projectDir ?? process.cwd();

  const discovered = await discover(options.distDir, projectDir);

  const finish = async (result: ReleaseBuildOutputResult) => {
    for (const failure of await cleanupSourceMaps(discovered.files)) {
      logger.warn(
        `[interfere] cleanup left ${failure.absolute} on disk: ${failure.code ?? failure.message}`,
      );
    }

    return result;
  };

  if (!options.apiKey) {
    return finish({
      reason:
        "INTERFERE_API_KEY is not set; retained source maps were deleted without a release.",
      status: "skipped",
    });
  }

  const { build } = await readReceipt(projectDir, options.receipt);

  if (!build.releaseSlug || !build.commitSha) {
    return finish({
      reason:
        "the build receipt has no release identity (no commit SHA at build time).",
      status: "skipped",
    });
  }

  if (discovered.files.length === 0) {
    return finish({
      reason: `no source maps paired with chunks under ${options.distDir}.`,
      status: "skipped",
    });
  }

  const environment =
    options.environment === undefined
      ? // oxlint-disable-next-line node/no-process-env
        resolveDeployEnvironment(process.env)
      : options.environment;

  const client = new InterfereClient({
    apiKey: { token: options.apiKey },
    baseUrl: options.apiUrl || API_URL,
    fetch: sourceMapUploadFetch,
    headers: {
      "user-agent": PRODUCER_VERSION,
      [PUBLIC_KEY_HEADER]: options.publicKey,
      "x-interfere-service": build.serviceName,
    },
    ...RELEASE_CLIENT_RETRY,
  });

  const buildId = build.buildId ?? build.commitSha;
  const { releaseSlug } = build;

  const release = async () => {
    const releasesConfig = await client.releases.getReleasesConfig({
      abortSignal: AbortSignal.timeout(RELEASE_REQUEST_BUDGET_MS),
    });

    return runReleasePipeline({
      buildId,
      bundler: "rollup",
      client,
      discovery: discovered,
      environment,
      producerVersion: PRODUCER_VERSION,
      releaseSlug,
      releasesConfig,
      service: build.serviceName ? { name: build.serviceName } : undefined,
      uploadSourceMaps: true,
    });
  };

  try {
    let result: Awaited<ReturnType<typeof release>> | undefined;

    for (let attempt = 1; !result; attempt += 1) {
      try {
        // oxlint-disable-next-line no-await-in-loop -- A bundle retries only after its previous attempt settles.
        result = await release();
      } catch (error) {
        if (
          attempt >= RELEASE_ATTEMPTS ||
          (error instanceof InterfereError &&
            error.statusCode !== undefined &&
            PERMANENT_STATUS(error.statusCode))
        ) {
          throw error;
        }

        logger.warn(
          `[interfere] release attempt ${attempt} for ${options.distDir} failed (${error instanceof Error ? error.message : String(error)}); retrying.`,
        );
      }
    }

    for (const failure of result.cleanupFailures) {
      logger.warn(
        `[interfere] cleanup left ${failure.absolute} on disk: ${failure.code ?? failure.message}`,
      );
    }

    logger.info(
      `[interfere] release ${result.releaseSlug} confirmed for ${environment ?? "no environment"}: ${result.fileCount} maps, ${result.timing?.total ?? 0}ms.`,
    );

    return {
      fileCount: result.fileCount,
      releaseSlug: result.releaseSlug,
      status: "released",
    } satisfies ReleaseBuildOutputResult;
  } catch (error) {
    if (
      error instanceof InterfereError &&
      error.statusCode === 409 &&
      releaseUploadConflictSchema.safeParse(error.body).success
    ) {
      return await finish({
        reason: `release ${build.releaseSlug} for ${environment ?? "no environment"} already holds source maps from another build of this bundle (receipt commit ${build.commitSha}); keeping the existing release.`,
        status: "skipped",
      });
    }

    return await finish({
      message: error instanceof Error ? error.message : String(error),
      status: "failed",
    });
  }
}
