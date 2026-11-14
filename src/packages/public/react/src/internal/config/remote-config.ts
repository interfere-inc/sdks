import { InterfereClient } from "@interfere/javascript";
import { remoteConfigInputSchema } from "@interfere/types/sdk/remote-config";
import type { RemotePluginConfig } from "@interfere/types/sdk/remote-config";
import { readBakedBackendHosts } from "@interfere/types/sdk/remote-config/backend-hosts";
import { readBakedAnonymousReplayPolicy } from "@interfere/types/sdk/remote-config/anonymous-replay";

import { createLogger } from "../../util/log.js";
import { resolveTargets, setIngestHost } from "./config.js";

const log = createLogger("remote-config");

async function fetchRemoteConfig(
  configTarget: InterfereClient.Options,
  fetcher: typeof globalThis.fetch,
) {
  try {
    const client = new InterfereClient({
      ...configTarget,
      fetch: Object.assign(
        async (
          url: Parameters<typeof fetch>[0],
          init?: Parameters<typeof fetch>[1],
        ) => fetcher(url, { ...init, credentials: "omit" }),
        fetcher,
      ),
    });

    return remoteConfigInputSchema.parse(await client.config.getSdkConfig());
  } catch {
    log.warn("remote config fetch failed, using local defaults");

    return null;
  }
}

export async function resolveRemoteConfig(
  apiHost: string | undefined,
  fetcher: typeof globalThis.fetch,
  baked: RemotePluginConfig | null,
  service?: Parameters<typeof resolveTargets>[2],
) {
  if (baked) {
    return {
      anonymousReplayPolicy: readBakedAnonymousReplayPolicy(),
      plugins: baked,
      backendHosts: readBakedBackendHosts(),
      protocolVersion: undefined,
    };
  }

  const remoteConfig = await fetchRemoteConfig(
    resolveTargets(apiHost, undefined, service).config,
    fetcher,
  );

  if (remoteConfig?.ingestUrl) {
    setIngestHost(remoteConfig.ingestUrl);
  }

  return {
    anonymousReplayPolicy:
      remoteConfig?.anonymousReplayPolicy ?? readBakedAnonymousReplayPolicy(),
    plugins: remoteConfig?.plugins ?? null,
    backendHosts: remoteConfig?.backendHosts,
    protocolVersion: remoteConfig?.protocolVersion,
  };
}
