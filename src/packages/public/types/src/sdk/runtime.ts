import { inferRuntimeFromEvidence } from "../internal/runtime-inference.ts";

/** Deployment environment the SDK is running in: a free-form label such as `production`, `preview`, `staging` or `development`. */
export type Env = string | null;

/**
 * Fallback environment when no environment variable and no bundler build mode
 * is available, such as unbundled code on a developer's laptop. `development`
 * (not `production`, which is what Sentry defaults to) because the collector
 * has an always-on per-surface environment filter, so a filtered-by-default
 * label keeps local noise out of product data. Built bundles report their
 * build mode (`production` for `vite build`) instead, so a production deploy
 * needs no configuration and only preview or staging deploys set
 * `INTERFERE_ENVIRONMENT`.
 */
export const DEFAULT_ENVIRONMENT = "development";

const ENVIRONMENT_ENV_VARS = [
  "INTERFERE_ENVIRONMENT",
  "VERCEL_ENV",
  "NODE_ENV",
] as const;

/**
 * Resolve the deployment environment using a precedence chain modelled on
 * Sentry's: an explicit value (init option) wins, then a dedicated
 * `INTERFERE_ENVIRONMENT`, then platform signals (`VERCEL_ENV`), then
 * `NODE_ENV`, then the bundler's build mode, then
 * {@link DEFAULT_ENVIRONMENT}.
 *
 * `NODE_ENV` and the build mode come last: they say how the code was built
 * (staging, preview and prod all ship `production`), not where it is deployed.
 * The dedicated var is the real abstraction. The build mode is the signal a
 * runtime without environment variables, such as a Cloudflare Worker, still
 * has.
 */
export function resolveEnvironment(opts?: {
  readonly buildMode?: string | null | undefined;
  readonly explicit?: string | null | undefined;
  readonly env?: Record<string, string | undefined>;
}) {
  const env = opts?.env ?? {};
  const explicit = normalizeEnv(opts?.explicit);

  if (explicit) {
    return explicit;
  }

  for (const key of ENVIRONMENT_ENV_VARS) {
    const value = normalizeEnv(env[key]);

    if (value) {
      return value;
    }
  }

  return normalizeEnv(opts?.buildMode) ?? DEFAULT_ENVIRONMENT;
}

/** JavaScript runtime the SDK is running in. */
export type Runtime = "browser" | "node" | "edge" | null;

const globalScope = globalThis as {
  readonly EdgeRuntime?: unknown;
  readonly document?: unknown;
  readonly navigator?: unknown;
  readonly process?: {
    readonly env?: Record<string, string | undefined>;
    readonly versions?: { readonly node?: unknown };
  };
  readonly window?: unknown;
};

/**
 * `process.env` when the runtime provides it, `undefined` otherwise. Accessed
 * structurally via `globalThis` so callers never need `@types/node`.
 */
export function readProcessEnv() {
  return globalScope.process?.env;
}

export function inferRuntime(options?: { readonly nextRuntime?: unknown }) {
  return inferRuntimeFromEvidence({
    hasWindow: globalScope.window !== undefined,
    hasDocument: globalScope.document !== undefined,
    hasNavigator: globalScope.navigator !== undefined,
    edgeRuntime: globalScope.EdgeRuntime,
    nodeVersion: globalScope.process?.versions?.node,
    nextRuntime: options?.nextRuntime,
  });
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Environment normalization accepts untrusted injected settings and rejects non-string values.
export function normalizeEnv(value: unknown) {
  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Only strings support the environment trimming and case-normalization contract.
  if (typeof value !== "string") {
    return null;
  }

  const env = value.trim().toLowerCase();

  return (env.length > 0 ? env : null) satisfies string | null;
}
