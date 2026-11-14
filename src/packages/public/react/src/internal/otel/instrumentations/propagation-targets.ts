import type {
  HeaderSet,
  OriginPolicy,
  PropagateToEntry,
  PropagationPolicy,
  TraceHeaderName,
  TraceHeadersOption,
} from "./header-policy.js";

const ALL_HEADERS: readonly TraceHeaderName[] = [
  "traceparent",
  "tracestate",
  "baggage",
];

function configuredOrigin(entry: string) {
  if (!/^https?:\/\/[^\s/?#@\\]+$/u.test(entry)) {
    return null;
  }

  try {
    const url = new URL(entry);

    return decodeURIComponent(url.hostname).includes("*") ? null : url.origin;
  } catch {
    return null;
  }
}

function explicit(headers: HeaderSet) {
  return { discovered: false, headers } satisfies OriginPolicy;
}

function headerSet(
  names: readonly TraceHeaderName[],
  writesTraceContext: boolean,
  propagateTraceBaggage: boolean,
) {
  return new Set(
    names.filter((name) =>
      name === "baggage" ? propagateTraceBaggage : writesTraceContext,
    ),
  ) satisfies HeaderSet;
}

/** The policy of the URL's origin, or `undefined` when it is not an allowed origin or cannot be parsed. */
export function originPolicy(policy: PropagationPolicy, url: string) {
  try {
    return policy.get(new URL(url, globalThis.location.href).origin);
  } catch {
    return;
  }
}

/**
 * Builds the exact-origin header policy shared by fetch and XHR. The page
 * origin, `propagateTo` strings and workspace-configured
 * `backendHosts` receive `headers` (every header by default); a
 * `propagateTo` entry with its own `headers` list receives exactly those;
 * `headers: false` writes nothing anywhere. Backend hosts are `discovered`:
 * the request observer proves them safe before non-GET requests carry
 * headers. When a tracing vendor on the page propagates W3C trace context
 * itself (`writesTraceContext` false), `traceparent` and `tracestate` are
 * removed from every set and only baggage remains.
 */
export function propagationTargets(input: {
  backendHosts?: readonly string[];
  headers?: TraceHeadersOption;
  pageOrigin: string;
  propagateTraceBaggage?: boolean;
  propagateTo?: readonly (PropagateToEntry | string)[];
  writesTraceContext?: boolean;
}) {
  const writesTraceContext = input.writesTraceContext ?? true;
  const propagateTraceBaggage = input.propagateTraceBaggage !== false;

  const base = headerSet(
    input.headers === false ? [] : (input.headers ?? ALL_HEADERS),
    writesTraceContext,
    propagateTraceBaggage,
  );

  const origins = new Map<string, OriginPolicy>([
    [input.pageOrigin, explicit(base)],
  ]);

  const seen = new Set<string>();

  for (const entry of input.propagateTo ?? []) {
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- propagateTo accepts a bare origin string or an entry object.
    const raw = typeof entry === "string" ? entry : entry.origin;

    if (seen.has(raw)) {
      continue;
    }

    seen.add(raw);

    const origin = configuredOrigin(raw);

    if (!origin) {
      console.warn(`[interfere] Invalid tracing.propagateTo origin: ${raw}`);

      continue;
    }

    origins.set(
      origin,
      explicit(
        // oxlint-disable-next-line anti-slop/no-runtime-typeof -- A bare origin inherits the SDK-wide header set; an entry carries its own.
        typeof entry === "string" || input.headers === false
          ? base
          : headerSet(entry.headers, writesTraceContext, propagateTraceBaggage),
      ),
    );
  }

  for (const host of input.backendHosts ?? []) {
    const origin = configuredOrigin(
      `https://${host.trim().toLowerCase().replace(/\.$/u, "")}`,
    );

    if (origin && !origins.has(origin)) {
      origins.set(origin, { discovered: true, headers: base });
    }
  }

  return origins satisfies PropagationPolicy;
}
