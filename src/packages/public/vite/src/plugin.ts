import {
  registerBuildRelease,
  registerLocalDevRelease,
} from "./internal/build/register-release";
import {
  DIAGNOSTIC_CONTEXT,
  preflightViteBinding,
  warnBuildPreflight,
} from "./internal/build/preflight";
import type { Interfere } from "@interfere/javascript";
import {
  buildConfigCredential,
  createBuildConfigResolver,
} from "@interfere/build/config";
import { API_URL } from "@interfere/constants/api";
import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { LOCAL_DEV_RELEASE_SLUG } from "@interfere/types/releases/slug";
import { checkReleaseConfig } from "@interfere/types/sdk/build-diagnostics";

import { loadEnv } from "vite";
import type { Plugin } from "vite";

import { rejectRemovedOptions } from "./internal/plugin-options";
import type { InterfereViteOptions } from "./internal/plugin-options";
import {
  isTestEnv,
  mirrorKeysForDevServer,
  pickEnvValue,
  resolveEnvDir,
  resolveRoot,
} from "./internal/plugin-env";
import { createBuildReceipt } from "./internal/build/build-receipt";
import {
  browserDefine,
  resolveBrowserConfig,
} from "./internal/build/browser-define";
import {
  CLOUDFLARE_PLUGIN,
  environmentOutDir,
  hasCloudflarePlugin,
} from "./internal/build/cloudflare";
import {
  forceSourcemapForUpload,
  warnForcedSourcemap,
  willUploadSourceMaps,
} from "./internal/build/force-sourcemap";
import type { SourcemapSetting } from "./internal/build/force-sourcemap";
import {
  MILLIS_PER_SECOND,
  announceSetup,
  buildPluginState,
  carryOverUploadProgress,
} from "./internal/build/plugin-state";
import {
  isReleaseReady,
  releaseBuild,
  releaseBuildWhenReady,
} from "./internal/build/release-build";
import type { PluginState } from "./internal/build/release-build";
import { remoteConfigDefine } from "./internal/build/remote-config-define";
import { renderDebugId } from "./internal/build/render-debug-id";
import { PRODUCER_VERSION } from "./version";

export type { InterfereViteOptions } from "./internal/plugin-options";

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

  let localDevReleaseFailed = false;

  const resolveBuildConfig = createBuildConfigResolver((result) => {
    receipt?.observeConfig(result);

    remoteConfigFallback ||= result.status === "fallback";
  });

  let state: PluginState | null = null;
  let overriddenSourcemap: SourcemapSetting | undefined;
  let forcesWorkerSourcemaps = false;
  const overriddenWorkerSourcemaps = new Map<string, SourcemapSetting>();
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
          .map((environment) => environmentOutDir(environment));

        state.environmentOutDirs = builder.config.plugins.some(
          (plugin) => plugin.name === CLOUDFLARE_PLUGIN,
        )
          ? [
              ...new Set(
                environments
                  .filter((environment) => environment.isBuilt)
                  .map((environment) => environmentOutDir(environment)),
              ),
            ]
          : [];

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

        state.buildStartedAt =
          Math.floor(Date.now() / MILLIS_PER_SECOND) * 1000;
      }

      if (state && !announced) {
        announceSetup(state);

        warnForcedSourcemap(state.logger, overriddenSourcemap);

        for (const [name, overridden] of overriddenWorkerSourcemaps) {
          warnForcedSourcemap(
            state.logger,
            overridden,
            `environments.${name}.build.sourcemap`,
          );
        }

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
    async config(userConfig, { command, isPreview, mode }) {
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

      const apiKey = pickEnvValue(releaseEnv, "INTERFERE_API_KEY");

      const apiUrl = pickEnvValue(releaseEnv, "INTERFERE_API_URL") ?? API_URL;

      const release = checkReleaseConfig(DIAGNOSTIC_CONTEXT, {
        apiKey,
        commitSha: browser.commitSha,
        releaseSlug: browser.releaseSlug,
      });

      const registersRelease =
        command === "build" &&
        options.sourceMaps?.retain !== true &&
        release.status === "ok";

      if (registersRelease) {
        releaseConfig = await preflightViteBinding({
          apiUrl,
          apiKey: release.apiKey,
          serviceName: browser.serviceName,
        });
      }

      const mintsLocalDevRelease =
        command === "serve" &&
        !isPreview &&
        apiKey !== undefined &&
        !isTestEnv();

      registeredRelease ??= mintsLocalDevRelease
        ? await registerLocalDevRelease({
            apiUrl,
            apiKey,
            serviceName: browser.serviceName,
          })
        : null;

      if (mintsLocalDevRelease) {
        localDevReleaseFailed = !registeredRelease?.token;
      }

      if (command === "serve" && registeredRelease?.token) {
        browser.releaseSlug = LOCAL_DEV_RELEASE_SLUG;
      }

      registeredRelease ??=
        registersRelease && browser.commitSha && browser.releaseSlug
          ? await registerBuildRelease({
              apiUrl,
              apiKey: release.apiKey,
              serviceName: browser.serviceName,
              commitSha: browser.commitSha,
              environment: browser.environment,
              releaseSlug: browser.releaseSlug,
              releasesConfig: releaseConfig,
            })
          : null;

      const forcesSourcemaps = willUploadSourceMaps(
        userConfig,
        options,
        command,
        env,
      );

      overriddenSourcemap = forcesSourcemaps
        ? forceSourcemapForUpload(userConfig)
        : undefined;

      forcesWorkerSourcemaps =
        forcesSourcemaps && hasCloudflarePlugin(userConfig.plugins);

      const credential = buildConfigCredential(registeredRelease?.token);

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
        ...browserDefine(browser, registeredRelease?.token ?? null, command),
        ...remoteConfigDefine(baked),
      });

      return { define, ssr: { noExternal: ["@interfere/vite"] } };
    },
    configEnvironment(name, environment) {
      const consumer =
        environment.consumer ?? (name === "client" ? "client" : "server");

      const previous = environment.build?.sourcemap;

      if (
        !forcesWorkerSourcemaps ||
        consumer !== "server" ||
        previous === undefined ||
        previous === true
      ) {
        return null;
      }

      overriddenWorkerSourcemaps.set(name, previous);

      return { build: { sourcemap: true } };
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

      if (localDevReleaseFailed) {
        localDevReleaseFailed = false;

        config.logger.warn(
          "[interfere] The dev server could not mint a local development release token with INTERFERE_API_KEY, so the browser sends no events. Check that Interfere is reachable, that the key has the release:write scope and that a service name resolves.",
        );
      }

      if (config.command === "build") {
        await receipt?.configure(config, {
          commitSha: browser.commitSha,
          releaseSlug: browser.releaseSlug,
          environment: browser.environment,
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
