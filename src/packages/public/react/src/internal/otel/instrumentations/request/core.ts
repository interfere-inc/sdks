import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { context, trace } from "@opentelemetry/api";
import type {
  Context,
  Span,
  SpanContext,
  TimeInput,
  Tracer,
} from "@opentelemetry/api";

import type { SessionResumeReason } from "../../../activity/activity.js";
import { ATTR_REQUEST_TRACE_ID } from "../../request-observer/request-observer-events.js";
import type { RequestObservation } from "../../request-observer/request-observer-events.js";
import { observeRequestTrace } from "../../request-observer/request-trace-correlation.js";
import { readTraceContext } from "../../trace-headers.js";
import type { HeaderCarrier, TraceHeaderWriter } from "../../trace-headers.js";
import type { HttpStatusCodeRange } from "../failure-policy.js";
import type { PropagationPolicy } from "../header-policy.js";
import { createPageExitWatch } from "./network-failure.js";
import { propagationDiagnostics } from "../propagation-diagnostics.js";
import { originPolicy } from "../propagation-targets.js";
import { createFailureGate } from "./outcome.js";

const TRACE_CONTEXT_HEADERS = new Set(["traceparent", "tracestate"]);

export interface RequestInstrumentationsInput {
  /** Whether a 200 response whose JSON body reports an error is recorded as a failure. */
  applicationErrors: boolean;
  canCorrelate?: (ctx?: Context) => boolean;
  /** Whether the per-origin CORS memo may be kept in `sessionStorage`; without it the memo lives in this page only. */
  canPersistDiagnostics?: () => boolean;
  captureContext?: (ctx: Context) => Context;
  /** Whether another tool will write `traceparent` on a request to the URL, so Interfere writes only baggage there. */
  defersTraceContext?: (url: string) => boolean;
  /** Records failed-request evidence that settles after the request span ended. */
  evidenceTracer: Tracer;
  /** Status ranges recorded as failed requests, or `null` when failed-request capture is off. */
  failureRanges: readonly HttpStatusCodeRange[] | null;
  /** URLs excluded from request spans: Interfere's own endpoints, known third parties and the customer's ignore list. */
  ignoreUrls: (string | RegExp)[];
  /** Receives the trace a request's response reports when it differs from the one the request left with. */
  onObservation?: (observation: RequestObservation, ctx: Context) => void;
  /** Subscribes to session resumes so rotation resets per-origin diagnostics. */
  onSessionResume?: (
    handler: (reason: SessionResumeReason) => void,
  ) => () => void;
  redactUrl: (url: string) => string;
  resolveRoute?: (pathname: string) => string | undefined;
  /** Exact-origin header policy; an allowed origin is also in scope for failed-request capture and span route naming. */
  targets: PropagationPolicy;
  /** Writes trace headers to requests for `targets`; `null` writes none. */
  traceHeaders: TraceHeaderWriter | null;
}

export interface RequestState {
  ctx: Context;
  injected: boolean;
  left: SpanContext | undefined;
  permitted: boolean;
  url: string;
}

/** Delays a span's end until its outcome settles, keeping the end time the instrumentation chose. */
export function holdSpanEnd(span: Span, pending: Promise<void> | undefined) {
  if (!pending) {
    return;
  }

  const end = span.end.bind(span);

  span.end = (endTime) => {
    void (async () => {
      try {
        await pending;
      } finally {
        end(endTime);
      }
    })();
  };
}

export function absoluteUrl(url: string | URL) {
  try {
    return new URL(url, globalThis.location.href).href;
  } catch {
    return null;
  }
}

/** Per-instrumentation state shared by the fetch and XHR hooks: the header policy, diagnostics and each request's departure. */
export function createRequestCore(input: RequestInstrumentationsInput) {
  const { targets, traceHeaders } = input;

  const diagnostics = propagationDiagnostics(
    globalThis.location.origin,
    input.canPersistDiagnostics,
  );

  const states = new WeakMap<Span, RequestState>();
  const open = new Map<Span, (endTime?: TimeInput) => void>();

  const unsubscribeSession = input.onSessionResume?.((reason) => {
    if (reason === "rotated") {
      diagnostics.reset();
    }
  });

  const core = {
    active: true,
    callsite: undefined as Error | undefined,
    diagnostics,
    failureGate: createFailureGate(),
    isTracked: (url: string) => originPolicy(targets, url) !== undefined,
    pageExit: createPageExitWatch(),
    states,
    allowedHeaders(url: string) {
      const entry = originPolicy(targets, url);

      if (!entry) {
        return;
      }

      const allowed = diagnostics.allowed(entry, url);

      if (!allowed.has("traceparent") || !input.defersTraceContext?.(url)) {
        return allowed;
      }

      return new Set(
        [...allowed].filter((name) => !TRACE_CONTEXT_HEADERS.has(name)),
      ) satisfies typeof allowed;
    },
    begin(span: Span, url: string, carrier: HeaderCarrier) {
      const ctx = input.captureContext?.(context.active()) ?? context.active();

      const state: RequestState = {
        ctx,
        injected: false,
        left: undefined,
        permitted: input.canCorrelate?.() !== false,
        url,
      };

      states.set(span, state);

      const end = span.end.bind(span);

      open.set(span, end);

      span.end = (endTime) => {
        if (open.delete(span)) {
          end(endTime);
        }
      };

      const allowed = core.allowedHeaders(url);

      state.left =
        traceHeaders && allowed
          ? (traceHeaders(
              trace.setSpan(ctx, span),
              {
                append: (name, value) => {
                  state.injected = true;

                  carrier.append(name, value);
                },
                get: (name) => carrier.get(name),
              },
              allowed,
            ) ?? undefined)
          : readTraceContext(carrier);

      if (state.left) {
        span.setAttribute(ATTR_REQUEST_TRACE_ID, state.left.traceId);
      }

      return state;
    },
    correlate(response: Response, span: Span, method: string) {
      const state = states.get(span);

      if (!state?.permitted) {
        return;
      }

      observeRequestTrace(response, (traceContext) => {
        if (
          input.canCorrelate?.(state.ctx) === false ||
          state.left?.traceId === traceContext.traceId
        ) {
          return;
        }

        input.onObservation?.(
          omitUndefined({
            correlation: true,
            kind: "fetch" as const,
            method,
            status: response.status,
            traceContext,
            url: state.url,
          }),
          state.ctx,
        );
      });
    },
    endOpenRequests() {
      for (const [span, end] of open) {
        open.delete(span);

        end();
      }
    },
    dispose() {
      core.active = false;

      unsubscribeSession?.();

      diagnostics.dispose();

      core.pageExit.dispose();
    },
  };

  return core;
}

export type RequestCore = ReturnType<typeof createRequestCore>;
