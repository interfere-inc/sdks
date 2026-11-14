import type { ReleaseDestinationMetadata } from "@interfere/types/integrations";
import { parseEnvValue } from "@interfere/types/sdk/env";

export function resolve() {
  const environment = parseEnvValue(
    // oxlint-disable-next-line node/no-process-env
    process.env["VERCEL_ENV"] ?? process.env["VERCEL_TARGET_ENV"],
  );

  // oxlint-disable-next-line node/no-process-env
  const deploymentId = parseEnvValue(process.env["VERCEL_DEPLOYMENT_ID"]);

  return {
    deploymentId,
    deploymentUrl: resolveDeploymentUrl(),
    destinationReleaseId: deploymentId,
    environment,
    environmentName: environment,
    environmentTarget: environment,
    provider: "vercel",
  } satisfies ReleaseDestinationMetadata;
}

function resolveDeploymentUrl() {
  // oxlint-disable-next-line node/no-process-env
  const deploymentUrl = parseEnvValue(process.env["VERCEL_URL"]);

  if (deploymentUrl === null) {
    return null;
  }

  if (
    deploymentUrl.startsWith("https://") ||
    deploymentUrl.startsWith("http://")
  ) {
    return deploymentUrl;
  }

  return `https://${deploymentUrl}`;
}
