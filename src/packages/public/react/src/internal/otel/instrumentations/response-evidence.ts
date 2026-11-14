import type { Attributes, Span } from "@opentelemetry/api";
import {
  responseDiscriminators,
  skippedResponseEvidence,
} from "./response-discriminators.js";
import type { ResponseEvidence } from "./response-discriminators.js";

const MAX_BYTES = 64 * 1024;

function skipReason(getHeader: (name: string) => string | null) {
  const length = getHeader("content-length");
  const type = getHeader("content-type")?.split(";", 1)[0]?.trim();

  if (length === null || !/^\d+$/u.test(length)) {
    return "no-content-length";
  }

  if (Number(length) > MAX_BYTES) {
    return "over-64KiB";
  }

  if (!(type === "application/json" || type?.endsWith("+json"))) {
    return "non-json";
  }

  return null;
}

async function readResponse(response: Response) {
  let clone: Response;

  try {
    clone = response.clone();
  } catch {
    return skippedResponseEvidence("unreadable");
  }

  const reader = clone.body?.getReader();

  if (!reader) {
    return skippedResponseEvidence("empty-body");
  }

  let timer: ReturnType<typeof setTimeout> | undefined;

  const expired = new Promise<undefined>((resolve) => {
    timer = setTimeout(() => resolve(undefined), 1000);
  });

  const decoder = new TextDecoder();
  let bytes = 0;
  let text = "";

  try {
    while (true) {
      // oxlint-disable-next-line eslint/no-await-in-loop -- Read each bounded stream chunk before requesting another.
      const chunk = await Promise.race([reader.read(), expired]);

      if (!chunk) {
        return skippedResponseEvidence("timeout");
      }

      if (chunk.done) {
        break;
      }

      bytes += chunk.value.byteLength;

      if (bytes > MAX_BYTES) {
        return skippedResponseEvidence("over-64KiB");
      }

      text += decoder.decode(chunk.value, { stream: true });
    }

    return responseDiscriminators(
      JSON.parse(text + decoder.decode()),
      response.status,
    );
  } catch {
    return skippedResponseEvidence("invalid-json");
  } finally {
    clearTimeout(timer);

    // oxlint-disable-next-line promise/prefer-await-to-then -- Tee cancellation can wait for the app to consume its branch; never delay telemetry settlement on it.
    void reader.cancel().catch(() => undefined);
  }
}

/** Collects bounded response identifiers from a clone, never consuming the application's body. */
export async function fetchResponseEvidence(response: Response) {
  const reason = skipReason((name) => response.headers.get(name));

  return reason ? skippedResponseEvidence(reason) : readResponse(response);
}

/** Extracts bounded protocol error identifiers and the capture disposition. */
export function xhrResponseEvidence(xhr: XMLHttpRequest) {
  try {
    const reason = skipReason((name) => xhr.getResponseHeader(name));

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
      xhr.responseText.length > MAX_BYTES ||
      new TextEncoder().encode(xhr.responseText).byteLength > MAX_BYTES
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
