import { InterfereClient } from "@interfere/javascript";
import { buildConfigSchema } from "@interfere/types/sdk/build-config";
import { remoteConfigInputSchema } from "@interfere/types/sdk/remote-config";

export function createBuildConfigResolver(
  observe?: (result: Awaited<ReturnType<typeof fetchBuildConfig>>) => void,
) {
  const pending = new Map<string, ReturnType<typeof fetchBuildConfig>>();

  return async (
    apiUrl: string,
    publicKey: string,
    buildScope = "",
    service?: {
      serviceName?: string | undefined;
      serviceNamespace?: string | undefined;
    },
  ) => {
    const key = JSON.stringify([
      buildScope,
      apiUrl,
      publicKey,
      service?.serviceName ?? "",
      service?.serviceNamespace ?? "",
    ]);

    let config = pending.get(key);

    if (!config) {
      config = fetchBuildConfig(apiUrl, publicKey, service);

      pending.set(key, config);
    }

    const result = await config;

    observe?.(result);

    return result.config;
  };
}

async function fetchBuildConfig(
  apiUrl: string,
  publicKey: string,
  service?: {
    serviceName?: string | undefined;
    serviceNamespace?: string | undefined;
  },
) {
  try {
    const client = new InterfereClient({
      baseUrl: apiUrl,
      publicKeyHeader: { apiKey: publicKey },
      headers: {
        "x-interfere-service": service?.serviceName || undefined,
        "x-interfere-service-namespace": service?.serviceName
          ? service.serviceNamespace || undefined
          : undefined,
      },
      timeoutInSeconds: 3,
      maxRetries: 0,
    });

    const response = await client.config.getSdkConfig();

    const config = buildConfigSchema.parse(response);

    return {
      config: { ...config, platform: config.platform ?? null },
      status: remoteConfigInputSchema.safeParse(response).success
        ? ("fetched" as const)
        : ("fallback" as const),
    };
  } catch {
    return {
      config: buildConfigSchema.parse({}),
      status: "fallback" as const,
    };
  }
}
