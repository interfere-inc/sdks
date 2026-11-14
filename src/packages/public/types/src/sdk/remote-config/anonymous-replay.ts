import { z } from "zod";

/** Global stamped into the bundle at build time from the surface's remote config. */
export const ANONYMOUS_REPLAY_GLOBAL = "__INTERFERE_ANONYMOUS_REPLAY__";

export const anonymousReplayPolicySchema = z
  .enum(["always", "on_error", "never"])
  .describe(
    "When sessions without an identified user record replay: always, only once an error occurs, or never.",
  );

/** When the surface records session replay for visitors that have not been identified. */
export type AnonymousReplayPolicy = z.infer<typeof anonymousReplayPolicySchema>;

/** Policy assumed when neither the remote config nor the build supplies one. */
export const DEFAULT_ANONYMOUS_REPLAY_POLICY =
  "always" satisfies AnonymousReplayPolicy;

/** Policy a surface receives when it is created without an explicit choice. */
export function defaultAnonymousReplayPolicy(anonymousUserTracking: boolean) {
  return (
    anonymousUserTracking ? "on_error" : "always"
  ) satisfies AnonymousReplayPolicy;
}

/** Reads the build-injected {@link ANONYMOUS_REPLAY_GLOBAL}, or null when the build supplied none. */
export function readBakedAnonymousReplayPolicy() {
  const scope = globalThis;

  const injected =
    ANONYMOUS_REPLAY_GLOBAL in scope
      ? scope[ANONYMOUS_REPLAY_GLOBAL]
      : undefined;

  return anonymousReplayPolicySchema.safeParse(injected).data ?? null;
}
