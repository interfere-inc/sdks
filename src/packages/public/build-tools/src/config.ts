import { InterfereClient } from "@interfere/javascript";
import { buildConfigSchema } from "@interfere/types/sdk/build-config";
import { remoteConfigInputSchema } from "@interfere/types/sdk/remote-config";

export type BuildConfigCredential =
  | { readonly publicKey: string }
  | { readonly releaseToken: string };

export function buildConfigCredential(
  publicKey: string | null | undefined,
  releaseToken: string | null | undefined,
) {
  if (publicKey) {
    return { publicKey } satisfies BuildConfigCredential;
  }

  return releaseToken
    ? ({ releaseToken } satisfies BuildConfigCredential)
    : null;
}

export function createBuildConfigResolver(
  observe?: (result: Awaited<ReturnType<typeof fetchBuildConfig>>) => void,
) {
  const pending = new Map<string, ReturnType<typeof fetchBuildConfig>>();

  return async (
    apiUrl: string,
    credential: BuildConfigCredential,
    buildScope = "",
    service?: {
      serviceName?: string | undefined;
      serviceNamespace?: string | undefined;
    },
  ) => {
    const key = JSON.stringify([
      buildScope,
      apiUrl,
      "publicKey" in credential ? "publicKey" : "releaseToken",
      "publicKey" in credential
        ? credential.publicKey
        : credential.releaseToken,
      service?.serviceName ?? "",
      service?.serviceNamespace ?? "",
    ]);

    let config = pending.get(key);

    if (!config) {
      config = fetchBuildConfig(apiUrl, credential, service);

      pending.set(key, config);
    }

    const result = await config;

    observe?.(result);

    return result.config;
  };
}

async function fetchBuildConfig(
  apiUrl: string,
  credential: BuildConfigCredential,
  service?: {
    serviceName?: string | undefined;
    serviceNamespace?: string | undefined;
  },
) {
  try {
    const client = new InterfereClient({
      baseUrl: apiUrl,
      ...("publicKey" in credential
        ? { publicKeyHeader: { apiKey: credential.publicKey } }
        : { releaseTokenHeader: { apiKey: credential.releaseToken } }),
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
