import { z } from "zod";
import type { Interfere, InterfereClient } from "@interfere/javascript";
import { releaseDestinationProviderSchema } from "@interfere/types/integrations";
import type { ReleaseDestinationProvider } from "@interfere/types/integrations";
import type { CreateReleaseRequest } from "@interfere/types/releases/definition";
import type { ReleaseSlug } from "@interfere/types/releases/slug";
import { resolveCliReleaseDestination } from "@interfere/types/sdk/cli-destination";
import { applyDeployEnvironment } from "@interfere/types/sdk/deploy-environment";
import { parseEnvValue } from "@interfere/types/sdk/env";
import { resolveGithubReleaseSource } from "@interfere/types/sdk/github-source";

import { execSync } from "node:child_process";

import { resolve as resolveCloudflare } from "./destinations/cloudflare-destination.ts";
import { resolve as resolveVercel } from "./destinations/vercel-destination.ts";

function git(command: string) {
  try {
    return (
      execSync(command, {
        encoding: "utf8",
        stdio: ["ignore", "pipe", "ignore"],
      }).trim() || null
    );
  } catch {
    return null;
  }
}

function buildEnvironmentProvider() {
  // oxlint-disable-next-line node/no-process-env
  if (parseEnvValue(process.env["VERCEL"])) {
    return "vercel" satisfies ReleaseDestinationProvider;
  }

  if (
    // oxlint-disable-next-line node/no-process-env
    parseEnvValue(process.env["WORKERS_CI"]) ??
    // oxlint-disable-next-line node/no-process-env
    parseEnvValue(process.env["CF_PAGES"])
  ) {
    return "cloudflare" satisfies ReleaseDestinationProvider;
  }

  return "cli" satisfies ReleaseDestinationProvider;
}

function destinationProvider(config: Interfere.ReleasesConfigResponse | null) {
  const configured = releaseDestinationProviderSchema.safeParse(
    config?.surface?.destinationProvider,
  );

  return configured.success ? configured.data : buildEnvironmentProvider();
}

/** Builds the release request for any build. The deploy target comes from the App when it names one, otherwise from the build environment; a missing repository link only leaves commit metadata to the build environment. */
export function resolveReleaseRequest(input: {
  buildId: string;
  environment: string | null;
  releaseSlug: ReleaseSlug;
  producerVersion: string;
  config: Interfere.ReleasesConfigResponse | null;
  service?: CreateReleaseRequest["service"];
}) {
  const destinations = {
    // oxlint-disable-next-line node/no-process-env
    cli: () => resolveCliReleaseDestination(process.env, input.releaseSlug),
    cloudflare: resolveCloudflare,
    vercel: resolveVercel,
  };

  const request: CreateReleaseRequest = {
    buildId: input.buildId,
    destination: applyDeployEnvironment(
      destinations[destinationProvider(input.config)](),
      input.environment,
    ),
    producerVersion: input.producerVersion,
    service: input.service,
    slug: input.releaseSlug,
    // oxlint-disable-next-line node/no-process-env
    source: resolveGithubReleaseSource(process.env, git),
  };

  return request;
}

const releaseBindingSchema = z.object({
  org: z.object({ id: z.string(), slug: z.string().min(1) }),
  surface: z.object({
    id: z.string(),
    slug: z.string().min(1),
    sourceProvider: z.string().nullable(),
    destinationProvider: z.string().nullable(),
  }),
});

/** Fetch release bindings before compilation, retaining runtime capture when lookup is unavailable. */
export async function preflightReleaseBinding(client: InterfereClient) {
  try {
    const config = await client.releases.getReleasesConfig({
      abortSignal: AbortSignal.timeout(10_000),
    });

    return releaseBindingSchema.safeParse(config).success ? config : null;
  } catch {
    return null;
  }
}
