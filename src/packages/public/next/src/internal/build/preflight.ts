import { BACKEND_HOSTS_GLOBAL } from "@interfere/types/sdk/remote-config/backend-hosts";
import { ANONYMOUS_REPLAY_GLOBAL } from "@interfere/types/sdk/remote-config/anonymous-replay";
import { createBuildConfigResolver } from "@interfere/build/config";
import {
  missingReleaseBinding,
  preflightReleaseBinding,
} from "@interfere/build/release-request";
import { PUBLIC_KEY_HEADER } from "@interfere/constants/api";
import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { InterfereClient } from "@interfere/javascript";
import { isInterfereApiKey } from "@interfere/types/sdk/build-diagnostics";
import { INGEST_URL_GLOBAL } from "@interfere/types/sdk/injected-globals";
import { LOG_SOURCING_GLOBAL } from "@interfere/types/sdk/log-sourcing";
import { PLUGINS_GLOBAL } from "@interfere/types/sdk/remote-config";
import { PLATFORM_GLOBAL } from "@interfere/types/integrations/platforms/hosting";
import { SERVER_TRACING_GLOBAL } from "@interfere/types/sdk/server-tracing";
import type { ResolvedBuildConfig } from "../../config.js";
import type { NextConfig } from "next";
import { log } from "../logger.js";
import { PRODUCER_VERSION } from "../version.js";

/** The surface configuration snapshot one production build resolves from `/v3/config`. */
export type SurfaceConfig = Awaited<
  ReturnType<ReturnType<typeof createBuildConfigResolver>>
>;

/** What the production preflight learned about the surface before the Next config is finalised. */
export interface BuildPreflight {
  readonly releaseConfig: ResolvedBuildConfig["releaseConfig"];
  /**
   * One snapshot of the App configuration, fetched by public key and service.
   * Every build-time decision (trace-meta gate, baked server globals) reads
   * this same object, so they cannot disagree. Absent for the dev server and
   * for builds without a public key.
   */
  readonly surfaceConfig: SurfaceConfig | undefined;
}

const WITHOUT_PREFLIGHT: BuildPreflight = {
  releaseConfig: undefined,
  surfaceConfig: undefined,
};

let fallbackLogged = false;

const resolveSurfaceConfig = createBuildConfigResolver((result) => {
  if (result.status !== "fallback" || fallbackLogged) {
    return;
  }

  fallbackLogged = true;

  log.warn("Using default SDK settings", [
    "Could not load SDK settings from Interfere. The build continues with the default settings.",
  ]);
});

/** The server-bundle globals a surface snapshot bakes, keyed like the loader renders them. */
export function surfaceGlobals(config: SurfaceConfig | undefined) {
  return omitUndefined({
    [SERVER_TRACING_GLOBAL]: config?.serverTracing,
    [LOG_SOURCING_GLOBAL]: config?.logSourcing,
    [PLATFORM_GLOBAL]: config?.platform,
    [INGEST_URL_GLOBAL]: config?.ingestUrl,
    [PLUGINS_GLOBAL]: config?.plugins,
    [BACKEND_HOSTS_GLOBAL]: config?.backendHosts,
    [ANONYMOUS_REPLAY_GLOBAL]: config?.anonymousReplayPolicy,
  });
}

// oxlint-disable-next-line typescript/promise-function-async -- Builds that cannot bind a release return null synchronously.
function releaseBinding(config: ResolvedBuildConfig) {
  if (
    !config.apiKey ||
    !isInterfereApiKey(config.apiKey) ||
    !config.publicKey ||
    !config.buildId ||
    !config.releaseSlug
  ) {
    return null;
  }

  return preflightReleaseBinding(
    new InterfereClient({
      baseUrl: config.apiUrl,
      apiKey: { token: config.apiKey },
      headers: {
        [PUBLIC_KEY_HEADER]: config.publicKey,
        "x-interfere-producer-version": PRODUCER_VERSION,
        "x-interfere-service": config.serviceName,
      },
      maxRetries: 0,
      timeoutInSeconds: 10,
    }),
  );
}

// oxlint-disable-next-line typescript/promise-function-async -- Ineligible builds do not start asynchronous preflight work.
function preflightBuild(config: ResolvedBuildConfig) {
  if (!config.publicKey) {
    return null;
  }

  return Promise.all([
    releaseBinding(config),
    resolveSurfaceConfig(config.apiUrl, config.publicKey, crypto.randomUUID(), {
      serviceName: config.serviceName,
    }),
  ]).then(([releaseConfig, surfaceConfig]) => {
    const surfaceSlug = releaseConfig?.surface.slug;

    if (
      config.serviceNameInferred &&
      config.serviceName &&
      surfaceSlug &&
      config.serviceName !== surfaceSlug
    ) {
      log.warn("Inferred service name differs from the App", [
        `The inferred service name ${JSON.stringify(config.serviceName)} differs from the key's App ${JSON.stringify(surfaceSlug)}. With workspace keys, changing the service name can select or create a different App.`,
        'To preserve your existing service name, set serviceName in withInterfere({ serviceName: "your-existing-service-name" }, nextConfig). Copy the name from the dashboard Services page.',
      ]);
    }

    const missing = releaseConfig && missingReleaseBinding(releaseConfig);

    if (missing) {
      log.warn(missing, []);
    }

    return {
      disabled: Boolean(missing),
      preflight: {
        releaseConfig: releaseConfig ?? undefined,
        surfaceConfig,
      } satisfies BuildPreflight,
    };
  });
}

export const withBuildPreflight: (
  enabled: boolean,
  config: ResolvedBuildConfig,
  configured: (preflight: BuildPreflight) => NextConfig,
  disabled: () => NextConfig,
  // oxlint-disable-next-line typescript/promise-function-async -- Preserve synchronous configuration when no production preflight is needed.
) => NextConfig | Promise<NextConfig> = (
  enabled,
  config,
  configured,
  disabled,
) => {
  const pending = enabled ? preflightBuild(config) : null;

  async function complete() {
    const binding = await pending;

    return binding?.disabled
      ? disabled()
      : configured(binding?.preflight ?? WITHOUT_PREFLIGHT);
  }

  return pending ? complete() : configured(WITHOUT_PREFLIGHT);
};
