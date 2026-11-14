import { z } from "zod";

import {
  PLATFORM_GLOBAL,
  hostingPlatformSchema,
} from "../integrations/platforms/hosting.ts";
import { normalizeEnv, readProcessEnv } from "./runtime.ts";

/**
 * Global stamped into the server bundle at build time (via `withInterfere`'s
 * value-injection loader) from the surface's remote config. Read at cold
 * start so the mode is known synchronously, before instrumentation registers.
 */
export const SERVER_TRACING_GLOBAL = "__INTERFERE_SERVER_TRACING__";

export const serverTracingModeSchema = z.enum(["full", "drain"]);

/**
 * How the server-side SDK sources traces.
 *
 * - `full`: the SDK runs its own outbound HTTP (undici) instrumentation and
 *   exports spans directly.
 * - `drain`: the SDK skips outbound HTTP instrumentation and defers
 *   server/edge traces to a platform trace drain, removing the per-fetch span
 *   creation from the request hot path.
 */
export type ServerTracingMode = z.infer<typeof serverTracingModeSchema>;

export const DEFAULT_SERVER_TRACING_MODE = "full" satisfies ServerTracingMode;

/** Per-attempt and flush ceiling for direct exports on Vercel, where a flush holds the invocation open. */
export const VERCEL_EXPORT_TIMEOUT_MS = 2000;

/**
 * Resolve the server tracing mode and hosting platform from the
 * build-injected surface configuration. Unrecognised or absent modes resolve
 * to {@link DEFAULT_SERVER_TRACING_MODE}. `vercel` reports whether the
 * injected platform is Vercel, which callers use to cap direct export
 * timeouts; `VERCEL` is read only when the build injected no platform.
 */
export function resolveServerTracing(opts?: {
  readonly injected?: unknown;
  readonly platform?: unknown;
  readonly env?: Record<string, string | undefined> | undefined;
}) {
  const parsed = serverTracingModeSchema.safeParse(
    normalizeEnv(opts?.injected),
  );

  const vercel =
    opts?.platform === undefined
      ? Boolean(normalizeEnv((opts?.env ?? readProcessEnv())?.["VERCEL"]))
      : hostingPlatformSchema.safeParse(normalizeEnv(opts.platform)).data ===
        "vercel";

  return {
    mode: parsed.success ? parsed.data : DEFAULT_SERVER_TRACING_MODE,
    vercel,
  };
}

/** Reads the build-injected {@link SERVER_TRACING_GLOBAL} and {@link PLATFORM_GLOBAL} globals. */
export function readServerTracing(
  env: Record<string, string | undefined> | undefined = readProcessEnv(),
) {
  const scope = globalThis;

  const injected =
    SERVER_TRACING_GLOBAL in scope ? scope[SERVER_TRACING_GLOBAL] : undefined;

  const platform =
    PLATFORM_GLOBAL in scope ? scope[PLATFORM_GLOBAL] : undefined;

  return resolveServerTracing({ env, injected, platform });
}
