import { preflightReleaseBinding } from "@interfere/build/release-request";
import { PUBLIC_KEY_HEADER } from "@interfere/constants/api";
import type { Interfere } from "@interfere/javascript";
import type { ReleaseDiagnosticContext } from "@interfere/types/sdk/build-diagnostics";
import { InterfereClient } from "@interfere/javascript";
import type { Logger } from "vite";
import { PRODUCER_VERSION } from "../../version";

export function warnBuildPreflight(
  logger: Logger,
  serviceNameWarning: string | undefined,
  fallback: boolean,
) {
  if (serviceNameWarning) {
    logger.warn(serviceNameWarning);
  }

  if (fallback) {
    logger.warn(
      "[interfere] Could not load SDK settings from Interfere. The build uses the default settings.",
    );
  }
}

export const DIAGNOSTIC_CONTEXT: ReleaseDiagnosticContext = {
  publicKeyEnvVar: "VITE_INTERFERE_PUBLIC_KEY",
  docsUrl: "https://interfere.com/docs/sdk/vite",
};

export function inferredServiceNameWarning(
  config: Interfere.ReleasesConfigResponse | null,
  serviceName: string | undefined,
  explicitName: string | undefined,
) {
  const surfaceSlug = config?.surface.slug;

  if (
    explicitName ||
    !serviceName ||
    !surfaceSlug ||
    serviceName === surfaceSlug
  ) {
    return;
  }

  return `[interfere] The inferred service name ${JSON.stringify(serviceName)} differs from the key's App ${JSON.stringify(surfaceSlug)}. With workspace keys, changing the service name can select or create a different App. To preserve your existing service name, set serviceName in interfere({ serviceName: "your-existing-service-name" }), browser init(), and server register(). Copy the name from the dashboard Services page.`;
}

export async function preflightViteBinding(input: {
  apiUrl: string;
  apiKey: string;
  publicKey: string | undefined;
  serviceName?: string | undefined;
}) {
  const client = new InterfereClient({
    baseUrl: input.apiUrl,
    apiKey: { token: input.apiKey },
    headers: {
      [PUBLIC_KEY_HEADER]: input.publicKey,
      "x-interfere-producer-version": PRODUCER_VERSION,
      "x-interfere-service": input.serviceName,
    },
    maxRetries: 0,
    timeoutInSeconds: 10,
  });

  return (await preflightReleaseBinding(
    client,
  )) satisfies Interfere.ReleasesConfigResponse | null;
}
