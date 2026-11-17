import { InterfereClient } from "@interfere/javascript";
import { remoteConfigInputSchema } from "@interfere/types/sdk/remote-config";
import type { RemotePluginConfig } from "@interfere/types/sdk/remote-config";
import { readBakedPlugins } from "@interfere/types/sdk/remote-config/plugins";

import { isEnabledOnServer, readInterfereServerEnv } from "./env";

let cachedConfig: RemotePluginConfig | null = null;

function serverCredential(
  publicKey: string | null,
  releaseToken: string | null,
) {
  if (publicKey) {
    return { publicKeyHeader: { apiKey: publicKey } };
  }

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

  const env = readInterfereServerEnv();

  const credential = serverCredential(env.publicKey, env.releaseToken);

  if (credential === null) {
    return;
  }

  try {
    const client = new InterfereClient({
      baseUrl: env.apiUrl,
      ...credential,
      headers: {
        "x-interfere-service":
          service?.serviceName ||
          import.meta.env.VITE_INTERFERE_SERVICE_NAME ||
          undefined,
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
