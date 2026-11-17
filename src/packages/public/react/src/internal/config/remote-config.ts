import { InterfereClient } from "@interfere/javascript";
import { parseRemoteConfigInput } from "@interfere/types/sdk/remote-config/input";
import type { RemoteConfigInput } from "@interfere/types/sdk/remote-config/input";
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
  let config: RemoteConfigInput | null;

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

    config = parseRemoteConfigInput(await client.config.getSdkConfig());
  } catch {
    config = null;
  }

  if (!config) {
    log.warn("remote config fetch failed, using local defaults");
  }

  return config;
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
