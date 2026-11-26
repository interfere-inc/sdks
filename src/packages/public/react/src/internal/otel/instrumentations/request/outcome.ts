/* oxlint-disable typescript/no-unnecessary-condition */
import { isString } from "@interfere/types/sdk/guards/json";
import { MECHANISM_TYPE } from "@interfere/types/sdk/errors";
import type { Span, Tracer } from "@opentelemetry/api";
import type { ReadableSpan } from "@opentelemetry/sdk-trace";

import { stampHttpSpanRoute } from "../../http-span-route.js";
import { ATTR_URL_FULL, ATTR_URL_PATH } from "../../url-attrs.js";
import { statusMatches } from "../failure-policy.js";
import type { HttpStatusCodeRange } from "../failure-policy.js";
import { settleFetchFailure, settleXhrFailure } from "./network-failure.js";
import type {
  FetchError,
  FetchSpanRequest,
  PageExitWatch,
} from "./network-failure-types.js";
import { recordHttpFailure, transferRequestEvidence } from "./callsite.js";
import { requestEvidenceSpan } from "./evidence-span.js";
import type { ResponseEvidence } from "@interfere/types/sdk/response/evidence";
import {
  recordResponseEvidence,
  responseEvidenceAttributes,
  xhrResponseEvidence,
} from "../response-evidence.js";

async function recordApplicationError(
  original: Span,
  pending: Promise<ResponseEvidence>,
  createSpan: () => Span,
  method: string,
  url: string,
  admit: () => boolean,
) {
  const evidence = await pending;

  if (!(evidence.applicationError && admit())) {
    return;
  }

  const span = createSpan();

  transferRequestEvidence(original, span);

  recordHttpFailure(
    span,
    200,
    method,
    url,
    MECHANISM_TYPE.browser.fetchFailedResponse,
    responseEvidenceAttributes(evidence),
  );

  span.end();
}

async function captureFetchResponse(
  span: Span,
  result: Response,
  failureRanges: readonly HttpStatusCodeRange[] | null,
  method: string,
  url: string,
  admit: () => boolean,
  pending?: Promise<ResponseEvidence>,
  applicationErrors = false,
  createSpan?: () => Span,
) {
  if (!failureRanges) {
    return;
  }

  if (result.status === 200) {
    if (applicationErrors && pending && createSpan) {
      void recordApplicationError(
        span,
        pending,
        createSpan,
        method,
        url,
        admit,
      );
    }

    return;
  }

  if (!statusMatches(result.status, failureRanges)) {
    return;
  }

  return recordResponseEvidence(span, pending, (attributes) => {
    if (admit()) {
      recordHttpFailure(
        span,
        result.status,
        method,
        url,
        MECHANISM_TYPE.browser.fetchFailedResponse,
        attributes,
      );
    }
  });
}

const FAILURE_DEDUP_CAP = 100;

const FAILURE_RATE_MAX = 20;

const FAILURE_RATE_WINDOW_MS = 60_000;

export function createFailureGate() {
  const seen = new Set<string>();

  let windowStart = 0;
  let windowCount = 0;

  return ((key: string) => {
    if (seen.has(key)) {
      return false;
    }

    const now = Date.now();

    if (now - windowStart > FAILURE_RATE_WINDOW_MS) {
      windowStart = now;

      windowCount = 0;
    }

    if (windowCount >= FAILURE_RATE_MAX) {
      return false;
    }

    if (seen.size >= FAILURE_DEDUP_CAP) {
      seen.clear();
    }

    seen.add(key);

    windowCount += 1;

    return true;
  }) satisfies (key: string) => boolean;
}

function failureKey(
  method: string,
  discriminator: string | number,
  url: string,
) {
  let path: string;

  try {
    path = new URL(url).pathname;
  } catch {
    path = url;
  }

  return `${method} ${discriminator} ${path}`;
}

export async function applyFetchSpanAttrs(
  span: Span,
  request: FetchSpanRequest,
  result: Response | FetchError,
  failureRanges: readonly HttpStatusCodeRange[] | null,
  pageExit: PageExitWatch,
  isTrackedUrl: (url: string) => boolean,
  resolveRoute: ((pathname: string) => string | undefined) | undefined,
  gate?: (key: string) => boolean,
  codes?: Promise<ResponseEvidence>,
  applicationErrors = false,
  evidenceTracer?: Tracer,
) {
  const url = resolveFetchUrl(span, request, result);

  if (!url) {
    return;
  }

  let parsed: URL;

  try {
    parsed = new URL(url);
  } catch {
    return;
  }

  span.setAttribute(ATTR_URL_PATH, parsed.pathname);

  const tracked = isTrackedUrl(url);
  const method = resolveFetchMethod(request);

  if (tracked) {
    stampHttpSpanRoute(span, method, parsed.pathname, resolveRoute);

    if (isPrefetchRequest(request)) {
      return;
    }

    const createSpan =
      evidenceTracer &&
      failureRanges &&
      (applicationErrors || !(result instanceof Response))
        ? requestEvidenceSpan(span, evidenceTracer, method, url, resolveRoute)
        : undefined;

    if (result instanceof Response) {
      return captureFetchResponse(
        span,
        result,
        failureRanges,
        method,
        url,
        () => !gate || gate(failureKey(method, result.status, url)),
        codes,
        applicationErrors,
        createSpan,
      );
    }

    if (failureRanges) {
      return settleFetchFailure(
        span,
        request,
        result,
        pageExit,
        method,
        url,
        () => !gate || gate(failureKey(method, "network", url)),
        createSpan,
      );
    }
  }
}

export async function applyXhrSpanAttrs(
  span: Span,
  xhr: XMLHttpRequest,
  failureRanges: readonly HttpStatusCodeRange[] | null,
  pageExit: PageExitWatch,
  isTrackedUrl: (url: string) => boolean,
  gate?: (key: string) => boolean,
  resolveRoute?: (pathname: string) => string | undefined,
  applicationErrors = false,
) {
  const url = xhr.responseURL || spanUrlFull(span);

  if (!(url && isTrackedUrl(url))) {
    return;
  }

  let parsed: URL;

  try {
    parsed = new URL(url);
  } catch {
    return;
  }

  const recordedMethod = isReadableSpan(span)
    ? (span.attributes["http.request.method"] ?? span.attributes["http.method"])
    : undefined;

  const method = isString(recordedMethod) ? recordedMethod : "XHR";

  span.setAttribute(ATTR_URL_PATH, parsed.pathname);

  stampHttpSpanRoute(span, method, parsed.pathname, resolveRoute);

  if (!failureRanges) {
    return;
  }

  const { status } = xhr;

  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Instrumented host XHR implementations may expose no numeric status before a response is available.
  if (typeof status !== "number") {
    return;
  }

  if (status === 0) {
    return settleXhrFailure(
      span,
      pageExit,
      url,
      () => !gate || gate(failureKey("XHR", "network", url)),
    );
  }

  if (
    status === 200 ? !applicationErrors : !statusMatches(status, failureRanges)
  ) {
    return;
  }

  const evidence = xhrResponseEvidence(xhr);
  const attributes = responseEvidenceAttributes(evidence);

  if (status === 200 && !evidence.applicationError) {
    return;
  }

  if (gate && !gate(failureKey("XHR", status, url))) {
    return;
  }

  recordHttpFailure(
    span,
    status,
    "XHR",
    url,
    MECHANISM_TYPE.browser.xhrFailedResponse,
    attributes,
  );
}

function resolveFetchUrl(
  span: Span,
  request: FetchSpanRequest,
  result: Response | FetchError,
) {
  return (
    (result instanceof Response && result.url ? result.url : null) ??
    requestUrl(request) ??
    spanUrlFull(span)
  );
}

function isReadableSpan(span: Span): span is Span & ReadableSpan {
  return "attributes" in span;
}

function spanUrlFull(span: Span) {
  if (!isReadableSpan(span)) {
    return null;
  }

  const url = span.attributes[ATTR_URL_FULL] ?? span.attributes["http.url"];

  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- OTel attribute values also include numbers, booleans and arrays, which are not request URLs.
  return (typeof url === "string" ? url : null) satisfies string | null;
}

function requestUrl(request: FetchSpanRequest) {
  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Fetch accepts URL strings as well as Request, URL and request-options objects.
  if (typeof request === "string") {
    return request;
  }

  return readStringProp(request, "url") ?? readStringProp(request, "href");
}

function resolveFetchMethod(request: FetchSpanRequest) {
  return readStringProp(request, "method") ?? "GET";
}

function readStringProp(value: FetchSpanRequest, key: string) {
  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- The fetch input union includes strings; property lookup is only valid for its object variants.
  if (!(value !== null && typeof value === "object" && key in value)) {
    return null;
  }

  // oxlint-disable-next-line anti-slop/no-reflect-get -- Fetch input variants expose different inherited getters, read by the requested URL or method field without assuming one host class.
  const prop: unknown = Reflect.get(value, key);

  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Only string-valued host URL and method properties are valid request metadata.
  return (typeof prop === "string" ? prop : null) satisfies string | null;
}

export const ATTR_HTTP_REQUEST_PREFETCH = "http.request.prefetch" as const;

const PREFETCH_HEADER_NAMES = [
  "next-router-prefetch",
  "next-router-segment-prefetch",
] as const;

export function isPrefetchRequest(request: FetchSpanRequest) {
  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- A bare URL string carries no request headers, so it can never be a router prefetch.
  if (typeof request === "string" || request instanceof URL) {
    return false;
  }

  let headers: Headers;

  try {
    headers =
      typeof Request !== "undefined" && request instanceof Request
        ? request.headers
        : new Headers(request.headers);
  } catch {
    return false;
  }

  return (
    PREFETCH_HEADER_NAMES.some((name) => headers.has(name)) ||
    headers.get("purpose") === "prefetch" ||
    headers.get("sec-purpose")?.includes("prefetch") === true
  );
}
