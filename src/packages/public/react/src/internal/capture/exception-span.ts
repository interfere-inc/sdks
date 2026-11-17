import type { IngestedFrame } from "@interfere/types/data/frame";
import { isNonErrorException, toExceptions } from "@interfere/types/sdk/errors";
import type { NonErrorException } from "@interfere/types/sdk/errors";
import {
  EXCEPTION_LIMITS,
  truncateExceptionMessage,
  truncateExceptionStack,
} from "@interfere/types/sdk/errors/truncate";
import { exceptionSpanName } from "@interfere/types/sdk/throw-site";
import type { ErrorMechanism } from "@interfere/types/sdk/plugins/payload/errors";

import {
  ROOT_CONTEXT,
  SpanStatusCode,
  TraceFlags,
  context,
  isSpanContextValid,
  trace,
} from "@opentelemetry/api";
import type {
  Attributes,
  Context,
  Span,
  SpanOptions,
  TimeInput,
  Tracer,
} from "@opentelemetry/api";

import { getGlobal } from "../../util/global.js";
import { safeStringify } from "../../util/stringify.js";
import { transportFailureAttrs } from "./transport-context.js";

export interface RecordExceptionOpts {
  /**
   * Frames always appended to the first exception's stack. Used by
   * `<ErrorBoundary>` to inject React's component-stack frames as
   * supplementary context — these add on top of the real JS frames, they
   * don't replace them.
   */
  readonly appendFrames?: readonly IngestedFrame[];
  /**
   * `error.digest` from React when an RSC server-side throw is
   * sanitised and rebuilt as a synthetic Error on the client. The only
   * identifier that pairs the redacted client capture with the
   * unredacted server capture — enrichment mixes this into the
   * fingerprint so both events group as one problem.
   */
  readonly errorDigest?: string;
  /**
   * Frames adopted only when the first exception's parsed stack is empty.
   * Used by the `window.onerror` handler when the browser hands us
   * source/line/col but the `Error` itself has a degenerate stack — never
   * glued on top of a real stack.
   */
  readonly fallbackFrames?: readonly IngestedFrame[];
  readonly mechanism: ErrorMechanism;
  /**
   * A distinct error sub-classification layered on top of the mechanism —
   * currently `"hydration"` for React hydration mismatches. Lets Interfere
   * triage by kind without re-parsing the message.
   */
  readonly subtype?: string;
}

export function buildExceptionEventAttrs(
  value: Error | NonErrorException,
  opts: RecordExceptionOpts,
) {
  const attrs: Attributes = {
    "interfere.exception.handled": opts.mechanism.handled,
    "interfere.exception.mechanism": opts.mechanism.type,
  };

  if (opts.errorDigest) {
    attrs["interfere.error.digest"] = opts.errorDigest;
  }

  if (opts.subtype) {
    attrs["interfere.exception.subtype"] = opts.subtype;
  }

  if (isNonErrorException(value)) {
    attrs["exception.type"] = value.type;
    attrs["exception.message"] = truncateExceptionMessage(value.value);
    attrs["interfere.exception.kind"] = "non-error";

    attrs["interfere.exception.serialized"] = safeStringify(value.serialized);

    return attrs;
  }

  attrs["exception.type"] = value.name;
  attrs["exception.message"] = truncateExceptionMessage(value.message);
  attrs["interfere.exception.kind"] = "error";

  if (value.stack) {
    attrs["exception.stacktrace"] = truncateExceptionStack(
      value.stack,
      value.message,
    );
  }

  attrs["interfere.exception.chain"] = safeStringify(
    toExceptions(value, opts.mechanism),
  );

  Object.assign(attrs, transportFailureAttrs(value));

  const debugIds = getGlobal("_debugIds");

  if (
    debugIds !== null && // oxlint-disable-next-line anti-slop/no-runtime-typeof -- The injected debug-ID global may be absent or overwritten by arbitrary host JavaScript.
    typeof debugIds === "object" &&
    Object.keys(debugIds).length > 0
  ) {
    attrs["interfere.exception.debug_ids"] = safeStringify(debugIds);
  }

  if (opts.appendFrames && opts.appendFrames.length > 0) {
    attrs["interfere.exception.appended_frames"] = safeStringify(
      opts.appendFrames.slice(0, EXCEPTION_LIMITS.stackFrames),
    );
  }

  if (opts.fallbackFrames && opts.fallbackFrames.length > 0) {
    attrs["interfere.exception.fallback_frames"] = safeStringify(
      opts.fallbackFrames.slice(0, EXCEPTION_LIMITS.stackFrames),
    );
  }

  return attrs;
}

export function recordExceptionOnSpan(
  span: Span,
  value: Error | NonErrorException,
  attrs: Attributes,
  handled: boolean,
  at?: TimeInput,
) {
  span.addEvent("exception", attrs, at);

  if (!handled) {
    span.setStatus({
      code: SpanStatusCode.ERROR,
      message: truncateExceptionMessage(
        isNonErrorException(value) ? value.value : value.message,
      ),
    });
  }
}

/**
 * Opens the span that carries a captured exception when no recording span
 * is active: named after the throw site and tagged `interfere.capture.kind`
 * so consumers recognise it by attribute rather than by name. An unsampled
 * active context (a host trace with flags `00`) would make a child span
 * unsampled too and the error would never export, so the span is then the
 * root of its own trace with a link to that context; a sampled context
 * parents it.
 */
export function startExceptionSpan(
  tracer: Tracer,
  value: Error | NonErrorException,
  origin?: { readonly at: number; readonly context: Context },
) {
  const name = exceptionSpanName(value);

  const options: SpanOptions = {
    attributes: { "interfere.capture.kind": "exception" },
  };

  if (origin) {
    options.startTime = origin.at;
  }

  const parent = origin?.context ?? context.active();
  const active = trace.getSpanContext(parent);

  if (
    active &&
    isSpanContextValid(active) &&
    // oxlint-disable-next-line no-bitwise -- W3C trace flags are a bitmask; SAMPLED must be tested independently of other flags.
    (active.traceFlags & TraceFlags.SAMPLED) === 0
  ) {
    return tracer.startSpan(
      name,
      { ...options, links: [{ context: active }] },
      ROOT_CONTEXT,
    );
  }

  return tracer.startSpan(name, options, parent);
}
