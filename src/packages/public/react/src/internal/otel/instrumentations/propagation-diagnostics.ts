import type { Span } from "@opentelemetry/api";
import { RandomIdGenerator } from "@opentelemetry/sdk-trace-base";
import { isPlainRecord } from "@interfere/types/sdk/guards/json";

import type {
  HeaderSet,
  OriginPolicy,
  TraceHeaderName,
} from "./header-policy.js";

const ATTR_PROPAGATION_SUSPECT_CORS =
  "interfere.propagation.suspect_cors" as const;

const PROVEN_KEY = "interfere:propagation_proven";

const DENIED_KEY = "interfere:propagation_denied";

const HEADER_NAMES: readonly TraceHeaderName[] = [
  "traceparent",
  "tracestate",
  "baggage",
];

const NONE: HeaderSet = new Set();

const ids = new RandomIdGenerator();

function parse(url: string) {
  try {
    return new URL(url, globalThis.location.href);
  } catch {
    return null;
  }
}

type ProvenMemo = Record<string, TraceHeaderName[]>;

function isHeaderName(value: unknown): value is TraceHeaderName {
  return HEADER_NAMES.some((name) => name === value);
}

function readProven() {
  try {
    const parsed: unknown = JSON.parse(
      sessionStorage.getItem(PROVEN_KEY) ?? "{}",
    );

    const memo = new Map<string, HeaderSet>();

    if (!isPlainRecord(parsed)) {
      return memo;
    }

    for (const [origin, names] of Object.entries(parsed)) {
      if (!Array.isArray(names) || !names.every((name) => isHeaderName(name))) {
        return new Map<string, HeaderSet>();
      }

      memo.set(origin, new Set<TraceHeaderName>(names));
    }

    return memo;
  } catch {
    return new Map<string, HeaderSet>();
  }
}

function readDenied() {
  try {
    const parsed: unknown = JSON.parse(
      sessionStorage.getItem(DENIED_KEY) ?? "[]",
    );

    return new Set(
      Array.isArray(parsed)
        ? parsed.filter((entry): entry is string => typeof entry === "string")
        : [],
    );
  } catch {
    return new Set<string>();
  }
}

function write(key: string, value: ProvenMemo | string[]) {
  try {
    sessionStorage.setItem(key, JSON.stringify(value));
  } catch {
    return;
  }
}

function ordered(names: HeaderSet) {
  return HEADER_NAMES.filter((name) => names.has(name));
}

function intersect(headers: HeaderSet, proof: HeaderSet) {
  return new Set(
    ordered(headers).filter((name) => proof.has(name)),
  ) satisfies HeaderSet;
}

function probeHeaders(names: HeaderSet) {
  const headers = new Headers();

  if (names.has("traceparent")) {
    headers.set(
      "traceparent",
      `00-${ids.generateTraceId()}-${ids.generateSpanId()}-00`,
    );
  }

  if (names.has("tracestate")) {
    headers.set("tracestate", "interfere=probe");
  }

  if (names.has("baggage")) {
    headers.set("baggage", "interfere.probe=1");
  }

  return headers;
}

function candidates(names: HeaderSet) {
  const withoutBaggage = new Set(
    ordered(names).filter((name) => name !== "baggage"),
  ) satisfies HeaderSet;

  return withoutBaggage.size > 0 && withoutBaggage.size < names.size
    ? [names, withoutBaggage]
    : [names];
}

/**
 * Decides per request which trace headers an allowed origin may carry,
 * proves discovered origins safe before any application request carries
 * one, and logs one CORS diagnostic per origin. Explicit origins (page
 * origin, `propagateTo`) always carry their headers. A discovered origin
 * carries none, on any method, until Interfere's own credentialed `HEAD`
 * probe to the path of its first request received a response while
 * carrying that origin's header set, or the set without baggage when the
 * full set was rejected; the origin then carries exactly the proven
 * headers. A rejected probe, or a later header-bearing request to the
 * origin failing at the network layer, marks it header-less for the
 * session. Application requests are never retried. The probe goes through
 * the `fetch` present when this was created, before Interfere patched it,
 * so it is not observed. The memo lives in `sessionStorage` when
 * available and `canPersist` allows it, else in this page.
 */
export function propagationDiagnostics(
  pageOrigin: string,
  canPersist: () => boolean = () => true,
) {
  const fetchOnce = globalThis.fetch.bind(globalThis);
  const persisted = canPersist();
  const proven = persisted ? readProven() : new Map<string, HeaderSet>();
  const denied = persisted ? readDenied() : new Set<string>();
  const discovered = new Set<string>();
  const explicitHeaders = new Map<string, HeaderSet>();
  const probing = new Set<string>();
  const warned = new Set<string>();

  let active = true;

  function prove(origin: string, names: HeaderSet) {
    proven.set(origin, names);

    if (canPersist()) {
      write(
        PROVEN_KEY,
        Object.fromEntries(
          [...proven].map(([key, value]) => [key, ordered(value)]),
        ),
      );
    }
  }

  function deny(origin: string, message: string) {
    if (!denied.has(origin)) {
      denied.add(origin);

      if (canPersist()) {
        write(DENIED_KEY, [...denied]);
      }
    }

    if (!warned.has(origin)) {
      warned.add(origin);

      console.warn(`[interfere] ${message}`);
    }
  }

  async function probe(
    origin: string,
    path: string,
    [candidate, ...rest]: HeaderSet[],
  ) {
    if (!candidate) {
      return;
    }

    try {
      await fetchOnce(`${origin}${path}`, {
        cache: "no-store",
        credentials: "include",
        headers: probeHeaders(candidate),
        method: "HEAD",
        mode: "cors",
        redirect: "manual",
        referrerPolicy: "no-referrer",
      });

      if (active) {
        prove(origin, candidate);
      }
    } catch {
      if (rest.length > 0) {
        await probe(origin, path, rest);

        return;
      }

      if (active) {
        deny(
          origin,
          `${origin} is a backend host configured for this workspace, but a CORS check carrying trace headers failed, so its requests carry none in this browser session and its server traces are not linked to the browser. Allow "traceparent, tracestate, baggage" in ${origin}'s Access-Control-Allow-Headers, or list ${origin} in tracing.propagateTo with the headers it allows.`,
        );
      }
    }
  }

  return {
    allowed(entry: OriginPolicy, url: string) {
      const target = parse(url);

      if (!target || denied.has(target.origin)) {
        return NONE;
      }

      const { origin } = target;

      if (origin === pageOrigin) {
        return entry.headers;
      }

      if (!entry.discovered) {
        explicitHeaders.set(origin, entry.headers);

        return entry.headers;
      }

      discovered.add(origin);

      const proof = proven.get(origin);

      if (proof) {
        return intersect(entry.headers, proof);
      }

      if (active && entry.headers.size > 0 && !probing.has(origin)) {
        probing.add(origin);

        void probe(origin, target.pathname, candidates(entry.headers));
      }

      return NONE;
    },
    reset() {
      warned.clear();
    },
    success(url: string) {
      const origin = parse(url)?.origin;
      const carried = origin ? explicitHeaders.get(origin) : undefined;

      if (active && origin && carried && !proven.has(origin)) {
        prove(origin, carried);
      }
    },
    failure(
      url: string,
      // oxlint-disable-next-line anti-slop/no-unknown-parameters -- Fetch rejects with any JavaScript value; only a TypeError is a network-layer failure.
      error: unknown,
      aborted: boolean,
      span?: Span,
    ) {
      const origin = parse(url)?.origin;

      if (
        !active ||
        !origin ||
        !(error instanceof TypeError) ||
        aborted ||
        origin === pageOrigin
      ) {
        return;
      }

      const carriedHeaders = discovered.has(origin)
        ? (proven.get(origin)?.size ?? 0) > 0
        : !proven.has(origin);

      if (!carriedHeaders) {
        return;
      }

      span?.setAttribute(ATTR_PROPAGATION_SUSPECT_CORS, true);

      deny(
        origin,
        `Requests to ${origin} fail at the network layer while carrying trace headers. If ${origin} does not allow "traceparent, tracestate, baggage" in Access-Control-Allow-Headers, add them there or give ${origin} a narrower headers list in tracing.propagateTo. Interfere did not retry the request and sends no trace headers to ${origin} for the rest of this browser session.`,
      );
    },
    dispose() {
      active = false;
    },
  };
}
