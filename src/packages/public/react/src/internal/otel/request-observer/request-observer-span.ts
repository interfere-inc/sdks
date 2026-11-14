import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { SpanKind, SpanStatusCode } from "@opentelemetry/api";
import type { Attributes, Context, Span, Tracer } from "@opentelemetry/api";

import { ATTR_URL_FULL } from "../url-attrs.js";

const DEFAULT_PORTS = new Map([
  ["http:", 80],
  ["https:", 443],
]);

function serverAttributes(url: string) {
  let parsed: URL;

  try {
    parsed = new URL(url);
  } catch {
    return {} satisfies Attributes;
  }

  return omitUndefined({
    "server.address": parsed.hostname,
    "server.port": parsed.port
      ? Number(parsed.port)
      : DEFAULT_PORTS.get(parsed.protocol),
  }) satisfies Attributes;
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Request bodies are an open union of host types; only sizes known without reading a stream are recorded.
function bodySize(body: unknown) {
  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- String bodies are measured as their UTF-8 encoding.
  if (typeof body === "string") {
    return new TextEncoder().encode(body).byteLength;
  }

  if (body instanceof URLSearchParams) {
    return new TextEncoder().encode(body.toString()).byteLength;
  }

  if (body instanceof Blob) {
    return body.size;
  }

  if (body instanceof ArrayBuffer || ArrayBuffer.isView(body)) {
    return body.byteLength;
  }

  return 0;
}

/** Resolves a request URL against the page, or `null` when it cannot be parsed. */
export function absoluteUrl(url: string | URL) {
  try {
    return new URL(url, globalThis.location.href).href;
  } catch {
    return null;
  }
}

/** Starts an HTTP client span named by its upper-cased method, with the request's URL, server and body-size attributes. */
export function startRequestSpan(
  tracer: Tracer,
  request: { body?: unknown; method: string; url: string },
  ctx: Context,
) {
  const method = request.method.toUpperCase();
  const size = bodySize(request.body);

  return tracer.startSpan(
    method,
    {
      attributes: omitUndefined({
        "http.request.method": method,
        "http.request.body.size": size > 0 ? size : undefined,
        [ATTR_URL_FULL]: request.url,
        ...serverAttributes(request.url),
      }),
      kind: SpanKind.CLIENT,
    },
    ctx,
  );
}

/** Records a received status, marking 4xx and 5xx as errors; a zero status means no response and records nothing. */
export function setResponseStatus(span: Span, status: number) {
  if (status <= 0) {
    return;
  }

  span.setAttribute("http.response.status_code", status);

  if (status >= 400) {
    span.setStatus({ code: SpanStatusCode.ERROR });

    span.setAttribute("error.type", String(status));
  }
}

/** Ends the span once the outcome still being recorded on it settles. */
export async function endWhenSettled(span: Span, pending: Promise<void>) {
  try {
    await pending;
  } finally {
    span.end();
  }
}

/** Marks a request that failed without a usable response. */
export function markRequestError(span: Span, errorType: string) {
  span.setStatus({ code: SpanStatusCode.ERROR, message: errorType });

  span.setAttribute("error.type", errorType);
}
