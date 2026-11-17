import type { Span } from "@opentelemetry/api";
import { XMLHttpRequestInstrumentation } from "@opentelemetry/instrumentation-xml-http-request";

import { ATTR_REQUEST_TRACE_ID } from "../request-observer/request-observer-events.js";
import { readTraceContext } from "../trace-headers.js";
import { absoluteUrl, holdSpanEnd } from "./request-core.js";
import type {
  RequestCore,
  RequestInstrumentationsInput,
} from "./request-core.js";
import { captureRequestEvidence } from "./request-callsite.js";
import { applyXhrSpanAttrs } from "./request-outcome.js";
import {
  composeXhrPatches,
  replaceXhrHeaderInjection,
} from "./upstream-request-members.js";
import type { XhrOpen, XhrSend } from "./upstream-request-members.js";

const CANCELLATIONS = ["abort", "timeout"] as const;

/** The upstream XMLHttpRequest instrumentation with Interfere's header writer and failed-request evidence attached. */
export function createXhrInstrumentation(
  core: RequestCore,
  input: RequestInstrumentationsInput,
) {
  const { failureRanges } = input;
  const recorded = new WeakMap<XMLHttpRequest, Map<string, string>>();
  const cancellations = new WeakMap<XMLHttpRequest, string>();
  const spans = new WeakMap<XMLHttpRequest, Span>();

  const instrumentation = new XMLHttpRequestInstrumentation({
    enabled: false,
    ignoreNetworkEvents: true,
    ignoreUrls: input.ignoreUrls,
    measureRequestSize: true,
    propagateTraceHeaderCorsUrls: [],
    applyCustomAttributesOnSpan: (span, xhr) => {
      const state = core.states.get(span);

      if (state?.injected) {
        if (xhr.status > 0) {
          core.diagnostics.success(state.url);
        } else {
          core.diagnostics.failure(
            state.url,
            new TypeError("network error"),
            false,
          );
        }
      }

      holdSpanEnd(
        span,
        applyXhrSpanAttrs(
          span,
          xhr,
          failureRanges,
          core.pageExit,
          core.isTracked,
          core.failureGate,
          input.resolveRoute,
          input.applicationErrors,
          cancellations.get(xhr),
        ),
      );
    },
  });

  replaceXhrHeaderInjection(instrumentation, (xhr, spanUrl, span) => {
    const url = absoluteUrl(spanUrl);

    if (!url) {
      return;
    }

    spans.set(xhr, span);

    const headers = recorded.get(xhr);
    const seen = headers ?? new Map<string, string>();

    core.begin(span, url, {
      append: (name, value) => {
        if (!headers) {
          seen.set(name, value);
        }

        xhr.setRequestHeader(name, value);
      },
      get: (name) => seen.get(name.toLowerCase()) ?? null,
    });

    if (core.callsite && failureRanges && core.isTracked(url)) {
      captureRequestEvidence(
        span,
        core.callsite,
        new Headers([...seen]),
        input.redactUrl,
      );
    }
  });

  const wrapOpen: (patched: XhrOpen) => XhrOpen = (patched) =>
    function interfereOpen(this: XMLHttpRequest, ...args) {
      if (core.active) {
        const seen = new Map<string, string>();

        recorded.set(this, seen);

        const setRequestHeader =
          XMLHttpRequest.prototype.setRequestHeader.bind(this);

        // WARNING: XHR cannot read back its request headers, so each request records what the application and wrappers below Interfere set; Interfere then never appends a second traceparent.
        this.setRequestHeader = (name: string, value: string) => {
          const key = name.toLowerCase();
          const prior = seen.get(key);

          seen.set(key, prior === undefined ? value : `${prior}, ${value}`);

          setRequestHeader(name, value);
        };
      }

      patched.apply(this, args);
    };

  const wrapSend: (patched: XhrSend) => XhrSend = (patched) =>
    function interfereSend(this: XMLHttpRequest, ...args) {
      if (!core.active) {
        patched.apply(this, args);

        return;
      }

      // WARNING: The request call site must be constructed in this frame, the one the application called; the recorded stack drops exactly one frame.
      core.callsite = failureRanges
        ? new Error("HTTP request call site")
        : undefined;

      cancellations.delete(this);

      const listeners = new AbortController();
      const { signal } = listeners;

      // WARNING: Upstream adds the terminal span event 300 ms after it reports the outcome, so the cancellation type is recorded here, ahead of its listeners.
      for (const type of CANCELLATIONS) {
        this.addEventListener(
          type,
          () => {
            cancellations.set(this, type);
          },
          { signal },
        );
      }

      this.addEventListener(
        "loadend",
        () => {
          listeners.abort();
        },
        { signal },
      );

      try {
        patched.apply(this, args);
      } finally {
        core.callsite = undefined;

        const span = spans.get(this);

        const left = readTraceContext({
          get: (name) => recorded.get(this)?.get(name) ?? null,
        });

        if (span && left) {
          span.setAttribute(ATTR_REQUEST_TRACE_ID, left.traceId);
        }

        spans.delete(this);

        recorded.delete(this);

        Reflect.deleteProperty(this, "setRequestHeader");
      }
    };

  composeXhrPatches(instrumentation, wrapOpen, wrapSend);

  return instrumentation;
}
