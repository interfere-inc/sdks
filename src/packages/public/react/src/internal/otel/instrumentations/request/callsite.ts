import type { Attributes, Span } from "@opentelemetry/api";

const requestEvidence = new WeakMap<
  Span,
  { callsite: Error; nextAction: boolean; redactUrl: (url: string) => string }
>();

function applicationFrames(callsite: Error) {
  const lines = (callsite.stack ?? "")
    .split("\n")
    .filter((line) => line.trim() !== "");

  const frames =
    lines[0] === `${callsite.name}: ${callsite.message}`
      ? lines.slice(1)
      : lines;

  return frames.slice(1, 21);
}

export function requestCallsiteAttributes(span: Span) {
  const evidence = requestEvidence.get(span);

  return (
    evidence
      ? {
          "http.request.next_action": evidence.nextAction,
          "exception.stacktrace": applicationFrames(evidence.callsite)
            .join("\n")
            .slice(0, 8192)
            .replaceAll(/https?:\/\/[^\s)]+/gu, evidence.redactUrl),
        }
      : {}
  ) satisfies Attributes;
}

export function recordHttpFailure(
  span: Span,
  status: number,
  method: string,
  url: string,
  mechanism: string,
  attributes: Attributes = {},
) {
  const details: Attributes = {
    ...attributes,
    ...requestCallsiteAttributes(span),
    "http.response.status_code": status,
  };

  requestEvidence.delete(span);

  span.setAttributes(attributes);

  span.addEvent("exception", {
    ...details,
    "exception.message": `HTTP ${status} ${method} ${url}`,
    "exception.type": "HttpResponseError",
    "interfere.exception.handled": false,
    "interfere.exception.kind": "error",
    "interfere.exception.mechanism": mechanism,
  });
}

/**
 * Retains a lazy call site and header presence for a failed request. The
 * call site must be created directly in the wrapper the application called,
 * so its stack holds exactly one SDK frame above the application's.
 */
export function captureRequestEvidence(
  span: Span,
  callsite: Error,
  headers: HeadersInit | undefined,
  redactUrl: (url: string) => string,
) {
  requestEvidence.set(span, {
    callsite,
    nextAction: new Headers(headers).has("next-action"),
    redactUrl,
  });
}

export function transferRequestEvidence(from: Span, to: Span) {
  const evidence = requestEvidence.get(from);

  if (evidence) {
    requestEvidence.set(to, evidence);

    requestEvidence.delete(from);
  }
}
