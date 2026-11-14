import type { ReleaseDestinationMetadata } from "../integrations/index.ts";
import type { ReleaseSlug } from "../releases/slug.ts";
import { parseEnvValue } from "./env.ts";

export function resolveCliReleaseDestination(
  env: Record<string, string | undefined>,
  releaseSlug: ReleaseSlug,
) {
  return {
    deploymentUrl: parseEnvValue(env["INTERFERE_DEPLOYMENT_URL"]),
    destinationReleaseId: releaseSlug,
    environment: parseEnvValue(env["INTERFERE_ENVIRONMENT"]),
    provider: "cli",
  } satisfies ReleaseDestinationMetadata;
}
