/* oxlint-disable typescript/no-unnecessary-condition */
import type { Interfere } from "@interfere/javascript";
import { resolveBuildReleaseSlug } from "@interfere/build/release-slug";
import { resolveBuildServiceName } from "@interfere/build/service-name";
import { omitUndefined } from "@interfere/helpers/omit-undefined";
import type { CreateReleaseResponse } from "@interfere/types/releases/definition";
import type { ReleaseSlug } from "@interfere/types/releases/slug";
import {
  checkReleaseConfig,
  diagnoseRelease,
} from "@interfere/types/sdk/build-diagnostics";
import type { ReleaseDiagnosticContext } from "@interfere/types/sdk/build-diagnostics";
import { resolveDeployEnvironment } from "@interfere/types/sdk/deploy-environment";
import {
  BUILD_MARKER_GLOBAL,
  DISABLED_GLOBAL,
} from "@interfere/types/sdk/injected-globals";

import type { NextConfig } from "next";
import { PHASE_PRODUCTION_BUILD } from "next/constants.js";

import {
  createBuildPreflight,
  preflightMode,
  surfaceGlobals,
} from "./internal/build/preflight.js";
import { resolveBrowserApiHost } from "./internal/build/browser-api-host.js";
import type { SurfaceConfig } from "./internal/build/preflight.js";
import { configureBuild } from "./internal/build/configure-build.js";
import { interfereCompilerConfig } from "./internal/build/production-publication.js";
import { withLegacyWebpackPublication } from "./internal/build/legacy-webpack-publication.js";
import {
  interfereExperimentalConfig,
  needsInstrumentationHookFlag,
} from "./internal/build/experimental-config.js";
import { runGitCommand } from "./internal/build/release/git.js";
import { interfereInjectedValues } from "./internal/build/injected-values.js";
import { readInterfereEnv } from "./internal/env.js";
import { log } from "./internal/logger.js";
import { resolveReleaseSlug } from "./internal/release-slug.js";
import {
  readNextMajorVersion,
  supportsProductionCompileHook,
  warnIfPublicKeyDeprecated,
  warnIfServerInstrumentationMissing,
} from "./internal/setup-warnings.js";

const DIAGNOSTIC_CONTEXT: ReleaseDiagnosticContext = {
  publicKeyEnvVar: "INTERFERE_PUBLIC_KEY",
  docsUrl: "https://interfere.com/docs/sdk/next-js",
};

export interface NextConfigWithInterfere extends NextConfig {
  interfere?: {
    /**
     * Reserved. Setting `buildId` here is rejected loudly — it caused
     * server/client slug drift in 9.x. Override the commit SHA via the
     * `INTERFERE_SOURCE_ID` env var (set on both build and runtime envs)
     * instead.
     */
    readonly buildId?: never;
    /**
     * The endpoint the browser SDK posts telemetry to. Defaults to
     * Interfere's endpoint. Set it to a subdomain of your own site that
     * resolves to Interfere via a DNS record (e.g.
     * `"https://in.yourapp.com"`) to survive ad-blockers, or to a
     * regional endpoint. Must be a full URL.
     */
    readonly apiHost?: string;
  };
}

/**
 * Interfere-specific build settings, passed as the first argument to
 * `withInterfere(settings, nextConfig)`. Kept separate from the Next.js
 * config so SDK options never collide with Next's own keys.
 */
export interface NextPluginConfig {
  /** Default service name for browser, server, and edge telemetry. Falls back to OTEL_SERVICE_NAME, then the app package name without its scope. */
  readonly serviceName?: string;
  readonly sourceMaps?: {
    /** Upload source maps to Interfere. Defaults to true. Release registration and telemetry stay enabled when false. */
    readonly upload?: boolean;
    /** Delete discovered source maps and their references after the build hook, including upload failures or disabled uploads. Defaults to true. */
    readonly purge?: boolean;
  };
  /** Turns Interfere off for this build: no release, no source-map upload, and the browser and server SDKs send no events. */
  readonly disable?: boolean;
}

export interface ResolvedBuildConfig {
  readonly releaseConfig?: Interfere.ReleasesConfigResponse | undefined;
  readonly apiKey: string | null;
  readonly apiUrl: string;
  /**
   * Server-internal — the build pipeline still uses this to key R2
   * source-map paths until Phase 7 re-keys them to slug. Never injected
   * into the bundle; never exposed via env.
   */
  readonly buildId: string | null;
  /**
   * Deploy-environment label (`INTERFERE_ENVIRONMENT` ?? `VERCEL_ENV`)
   * resolved at build time. Stamped into the bundle as
   * `__INTERFERE_ENVIRONMENT__` so the browser kernel — which can't read
   * non-public env vars — labels events with the same environment as the
   * server. `NODE_ENV` is deliberately not folded in: it stays the browser's
   * bundler-inlined runtime fallback, so dev bundles keep resolving
   * `development` even when this is unset.
   */
  readonly environment: string | null;
  readonly publicKey: string | null;
  /**
   * The release a production build registered during preflight, or the shared local-development release the dev server minted. Its token is
   * stamped into the bundle as `__INTERFERE_RELEASE_TOKEN__` and the build
   * pipeline reuses it, so a build creates its release once.
   */
  readonly registeredRelease?: CreateReleaseResponse | undefined;
  readonly serviceName?: string | undefined;
  /**
   * When `true` (default), the build pipeline deletes source maps from the
   * build output whether uploading succeeds, fails, or is disabled.
   * Resolved from `NextPluginConfig.sourceMaps.purge`; set it `false` to
   * leave maps on disk for another uploader that shares the same `.next`
   * output.
   */
  readonly purgeSourceMaps: boolean;
  readonly uploadSourceMaps: boolean;
  /**
   * Deterministically derived from the leading 16 hex of the commit SHA, so
   * the build step and the collector arrive at the same
   * `rel_xxxxxxxxxxxxxxxx` without a round-trip. Stamped into the bundle as
   * `__INTERFERE_RELEASE_SLUG__`. `null` when no commit SHA is available —
   * release-pipeline calls bail in that case.
   */
  readonly releaseSlug: ReleaseSlug | null;
  /**
   * The surface configuration snapshot a production build resolves once by
   * public key. The server globals baked by the value-injection loader read
   * this object, so one build cannot see two different responses. Absent
   * for the dev server and for builds without a public key; the dev server's only preflight result is its minted token.
   */
  readonly surfaceConfig?: SurfaceConfig | undefined;
}

type InterfereConfigFn = (
  phase: string,
  context: { defaultConfig: NextConfig },
) =>
  | NextConfigWithInterfere
  | Promise<NextConfigWithInterfere>
  | PromiseLike<NextConfigWithInterfere>;

type InterfereConfigInput = NextConfigWithInterfere | InterfereConfigFn;

type WithInterfereResult<TConfig> = (
  phase: string,
  context?: { defaultConfig: NextConfig },
) => TConfig extends (...args: never[]) => PromiseLike<NextConfigWithInterfere>
  ? Promise<NextConfig>
  : NextConfig | Promise<NextConfig>;

function isThenable(
  value: unknown,
): value is PromiseLike<NextConfigWithInterfere> {
  return (
    typeof value === "object" &&
    value !== null &&
    "then" in value &&
    typeof value.then === "function"
  );
}

export function withInterfere<TConfig extends InterfereConfigInput>(
  upstream?: TConfig,
): WithInterfereResult<TConfig>;

export function withInterfere<TConfig extends InterfereConfigInput>(
  settings: NextPluginConfig,
  config: TConfig,
): WithInterfereResult<TConfig>;

export function withInterfere(
  ...args:
    | [upstream?: InterfereConfigInput]
    | [settings: NextPluginConfig, config: InterfereConfigInput]
) {
  const [settings, nextConfig] =
    args.length === 2 ? [args[0], args[1]] : [{}, args[0] ?? {}];

  const withBuildPreflight = createBuildPreflight();

  const returnValue: (
    phase: string,
    context?: { defaultConfig: NextConfig },
    // oxlint-disable-next-line typescript/promise-function-async
  ) => NextConfig | Promise<NextConfig> = (phase, context) => {
    const resolved =
      // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Next configuration accepts either an object or a phase-dependent function; only the function variant is invoked.
      typeof nextConfig === "function"
        ? nextConfig(phase, context ?? { defaultConfig: {} })
        : nextConfig;

    if (isThenable(resolved)) {
      return Promise.resolve(resolved).then(async (config) =>
        buildInterfereConfig(phase, settings, config, withBuildPreflight),
      );
    }

    return buildInterfereConfig(phase, settings, resolved, withBuildPreflight);
  };

  return returnValue;
}

function resolveBrowserSourceMaps(
  config: ResolvedBuildConfig,
  productionBrowserSourceMaps: boolean | undefined,
) {
  const releasing = config.apiKey !== null && config.uploadSourceMaps;

  if (releasing && productionBrowserSourceMaps === false) {
    log.warn("Forcing productionBrowserSourceMaps", [
      "withInterfere() requires production source maps to symbolicate errors.",
      "Your `productionBrowserSourceMaps: false` was overridden to `true`.",
    ]);
  }

  return (releasing ? true : productionBrowserSourceMaps) satisfies
    | boolean
    | undefined;
}

function disabledInterfereConfig(nextConfig: NextConfigWithInterfere) {
  const { interfere: _interfere, webpack, turbopack, ...rest } = nextConfig;
  const projectDir = process.cwd();

  const build = configureBuild({
    existingTurbopack: turbopack,
    existingWebpack: webpack,
    nextMajor: readNextMajorVersion(projectDir),
    projectDir,
    values: {
      [BUILD_MARKER_GLOBAL]: true,
      [DISABLED_GLOBAL]: true,
    },
  });

  return omitUndefined({
    ...rest,
    turbopack: build.turbopack,
    webpack: build.webpack,
  });
}

// oxlint-disable-next-line typescript/promise-function-async -- Next preserves synchronous development configuration and preflights only eligible production builds.
function buildInterfereConfig(
  phase: string,
  settings: NextPluginConfig,
  nextConfig: NextConfigWithInterfere,
  withBuildPreflight: ReturnType<typeof createBuildPreflight>,
) {
  const config = resolveBuildConfig(settings, phase);

  const mode = settings.disable ? null : preflightMode(phase, config);

  return withBuildPreflight(mode, config, (preflight) =>
    buildResolvedInterfereConfig(phase, settings, nextConfig, {
      ...config,
      ...preflight,
    }),
  );
}

function buildResolvedInterfereConfig(
  phase: string,
  settings: NextPluginConfig,
  nextConfig: NextConfigWithInterfere,
  resolvedBuild?: ResolvedBuildConfig,
) {
  if (settings.disable) {
    return disabledInterfereConfig(nextConfig);
  }

  const {
    interfere,
    env: userEnv,
    webpack,
    turbopack,
    compiler,
    productionBrowserSourceMaps,
    ...rest
  } = nextConfig;

  if (interfere && "buildId" in interfere && interfere.buildId !== undefined) {
    return log.fatal("Removed: interfere.buildId", [
      "`interfere.buildId` was removed in SDK 10. It only worked at build time, so server-side spans drifted from the bundle's slug.",
      "Override the commit SHA via the `INTERFERE_SOURCE_ID` env var on both build and runtime envs instead.",
    ]);
  }

  const projectDir = process.cwd();

  const config: ResolvedBuildConfig =
    resolvedBuild ?? resolveBuildConfig(settings, phase);

  const apiHost = resolveBrowserApiHost(interfere?.apiHost, config.apiUrl);

  const nextMajor = readNextMajorVersion(projectDir);
  const legacyCompiler = !supportsProductionCompileHook(projectDir);

  const configuredCompiler = interfereCompilerConfig(
    compiler,
    config,
    runBuildReleasePipeline,
  );

  const { runAfterProductionCompile, ...legacyCompilerConfig } =
    configuredCompiler;

  const build = configureBuild({
    existingTurbopack: turbopack,
    existingWebpack: webpack,
    nextMajor,
    projectDir,
    values: {
      ...interfereInjectedValues(config, apiHost, config.serviceName),
      ...surfaceGlobals(config.surfaceConfig),
    },
  });

  warnIfPublicKeyDeprecated(config.publicKey);

  const browserCredential =
    config.publicKey !== null || Boolean(config.registeredRelease?.token);

  if (browserCredential && !build.clientInstrumentationApplies) {
    log.warn("Missing instrumentation client", [
      "This build has no instrumentation-client file, so it sends no browser events.",
      "Create an instrumentation-client.ts file in your project root or src/ directory.",
    ]);
  }

  if (browserCredential) {
    warnIfServerInstrumentationMissing(projectDir);
  }

  const experimental = interfereExperimentalConfig({
    existing: rest.experimental,
    instrumentationHook: needsInstrumentationHookFlag(nextMajor),
    traceMetadata: browserCredential,
    sourceMaps: config.apiKey !== null && config.uploadSourceMaps,
  });

  return omitUndefined({
    ...rest,
    env: userEnv,

    compiler: legacyCompiler ? legacyCompilerConfig : configuredCompiler,

    productionBrowserSourceMaps: resolveBrowserSourceMaps(
      config,
      productionBrowserSourceMaps,
    ),
    turbopack: build.turbopack,
    webpack:
      legacyCompiler && phase === PHASE_PRODUCTION_BUILD
        ? withLegacyWebpackPublication(
            build.webpack,
            {
              projectDir,
              distDir: nextConfig.distDir ?? ".next",
            },
            runAfterProductionCompile,
          )
        : build.webpack,

    experimental,
  });
}

function resolveBuildConfig(settings: NextPluginConfig, phase: string) {
  const { apiKey, apiUrl, publicKey } = readInterfereEnv();
  const { commitSha, slug } = resolveReleaseSlug(runGitCommand);
  const isProductionBuild = phase === PHASE_PRODUCTION_BUILD;

  const environment = isProductionBuild
    ? // oxlint-disable-next-line node/no-process-env
      resolveDeployEnvironment(process.env)
    : null;

  return {
    apiKey: apiKey ?? null,
    apiUrl,
    buildId: commitSha,
    environment,
    publicKey: publicKey ?? null,
    serviceName: resolveBuildServiceName({
      root: process.cwd(),
      option: settings.serviceName,
    }),
    purgeSourceMaps: settings.sourceMaps?.purge ?? true,
    uploadSourceMaps: settings.sourceMaps?.upload ?? true,
    releaseSlug:
      isProductionBuild && commitSha
        ? resolveBuildReleaseSlug({
            commitSha,
            // oxlint-disable-next-line node/no-process-env
            deploymentId: process.env["VERCEL_DEPLOYMENT_ID"],
          })
        : slug,
  } satisfies ResolvedBuildConfig;
}

export interface ProductionCompileContext {
  readonly distDir: string;
  readonly projectDir: string;
}

async function runBuildReleasePipeline(
  context: ProductionCompileContext,
  config: ResolvedBuildConfig,
) {
  const release = checkReleaseConfig(DIAGNOSTIC_CONTEXT, {
    apiKey: config.apiKey,
    commitSha: config.buildId,
    releaseSlug: config.releaseSlug,
  });

  if (release.status === "skip" || release.status === "warn") {
    log.warn(release.title, [...release.details]);

    if (config.purgeSourceMaps) {
      const { cleanupBuildSourceMaps } =
        await import("./internal/build/source-maps/cleanup.js");

      const failures = await cleanupBuildSourceMaps(context, []);

      if (failures.length > 0) {
        log.warn(
          "Source-map cleanup left files on disk",
          failures.map((failure) => `${failure.absolute}: ${failure.message}`),
        );
      }
    }

    return;
  }

  if (release.status === "fail") {
    return log.fatal(release.title, [...release.details]);
  }

  const { runBuildPipeline } = await import("./internal/build/pipeline.js");

  try {
    const result = await runBuildPipeline(context, {
      ...config,
      apiKey: release.apiKey,
      buildId: release.commitSha,
      releaseSlug: release.releaseSlug,
    });

    if (!result.accepted) {
      if (result.reason === "no_source_maps") {
        log.warn("Release remains unready", [
          "No source maps found. Enable source-map generation and upload them during the build.",
        ]);
      }

      return;
    }

    log.info("Completed", [
      `https://interfere.com/~/${result.release.org.slug}/services`,
      `Release: ${result.release.destination.slug}`,
      `Build: ${result.buildId}`,
      config.uploadSourceMaps
        ? `Artifacts: ${result.fileCount} source maps`
        : "Source-map uploads disabled",
    ]);
  } catch (error) {
    const diagnostic = diagnoseRelease(DIAGNOSTIC_CONTEXT, error);

    log[diagnostic.severity](diagnostic.title, [...diagnostic.details]);
  }
}
