import type {
  DestinationProvider,
  ReleaseDestinationMetadata,
} from "../integrations/index.ts";
import { normalizeEnv } from "./runtime.ts";

type EnvironmentResolver = (
  env: Record<string, string | undefined>,
) => string | null;

function fromEnv(...names: readonly string[]) {
  return ((env) => {
    for (const name of names) {
      const value = normalizeEnv(env[name]);

      if (value) {
        return value;
      }
    }

    return null;
  }) satisfies EnvironmentResolver;
}

const PROVIDER_RESOLVERS: Record<
  DestinationProvider,
  readonly EnvironmentResolver[]
> = {
  cloudflare: [fromEnv("CLOUDFLARE_ENV")],
  vercel: [fromEnv("VERCEL_TARGET_ENV", "VERCEL_ENV")],
};

export const DEPLOY_ENVIRONMENT_MAX_LENGTH = 32;

export class DeployEnvironmentError extends Error {
  override name = "DeployEnvironmentError";
}

function resolveEnvironmentValue(env: Record<string, string | undefined>) {
  const explicit = normalizeEnv(env["INTERFERE_ENVIRONMENT"]);

  if (explicit) {
    return explicit;
  }

  for (const resolvers of Object.values(PROVIDER_RESOLVERS)) {
    for (const resolve of resolvers) {
      const value = resolve(env);

      if (value) {
        return value;
      }
    }
  }

  return null;
}

export function resolveDeployEnvironment(
  env: Record<string, string | undefined>,
) {
  const environment = resolveEnvironmentValue(env);

  if (!environment) {
    return null;
  }

  if (environment.length > DEPLOY_ENVIRONMENT_MAX_LENGTH) {
    throw new DeployEnvironmentError(
      `Deploy environment "${environment}" is ${environment.length} characters; the maximum is ${DEPLOY_ENVIRONMENT_MAX_LENGTH}. Set a shorter INTERFERE_ENVIRONMENT.`,
    );
  }

  return environment;
}

export function applyDeployEnvironment(
  destination: ReleaseDestinationMetadata,
  environment: string | null,
) {
  if (!environment) {
    return destination;
  }

  if (destination.provider === "cli") {
    return { ...destination, environment } satisfies ReleaseDestinationMetadata;
  }

  return {
    ...destination,
    environment,
    environmentName: environment,
    environmentTarget: environment,
  } satisfies ReleaseDestinationMetadata;
}
