/** Global stamped into the bundle at build time from the surface's remote config. */
export const ANONYMOUS_REPLAY_GLOBAL = "__INTERFERE_ANONYMOUS_REPLAY__";

/** Values the anonymous replay policy takes; the zod contract lives in `anonymousReplayPolicySchema`. */
export const ANONYMOUS_REPLAY_POLICIES = [
  "always",
  "on_error",
  "never",
] as const;

/** When the surface records session replay for visitors that have not been identified. */
export type AnonymousReplayPolicy = (typeof ANONYMOUS_REPLAY_POLICIES)[number];

/** Policy assumed when neither the remote config nor the build supplies one. */
export const DEFAULT_ANONYMOUS_REPLAY_POLICY =
  "always" satisfies AnonymousReplayPolicy;

/** Policy a surface receives when it is created without an explicit choice. */
export function defaultAnonymousReplayPolicy(anonymousUserTracking: boolean) {
  return (
    anonymousUserTracking ? "on_error" : "always"
  ) satisfies AnonymousReplayPolicy;
}

function isAnonymousReplayPolicy(
  value: unknown,
): value is AnonymousReplayPolicy {
  return ANONYMOUS_REPLAY_POLICIES.some((policy) => policy === value);
}

/** Reads the build-injected {@link ANONYMOUS_REPLAY_GLOBAL}, or null when the build supplied none. */
export function readBakedAnonymousReplayPolicy() {
  const scope = globalThis;

  const injected =
    ANONYMOUS_REPLAY_GLOBAL in scope
      ? scope[ANONYMOUS_REPLAY_GLOBAL]
      : undefined;

  return isAnonymousReplayPolicy(injected) ? injected : null;
}
