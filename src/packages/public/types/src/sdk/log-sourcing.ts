import { z } from "zod";

import { normalizeEnv } from "./runtime.ts";

/**
 * Global stamped into the server bundle at build time (via `withInterfere`'s
 * value-injection loader) from the surface's remote config. Read at cold
 * start so the mode is known synchronously, before the console bridge and
 * logger provider register.
 */
export const LOG_SOURCING_GLOBAL = "__INTERFERE_LOG_SOURCING__";

export const logSourcingModeSchema = z.enum(["sdk", "drain"]);

/**
 * How the server-side SDK sources logs.
 *
 * - `sdk`: the SDK intercepts `console.*` (`bridgeConsoleToOtel`) and ships
 *   log records through its own OTLP log exporter.
 * - `drain`: the SDK does not intercept `console.*` and does not register a
 *   logger provider; a platform log drain (e.g. Vercel) captures stdout
 *   out-of-process and posts records to the collector directly.
 */
export type LogSourcingMode = z.infer<typeof logSourcingModeSchema>;

export const DEFAULT_LOG_SOURCING_MODE = "sdk" satisfies LogSourcingMode;

/** The build-injected surface configuration wins; otherwise {@link DEFAULT_LOG_SOURCING_MODE}. */
// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Injected settings are untrusted host values; non-mode values fall back to the default.
export function resolveLogSourcingMode(injected?: unknown) {
  const parsed = logSourcingModeSchema.safeParse(normalizeEnv(injected));

  return parsed.success ? parsed.data : DEFAULT_LOG_SOURCING_MODE;
}

/** Reads the build-injected {@link LOG_SOURCING_GLOBAL} global. */
export function readLogSourcingMode() {
  const scope = globalThis;

  return resolveLogSourcingMode(
    LOG_SOURCING_GLOBAL in scope ? scope[LOG_SOURCING_GLOBAL] : undefined,
  );
}
