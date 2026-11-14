import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { context, trace } from "@opentelemetry/api";
import type { Context, Span, SpanContext, Tracer } from "@opentelemetry/api";

import type { HeaderSet } from "../instrumentations/header-policy.js";
import { readTraceContext } from "../trace-headers.js";
import type { TraceHeaderWriter } from "../trace-headers.js";
import { ATTR_REQUEST_TRACE_ID } from "./request-observer-events.js";
import type { RequestObservation } from "./request-observer-events.js";
import {
  absoluteUrl,
  endWhenSettled,
  markRequestError,
  setResponseStatus,
  startRequestSpan,
} from "./request-observer-span.js";

interface XhrRequest {
  method: string;
  url: string;
}

export interface XhrObserverInput {
  captureContext?: (ctx: Context) => Context;
  /** The header set a request to the URL may carry now, or `undefined` when its origin is not allowed. */
  allowedHeaders: (url: string, method: string) => HeaderSet | undefined;
  isActive: () => boolean;
  /** Whether the URL is observed at all; ignored URLs pass straight through. */
  isObserved: (url: string) => boolean;
  isTracked: (url: string) => boolean;
  /** A header-bearing request to the URL failed at the network layer before any response. */
  onFailure: (url: string) => void;
  /** A header-bearing request to the URL received response headers. */
  onHeaders: (url: string) => void;
  onObservation?: (observation: RequestObservation, ctx: Context) => void;
  /** Receives a tracked request's span, its headers and a call site created in the `send` wrapper the application called. */
  onStart?: (span: Span, headers: Headers, callsite: Error) => void;
  /** Records the request outcome after the terminal event is on the span; a returned promise holds the span open until it settles. */
  onOutcome: (span: Span, xhr: XMLHttpRequest) => Promise<void>;
  traceHeaders: TraceHeaderWriter | null;
  tracer: Tracer | null;
}

const TERMINAL_EVENTS = ["load", "error", "abort", "timeout"] as const;

function watchXhr(
  input: XhrObserverInput,
  xhr: XMLHttpRequest,
  request: XhrRequest,
  ctx: Context,
  span: Span | null,
  finalContext: () => SpanContext | undefined,
  injected: boolean,
) {
  const listeners = new AbortController();
  const { signal } = listeners;

  let headersSeen = false;

  const settle = (event: Event) => {
    listeners.abort();

    const left = finalContext();

    if (span && left) {
      span.setAttribute(ATTR_REQUEST_TRACE_ID, left.traceId);
    }

    const failure = event.type === "load" ? undefined : event.type;

    if (injected && failure === "error" && !headersSeen) {
      input.onFailure(request.url);
    }

    if (!span) {
      input.onObservation?.(
        omitUndefined({
          errorType: failure === "error" ? "NetworkError" : failure,
          kind: "xhr" as const,
          method: request.method.toUpperCase(),
          status: xhr.status > 0 ? xhr.status : undefined,
          traceContext: left,
          url: request.url,
        }),
        ctx,
      );

      return;
    }

    if (failure === "error" || failure === "timeout") {
      markRequestError(span, failure);
    }

    span.addEvent(failure ?? (xhr.status < 299 ? "loaded" : "error"));

    const pending = input.onOutcome(span, xhr);

    setResponseStatus(span, xhr.status);

    void endWhenSettled(span, pending);
  };

  const onReadyStateChange = () => {
    if (
      !headersSeen &&
      xhr.readyState >= XMLHttpRequest.HEADERS_RECEIVED &&
      xhr.status > 0
    ) {
      headersSeen = true;

      if (injected) {
        input.onHeaders(request.url);
      }
    }
  };

  xhr.addEventListener("readystatechange", onReadyStateChange, { signal });

  for (const type of TERMINAL_EVENTS) {
    xhr.addEventListener(type, settle, { signal });
  }

  return listeners;
}

/**
 * Wraps `XMLHttpRequest.prototype.open`, `.setRequestHeader` and `.send`
 * once. `open` remembers the method and absolute URL, `setRequestHeader`
 * records the headers an observed request already carries (the request
 * object cannot read them back), and `send` either traces the request (a
 * tracer is present) or reports its outcome to `onObservation`. Every
 * listener a send attaches is released by the first terminal event.
 * Disposal restores each prototype method only while it is still this
 * wrapper.
 */
export function installXhrObserver(input: XhrObserverInput) {
  const proto = XMLHttpRequest.prototype;
  // oxlint-disable-next-line typescript/unbound-method -- The original prototype method is saved to chain and restore it.
  const originalOpen = proto.open;
  // oxlint-disable-next-line typescript/unbound-method -- The original prototype method is saved to chain and restore it.
  const originalSend = proto.send;
  // oxlint-disable-next-line typescript/unbound-method -- The original prototype method is saved to chain and restore it.
  const originalSetRequestHeader = proto.setRequestHeader;

  const requests = new WeakMap<XMLHttpRequest, XhrRequest>();
  const headers = new WeakMap<XMLHttpRequest, Map<string, string>>();

  const observedOpen: typeof originalOpen = function observedOpen(
    this: XMLHttpRequest,
    ...args: [
      string,
      string | URL,
      boolean?,
      (string | null)?,
      (string | null)?,
    ]
  ) {
    const [method, url, async = true, username, password] = args;

    requests.delete(this);

    headers.delete(this);

    const absolute = input.isActive() ? absoluteUrl(url) : null;

    if (absolute && input.isObserved(absolute)) {
      requests.set(this, { method, url: absolute });

      headers.set(this, new Map());
    }

    originalOpen.call(this, method, url, async, username, password);
  };

  const observedSetRequestHeader: typeof originalSetRequestHeader =
    function observedSetRequestHeader(
      this: XMLHttpRequest,
      name: string,
      value: string,
    ) {
      const seen = headers.get(this);

      if (seen) {
        const prior = seen.get(name.toLowerCase());

        seen.set(
          name.toLowerCase(),
          prior === undefined ? value : `${prior}, ${value}`,
        );
      }

      originalSetRequestHeader.call(this, name, value);
    };

  const observedSend: typeof originalSend = function observedSend(
    this: XMLHttpRequest,
    ...args: Parameters<typeof originalSend>
  ) {
    const request = input.isActive() ? requests.get(this) : undefined;

    if (!request) {
      originalSend.apply(this, args);

      return;
    }

    requests.delete(this);

    const ctx = input.captureContext?.(context.active()) ?? context.active();

    const span = input.tracer
      ? startRequestSpan(input.tracer, { ...request, body: args[0] }, ctx)
      : null;

    let left: SpanContext | undefined;
    let injected = false;

    const allowed = input.allowedHeaders(request.url, request.method);

    if (input.traceHeaders && allowed) {
      const seen = headers.get(this) ?? new Map<string, string>();

      left =
        input.traceHeaders(
          span ? trace.setSpan(ctx, span) : ctx,
          {
            append: (name, value) => {
              injected = true;

              this.setRequestHeader(name, value);
            },
            get: (name) => seen.get(name.toLowerCase()) ?? null,
          },
          allowed,
        ) ?? undefined;

      if (span && left) {
        span.setAttribute(ATTR_REQUEST_TRACE_ID, left.traceId);
      }
    }

    const seen = headers.get(this) ?? new Map<string, string>();

    if (span && input.isTracked(request.url)) {
      // WARNING: The request call site must be constructed in this frame, the one the application called; the recorded stack drops exactly one frame.
      input.onStart?.(
        span,
        new Headers([...seen]),
        new Error("HTTP request call site"),
      );
    }

    const finalContext = () =>
      readTraceContext({
        get: (name) => seen.get(name.toLowerCase()) ?? null,
      });

    const listeners = watchXhr(
      input,
      this,
      request,
      ctx,
      span,
      finalContext,
      injected,
    );

    try {
      context.with(span ? trace.setSpan(ctx, span) : ctx, () => {
        originalSend.apply(this, args);
      });
    } catch (error) {
      listeners.abort();

      span?.end();

      throw error;
    } finally {
      headers.delete(this);
    }
  };

  proto.open = observedOpen;

  proto.setRequestHeader = observedSetRequestHeader;

  proto.send = observedSend;

  return {
    dispose: () => {
      if (proto.open === observedOpen) {
        proto.open = originalOpen;
      }

      if (proto.setRequestHeader === observedSetRequestHeader) {
        proto.setRequestHeader = originalSetRequestHeader;
      }

      if (proto.send === observedSend) {
        proto.send = originalSend;
      }
    },
  };
}
