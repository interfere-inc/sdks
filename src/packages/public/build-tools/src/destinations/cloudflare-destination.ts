import type { ReleaseDestinationMetadata } from "@interfere/types/integrations";
import { parseEnvValue } from "@interfere/types/sdk/env";

function parseVersionMetadata() {
  // oxlint-disable-next-line node/no-process-env
  const raw = parseEnvValue(process.env["CF_VERSION_METADATA"]);

  if (!raw) {
    return null;
  }

  try {
    const parsed: unknown = JSON.parse(raw);

    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Provider version metadata may contain valid primitive JSON instead of an object.
    if (parsed === null || typeof parsed !== "object") {
      return null;
    }

    return {
      id:
        // oxlint-disable-next-line anti-slop/no-runtime-typeof -- A malformed provider id must not discard an independently valid tag or become a release identifier.
        "id" in parsed && typeof parsed.id === "string" ? parsed.id : undefined,
      tag:
        // oxlint-disable-next-line anti-slop/no-runtime-typeof -- A malformed provider tag must not discard an independently valid id or become an environment label.
        "tag" in parsed && typeof parsed.tag === "string"
          ? parsed.tag
          : undefined,
    };
  } catch {
    return null;
  }
}

function resolveDeploymentUrl() {
  const candidate =
    // oxlint-disable-next-line node/no-process-env
    parseEnvValue(process.env["CF_PAGES_URL"]) ??
    // oxlint-disable-next-line node/no-process-env
    parseEnvValue(process.env["WORKERS_CI_DEPLOY_URL"]);

  if (candidate === null) {
    return null;
  }

  if (candidate.startsWith("https://") || candidate.startsWith("http://")) {
    return candidate;
  }

  return `https://${candidate}`;
}

export function resolve() {
  const versionMetadata = parseVersionMetadata();

  const versionId =
    // oxlint-disable-next-line node/no-process-env
    parseEnvValue(process.env["CF_VERSION_ID"]) ??
    versionMetadata?.id ??
    // oxlint-disable-next-line node/no-process-env
    parseEnvValue(process.env["WORKERS_CI_BUILD_UUID"]) ??
    null;

  const scriptName =
    // oxlint-disable-next-line node/no-process-env
    parseEnvValue(process.env["CF_WORKER_NAME"]) ??
    // oxlint-disable-next-line node/no-process-env
    parseEnvValue(process.env["WORKERS_CI_SCRIPT_NAME"]) ??
    null;

  const accountId =
    // oxlint-disable-next-line node/no-process-env
    parseEnvValue(process.env["CF_ACCOUNT_ID"]) ??
    // oxlint-disable-next-line node/no-process-env
    parseEnvValue(process.env["CLOUDFLARE_ACCOUNT_ID"]) ??
    null;

  const environment =
    // oxlint-disable-next-line node/no-process-env
    parseEnvValue(process.env["WORKERS_CI_BRANCH"]) ??
    versionMetadata?.tag ??
    null;

  return {
    accountId,
    deploymentUrl: resolveDeploymentUrl(),
    destinationReleaseId: versionId,
    environment,
    environmentName: environment,
    environmentTarget: environment,
    provider: "cloudflare",
    scriptName,
    versionId,
  } satisfies ReleaseDestinationMetadata;
}
