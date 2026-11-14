import type { Span } from "@opentelemetry/api";

import type { HeaderSet, OriginPolicy } from "./header-policy.js";

const PROVEN_KEY = "interfere:propagation_proven";

const DENIED_KEY = "interfere:propagation_denied";

const SAFE_METHODS = new Set(["GET", "HEAD"]);

const NONE: HeaderSet = new Set();

function originOf(url: string) {
  try {
    return new URL(url, globalThis.location.href).origin;
  } catch {
    return null;
  }
}

function readOrigins(key: string) {
  try {
    const parsed: unknown = JSON.parse(sessionStorage.getItem(key) ?? "[]");

    return new Set(
      Array.isArray(parsed)
        ? parsed.filter((entry): entry is string => typeof entry === "string")
        : [],
    );
  } catch {
    return new Set<string>();
  }
}

function writeOrigins(key: string, origins: ReadonlySet<string>) {
  try {
    sessionStorage.setItem(key, JSON.stringify([...origins]));
  } catch {
    return;
  }
}

/**
 * Decides per request whether a discovered origin may carry trace headers
 * yet, remembers which origins answered a header-bearing request and which
 * rejected one, and logs the one CORS diagnostic per origin. Explicit
 * origins (page origin, `propagateTo`) always carry their
 * headers; a discovered origin carries them on GET and HEAD from the
 * start, and on other methods only after one header-bearing GET or HEAD
 * received a response in this browser session, so an unmodified CORS
 * policy never breaks a first POST. An origin whose header-bearing request
 * failed at the network layer before any response was seen is marked
 * header-less for the session; nothing is retried. The memo lives in
 * `sessionStorage` when available, else in this page.
 */
export function propagationDiagnostics(pageOrigin: string) {
  const proven = readOrigins(PROVEN_KEY);
  const denied = readOrigins(DENIED_KEY);
  const warned = new Set<string>();

  let active = true;

  return {
    allowed(entry: OriginPolicy, url: string, method: string) {
      const origin = originOf(url);

      if (!origin || denied.has(origin)) {
        return NONE;
      }

      if (
        !entry.discovered ||
        origin === pageOrigin ||
        proven.has(origin) ||
        SAFE_METHODS.has(method.toUpperCase())
      ) {
        return entry.headers;
      }

      return NONE;
    },
    reset() {
      warned.clear();
    },
    success(url: string) {
      const origin = originOf(url);

      if (active && origin && !proven.has(origin)) {
        proven.add(origin);

        writeOrigins(PROVEN_KEY, proven);
      }
    },
    failure(
      url: string,
      // oxlint-disable-next-line anti-slop/no-unknown-parameters -- Fetch rejects with any JavaScript value; only a TypeError is a network-layer failure.
      error: unknown,
      aborted: boolean,
      span?: Span,
    ) {
      const origin = originOf(url);

      if (
        !active ||
        !origin ||
        !(error instanceof TypeError) ||
        aborted ||
        origin === pageOrigin ||
        proven.has(origin)
      ) {
        return;
      }

      span?.setAttribute("interfere.propagation.suspect_cors", true);

      if (!denied.has(origin)) {
        denied.add(origin);

        writeOrigins(DENIED_KEY, denied);
      }

      if (!warned.has(origin)) {
        warned.add(origin);

        console.warn(
          `[interfere] Requests to ${origin} fail at the network layer while carrying trace headers. If ${origin} does not allow "traceparent, tracestate, baggage" in Access-Control-Allow-Headers, add them there or give ${origin} a narrower headers list in tracing.propagateTo. Interfere did not retry the request and sends no trace headers to ${origin} for the rest of this browser session.`,
        );
      }
    },
    dispose() {
      active = false;
    },
  };
}
