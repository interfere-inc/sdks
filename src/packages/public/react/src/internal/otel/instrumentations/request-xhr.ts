import type { Span, TracerProvider } from "@opentelemetry/api";
import { XhrInstrumentation } from "@opentelemetry/browser-instrumentation/experimental/xhr";

import { ATTR_REQUEST_TRACE_ID } from "../request-observer/request-observer-events.js";
import { readTraceContext } from "../trace-headers.js";
import { absoluteUrl, holdSpanEnd } from "./request-core.js";
import type {
  RequestCore,
  RequestInstrumentationsInput,
} from "./request-core.js";
import { captureRequestEvidence } from "./request-callsite.js";
import { applyXhrSpanAttrs } from "./request-outcome.js";
import { composeRequestMethod, requestPropagator } from "./upstream-request.js";

/** The upstream XMLHttpRequest instrumentation with Interfere's header writer and failed-request evidence attached. */
export function createXhrInstrumentation(
  core: RequestCore,
  input: RequestInstrumentationsInput,
  tracerProvider: TracerProvider,
) {
  const { failureRanges } = input;

  const recorded = new WeakMap<
    XMLHttpRequest,
    { url: string | null; headers: Map<string, string> }
  >();

  const spans = new WeakMap<XMLHttpRequest, Span>();

  const instrumentation = new XhrInstrumentation({
    sanitizeUrl: input.redactUrl,
    ignoreUrls: input.ignoreUrls,
    measureRequestSize: true,
    propagateTraceHeaderCorsUrls: [],
    requestHook: (span, xhr) => {
      const entry = recorded.get(xhr);
      const url = entry?.url;

      if (!url) {
        return;
      }

      spans.set(xhr, span);

      const seen = entry.headers;

      core.begin(span, url, {
        append: (name, value) => {
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
    },
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
        ),
      );
    },
  });

  const proto = XMLHttpRequest.prototype;
  // oxlint-disable-next-line typescript/unbound-method -- Restored by identity on disposal.
  const originalOpen = proto.open;
  // oxlint-disable-next-line typescript/unbound-method -- Restored by identity on disposal.
  const originalSend = proto.send;

  instrumentation.enable({ tracerProvider, propagator: requestPropagator });

  const restoreOpen = composeRequestMethod(
    proto,
    "open",
    originalOpen,
    (patched) =>
      function interfereOpen(
        this: XMLHttpRequest,
        ...args: [
          method: string,
          url: string | URL,
          async?: boolean,
          username?: string | null,
          password?: string | null,
        ]
      ) {
        if (core.active) {
          const seen = new Map<string, string>();

          recorded.set(this, { url: absoluteUrl(args[1]), headers: seen });

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

        const open: (this: XMLHttpRequest, ...values: typeof args) => void =
          patched;

        open.apply(this, args);
      },
  );

  const restoreSend = composeRequestMethod(
    proto,
    "send",
    originalSend,
    (patched) =>
      function interfereSend(this: XMLHttpRequest, ...args) {
        const send: (this: XMLHttpRequest, ...values: typeof args) => void =
          patched;

        if (!core.active) {
          send.apply(this, args);

          return;
        }

        // WARNING: The request call site must be constructed in this frame, the one the application called; the recorded stack drops exactly one frame.
        core.callsite = failureRanges
          ? new Error("HTTP request call site")
          : undefined;

        try {
          send.apply(this, args);
        } finally {
          core.callsite = undefined;

          const span = spans.get(this);

          const left = readTraceContext({
            get: (name) => recorded.get(this)?.headers.get(name) ?? null,
          });

          if (span && left) {
            span.setAttribute(ATTR_REQUEST_TRACE_ID, left.traceId);
          }

          spans.delete(this);

          recorded.delete(this);

          Reflect.deleteProperty(this, "setRequestHeader");
        }
      },
  );

  return () => {
    instrumentation.disable();

    restoreOpen();

    restoreSend();
  };
}
