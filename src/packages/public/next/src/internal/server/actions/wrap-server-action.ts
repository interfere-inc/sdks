import { omitUndefined } from "@interfere/helpers/omit-undefined";

import { SpanStatusCode, context, trace } from "@opentelemetry/api";

import { contain } from "@interfere/server-core/contain";
import { controlFlowOutcome } from "../capture/control-flow.js";
import { markServerAction } from "./server-action-marker.js";

const SITE = "withInterfereServerAction()";

const ATTR_SERVER_ACTION_OUTCOME = "interfere.server_action.outcome" as const;

const TRACER_NAME = "@interfere/next/server";

interface ServerActionOptions {
  /** Derives argument types and class names for args_shape without recording values. */
  readonly args?: readonly unknown[];
}

/**
 * Wraps a Next.js Server Action so a failure is attributed to the action.
 * Records the action's name, args-shape (no values), and outcome on a dedicated
 * span, and annotates any thrown error with that identity — it does **not**
 * emit its own exception event. Next routes the rethrown error to
 * `onRequestError`, which becomes the single server-side capture and is tagged
 * `auto.function.nextjs.server_action` once it reads the annotation. That one
 * event carries Next's `errorDigest`, so it groups with the client report
 * instead of forking a second, digest-less issue.
 *
 * ```ts
 * export async function createUser(formData: FormData) {
 *   "use server";
 *   return withInterfereServerAction(
 *     "createUser",
 *     async () => {
 *       // ...action body
 *     },
 *     { args: [formData] }
 *   );
 * }
 * ```
 */
export async function withInterfereServerAction<T>(
  name: string,
  action: () => T | Promise<T>,
  options?: ServerActionOptions,
) {
  const signature = contain(SITE, () =>
    // oxlint-disable-next-line anti-slop/no-shape-in-symbol-names -- The existing args_shape serializer records structure only, never values.
    options?.args ? describeArgsShape(options.args) : null,
  );

  const span = contain(SITE, () =>
    trace.getTracer(TRACER_NAME).startSpan(`serverAction ${name}`, {
      attributes: omitUndefined({
        "interfere.server_action.name": name,
        "interfere.server_action.args_shape": signature || undefined,
      }),
    }),
  );

  try {
    const result = await (span
      ? context.with(trace.setSpan(context.active(), span), action)
      : action());

    contain(SITE, () =>
      span?.setAttribute(ATTR_SERVER_ACTION_OUTCOME, "success"),
    );

    return result;
  } catch (error) {
    contain(SITE, () => {
      const outcome = controlFlowOutcome(error);

      if (outcome) {
        span?.setAttribute(ATTR_SERVER_ACTION_OUTCOME, outcome);

        return;
      }

      span?.setAttribute(ATTR_SERVER_ACTION_OUTCOME, "error");

      span?.setStatus({ code: SpanStatusCode.ERROR });

      markServerAction(
        error,
        omitUndefined({
          name,
          // oxlint-disable-next-line anti-slop/no-shape-in-symbol-names -- The error marker and its consumer share this established structural-signature field.
          argsShape: signature || undefined,
        }),
      );
    });

    throw error;
  } finally {
    contain(SITE, () => span?.end());
  }
}

// oxlint-disable-next-line anti-slop/no-shape-in-symbol-names -- This serializer deliberately describes argument structure while discarding customer values.
function describeArgsShape(args: readonly unknown[]) {
  return args
    .map((arg) => {
      if (arg === null) {
        return "null";
      }

      if (arg === undefined) {
        return "undefined";
      }

      // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Objects are represented by class name rather than their potentially sensitive property values.
      if (typeof arg === "object") {
        return (
          (arg as { constructor?: { name?: string } }).constructor?.name ??
          "object"
        );
      }

      // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Primitive arguments are represented only by their JavaScript type to avoid capturing customer values.
      return typeof arg;
    })
    .join(",");
}
