import {
  MECHANISM_TYPE,
  toError,
  toException,
} from "@interfere/types/sdk/errors";

import { SpanStatusCode, context, trace } from "@opentelemetry/api";
import type { Context, Span, SpanOptions } from "@opentelemetry/api";

import { activeKernel } from "./internal/kernel-registry.js";

const TRACER_NAME = "@interfere/react";

let scope: Context | undefined;

function tracer() {
  return activeKernel()?.getTracer(TRACER_NAME) ?? trace.getTracer(TRACER_NAME);
}

function isPromise<T>(value: unknown): value is Promise<T> {
  return (
    value !== null &&
    (typeof value === "object" || typeof value === "function") &&
    "then" in value &&
    typeof value.then === "function"
  );
}

/**
 * Runs `fn` inside a span that lasts until `fn` returns or, when it returns a
 * promise, until that promise settles. A throw or rejection is recorded on the
 * span and rethrown unchanged.
 *
 * The span is the active context only for the synchronous part of `fn`, up to
 * its first `await`. Code after an `await` runs under whatever context is
 * active at that moment, so spans it starts are not children of this span. To
 * parent them explicitly, use
 * `context.with(trace.setSpan(context.active(), span), () => ...)`; pass
 * `{ root: true }` to start a new trace.
 *
 * Before `init()`, `fn` still runs and its result is returned, but nothing is
 * recorded.
 */
export function span<T>(
  name: string,
  fn: (span: Span) => T,
  options?: SpanOptions,
): T;

export function span(
  name: string,
  // oxlint-disable-next-line anti-slop/no-unknown-returns -- The overload preserves the caller's result type while this implementation handles arbitrary synchronous and asynchronous results.
  fn: (span: Span) => unknown,
  options?: SpanOptions,
) {
  const parent = scope ?? context.active();

  return tracer().startActiveSpan(name, options ?? {}, parent, (s) => {
    try {
      const previous = scope;

      scope = trace.setSpan(parent, s);

      let result: unknown;

      try {
        result = fn(s);
      } finally {
        scope = previous;
      }

      if (isPromise(result)) {
        return result.then(
          (value) => {
            s.end();

            return value;
          },
          // oxlint-disable-next-line anti-slop/no-unknown-parameters -- Promise rejection reasons can be any JavaScript value and must be rethrown unchanged.
          (error: unknown) => {
            const e = toError(error);

            s.recordException(e);

            s.setStatus({ code: SpanStatusCode.ERROR, message: e.message });

            s.end();

            throw error;
          },
        );
      }

      s.end();

      return result;
    } catch (error) {
      const e = toError(error);

      s.recordException(e);

      s.setStatus({ code: SpanStatusCode.ERROR, message: e.message });

      s.end();

      throw error;
    }
  });
}

/** Captures any thrown value; calls before initialization are dropped. */
// oxlint-disable-next-line anti-slop/no-unknown-parameters -- The public capture API accepts arbitrary thrown values before normalizing them into exception telemetry.
export function capture(error: unknown) {
  activeKernel()?.recordException(toException(error), {
    mechanism: { handled: true, type: MECHANISM_TYPE.manual.capture },
  });
}

export {
  requestCancellation,
  requestRecovered,
} from "./internal/otel/instrumentations/request/cancellation.js";
