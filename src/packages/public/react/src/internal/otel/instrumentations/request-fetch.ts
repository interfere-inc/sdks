import { SpanStatusCode } from "@opentelemetry/api";
import type { Span } from "@opentelemetry/api";
import { FetchInstrumentation } from "@opentelemetry/instrumentation-fetch";
import { isPlainRecord, isString } from "@interfere/types/sdk/guards/json";

import { rememberFailedRequest } from "../request-observer/failed-requests.js";
import { statusMatches } from "./failure-policy.js";
import type { FetchSpanRequest } from "./network-failure-types.js";
import { absoluteUrl, holdSpanEnd } from "./request-core.js";
import type {
  RequestCore,
  RequestInstrumentationsInput,
  RequestState,
} from "./request-core.js";
import { captureRequestEvidence } from "./request-callsite.js";
import { spanStringAttribute } from "../request-observer/span-attributes.js";
import {
  ATTR_HTTP_REQUEST_PREFETCH,
  applyFetchSpanAttrs,
  isPrefetchRequest,
} from "./request-outcome.js";
import { fetchResponseEvidence } from "./response-evidence.js";
import type { ResponseEvidence } from "./response-discriminators.js";
import {
  composeFetchPatch,
  skipFetchHeaderInjection,
} from "./upstream-request-members.js";

const ATTR_ERROR_TYPE = "error.type" as const;

const ATTR_HTTP_REQUEST_BODY_SIZE = "http.request.body.size" as const;

function spanUrl(span: Span) {
  return spanStringAttribute(span, "url.full");
}

function requestInitMethod(request: FetchSpanRequest) {
  return isPlainRecord(request) && isString(request["method"])
    ? request["method"]
    : undefined;
}

function requestMethod(request: FetchSpanRequest) {
  const method =
    request instanceof Request ? request.method : requestInitMethod(request);

  return (method ?? "GET").toUpperCase();
}

// WARNING: Upstream measureRequestSize clones the Request, which throws into the application when another wrapper already used its body; only sizes known without reading are recorded.
function bodySize(body: BodyInit | null | undefined) {
  if (body instanceof Blob) {
    return body.size;
  }

  if (body instanceof ArrayBuffer || ArrayBuffer.isView(body)) {
    return body.byteLength;
  }

  const text = body instanceof URLSearchParams ? body.toString() : body;

  return isString(text) ? new TextEncoder().encode(text).byteLength : 0;
}

/** The upstream fetch instrumentation with Interfere's header policy, failed-request evidence and correlation attached through its hooks. */
export function createFetchInstrumentation(
  core: RequestCore,
  input: RequestInstrumentationsInput,
) {
  const { applicationErrors, evidenceTracer, failureRanges } = input;
  const evidence = new WeakMap<Response, Promise<ResponseEvidence>>();

  let sending: RequestState | undefined;

  const takeSending = () => {
    const state = sending;

    sending = undefined;

    return state;
  };

  const instrumentation = new FetchInstrumentation({
    enabled: false,
    ignoreNetworkEvents: true,
    ignoreUrls: input.ignoreUrls,
    measureRequestSize: false,
    propagateTraceHeaderCorsUrls: [],
    requestHook: (span, request) => {
      const url = absoluteUrl(
        request instanceof Request ? request.url : (spanUrl(span) ?? ""),
      );

      if (!url) {
        return;
      }

      let headers: Headers;

      if (request instanceof Request) {
        ({ headers } = request);
      } else {
        headers = new Headers(request.headers);

        request.headers = headers;
      }

      sending = core.begin(span, url, headers);

      const size = request instanceof Request ? 0 : bodySize(request.body);

      if (size > 0) {
        span.setAttribute(ATTR_HTTP_REQUEST_BODY_SIZE, size);
      }

      if (isPrefetchRequest(request)) {
        span.setAttribute(ATTR_HTTP_REQUEST_PREFETCH, true);
      } else if (core.callsite && failureRanges && core.isTracked(url)) {
        captureRequestEvidence(span, core.callsite, headers, input.redactUrl);
      }
    },
    applyCustomAttributesOnSpan: (span, request, result) => {
      const state = core.states.get(span);

      if (result instanceof Response) {
        core.correlate(result, span, requestMethod(request));
      } else {
        rememberFailedRequest(result, span.spanContext());

        const aborted = request.signal?.aborted === true;
        const name = result instanceof Error ? result.name : "Error";

        if (!aborted && name !== "AbortError") {
          span.setStatus({ code: SpanStatusCode.ERROR, message: name });

          span.setAttribute(ATTR_ERROR_TYPE, name);
        }

        if (state?.injected) {
          core.diagnostics.failure(state.url, result, aborted, span);
        }
      }

      holdSpanEnd(
        span,
        applyFetchSpanAttrs(
          span,
          request,
          result,
          failureRanges,
          core.pageExit,
          core.isTracked,
          input.resolveRoute,
          core.failureGate,
          result instanceof Response ? evidence.get(result) : undefined,
          applicationErrors,
          evidenceTracer,
        ),
      );
    },
  });

  skipFetchHeaderInjection(instrumentation);

  composeFetchPatch(instrumentation, (patched) =>
    Object.assign(async function interfereFetch(
      this: typeof globalThis,
      ...args: Parameters<typeof globalThis.fetch>
    ) {
      if (!core.active) {
        return patched.apply(this, args);
      }

      // WARNING: The request call site must be constructed in this frame, the one the application called; the recorded stack drops exactly one frame.
      core.callsite = failureRanges
        ? new Error("HTTP request call site")
        : undefined;

      sending = undefined;

      let pending: Promise<Response>;

      try {
        pending = patched.apply(this, args);
      } finally {
        core.callsite = undefined;
      }

      const state = takeSending();

      const [resource, init] = args;

      const response = await pending;

      if (state?.injected) {
        core.diagnostics.success(state.url);
      }

      const url = absoluteUrl(response.url) ?? "";

      if (
        failureRanges !== null &&
        (response.status === 200
          ? applicationErrors
          : statusMatches(response.status, failureRanges)) &&
        core.isTracked(url) &&
        !isPrefetchRequest(init ?? resource)
      ) {
        evidence.set(response, fetchResponseEvidence(response));
      }

      return response;
    }, patched),
  );

  return instrumentation;
}
