import { ROOT_CONTEXT, trace } from "@opentelemetry/api";
import type { SpanContext } from "@opentelemetry/api";

const failedRequests = new WeakMap<object, SpanContext>();

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Fetch rejects with any JavaScript value; only objects can key the registry.
export function rememberFailedRequest(error: unknown, span: SpanContext) {
  if (error instanceof Object) {
    failedRequests.set(error, span);
  }
}

/** The context of the request span whose fetch rejected with this error or one in its `cause` chain, so a later capture joins the request's trace. */
// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Captured values are arbitrary thrown values.
export function failedRequestContext(error: unknown) {
  let current = error;

  for (let depth = 0; current instanceof Object && depth < 5; depth += 1) {
    const span = failedRequests.get(current);

    if (span) {
      return trace.setSpanContext(ROOT_CONTEXT, span);
    }

    current = current instanceof Error ? current.cause : undefined;
  }

  return;
}
