import { omitUndefined } from "@interfere/helpers/omit-undefined";
import {
  TRACESTATE_FRESHNESS_KEY,
  isFreshnessStampExpired,
} from "@interfere/types/sdk/trace-freshness";

import { ROOT_CONTEXT, defaultTextMapGetter, trace } from "@opentelemetry/api";

import {
  W3CBaggagePropagator,
  W3CTraceContextPropagator,
} from "@opentelemetry/core";

const TRACEPARENT_META = "traceparent";

const TRACESTATE_META = "tracestate";

const BAGGAGE_META = "baggage";

const INVALID_TRACE_ID = "00000000000000000000000000000000";

const FRESHNESS_STAMP_RE = /^\d+$/u;

const W3C_PROPAGATOR = new W3CTraceContextPropagator();

const W3C_BAGGAGE_PROPAGATOR = new W3CBaggagePropagator();

function readMeta(name: string) {
  if (typeof document === "undefined") {
    return null;
  }

  const el = document.querySelector(`meta[name="${name}"]`);

  return el?.getAttribute("content") ?? null;
}

function readTracestateMeta() {
  if (typeof document === "undefined") {
    return null;
  }

  const contents = [
    ...document.querySelectorAll(`meta[name="${TRACESTATE_META}"]`),
  ].flatMap((el) => {
    const content = el.getAttribute("content");

    return content === null ? [] : [content];
  });

  return (
    contents.find((content) =>
      new RegExp(`(?:^|,)\\s*${TRACESTATE_FRESHNESS_KEY}=`, "u").test(content),
    ) ??
    contents.at(0) ??
    null
  );
}

/**
 * Reads the W3C `traceparent` (and optional `tracestate` / `baggage`) meta
 * tags from the document head and extracts an OTel `Context`. Interfere's
 * server adapters (Next `withInterfere()` with the managed `register()`,
 * Vite `TraceMeta`) inject the server-side request span's context so the
 * browser can correlate the page view with the server render.
 *
 * Extraction and sampling are distinct. The returned context keeps the
 * served `traceparent` flags and the whole `tracestate`, sampled or not:
 * the browser decides nothing about sampling from the meta. The trace root
 * made that decision and the private tracer provider's
 * `ParentBased(AlwaysOn)` sampler follows it, while Interfere's recording
 * (errors, replay, logs, vitals, request observations) never depends on a
 * trace flag.
 *
 * Returns `null` only when:
 *  - called server-side (no `document`)
 *  - no `traceparent` meta tag is present, or it does not parse
 *  - the extracted context has an invalid trace id
 *  - the `interfere` tracestate freshness stamp (`interfere=<unix
 *    seconds>`, written by every Interfere server adapter since 12.x) is
 *    missing, unparseable or older than `TRACEPARENT_MAX_AGE_SECONDS`.
 *    Cached HTML (ISR revalidation, CDN full-page caches) serves one
 *    render's traceparent to every visitor in the cache window, and a
 *    traceparent without a fresh stamp cannot be tied to this visitor's
 *    render, so it is dropped instead of cross-linking cached visitors.
 *    Hosts that own tracing themselves (application mode) never receive
 *    Interfere meta, so no unstamped traceparent is expected here.
 */
export function readPropagationFromDocument() {
  const traceparent = readMeta(TRACEPARENT_META);

  if (!traceparent) {
    return null;
  }

  const carrier = omitUndefined({
    traceparent,
    [TRACESTATE_META]: readTracestateMeta() || undefined,
    [BAGGAGE_META]: readMeta(BAGGAGE_META) || undefined,
  });

  const withTrace = W3C_PROPAGATOR.extract(
    ROOT_CONTEXT,
    carrier,
    defaultTextMapGetter,
  );

  const extracted = W3C_BAGGAGE_PROPAGATOR.extract(
    withTrace,
    carrier,
    defaultTextMapGetter,
  );

  const spanCtx = trace.getSpanContext(extracted);

  if (
    !spanCtx ||
    spanCtx.traceId === INVALID_TRACE_ID ||
    !hasFreshStamp(spanCtx.traceState?.get(TRACESTATE_FRESHNESS_KEY))
  ) {
    return null;
  }

  return extracted;
}

function hasFreshStamp(freshnessStamp: string | undefined) {
  if (!freshnessStamp || !FRESHNESS_STAMP_RE.test(freshnessStamp)) {
    return false;
  }

  return !isFreshnessStampExpired(freshnessStamp, Date.now());
}
