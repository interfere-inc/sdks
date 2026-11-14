import { z } from "zod";
import { inferRuntimeFromEvidence } from "../internal/runtime-inference.ts";

export const envSchema = z.string().min(1).nullable().meta({
  description:
    "Deployment environment the SDK is running in. Free-form label (e.g. `production`, `preview`, `staging`, `development`).",
  id: "Environment",
  title: "Environment",
});

export type Env = z.infer<typeof envSchema>;

/**
 * Fallback environment when nothing is configured. `development` (not
 * `production`, which is what Sentry defaults to) because the collector has
 * an always-on per-surface environment filter — an unconfigured run is far
 * more likely a developer's laptop than a real deploy, so a filtered-by-default
 * label keeps local noise out of product data. A real deploy sets one of the
 * higher-precedence signals below.
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
 * `NODE_ENV` as a last-resort fallback, then {@link DEFAULT_ENVIRONMENT}.
 *
 * `NODE_ENV` is deliberately last: it's a build-mode flag (staging, preview,
 * and prod all ship `NODE_ENV=production`), not a deploy-environment
 * identifier. The dedicated var is the real abstraction.
 */
export function resolveEnvironment(opts?: {
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

  return DEFAULT_ENVIRONMENT;
}

export const runtimeSchema = z
  .enum(["browser", "node", "edge"])
  .nullable()
  .meta({
    description: "JavaScript runtime the SDK is running in.",
    id: "Runtime",
    title: "Runtime",
  });

export type Runtime = z.infer<typeof runtimeSchema>;

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
