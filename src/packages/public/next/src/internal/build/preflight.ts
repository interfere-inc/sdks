import { BACKEND_HOSTS_GLOBAL } from "@interfere/types/sdk/remote-config/backend-hosts";
import { ANONYMOUS_REPLAY_GLOBAL } from "@interfere/types/sdk/remote-config/anonymous-replay";
import { createBuildConfigResolver } from "@interfere/build/config";
import {
  preflightReleaseBinding,
  resolveReleaseRequest,
} from "@interfere/build/release-request";
import { PUBLIC_KEY_HEADER } from "@interfere/constants/api";
import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { InterfereClient } from "@interfere/javascript";
import type { Interfere } from "@interfere/javascript";
import { createReleaseResponse } from "@interfere/types/releases/definition";
import { isInterfereApiKey } from "@interfere/types/sdk/build-diagnostics";
import { INGEST_URL_GLOBAL } from "@interfere/types/sdk/injected-globals";
import { LOG_SOURCING_GLOBAL } from "@interfere/types/sdk/log-sourcing";
import { PLUGINS_GLOBAL } from "@interfere/types/sdk/remote-config/plugins";
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

interface BuildPreflight {
  readonly releaseConfig: ResolvedBuildConfig["releaseConfig"];
  readonly surfaceConfig: SurfaceConfig | undefined;
  readonly registeredRelease: ResolvedBuildConfig["registeredRelease"];
}

const WITHOUT_PREFLIGHT: BuildPreflight = {
  releaseConfig: undefined,
  registeredRelease: undefined,
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

function releaseClient(config: ResolvedBuildConfig) {
  if (
    !config.apiKey ||
    !isInterfereApiKey(config.apiKey) ||
    !config.buildId ||
    !config.releaseSlug
  ) {
    return null;
  }

  return new InterfereClient({
    baseUrl: config.apiUrl,
    apiKey: { token: config.apiKey },
    headers: {
      [PUBLIC_KEY_HEADER]: config.publicKey ?? undefined,
      "x-interfere-producer-version": PRODUCER_VERSION,
      "x-interfere-service": config.serviceName,
    },
    maxRetries: 0,
    timeoutInSeconds: 10,
  });
}

async function registerRelease(
  client: InterfereClient,
  config: ResolvedBuildConfig,
  releaseConfig: Interfere.ReleasesConfigResponse | null,
) {
  if (!config.buildId || !config.releaseSlug) {
    return;
  }

  const request = resolveReleaseRequest({
    buildId: config.buildId,
    config: releaseConfig,
    environment: config.environment,
    producerVersion: PRODUCER_VERSION,
    releaseSlug: config.releaseSlug,
    service: config.serviceName ? { name: config.serviceName } : undefined,
  });

  try {
    return createReleaseResponse.parse(
      await client.releases.createRelease(request, {
        abortSignal: AbortSignal.timeout(60_000),
      }),
    );
  } catch {
    return;
  }
}

// oxlint-disable-next-line typescript/promise-function-async -- Ineligible builds do not start asynchronous preflight work.
function preflightBuild(config: ResolvedBuildConfig) {
  const client = releaseClient(config);

  if (!client && !config.publicKey) {
    return null;
  }

  return Promise.all([
    client && preflightReleaseBinding(client),
    config.publicKey
      ? resolveSurfaceConfig(
          config.apiUrl,
          { publicKey: config.publicKey },
          crypto.randomUUID(),
          { serviceName: config.serviceName },
        )
      : undefined,
  ]).then(async ([releaseConfig, publicKeyConfig]) => {
    const surfaceSlug = releaseConfig?.surface?.slug;

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

    const registeredRelease =
      client && (releaseConfig || !config.publicKey)
        ? await registerRelease(client, config, releaseConfig ?? null)
        : undefined;

    return {
      releaseConfig: releaseConfig ?? undefined,
      registeredRelease,
      surfaceConfig:
        publicKeyConfig ??
        (registeredRelease?.token
          ? await resolveSurfaceConfig(
              config.apiUrl,
              { releaseToken: registeredRelease.token },
              crypto.randomUUID(),
              { serviceName: config.serviceName },
            )
          : undefined),
    } satisfies BuildPreflight;
  });
}

/**
 * Next evaluates the production config more than once per build, through the
 * same `withInterfere` result. One preflight per resolved build config keeps
 * that to one release registration and one set of warnings.
 */
export function createBuildPreflight() {
  const preflights = new Map<string, ReturnType<typeof preflightBuild>>();

  const withBuildPreflight: (
    enabled: boolean,
    config: ResolvedBuildConfig,
    configured: (preflight: BuildPreflight) => NextConfig,
    // oxlint-disable-next-line typescript/promise-function-async -- Preserve synchronous configuration when no production preflight is needed.
  ) => NextConfig | Promise<NextConfig> = (enabled, config, configured) => {
    const key = JSON.stringify(config);

    if (enabled && !preflights.has(key)) {
      preflights.set(key, preflightBuild(config));
    }

    const pending = enabled ? (preflights.get(key) ?? null) : null;

    async function complete() {
      return configured((await pending) ?? WITHOUT_PREFLIGHT);
    }

    return pending ? complete() : configured(WITHOUT_PREFLIGHT);
  };

  return withBuildPreflight;
}
