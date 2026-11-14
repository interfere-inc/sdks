type ControlFlowOutcome =
  | "aborted"
  | "control_flow"
  | "forbidden"
  | "not_found"
  | "redirect"
  | "unauthorized";

const DIGEST_PREFIX_OUTCOMES: readonly (readonly [
  string,
  ControlFlowOutcome,
])[] = [
  ["NEXT_REDIRECT", "redirect"],
  ["NEXT_NOT_FOUND", "not_found"],
  ["NEXT_HTTP_ERROR_FALLBACK;404", "not_found"],
  ["NEXT_HTTP_ERROR_FALLBACK;403", "forbidden"],
  ["NEXT_HTTP_ERROR_FALLBACK;401", "unauthorized"],
  ["NEXT_HTTP_ERROR_FALLBACK", "control_flow"],
];

const CONTROL_FLOW_DIGESTS: ReadonlySet<string> = new Set([
  "BAILOUT_TO_CLIENT_SIDE_RENDERING",
  "CLIENT_HOOK_DYNAMIC",
  "DYNAMIC_SERVER_USAGE",
  "HANGING_PROMISE_REJECTION",
  "NEXT_PRERENDER_INTERRUPTED",
]);

const CONTROL_FLOW_CODES: ReadonlySet<string> = new Set([
  "NEXT_STATIC_GEN_BAILOUT",
]);

const ABORT_ERROR_NAMES: ReadonlySet<string> = new Set([
  "AbortError",
  "ResponseAborted",
]);

const REACT_POSTPONE = Symbol.for("react.postpone");

function digestOutcome(digest: string) {
  if (CONTROL_FLOW_DIGESTS.has(digest)) {
    return "control_flow" satisfies ControlFlowOutcome;
  }

  return DIGEST_PREFIX_OUTCOMES.find(([prefix]) =>
    digest.startsWith(prefix),
  )?.[1];
}

/** Classifies a thrown value that Next.js or React use for control flow rather than failure. */
export function controlFlowOutcome(
  // oxlint-disable-next-line anti-slop/no-unknown-parameters -- Capture receives arbitrary thrown values; only framework sentinels are classified.
  error: unknown,
) {
  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Primitive throws are never framework sentinels.
  if (error === null || typeof error !== "object") {
    return;
  }

  if ("$$typeof" in error && error.$$typeof === REACT_POSTPONE) {
    return "control_flow" satisfies ControlFlowOutcome;
  }

  if (!(error instanceof Error)) {
    return;
  }

  if (ABORT_ERROR_NAMES.has(error.name)) {
    return "aborted" satisfies ControlFlowOutcome;
  }

  const digest = "digest" in error ? error.digest : undefined;

  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Customer errors may carry non-string digest properties that are not Next control-flow sentinels.
  if (typeof digest === "string") {
    const outcome = digestOutcome(digest);

    if (outcome) {
      return outcome;
    }
  }

  const code = "code" in error ? error.code : undefined;

  const isStaticGenerationBailout =
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Node system errors carry string codes; only Next's static generation bailout code is control flow.
    typeof code === "string" && CONTROL_FLOW_CODES.has(code);

  return isStaticGenerationBailout
    ? ("control_flow" satisfies ControlFlowOutcome)
    : undefined;
}
