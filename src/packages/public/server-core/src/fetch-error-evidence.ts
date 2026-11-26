import { omitUndefined } from "@interfere/helpers/omit-undefined";
import {
  readJsonResponse,
  responseDiscriminators,
  responseErrorDetail,
  responseSkipReason,
} from "@interfere/types/sdk/response/evidence";
import { SpanKind, trace } from "@opentelemetry/api";
import type { Span } from "@opentelemetry/api";
import { containAsync, reportContainedFailure } from "./contain.js";

export const ERROR_RESPONSE_EVENT = "http.client.error_response";

const SITE = "fetch error evidence";

const INSTALLED = Symbol.for("interfere.fetch-error-evidence");

function isClientSpan(span: Span) {
  return "kind" in span && span.kind === SpanKind.CLIENT;
}

function requestUrl(input: Parameters<typeof fetch>[0], response: Response) {
  if (response.url) {
    return response.url;
  }

  return input instanceof Request ? input.url : new URL(input).href;
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Stream cancellation rejects with whatever the host runtime throws.
function cancelFailed(reason: unknown) {
  reportContainedFailure(SITE, reason);
}

async function errorResponseAttributes(
  input: Parameters<typeof fetch>[0],
  response: Response,
) {
  const reason = responseSkipReason((name) => response.headers.get(name), {
    lengthRequired: false,
  });

  if (reason) {
    return null;
  }

  const read = await readJsonResponse(response, cancelFailed);

  if ("skipped" in read) {
    return null;
  }

  const { codes } = responseDiscriminators(read.body, response.status);
  const detail = responseErrorDetail(read.body);

  if (codes.length === 0 && detail === null) {
    return null;
  }

  return omitUndefined({
    "url.full": requestUrl(input, response),
    "http.response.status_code": response.status,
    "http.response.error_codes": codes.length > 0 ? codes : undefined,
    "http.response.error_detail": detail ?? undefined,
  });
}

/** Wraps `globalThis.fetch` so a JSON 4xx/5xx response records its error codes and a redacted message on the active span before the caller sees it; returns the restore function. */
export function installFetchErrorEvidence() {
  const original = globalThis.fetch;

  if (INSTALLED in original) {
    return () => undefined;
  }

  async function fetchWithErrorEvidence(...args: Parameters<typeof fetch>) {
    const span = trace.getActiveSpan();

    const response = await original(...args);

    if (!span?.isRecording() || response.status < 400) {
      return response;
    }

    const attributes = await containAsync(SITE, async () =>
      errorResponseAttributes(args[0], response),
    );

    if (attributes) {
      if (isClientSpan(span)) {
        span.setAttributes(attributes);
      } else {
        span.addEvent(ERROR_RESPONSE_EVENT, attributes);
      }
    }

    return response;
  }

  const wrapped = Object.assign(fetchWithErrorEvidence, original, {
    [INSTALLED]: true,
  });

  globalThis.fetch = wrapped;

  return () => {
    if (globalThis.fetch === wrapped) {
      globalThis.fetch = original;
    }
  };
}
