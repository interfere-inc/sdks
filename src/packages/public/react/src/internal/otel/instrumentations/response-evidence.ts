import type { Attributes, Span } from "@opentelemetry/api";
import {
  RESPONSE_EVIDENCE_MAX_BYTES,
  responseDiscriminators,
  responseSkipReason,
  skippedResponseEvidence,
} from "@interfere/types/sdk/response/evidence";
import type { ResponseEvidence } from "@interfere/types/sdk/response/evidence";

/** Extracts bounded protocol error identifiers and the capture disposition. */
export function xhrResponseEvidence(xhr: XMLHttpRequest) {
  try {
    const reason = responseSkipReason((name) => xhr.getResponseHeader(name), {
      lengthRequired: true,
    });

    if (reason) {
      return skippedResponseEvidence(reason);
    }

    if (xhr.responseType === "json") {
      return responseDiscriminators(xhr.response, xhr.status);
    }

    if (xhr.responseType !== "" && xhr.responseType !== "text") {
      return skippedResponseEvidence("unsupported-response-type");
    }

    if (
      xhr.responseText.length > RESPONSE_EVIDENCE_MAX_BYTES ||
      new TextEncoder().encode(xhr.responseText).byteLength >
        RESPONSE_EVIDENCE_MAX_BYTES
    ) {
      return skippedResponseEvidence("over-64KiB");
    }

    return responseDiscriminators(JSON.parse(xhr.responseText), xhr.status);
  } catch {
    return skippedResponseEvidence("invalid-json");
  }
}

export function responseEvidenceAttributes(evidence: ResponseEvidence) {
  const attributes: Attributes = {
    "interfere.http.capture_status": evidence.status,
  };

  if (evidence.reason) {
    attributes["interfere.http.capture_skip_reason"] = evidence.reason;
  }

  if (evidence.codes.length > 0) {
    attributes["http.response.error_codes"] = evidence.codes;
  }

  if (evidence.applicationError) {
    attributes["interfere.http.application_error"] = true;
  }

  return attributes;
}

async function settleResponseEvidence(
  pending: Promise<ResponseEvidence>,
  apply: (evidence: ResponseEvidence) => void,
) {
  apply(await pending);
}

/** Applies bounded response evidence to the span and passes it to `record`; a returned promise settles when the span may end. */
export async function recordResponseEvidence(
  span: Span,
  pending: Promise<ResponseEvidence> | undefined,
  record: (attributes: Attributes, evidence: ResponseEvidence) => void,
) {
  const apply = (evidence: ResponseEvidence) => {
    const attributes = responseEvidenceAttributes(evidence);

    span.setAttributes(attributes);

    record(attributes, evidence);
  };

  if (pending) {
    return settleResponseEvidence(pending, apply);
  }

  apply(skippedResponseEvidence("unavailable"));
}
