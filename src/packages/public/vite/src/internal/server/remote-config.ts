import { InterfereClient } from "@interfere/javascript";
import { containAsync } from "@interfere/server-core/contain";
import { onWorkerd } from "@interfere/server-core/runtime";
import { remoteConfigInputSchema } from "@interfere/types/sdk/remote-config";
import type { RemotePluginConfig } from "@interfere/types/sdk/remote-config";
import { readBakedPlugins } from "@interfere/types/sdk/remote-config/plugins";

import { isEnabledOnServer, readInterfereServerEnv } from "./env";

const REMOTE_CONFIG_TIMEOUT_SECONDS = 10;

const STRANDED_AFTER_MS = (REMOTE_CONFIG_TIMEOUT_SECONDS + 5) * 1000;

let cachedConfig: RemotePluginConfig | null = null;

let settled = false;

let attempt: { done: Promise<void>; strandedAt: number } | null = null;

async function fetchAndCacheRemoteConfig(service?: {
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

  if (env.releaseToken === null) {
    return;
  }

  try {
    const client = new InterfereClient({
      baseUrl: env.apiUrl,
      releaseTokenHeader: { apiKey: env.releaseToken },
      headers: {
        "x-interfere-service":
          service?.serviceName ||
          import.meta.env.VITE_INTERFERE_SERVICE_NAME ||
          undefined,
        "x-interfere-service-namespace": service?.serviceNamespace || undefined,
      },
      timeoutInSeconds: REMOTE_CONFIG_TIMEOUT_SECONDS,
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

/** Fetches remote plugin config once per isolate; on workerd a request never awaits a fetch another request owns. */
export async function ensureRemoteConfig(service?: {
  serviceName?: string | undefined;
  serviceNamespace?: string | undefined;
}) {
  if (settled) {
    return;
  }

  if (attempt) {
    if (!onWorkerd()) {
      return attempt.done;
    }

    if (Date.now() < attempt.strandedAt) {
      return;
    }
  }

  const done = (async () => {
    await containAsync("remote config", async () =>
      fetchAndCacheRemoteConfig(service),
    );

    settled = true;
  })();

  attempt = { done, strandedAt: Date.now() + STRANDED_AFTER_MS };

  return done;
}

export function isPluginEnabled(plugin: string) {
  const config: Readonly<Partial<Record<string, boolean>>> | null =
    readBakedPlugins() ?? cachedConfig;

  if (!config) {
    return true;
  }

  return config[plugin] !== false;
}
