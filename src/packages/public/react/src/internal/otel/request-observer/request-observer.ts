import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { context, trace } from "@opentelemetry/api";
import type { Context, Span, Tracer } from "@opentelemetry/api";
import { isUrlIgnored } from "@opentelemetry/core";

import type { SessionResumeReason } from "../../activity/activity.js";
import { statusMatches } from "../instrumentations/failure-policy.js";
import type { HttpStatusCodeRange } from "../instrumentations/failure-policy.js";
import { createPageExitWatch } from "../instrumentations/network-failure.js";
import type { FetchError } from "../instrumentations/network-failure-types.js";
import { propagationDiagnostics } from "../instrumentations/propagation-diagnostics.js";
import { originPolicy } from "../instrumentations/propagation-targets.js";
import type { PropagationPolicy } from "../instrumentations/header-policy.js";
import { captureRequestEvidence } from "../instrumentations/request-callsite.js";
import {
  ATTR_HTTP_REQUEST_PREFETCH,
  applyFetchSpanAttrs,
  applyXhrSpanAttrs,
  createFailureGate,
  isPrefetchRequest,
} from "../instrumentations/request-outcome.js";
import { fetchResponseEvidence } from "../instrumentations/response-evidence.js";
import type { TraceHeaderWriter } from "../trace-headers.js";
import { observeRequestTrace } from "./request-trace-correlation.js";
import { ATTR_REQUEST_TRACE_ID } from "./request-observer-events.js";
import type { RequestObservation } from "./request-observer-events.js";
import {
  absoluteUrl,
  endWhenSettled,
  setResponseStatus,
  startRequestSpan,
} from "./request-observer-span.js";
import { installXhrObserver } from "./request-observer-xhr.js";

export const REQUEST_TRACER_NAME = "@interfere/react/request";

export interface RequestObserverInput {
  captureContext?: (ctx: Context) => Context;
  canCorrelate?: (ctx?: Context) => boolean;
  /** Whether a 200 response whose JSON body reports an error is recorded as a failure. */
  applicationErrors: boolean;
  /** Status ranges recorded as failed requests, or `null` when failed-request capture is off. */
  failureRanges: readonly HttpStatusCodeRange[] | null;
  /** URLs excluded from observation: Interfere's own endpoints, known third parties and the customer's ignore list. */
  ignoreUrls: (string | RegExp)[];
  /** Receives each request outcome when no tracer is given (application mode). */
  onObservation?: (observation: RequestObservation, ctx: Context) => void;
  /** Subscribes to session resumes so rotation resets per-origin diagnostics. */
  onSessionResume?: (
    handler: (reason: SessionResumeReason) => void,
  ) => () => void;
  /** Writes trace headers to requests for `targets`; `null` writes none. */
  traceHeaders: TraceHeaderWriter | null;
  /** Redacts URLs recorded from a request's call site. */
  redactUrl: (url: string) => string;
  resolveRoute?: (pathname: string) => string | undefined;
  /** Exact-origin header policy; an allowed origin is also in scope for failed-request capture and span route naming. */
  targets: PropagationPolicy;
  /** Interfere's private request tracer in managed mode; `null` observes requests as events instead of spans. */
  tracer: Tracer | null;
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Fetch rejects with any JavaScript value; only an Error carries a name worth recording.
function errorName(error: unknown) {
  return error instanceof Error ? error.name : "Error";
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Fetch rejects with any JavaScript value; the outcome path needs an Error or a message.
function fetchError(error: unknown) {
  return (
    error instanceof Error ? error : { message: String(error) }
  ) satisfies FetchError;
}

/**
 * Installs Interfere's only `fetch` and `XMLHttpRequest` wrappers: client
 * spans when a tracer is given, otherwise `onObservation` events, with the
 * trace headers the writer decides on for allowed origins. An origin is
 * reachable once response headers arrive. Disposal goes inert and restores
 * only wrappers still installed.
 */
export function installRequestObserver(input: RequestObserverInput) {
  const {
    applicationErrors,
    failureRanges,
    ignoreUrls,
    onObservation,
    redactUrl,
    resolveRoute,
    targets,
    traceHeaders,
    tracer,
  } = input;

  const isTracked = (url: string) => originPolicy(targets, url) !== undefined;
  const diagnostics = propagationDiagnostics(globalThis.location.origin);

  const unsubscribeSession = input.onSessionResume?.((reason) => {
    if (reason === "rotated") {
      diagnostics.reset();
    }
  });

  const failureGate = createFailureGate();
  const pageExit = createPageExitWatch();

  let active = true;

  const originalFetch = globalThis.fetch;

  function allowedHeaders(url: string, method: string) {
    const entry = originPolicy(targets, url);

    if (!entry) {
      return;
    }

    return diagnostics.allowed(entry, url, method);
  }

  function correlate(
    response: Response,
    observation: RequestObservation,
    ctx: Context,
    permitted: boolean,
  ) {
    if (!permitted) {
      return;
    }

    observeRequestTrace(response, (traceContext) => {
      if (
        !active ||
        input.canCorrelate?.(ctx) === false ||
        observation.traceContext?.traceId === traceContext.traceId
      ) {
        return;
      }

      onObservation?.({ ...observation, correlation: true, traceContext }, ctx);
    });
  }

  function prepare(
    ctx: Context,
    url: string,
    method: string,
    resource: Parameters<typeof fetch>[0],
    init: Parameters<typeof fetch>[1],
  ) {
    const headers = new Headers(
      init?.headers ??
        (resource instanceof Request ? resource.headers : undefined),
    );

    const args: Parameters<typeof fetch> = [resource, init];

    const entry = originPolicy(targets, url);

    if (!traceHeaders || !entry) {
      return { args, headers, injected: false, left: undefined };
    }

    const allowed = allowedHeaders(url, method);

    if (!allowed) {
      return { args, headers, injected: false, left: undefined };
    }

    const snapshot = () =>
      `${headers.get("traceparent")}|${headers.get("baggage")}`;

    const before = snapshot();

    const left = traceHeaders(ctx, headers, allowed) ?? undefined;

    args[1] = { ...init, headers };

    return { args, headers, injected: snapshot() !== before, left };
  }

  async function tracedFetch(
    requestTracer: Tracer,
    self: typeof globalThis,
    resource: Parameters<typeof fetch>[0],
    init: Parameters<typeof fetch>[1],
    url: string,
    method: string,
    ctx: Context,
    callsite: Error | undefined,
  ) {
    const timestamp = Date.now();
    const permitted = input.canCorrelate?.() !== false;

    const span = startRequestSpan(
      requestTracer,
      { body: init?.body, method, url },
      ctx,
    );

    const { args, headers, injected, left } = prepare(
      trace.setSpan(ctx, span),
      url,
      method,
      resource,
      init,
    );

    if (left) {
      span.setAttribute(ATTR_REQUEST_TRACE_ID, left.traceId);
    }

    const request: RequestInit & { url: string } = {
      headers,
      method,
      signal:
        init?.signal ?? (resource instanceof Request ? resource.signal : null),
      url,
    };

    if (isPrefetchRequest(request)) {
      span.setAttribute(ATTR_HTTP_REQUEST_PREFETCH, true);
    } else if (callsite) {
      captureRequestEvidence(span, callsite, headers, redactUrl);
    }

    let response: Response;

    try {
      response = await context.with(trace.setSpan(ctx, span), async () =>
        originalFetch.apply(self, args),
      );
    } catch (error) {
      if (injected) {
        diagnostics.failure(url, error, request.signal?.aborted === true, span);
      }

      void endWhenSettled(
        span,
        applyFetchSpanAttrs(
          span,
          request,
          fetchError(error),
          failureRanges,
          pageExit,
          isTracked,
          resolveRoute,
          failureGate,
          undefined,
          applicationErrors,
          requestTracer,
        ),
      );

      throw error;
    }

    if (injected) {
      diagnostics.success(url);
    }

    correlate(
      response,
      omitUndefined({
        kind: "fetch" as const,
        method,
        url,
        timestamp,
        traceContext: left,
        status: response.status,
      }),
      ctx,
      permitted,
    );

    setResponseStatus(span, response.status);

    const readsEvidence =
      failureRanges !== null &&
      (response.status === 200
        ? applicationErrors
        : statusMatches(response.status, failureRanges)) &&
      isTracked(response.url) &&
      !isPrefetchRequest(request);

    void endWhenSettled(
      span,
      applyFetchSpanAttrs(
        span,
        request,
        response,
        failureRanges,
        pageExit,
        isTracked,
        resolveRoute,
        failureGate,
        readsEvidence ? fetchResponseEvidence(response) : undefined,
        applicationErrors,
        requestTracer,
      ),
    );

    return response;
  }

  async function observedFetch(
    this: typeof globalThis,
    ...args: Parameters<typeof fetch>
  ) {
    if (!active) {
      return originalFetch.apply(this, args);
    }

    const [resource, init] = args;

    const url = absoluteUrl(
      resource instanceof Request ? resource.url : String(resource),
    );

    if (!url || isUrlIgnored(url, ignoreUrls)) {
      return originalFetch.apply(this, args);
    }

    const ctx = input.captureContext?.(context.active()) ?? context.active();
    const permitted = input.canCorrelate?.() !== false;
    const timestamp = Date.now();

    const method = (
      init?.method ?? (resource instanceof Request ? resource.method : "GET")
    ).toUpperCase();

    if (tracer) {
      // WARNING: The request call site must be constructed in this frame, the one the application called; the recorded stack drops exactly one frame.
      const callsite =
        failureRanges && isTracked(url)
          ? new Error("HTTP request call site")
          : undefined;

      return tracedFetch(
        tracer,
        this,
        resource,
        init,
        url,
        method,
        ctx,
        callsite,
      );
    }

    const {
      args: prepared,
      injected,
      left,
    } = prepare(ctx, url, method, resource, init);

    const signal =
      init?.signal ?? (resource instanceof Request ? resource.signal : null);

    const observed = {
      kind: "fetch" as const,
      method,
      traceContext: left,
      timestamp,
      url,
    };

    try {
      const response = await originalFetch.apply(this, prepared);

      if (injected) {
        diagnostics.success(url);
      }

      correlate(
        response,
        omitUndefined({ ...observed, status: response.status }),
        ctx,
        permitted,
      );

      onObservation?.(
        omitUndefined({ ...observed, status: response.status }),
        ctx,
      );

      return response;
    } catch (error) {
      if (injected) {
        diagnostics.failure(url, error, signal?.aborted === true);
      }

      onObservation?.(
        omitUndefined({ ...observed, errorType: errorName(error) }),
        ctx,
      );

      throw error;
    }
  }

  // WARNING: Host fetch typings (Bun, workerd) add static members such as preconnect; copying the original's own properties keeps the wrapper assignable and keeps them reachable.
  globalThis.fetch = Object.assign(observedFetch, originalFetch);

  const xhr = installXhrObserver(
    omitUndefined({
      allowedHeaders,
      captureContext: input.captureContext,
      isActive: () => active,
      isObserved: (url: string) => !isUrlIgnored(url, ignoreUrls),
      isTracked,
      onFailure: (url: string) => {
        diagnostics.failure(url, new TypeError("network error"), false);
      },
      onHeaders: (url: string) => {
        diagnostics.success(url);
      },
      onOutcome: async (span: Span, request: XMLHttpRequest) =>
        applyXhrSpanAttrs(
          span,
          request,
          failureRanges,
          pageExit,
          isTracked,
          failureGate,
          resolveRoute,
          applicationErrors,
        ),
      onObservation,
      onStart: failureRanges
        ? (span: Span, headers: Headers, callsite: Error) => {
            captureRequestEvidence(span, callsite, headers, redactUrl);
          }
        : undefined,
      traceHeaders,
      tracer,
    }),
  );

  return {
    dispose: () => {
      active = false;

      unsubscribeSession?.();

      diagnostics.dispose();

      pageExit.dispose();

      if (globalThis.fetch === observedFetch) {
        globalThis.fetch = originalFetch;
      }

      xhr.dispose();
    },
  };
}
