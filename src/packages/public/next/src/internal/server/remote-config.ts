import { InterfereClient } from "@interfere/javascript";
import { remoteConfigInputSchema } from "@interfere/types/sdk/remote-config";
import type { RemotePluginConfig } from "@interfere/types/sdk/remote-config";
import { readBakedPlugins } from "@interfere/types/sdk/remote-config/plugins";

import { isEnabledOnServer, readInterfereEnv } from "../env.js";

let cachedConfig: RemotePluginConfig | null = null;

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

  if (env.publicKey === null) {
    return;
  }

  try {
    const client = new InterfereClient({
      baseUrl: env.apiUrl,
      publicKeyHeader: { apiKey: env.publicKey },
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
