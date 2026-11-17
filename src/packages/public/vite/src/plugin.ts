import { registerBuildRelease } from "./internal/build/register-release";
import {
  DIAGNOSTIC_CONTEXT,
  preflightViteBinding,
  warnBuildPreflight,
} from "./internal/build/preflight";
import { resolveBuildReleaseSlug } from "@interfere/build/release-slug";
import { resolveBuildServiceName } from "@interfere/build/service-name";
import type { Interfere } from "@interfere/javascript";
import {
  buildConfigCredential,
  createBuildConfigResolver,
} from "@interfere/build/config";
import { API_URL } from "@interfere/constants/api";
import { omitUndefined } from "@interfere/helpers/omit-undefined";
import type { ReleaseSlug } from "@interfere/types/releases/slug";
import { checkReleaseConfig } from "@interfere/types/sdk/build-diagnostics";
import { resolveDeployEnvironment } from "@interfere/types/sdk/deploy-environment";

import path from "node:path";
import { loadEnv } from "vite";
import type { Logger, Plugin, ResolvedConfig, UserConfig } from "vite";

import { runGitCommand } from "./internal/build/git";
import { rejectRemovedOptions } from "./internal/plugin-options";
import type { InterfereViteOptions } from "./internal/plugin-options";
import { createBuildReceipt } from "./internal/build/build-receipt";
import {
  isReleaseReady,
  releaseBuild,
  releaseBuildWhenReady,
} from "./internal/build/release-build";
import type { PluginState } from "./internal/build/release-build";
import { remoteConfigDefine } from "./internal/build/remote-config-define";
import { renderDebugId } from "./internal/build/render-debug-id";
import { resolveCommitSha } from "./internal/release-slug";
import { PRODUCER_VERSION } from "./version";

export type { InterfereViteOptions } from "./internal/plugin-options";

const SERVER_RUNTIME_PLUGIN_HINTS = [
  "tanstack-start",
  "nitro",
  "@tanstack/react-start",
] as const;

function buildError(title: string, details: readonly string[]) {
  return new Error([`Interfere: ${title}`, ...details].join("\n"));
}

function isTestEnv() {
  // oxlint-disable-next-line node/no-process-env
  return Boolean(process.env["VITEST"] || process.env["VITEST_WORKER_ID"]);
}

function pickEnvValue(env: Record<string, string | undefined>, key: string) {
  const value = env[key];

  return (value && value.length > 0 ? value : undefined) satisfies
    | string
    | undefined;
}

interface BrowserConfig {
  apiHost: string | undefined;
  commitSha: string | null;
  environment: string | null;
  publicKey: string | undefined;
  releaseSlug: ReleaseSlug | null;
  serviceName: string | undefined;
}

function hasServerRuntimePlugin(pluginNames: string[]) {
  return pluginNames.some((name) =>
    SERVER_RUNTIME_PLUGIN_HINTS.some((hint) => name.includes(hint)),
  );
}

function normalizeOrigin(host: string) {
  try {
    return new URL(host).origin;
  } catch {
    return null;
  }
}

function resolveApiHost(options: InterfereViteOptions) {
  const host = options.apiHost;

  if (!host) {
    return;
  }

  const origin = normalizeOrigin(host);

  if (!origin) {
    throw new Error(
      'interfere({ apiHost }) requires a full URL, e.g. apiHost: "https://in.yourapp.com"',
    );
  }

  return origin;
}

function resolveBrowserConfig(
  options: InterfereViteOptions,
  env: Record<string, string | undefined>,
  command: "build" | "serve",
  root: string,
) {
  const commitSha =
    options.commitSha ??
    resolveCommitSha((gitCommand) => runGitCommand(gitCommand, root));

  return {
    apiHost: resolveApiHost(options),
    commitSha,
    environment:
      command === "build"
        ? // oxlint-disable-next-line node/no-process-env
          resolveDeployEnvironment({ ...env, ...process.env })
        : null,
    publicKey: env["VITE_INTERFERE_PUBLIC_KEY"],
    serviceName: resolveBuildServiceName({
      option: options.serviceName,
      env,
      root,
    }),
    releaseSlug: resolveBuildReleaseSlug({
      commitSha,
      deploymentId:
        command === "build"
          ? // oxlint-disable-next-line node/no-process-env
            (process.env["VERCEL_DEPLOYMENT_ID"] ?? env["VERCEL_DEPLOYMENT_ID"])
          : undefined,
      variant: options.buildVariant,
    }),
  } satisfies BrowserConfig;
}

function browserDefine(browser: BrowserConfig, releaseToken: string | null) {
  return {
    "import.meta.env.VITE_INTERFERE_API_HOST": JSON.stringify(
      browser.apiHost ?? null,
    ),
    "import.meta.env.VITE_INTERFERE_ENVIRONMENT": JSON.stringify(
      browser.environment,
    ),
    "import.meta.env.VITE_INTERFERE_RELEASE_SLUG": JSON.stringify(
      browser.releaseSlug,
    ),
    "import.meta.env.VITE_INTERFERE_RELEASE_TOKEN":
      JSON.stringify(releaseToken),
    "import.meta.env.VITE_INTERFERE_SERVICE_NAME": JSON.stringify(
      browser.serviceName ?? null,
    ),
  };
}

function resolveReleaseEnv(config: ResolvedConfig) {
  // oxlint-disable-next-line node/no-process-env
  return { ...loadEnv(config.mode, config.envDir, ""), ...process.env };
}

function buildPluginState(
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
    buildStartedAt: Math.floor(Date.now() / 1000) * 1000,
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
    publicKey: browser.publicKey,
    serviceName: browser.serviceName,
    releaseSlug: browser.releaseSlug,
    serverBuild,
    ssrBuild: Boolean(config.build.ssr),
    serverSourcemapsDir,
    uploadOutDir,
    releaseStarted: false,
    uploadComplete: false,
    clientOutDirs: [],
    watch: isProductionBuild && Boolean(config.build.watch),
  } satisfies PluginState;
}

function announceSetup(state: PluginState) {
  state.logger.info(
    `[interfere] ingest → ${state.apiHost ?? "Interfere (default)"}`,
  );

  if (!state.apiHost && state.serverBuild) {
    state.logger.warn(
      "[interfere] Telemetry posts directly to Interfere. On a server-rendered app, set interfere({ apiHost }) to a subdomain of your own site to survive ad-blockers.",
    );
  }

  if (state.isProductionBuild && !state.publicKey && !state.releaseToken) {
    state.logger.warn(
      "[interfere] This build has no release token and no VITE_INTERFERE_PUBLIC_KEY, so it sends no events. Set INTERFERE_API_KEY on builds that you deploy and the plugin bakes in a release token.",
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

type SourcemapSetting = boolean | "inline" | "hidden";

function mirrorKeysForDevServer(
  command: "build" | "serve",
  env: Record<string, string | undefined>,
) {
  if (command !== "serve") {
    return;
  }

  for (const name of ["VITE_INTERFERE_PUBLIC_KEY", "INTERFERE_API_KEY"]) {
    const value = env[name];

    // oxlint-disable-next-line node/no-process-env
    if (value && !process.env[name]) {
      // oxlint-disable-next-line node/no-process-env
      process.env[name] = value;
    }
  }
}

function resolveRoot(userConfig: UserConfig) {
  return userConfig.root
    ? path.resolve(process.cwd(), userConfig.root)
    : process.cwd();
}

function resolveEnvDir(userConfig: UserConfig) {
  const root = resolveRoot(userConfig);

  return userConfig.envDir === false ||
    userConfig.envDir === undefined ||
    userConfig.envDir === ""
    ? root
    : path.resolve(root, userConfig.envDir);
}

function forceSourcemapForUpload(
  userConfig: UserConfig,
  options: InterfereViteOptions,
  command: "build" | "serve",
  env: Record<string, string | undefined>,
) {
  const willUpload =
    command === "build" &&
    (userConfig.build?.lib === undefined ||
      userConfig.build.lib === false ||
      options.sourceMaps?.upload === true ||
      options.sourceMaps?.retain === true) &&
    (options.sourceMaps?.retain === true ||
      (options.sourceMaps?.upload !== false &&
        Boolean(pickEnvValue(env, "INTERFERE_API_KEY"))));

  if (!willUpload) {
    return;
  }

  const previous = userConfig.build?.sourcemap;

  userConfig.build = { ...userConfig.build, sourcemap: true };

  return (
    previous === undefined || previous === true ? undefined : previous
  ) satisfies SourcemapSetting | undefined;
}

function carryOverUploadProgress(
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

function warnForcedSourcemap(
  logger: Logger,
  overridden: SourcemapSetting | undefined,
) {
  if (overridden === undefined) {
    return;
  }

  logger.warn(
    `[interfere] Forced build.sourcemap to true (was ${JSON.stringify(overridden)}). Source maps are required to symbolicate errors; they are uploaded then deleted from the build output, so they are never served. Pass interfere({ sourceMaps: { upload: false } }) to opt out.`,
  );
}

function disabledPlugin() {
  const plugin: Plugin = {
    name: "interfere",
    config() {
      return {
        define: {
          "import.meta.env.VITE_INTERFERE_DISABLED": JSON.stringify("true"),
        },
        ssr: { noExternal: ["@interfere/vite"] },
      };
    },
  };

  return plugin;
}

export function interfere(options: InterfereViteOptions = {}) {
  rejectRemovedOptions(options);

  if (options.disable) {
    return disabledPlugin();
  }

  const receipt = options.buildReceipt
    ? createBuildReceipt(options.buildReceipt)
    : undefined;

  let remoteConfigFallback = false;
  let releaseConfig: Interfere.ReleasesConfigResponse | null = null;

  let registeredRelease: Awaited<ReturnType<typeof registerBuildRelease>> =
    null;

  const resolveBuildConfig = createBuildConfigResolver((result) => {
    receipt?.observeConfig(result);

    remoteConfigFallback ||= result.status === "fallback";
  });

  let state: PluginState | null = null;
  let overriddenSourcemap: SourcemapSetting | undefined;
  let announced = false;
  let uploadAfterAppBuild = false;

  const writeReceipt = async () => {
    try {
      await receipt?.write();
    } catch {
      state?.logger.warn(
        "[interfere] Private build receipt could not be written.",
      );
    }
  };

  const returnValue: Plugin = {
    buildApp: {
      order: "post",
      async handler(builder) {
        if (!state) {
          return;
        }

        uploadAfterAppBuild = true;

        const environments = Object.values(builder.environments);

        if (environments.every((environment) => !environment.isBuilt)) {
          for (const environment of environments) {
            // oxlint-disable-next-line eslint/no-await-in-loop
            await builder.build(environment);
          }
        }

        state.clientOutDirs = environments
          .filter((environment) => environment.config.consumer === "client")
          .map((environment) =>
            path.resolve(
              environment.config.root,
              environment.config.build.outDir,
            ),
          );

        if (isReleaseReady(state)) {
          state.releaseStarted = true;

          await releaseBuild(state);
        }

        await writeReceipt();
      },
    },
    buildStart() {
      if (state?.watch && state.releaseStarted) {
        state.releaseStarted = false;
        state.uploadComplete = false;
        state.buildStartedAt = Math.floor(Date.now() / 1000) * 1000;
      }

      if (state && !announced) {
        announceSetup(state);

        warnForcedSourcemap(state.logger, overriddenSourcemap);

        warnBuildPreflight(state.logger, remoteConfigFallback);

        announced = true;
      }
    },
    async closeBundle() {
      if (state && !uploadAfterAppBuild) {
        await releaseBuildWhenReady(state);
      }

      await writeReceipt();
    },
    async writeBundle(output, bundle) {
      if (output.dir) {
        await receipt?.recordOutput(output.dir, Object.keys(bundle));
      }
    },
    outputOptions(output) {
      if (state?.uploadSourceMaps || state?.retainSourceMaps) {
        return { ...output, sourcemapDebugIds: false };
      }

      return null;
    },
    generateBundle: {
      order: "post",
      handler(_output, bundle) {
        if (!(state?.uploadSourceMaps || state?.retainSourceMaps)) {
          return;
        }

        for (const chunk of Object.values(bundle)) {
          if (chunk.type === "chunk" && chunk.map) {
            chunk.code = renderDebugId(chunk.code);
          }
        }
      },
    },
    async config(userConfig, { command, mode }) {
      const env = loadEnv(mode, resolveEnvDir(userConfig), "");

      mirrorKeysForDevServer(command, env);

      const browser = resolveBrowserConfig(
        options,
        env,
        command,
        resolveRoot(userConfig),
      );

      // oxlint-disable-next-line node/no-process-env
      const releaseEnv = { ...env, ...process.env };

      const release = checkReleaseConfig(DIAGNOSTIC_CONTEXT, {
        apiKey: pickEnvValue(releaseEnv, "INTERFERE_API_KEY"),
        commitSha: browser.commitSha,
        releaseSlug: browser.releaseSlug,
      });

      const registersRelease =
        command === "build" &&
        options.sourceMaps?.retain !== true &&
        release.status === "ok";

      if (registersRelease) {
        releaseConfig = await preflightViteBinding({
          apiUrl: pickEnvValue(releaseEnv, "INTERFERE_API_URL") ?? API_URL,
          apiKey: release.apiKey,
          publicKey: browser.publicKey,
          serviceName: browser.serviceName,
        });
      }

      registeredRelease ??=
        registersRelease &&
        (releaseConfig || !browser.publicKey) &&
        browser.commitSha &&
        browser.releaseSlug
          ? await registerBuildRelease({
              apiUrl: pickEnvValue(releaseEnv, "INTERFERE_API_URL") ?? API_URL,
              apiKey: release.apiKey,
              publicKey: browser.publicKey,
              serviceName: browser.serviceName,
              commitSha: browser.commitSha,
              environment: browser.environment,
              releaseSlug: browser.releaseSlug,
              releasesConfig: releaseConfig,
            })
          : null;

      overriddenSourcemap = forceSourcemapForUpload(
        userConfig,
        options,
        command,
        env,
      );

      const credential = buildConfigCredential(
        browser.publicKey,
        registeredRelease?.token,
      );

      const baked =
        command === "build" && credential && !isTestEnv()
          ? await resolveBuildConfig(
              browser.apiHost ?? API_URL,
              credential,
              undefined,
              { serviceName: browser.serviceName },
            )
          : null;

      const define = omitUndefined({
        ...browserDefine(browser, registeredRelease?.token ?? null),
        ...remoteConfigDefine(baked),
      });

      return { define, ssr: { noExternal: ["@interfere/vite"] } };
    },
    async configResolved(config) {
      const browser = resolveBrowserConfig(
        options,
        config.env,
        config.command,
        config.root,
      );

      const next = buildPluginState(options, config, browser);

      carryOverUploadProgress(state, next);

      state = {
        ...next,
        receipt,
        releaseConfig: releaseConfig ?? undefined,
        registeredRelease: registeredRelease ?? undefined,
        releaseToken: Boolean(registeredRelease?.token),
      };

      uploadAfterAppBuild ||= config.builder !== undefined;

      if (config.command === "build") {
        await receipt?.configure(config, {
          commitSha: browser.commitSha,
          releaseSlug: browser.releaseSlug,
          environment: browser.environment,
          publicKey: browser.publicKey,
          apiOrigin: browser.apiHost ?? API_URL,
          producerVersion: PRODUCER_VERSION,
          serviceName: browser.serviceName,
        });
      }
    },
    enforce: "pre",
    name: "interfere",
    sharedDuringBuild: true,
  };

  return returnValue;
}
