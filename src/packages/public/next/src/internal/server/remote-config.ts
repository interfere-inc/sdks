import { InterfereClient } from "@interfere/javascript";
import { remoteConfigInputSchema } from "@interfere/types/sdk/remote-config";
import type { RemotePluginConfig } from "@interfere/types/sdk/remote-config";
import { getStringGlobal } from "@interfere/react/util/global";
import { readBakedPlugins } from "@interfere/types/sdk/remote-config/plugins";
import { RELEASE_TOKEN_GLOBAL } from "@interfere/types/sdk/release/token";

import { isEnabledOnServer, readInterfereEnv } from "../env.js";

let cachedConfig: RemotePluginConfig | null = null;

function bakedReleaseToken() {
  return getStringGlobal(RELEASE_TOKEN_GLOBAL) || null;
}

function serverCredential(publicKey: string | null) {
  if (publicKey) {
    return { publicKeyHeader: { apiKey: publicKey } };
  }

  const releaseToken = bakedReleaseToken();

  return releaseToken ? { releaseTokenHeader: { apiKey: releaseToken } } : null;
}

export async function fetchAndCacheRemoteConfig(service?: {
  serviceName?: string | undefined;
  serviceNamespace?: string | undefined;
}) {
  if (readBakedPlugins()) {
    return;
  }

  if (!isEnabledOnServer()) {
    return;
  }

  const env = readInterfereEnv();

  const credential = serverCredential(env.publicKey);

  if (credential === null) {
    return;
  }

  try {
    const client = new InterfereClient({
      baseUrl: env.apiUrl,
      ...credential,
      headers: {
        "x-interfere-service":
          service?.serviceName || env.serviceName || undefined,
        "x-interfere-service-namespace": service?.serviceNamespace || undefined,
      },
      timeoutInSeconds: 10,
      maxRetries: 0,
    });

    const config = remoteConfigInputSchema.parse(
      await client.config.getSdkConfig(),
    );

    if (config.plugins) {
      cachedConfig = config.plugins;
    }
  } catch {
    return;
  }
}

export function isPluginEnabled(plugin: string) {
  const config: Readonly<Partial<Record<string, boolean>>> | null =
    readBakedPlugins() ?? cachedConfig;

  if (!config) {
    return true;
  }

  return config[plugin] !== false;
}
