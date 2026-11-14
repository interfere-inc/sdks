import { z } from "zod";
import type { Interfere, InterfereClient } from "@interfere/javascript";
import {
  configuredSourceProviderSchema,
  releaseDestinationProviderSchema,
} from "@interfere/types/integrations";
import type { CreateReleaseRequest } from "@interfere/types/releases/definition";
import type { ReleaseSlug } from "@interfere/types/releases/slug";
import { resolveCliReleaseDestination } from "@interfere/types/sdk/cli-destination";
import { applyDeployEnvironment } from "@interfere/types/sdk/deploy-environment";
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

export function resolveReleaseRequest(input: {
  buildId: string;
  environment: string | null;
  releaseSlug: ReleaseSlug;
  producerVersion: string;
  config: Interfere.ReleasesConfigResponse;
}) {
  if (
    !configuredSourceProviderSchema.safeParse(
      input.config.surface.sourceProvider,
    ).success
  ) {
    return { ok: false, reason: "missing_source_provider" } satisfies
      | { ok: true; request: CreateReleaseRequest }
      | {
          ok: false;
          reason: "missing_source_provider" | "missing_destination_provider";
        };
  }

  const provider = releaseDestinationProviderSchema.safeParse(
    input.config.surface.destinationProvider,
  );

  if (!provider.success) {
    return { ok: false, reason: "missing_destination_provider" } satisfies
      | { ok: true; request: CreateReleaseRequest }
      | {
          ok: false;
          reason: "missing_source_provider" | "missing_destination_provider";
        };
  }

  const destinations = {
    // oxlint-disable-next-line node/no-process-env
    cli: () => resolveCliReleaseDestination(process.env, input.releaseSlug),
    cloudflare: resolveCloudflare,
    vercel: resolveVercel,
  };

  const request: CreateReleaseRequest = {
    buildId: input.buildId,
    destination: applyDeployEnvironment(
      destinations[provider.data](),
      input.environment,
    ),
    producerVersion: input.producerVersion,
    slug: input.releaseSlug,
    // oxlint-disable-next-line node/no-process-env
    source: resolveGithubReleaseSource(process.env, git),
  };

  return { ok: true, request } satisfies
    | { ok: true; request: CreateReleaseRequest }
    | {
        ok: false;
        reason: "missing_source_provider" | "missing_destination_provider";
      };
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

/** Describe a confirmed incomplete surface without exposing credentials. */
export function missingReleaseBinding(
  config: Interfere.ReleasesConfigResponse,
) {
  const source = configuredSourceProviderSchema.safeParse(
    config.surface.sourceProvider,
  );

  const destination = releaseDestinationProviderSchema.safeParse(
    config.surface.destinationProvider,
  );

  if (source.success && destination.success) {
    return null;
  }

  return `Interfere is disabled for this build: configure the surface's ${source.success ? "deployment target" : "version control provider"} at https://interfere.com/~/${config.org.slug}/apps/${config.surface.slug}`;
}
