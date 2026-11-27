import { preflightReleaseBinding } from "@interfere/build/release-request";
import type { Interfere } from "@interfere/javascript";
import type { ReleaseDiagnosticContext } from "@interfere/types/sdk/build-diagnostics";
import { InterfereClient } from "@interfere/javascript";
import type { Logger } from "vite";
import { PRODUCER_VERSION } from "../../version";

export function warnBuildPreflight(logger: Logger, fallback: boolean) {
  if (fallback) {
    logger.warn(
      "[interfere] Could not load SDK settings from Interfere. The build uses the default settings.",
    );
  }
}

export const DIAGNOSTIC_CONTEXT: ReleaseDiagnosticContext = {
  docsUrl: "https://interfere.com/docs/sdk/vite",
};

export async function preflightViteBinding(input: {
  apiUrl: string;
  apiKey: string;
  serviceName?: string | undefined;
}) {
  const client = new InterfereClient({
    baseUrl: input.apiUrl,
    apiKey: { token: input.apiKey },
    headers: {
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
