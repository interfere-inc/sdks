import { omitUndefined } from "@interfere/helpers/omit-undefined";
import {
  parseReactComponentStack,
  toException,
} from "@interfere/types/sdk/errors";
import type { ErrorMechanism } from "@interfere/types/sdk/plugins/payload/errors";

import { isHydrationError } from "../hydration/hydration.js";
import type { Kernel } from "../kernel.js";
import { hasServerDigest } from "../server-digest/server-digest.js";

/**
 * Routes a React error through `kernel.recordException()`, attaching the
 * component stack reported by React as additional frames. Dedup of the
 * same `Error` instance caught by both the boundary and `window.onerror`
 * happens inside `recordException`.
 *
 * React forwards whatever was thrown, so non-Error values (`throw "x"`,
 * `throw null`, `throw { code }`) are normalised with `toException` and
 * marked synthetic.
 *
 * `error.digest` is React's identifier for an RSC server-side throw that
 * was sanitised and rebuilt on the client — promote it so enrichment can
 * fingerprint the redacted client capture into the same problem as the
 * unredacted server capture.
 *
 * Accepts a nullable kernel — error boundaries can mount outside the
 * `<InterfereProvider>` tree, in which case we silently skip capture and
 * let the host render its fallback / call its `onError` callback.
 */
export function captureReactError(
  kernel: Pick<Kernel, "recordException"> | null,
  // oxlint-disable-next-line anti-slop/no-unknown-parameters -- React forwards arbitrary thrown JavaScript values to boundaries and root callbacks.
  thrown: unknown,
  componentStack: string | null | undefined,
  mechanism: ErrorMechanism,
) {
  if (!kernel) {
    return;
  }

  const isError = thrown instanceof Error;
  const hydration = isHydrationError(thrown);

  let resolved = mechanism;

  if (!isError) {
    resolved = { ...mechanism, synthetic: true };
  } else if (hydration) {
    resolved = { ...mechanism, synthetic: false };
  }

  kernel.recordException(
    isError ? thrown : toException(thrown),
    omitUndefined({
      mechanism: resolved,
      subtype: hydration ? "hydration" : undefined,
      appendFrames: componentStack
        ? parseReactComponentStack(componentStack)
        : undefined,
      errorDigest: hasServerDigest(thrown)
        ? thrown.digest || undefined
        : undefined,
    }),
  );
}
