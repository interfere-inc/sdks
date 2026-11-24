import type { ErrorFilterOptions } from "@interfere/types/sdk/options/filter-options";

import type { LogRecordProcessor } from "@opentelemetry/sdk-logs";
import type { MetricReader } from "@opentelemetry/sdk-metrics";
import type { SpanProcessor } from "@opentelemetry/sdk-trace";

import type { PrivacyOptions } from "../dom/privacy.js";
import type { CaptureFailedRequestsOption } from "../otel/instrumentations/instrumentations.js";
import type {
  PropagateToEntry,
  TraceHeadersOption,
} from "../otel/instrumentations/header-policy.js";

export interface KernelOptions extends ErrorFilterOptions {
  /**
   * Set to `"required"` when this app must collect consent before analytics. Until `consent.set` grants it, the SDK
   * stores and reads nothing on the device and sends errors and request diagnostics under a page-scoped id.
   * Omit to capture everything.
   */
  consent?: "required" | undefined;
  /**
   * The endpoint the SDK posts telemetry to. Defaults to Interfere's
   * endpoint. Set it to a subdomain of your own site that resolves to
   * Interfere via a DNS record (e.g. `"https://in.example.com"`) to
   * survive ad-blockers, or to a regional endpoint. Must be a full URL.
   */
  apiHost?: string;
  /**
   * Override the automatic dev-mode guard. When `undefined`, the SDK
   * auto-detects: it disables itself if `process.env["NODE_ENV"]` is not
   * `"production"` (Node / webpack / Next.js). In environments where
   * `process` does not exist (Vite, CRA, plain browser) the SDK
   * defaults to **enabled** — pass `false` to disable explicitly.
   */
  enabled?: boolean;
  /** Override `globalThis.fetch` for tests / non-browser hosts. */
  fetch?: typeof globalThis.fetch;
  /**
   * Override the OTel `service.name` resource attribute. Defaults to the
   * name the build stamped into the page (the Vite plugin derives it from
   * the package name), then to `"unknown_service"`. Customers running
   * multiple frontends in one monitoring backend (e.g. `"website"` vs
   * `"app"`) set this so spans/metrics/logs slice cleanly by surface.
   */
  serviceName?: string;
  /** Optional application namespace for grouping related services. */
  serviceNamespace?: string;
  /**
   * What the SDK never records. `blockSelector` and `maskTextSelector` apply
   * to session replay and to click, submit and rage-click records from the
   * first capture; the `interfere-block` and `interfere-mask` classes always
   * apply. `urls` controls query and fragment redaction; URL credentials are
   * always redacted.
   */
  privacy?: PrivacyOptions;
  /**
   * `false` skips the OTel module entirely — the bundler code-splits
   * `internal/otel/*` out of this path so the error-only bundle never
   * imports the OTel SDK or the OTLP exporters, and exceptions are not
   * exported (the capture-only transport is tracked separately). Omitted
   * (default) loads the OTel module; an object supplies its config.
   */
  tracing?: false | TracingOptions;
}

/**
 * Per-instrumentation tuning. Every field is optional; defaults match
 * what `interfere/homepage` and `interfere/dashboard` use today via
 * the internal `observability/browser/rum.ts`. New customers don't
 * need to touch any of these — sensible defaults ship out of the box.
 */
export interface TracingOptions {
  /** Who writes `traceparent`: unset detects Datadog RUM, Sentry and other OpenTelemetry setups per request and leaves it to them; `"external"` leaves it to another tool everywhere; `"interfere"` always writes it where absent. Baggage is controlled separately. */
  propagation?: "interfere" | "external";
  /**
   * Additional OpenTelemetry processors, in array order after built-in enrichment
   * and before wrapper processors and export. Complete redaction synchronously
   * in onEnd. The SDK owns forceFlush and shutdown for these instances.
   */
  spanProcessors?: SpanProcessor[];
  /**
   * Exact HTTP(S) origins allowed to receive trace headers, in addition to
   * the page origin and configured backend hosts. A string receives the
   * `headers` set; `{ origin, headers }` receives exactly the listed
   * headers, for an API that allows only some of them in
   * `Access-Control-Allow-Headers`.
   */
  propagateTo?: (PropagateToEntry | string)[];
  /**
   * Which headers Interfere writes to allowed origins. Every one by default
   * (`traceparent`, `tracestate`, `baggage`); a list restricts the page
   * origin, configured backend hosts and bare `propagateTo` entries to
   * those names; `false` writes no header anywhere, page origin included,
   * while requests are still observed. Propagation ownership and baggage
   * settings further restrict writes. A `propagateTo` entry with its own
   * list keeps it under a restricting list but not under `false`.
   */
  headers?: TraceHeadersOption;
  /** Defaults to true. False disables Interfere baggage injection everywhere without changing trace context, browser identity attributes, or caller headers. */
  propagateTraceBaggage?: boolean;
  /**
   * Promote failed `fetch` / `XHR` responses to exception events on the
   * active span. **On by default** — captures same-origin (and
   * `propagateTo`) 4xx + 5xx, deduped by `method+status+path` and
   * rate-limited so a flapping endpoint can't flood. Pass `false` to
   * disable, or `{ statusCodes }` to tune the ranges.
   * See {@link CaptureFailedRequestsOption}.
   */
  captureFailedRequests?: CaptureFailedRequestsOption;
  /**
   * Additional URL patterns to skip when creating fetch / XHR spans.
   * The SDK's own collector endpoint is always ignored; customer-side
   * third-party telemetry endpoints are also skipped by
   * default — supplement with anything specific to your app.
   */
  ignoreUrls?: (string | RegExp)[];
  /**
   * Resolve a URL pathname to a low-cardinality route template
   * (e.g. `/blog/[slug]`). Without this, dynamic routes produce one
   * unique span / metric label per visited slug — a cardinality
   * hazard. Used by fetch, document-load resource enrichment,
   * user-interaction, long-task, and web-vitals.
   */
  resolveRoute?: (pathname: string) => string | undefined;
  /**
   * `false` disables Core Web Vitals capture (LCP/FCP/TTFB/INP/CLS).
   * Defaults to enabled; customers who run their own vitals reporting
   * can opt out.
   */
  webVitals?: boolean;
}

export interface KernelInternalOptions extends KernelOptions {
  _internalAdditionalLogRecordProcessors?: LogRecordProcessor[];
  _internalAdditionalMetricReaders?: MetricReader[];
  _internalAdditionalSpanProcessors?: SpanProcessor[];
  /** Wrapper SDK versions (e.g. `@interfere/next@10.0.0`). */
  _wrapperVersions?: string[];
}
