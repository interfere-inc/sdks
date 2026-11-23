import { MECHANISM_TYPE } from "@interfere/types/sdk/errors";
import type { ExceptionValue } from "@interfere/types/sdk/plugins/payload/errors";

import type { Attributes, Span } from "@opentelemetry/api";
import type {
  FetchError,
  FetchSpanRequest,
  PageExitWatch,
} from "./network-failure-types.js";
import type { ReadableSpan } from "@opentelemetry/sdk-trace";

import {
  ATTR_HTTP_CANCELLATION_INITIATOR,
  bindRequestRecovery,
  cancellationEvidence,
} from "./request-cancellation.js";

import { transportContextAttrs } from "../../capture/transport-context.js";
import { safeStringify } from "../../../util/stringify.js";
import {
  requestCallsiteAttributes,
  transferRequestEvidence,
} from "./request-callsite.js";

type RequestCancellation = "abort" | "timeout";

const ATTR_HTTP_CANCELLED = "interfere.http.cancelled" as const;

const TRANSPORT_FAILURE_HOLD_MS = 1000;

export function createPageExitWatch() {
  let exitedAt = Number.NEGATIVE_INFINITY;
  let pageHiddenAt = Number.NEGATIVE_INFINITY;

  const onPageHide = () => {
    exitedAt = Date.now();

    pageHiddenAt = exitedAt;
  };

  const onVisibilityChange = () => {
    if (document.visibilityState === "hidden") {
      exitedAt = Date.now();
    }
  };

  globalThis.addEventListener("pagehide", onPageHide);

  document.addEventListener("visibilitychange", onVisibilityChange);

  return {
    dispose: () => {
      globalThis.removeEventListener("pagehide", onPageHide);

      document.removeEventListener("visibilitychange", onVisibilityChange);
    },
    initiatorSince: (time: number) =>
      pageHiddenAt >= time ? "unload" : undefined,
    leftSince: (time: number) =>
      exitedAt >= time || document.visibilityState === "hidden",
  } satisfies PageExitWatch;
}

function recordNetworkFailure(
  span: Span,
  method: string,
  url: string,
  mechanism: string,
  cause: Error | undefined,
  cancellation?: RequestCancellation,
  evidence: Attributes = {},
) {
  const message = `Network ${cancellation ?? "network"} ${method} ${url}`;
  const callsite = requestCallsiteAttributes(span);

  const chain: ExceptionValue[] = [
    {
      kind: "error",
      mechanism: { handled: false, type: mechanism },
      stack: callsite["exception.stacktrace"] ?? "",
      type: "NetworkError",
      value: message,
    },
  ];

  if (cause) {
    chain.push({
      kind: "error",
      stack: "",
      type: cause.name,
      value: cause.message,
    });
  }

  span.setAttributes(evidence);

  span.addEvent("exception", {
    ...evidence,
    ...callsite,
    ...transportContextAttrs(),
    "exception.message": message,
    "exception.type": "NetworkError",
    "interfere.exception.chain": safeStringify(chain),
    "interfere.exception.handled": false,
    "interfere.exception.kind": "error",
    "interfere.exception.mechanism": mechanism,
    "interfere.exception.subtype": "network",
  });
}

async function settleAfterTransportHold(
  span: Span,
  failedAt: number,
  pageExit: PageExitWatch,
  record: () => void,
) {
  await new Promise<void>((resolve) => {
    setTimeout(
      () => {
        const initiator = pageExit.initiatorSince?.(failedAt);

        if (initiator) {
          span.setAttribute(ATTR_HTTP_CANCELLATION_INITIATOR, initiator);
        } else if (!pageExit.leftSince(failedAt)) {
          record();
        }

        resolve();
      },
      Math.max(0, failedAt + TRANSPORT_FAILURE_HOLD_MS - Date.now()),
    );
  });
}

function requestSignal(request: FetchSpanRequest) {
  if (request instanceof Request) {
    return request.signal;
  }

  if (!(request instanceof Object && "signal" in request)) {
    return null;
  }

  return request.signal instanceof AbortSignal ? request.signal : null;
}

function fetchCancellation(request: FetchSpanRequest, error: FetchError) {
  const signal = requestSignal(request);
  const name = error instanceof Error ? error.name : undefined;

  if (
    name === "TimeoutError" ||
    (signal?.aborted &&
      signal.reason instanceof Error &&
      signal.reason.name === "TimeoutError")
  ) {
    return "timeout" satisfies RequestCancellation;
  }

  if (name === "AbortError" || signal?.aborted) {
    return "abort" satisfies RequestCancellation;
  }

  return null;
}

function isReadableSpan(span: Span): span is Span & ReadableSpan {
  return "events" in span;
}

function xhrCancellation(span: Span) {
  if (!isReadableSpan(span)) {
    return null;
  }

  const error = span.attributes["error.type"];

  if (error === "TimeoutError") {
    return "timeout" satisfies RequestCancellation;
  }

  if (
    error === undefined &&
    span.attributes["http.response.status_code"] === undefined
  ) {
    return "abort" satisfies RequestCancellation;
  }

  return null;
}

/** Marks a cancelled fetch, or records its transport failure unless the page leaves within the hold; the returned promise settles when the span may end. */
export async function settleFetchFailure(
  span: Span,
  request: FetchSpanRequest,
  error: FetchError,
  pageExit: PageExitWatch,
  method: string,
  url: string,
  admit: () => boolean,
  createSpan?: () => Span,
) {
  const cancellation = fetchCancellation(request, error);

  if (cancellation) {
    span.setAttribute(ATTR_HTTP_CANCELLED, cancellation);

    const reason: unknown = requestSignal(request)?.reason;
    const evidence = cancellationEvidence(reason);

    if (evidence && admit()) {
      recordNetworkFailure(
        span,
        method,
        url,
        MECHANISM_TYPE.browser.fetchNetworkError,
        error instanceof Error ? error : undefined,
        cancellation,
        evidence,
      );

      if (createSpan) {
        bindRequestRecovery(reason, () => {
          const recovered = createSpan();

          transferRequestEvidence(span, recovered);

          recordNetworkFailure(
            recovered,
            method,
            url,
            MECHANISM_TYPE.browser.fetchNetworkError,
            undefined,
            cancellation,
            { ...evidence, "interfere.http.request_outcome": "recovered" },
          );

          recovered.end();
        });
      }
    }

    return;
  }

  if (!(error instanceof Error)) {
    return;
  }

  return settleAfterTransportHold(span, Date.now(), pageExit, () => {
    if (admit()) {
      recordNetworkFailure(
        span,
        method,
        url,
        MECHANISM_TYPE.browser.fetchNetworkError,
        error,
      );
    }
  });
}

/** Marks an aborted or timed-out XHR from its terminal event, or records its transport failure unless the page leaves within the hold. */
export async function settleXhrFailure(
  span: Span,
  pageExit: PageExitWatch,
  url: string,
  admit: () => boolean,
) {
  const cancellation = xhrCancellation(span);

  if (cancellation) {
    span.setAttribute(ATTR_HTTP_CANCELLED, cancellation);

    return;
  }

  return settleAfterTransportHold(span, Date.now(), pageExit, () => {
    if (admit()) {
      recordNetworkFailure(
        span,
        "XHR",
        url,
        MECHANISM_TYPE.browser.xhrNetworkError,
        undefined,
      );
    }
  });
}
