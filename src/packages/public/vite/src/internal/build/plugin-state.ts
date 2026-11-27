import { API_URL } from "@interfere/constants/api";
import { checkReleaseConfig } from "@interfere/types/sdk/build-diagnostics";

import path from "node:path";
import { loadEnv } from "vite";
import type { ResolvedConfig } from "vite";

import type { BrowserConfig } from "./browser-define";
import { DIAGNOSTIC_CONTEXT } from "./preflight";
import type { PluginState } from "./release-build";
import { pickEnvValue } from "../plugin-env";
import type { InterfereViteOptions } from "../plugin-options";

export const MILLIS_PER_SECOND = 1000;

const SERVER_RUNTIME_PLUGIN_HINTS = [
  "tanstack-start",
  "nitro",
  "@tanstack/react-start",
] as const;

function buildError(title: string, details: readonly string[]) {
  return new Error([`Interfere: ${title}`, ...details].join("\n"));
}

function hasServerRuntimePlugin(pluginNames: string[]) {
  return pluginNames.some((name) =>
    SERVER_RUNTIME_PLUGIN_HINTS.some((hint) => name.includes(hint)),
  );
}

function resolveReleaseEnv(config: ResolvedConfig) {
  // oxlint-disable-next-line node/no-process-env
  return { ...loadEnv(config.mode, config.envDir, ""), ...process.env };
}

export function buildPluginState(
  options: InterfereViteOptions,
  config: ResolvedConfig,
  browser: BrowserConfig,
) {
  const { logger } = config;
  const isProductionBuild = config.command === "build";

  const releaseEnv = resolveReleaseEnv(config);
  const apiKey = pickEnvValue(releaseEnv, "INTERFERE_API_KEY");
  const apiUrl = pickEnvValue(releaseEnv, "INTERFERE_API_URL") ?? API_URL;

  const retainSourceMaps =
    isProductionBuild && options.sourceMaps?.retain === true;

  const libraryBuild =
    Boolean(config.build.lib) &&
    !retainSourceMaps &&
    options.sourceMaps?.upload !== true;

  const release =
    isProductionBuild && !retainSourceMaps && !libraryBuild
      ? checkReleaseConfig(DIAGNOSTIC_CONTEXT, {
          apiKey,
          commitSha: browser.commitSha,
          releaseSlug: browser.releaseSlug,
        })
      : null;

  if (release?.status === "fail") {
    throw buildError(release.title, release.details);
  }

  const releaseEnabled = release?.status === "ok";

  const uploadOutDir = path.resolve(config.root, config.build.outDir);

  const serverSourcemapsDir = options.serverSourcemapsDir
    ? path.resolve(config.root, options.serverSourcemapsDir)
    : path.join(uploadOutDir, "server");

  const serverBuild =
    hasServerRuntimePlugin(config.plugins.map((p) => p.name)) ||
    Boolean(config.build.ssr);

  return {
    apiHost: browser.apiHost,
    apiKey,
    apiUrl,
    buildStartedAt: Math.floor(Date.now() / MILLIS_PER_SECOND) * 1000,
    commitSha: browser.commitSha,
    releaseEnabled,
    retainSourceMaps,
    skippedRelease:
      release?.status === "skip" || release?.status === "warn" ? release : null,
    uploadSourceMaps: releaseEnabled && options.sourceMaps?.upload !== false,
    environment: browser.environment,
    isProductionBuild,
    libraryBuild: isProductionBuild && libraryBuild,
    logger,
    projectDir: config.root,
    serviceName: browser.serviceName,
    releaseSlug: browser.releaseSlug,
    serverBuild,
    ssrBuild: Boolean(config.build.ssr),
    serverSourcemapsDir,
    uploadOutDir,
    releaseStarted: false,
    uploadComplete: false,
    clientOutDirs: [],
    environmentOutDirs: [],
    watch: isProductionBuild && Boolean(config.build.watch),
  } satisfies PluginState;
}

export function announceSetup(state: PluginState) {
  state.logger.info(
    `[interfere] ingest → ${state.apiHost ?? "Interfere (default)"}`,
  );

  if (!state.apiHost && state.serverBuild) {
    state.logger.warn(
      "[interfere] Telemetry posts directly to Interfere. On a server-rendered app, set interfere({ apiHost }) to a subdomain of your own site to survive ad-blockers.",
    );
  }

  if (state.isProductionBuild && !state.releaseToken) {
    state.logger.warn(
      "[interfere] This build has no release token, so it sends no events. Set INTERFERE_API_KEY on builds that you deploy and the plugin bakes in a release token.",
    );
  }

  if (state.libraryBuild) {
    state.logger.info(
      "[interfere] library build: no release is registered and the output is left untouched. Pass interfere({ sourceMaps: { upload: true } }) to upload its source maps.",
    );
  }

  if (state.retainSourceMaps) {
    state.logger.info(
      "[interfere] source maps are retained in the build output for a release step to upload per environment.",
    );
  }

  if (state.skippedRelease) {
    state.logger.warn(
      [state.skippedRelease.title, ...state.skippedRelease.details]
        .map((line) => `[interfere] ${line}`)
        .join("\n"),
    );
  }
}

export function carryOverUploadProgress(
  previous: PluginState | null,
  next: PluginState,
) {
  if (!(previous?.serverBuild && next.serverBuild)) {
    return;
  }

  next.uploadOutDir = previous.uploadOutDir;
  next.serverSourcemapsDir = previous.serverSourcemapsDir;
  next.releaseStarted = previous.releaseStarted;
  next.uploadComplete = previous.uploadComplete;
  next.buildStartedAt = previous.buildStartedAt;
}
