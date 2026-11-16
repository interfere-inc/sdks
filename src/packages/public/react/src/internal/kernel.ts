import { clearExportCarryover } from "./otel/export-carryover.js";
import { omitUndefined } from "@interfere/helpers/omit-undefined";
import type { PageSessionId, SessionId } from "@interfere/types/data/session";
import { releaseSlugSchema } from "@interfere/types/releases/slug";
import {
  isNonErrorException,
  shouldDropBrowserExtensionNoise,
  shouldDropUnresolvableStack,
} from "@interfere/types/sdk/errors";
import type { NonErrorException } from "@interfere/types/sdk/errors";
import type { IdentifyParams } from "@interfere/types/sdk/identify";
import type { ErrorFilterOptions } from "@interfere/types/sdk/options/filter-options";
import {
  FORCE_ENABLE_GLOBAL,
  RELEASE_SLUG_GLOBAL,
  SDK_STACK_GLOBAL,
} from "@interfere/types/sdk/injected-globals";
import type {
  ConsentDecision,
  ConsentState,
  PluginKey,
} from "@interfere/types/sdk/plugins/manifest";
import { readBakedPlugins } from "@interfere/types/sdk/remote-config";
import type { AnonymousReplayPolicy } from "@interfere/types/sdk/remote-config/anonymous-replay";
import { SERVICE_NAME_GLOBAL } from "@interfere/types/sdk/service-name";
import {
  redactUrl,
  resolveUrlRedaction,
} from "@interfere/types/sdk/url-redaction";
import type { UrlRedactionPolicy } from "@interfere/types/sdk/url-redaction";

import { context, trace } from "@opentelemetry/api";
import type { SeverityNumber } from "@opentelemetry/api-logs";

import type { LogRecordProcessor } from "@opentelemetry/sdk-logs";
import type { MetricReader } from "@opentelemetry/sdk-metrics";
import type { SpanProcessor } from "@opentelemetry/sdk-trace-base";

import type { PluginOverrides } from "../plugins/lib/loader.js";
import type {
  PluginCleanup,
  PluginConfigState,
  PluginContext,
} from "../plugins/lib/types.js";

import { SessionTracker } from "../tracking/api.js";
import { sessionStartRecord } from "../tracking/session.js";
import { ViewIds, identityAttributes } from "../tracking/view.js";
import { getGlobal, getStringGlobal } from "../util/global.js";
import { createLogger } from "../util/log.js";
import type {
  SessionPauseReason,
  SessionResumeReason,
} from "./activity/activity.js";
import { resolveTargets } from "./config/config.js";
import { resolveRemoteConfig } from "./config/remote-config.js";
import { resolvePrivacyPolicy } from "./dom/privacy.js";
import type { PrivacyOptions, PrivacyPolicy } from "./dom/privacy.js";
import type {
  BundledInstrumentations,
  CaptureFailedRequestsOption,
  OtelProviderHandle,
} from "./otel/index.js";
import { installViewCorrelation } from "./otel/correlation.js";
import { installViewNavigation } from "./otel/navigation.js";
import type { ResolvedTracingOptions } from "./otel/wire.js";
import type {
  PropagateToEntry,
  TraceHeadersOption,
} from "./otel/instrumentations/header-policy.js";
import {
  onPageHidden,
  onPageUnloading,
} from "./page-lifecycle/page-lifecycle.js";
import {
  buildExceptionEventAttrs,
  recordExceptionOnSpan,
} from "./capture/exception-span.js";
import type { RecordExceptionOpts } from "./capture/exception-span.js";
import { filterException } from "./capture/error-filters.js";
import { createOtelSink } from "./capture/otel-sink.js";
import { StartupExceptionBuffer } from "./capture/startup-buffer.js";
import { createTurnDedupe, errorFingerprint } from "./capture/turn-dedupe.js";
import type { PendingException } from "./capture/startup-buffer.js";
import { createCaptureOnlySink } from "./capture/capture-only-sink.js";
import type { CaptureOnlySinkInput } from "./capture/capture-only-sink.js";
import { buildResourceAttributes } from "./capture/resource-attributes.js";
import type { TelemetrySink } from "./capture/sink.js";
import { PluginRuntime } from "./plugins/plugin-runtime.js";
import { PRODUCER_VERSION } from "./version.js";

export type {
  CaptureFailedRequestsOption,
  CaptureFailedRequestsOptions,
  HttpStatusCodeRange,
} from "./otel/index.js";

export type { PrivacyOptions } from "./dom/privacy.js";

export type { RecordExceptionOpts } from "./capture/exception-span.js";

const log = createLogger("kernel");

export function buildSdkStack(wrapperVersions?: string[]) {
  return [...(wrapperVersions ?? []), PRODUCER_VERSION] satisfies string[];
}

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

export interface KernelConsent {
  get: () => ConsentState;
  /** Records the visitor's decision: `true` grants analytics and replay, `false` denies both, `{ replay: false }` grants analytics only. */
  set: (decision: ConsentDecision) => void;
}

export interface KernelIdentity {
  clear: () => void;
  get: () => IdentifyParams | null;
  set: (params: IdentifyParams) => Promise<void>;
}

export interface KernelDevice {
  getDeviceId: () => string | null;
  getFpHash: () => string | null;
}

export interface KernelSession {
  getId: () => PageSessionId | SessionId | null;
  getWindowId: () => string | null;
}

/**
 * The replacement for the old `Client` god-object. Pure construction graph
 * via `createKernel`; no module-level mutable state, no global lookups.
 * Framework wrappers (`@interfere/next`, `@interfere/vite`) own the
 * singleton lifecycle.
 *
 * Implements `PluginContext` directly: plugins are wired with the kernel
 * itself rather than a closure adapter. The kernel⇄runtime cycle is
 * resolved by constructing the kernel first and binding the runtime via
 * `attachRuntime` immediately after.
 */
type OtelControls = Partial<
  Pick<BundledInstrumentations, "setPageEventsEnabled">
> & {
  recordCurrentView: () => void;
};

export class Kernel implements PluginContext {
  readonly consent: KernelConsent;
  readonly identity: KernelIdentity;
  readonly device: KernelDevice;
  readonly session: KernelSession;
  readonly privacyPolicy: PrivacyPolicy;
  readonly serviceIdentity: NonNullable<PluginContext["serviceIdentity"]>;
  readonly urlRedaction: UrlRedactionPolicy;
  readonly redactUrl = (url: string) => redactUrl(url, this.urlRedaction);
  readonly views = new ViewIds();

  remoteProtocolVersion: number | null = null;

  anonymousReplayPolicy: AnonymousReplayPolicy | null = null;
  readonly getAnonymousReplayPolicy = () => this.anonymousReplayPolicy;
  readonly admitReplay = () => this.tracker.admitReplay();
  readonly isIdentified = () => this.tracker.isIdentified();
  readonly onIdentified = (h: () => void) => this.tracker.onIdentified(h);
  readonly onSessionException = (h: () => void) =>
    this.tracker.onSessionException(h);

  private readonly seen = new WeakSet<Error>();
  private readonly sameTurn = createTurnDedupe();
  private readonly startup = new StartupExceptionBuffer((pending) => {
    this.deliverStartupExceptions(pending);
  });

  private runtime: PluginRuntime | null = null;
  private otel: OtelProviderHandle | null = null;
  private otelDispose: (() => void) | null = null;
  private sink: TelemetrySink | null = null;
  private otelControls: OtelControls | null = null;
  private deviceInfoCaptureEnabled = false;
  private pageEventsCaptureEnabled = false;
  private unsubscribePageHidden: (() => void) | null = null;

  private unsubscribePageUnloading: (() => void) | null = null;

  private readonly tracker: SessionTracker;

  private readonly filters: ErrorFilterOptions;

  constructor(
    tracker: SessionTracker,
    privacy: PrivacyOptions = {},
    serviceIdentity: NonNullable<PluginContext["serviceIdentity"]> = {},
    filters: ErrorFilterOptions = {},
  ) {
    this.serviceIdentity = serviceIdentity;
    this.filters = filters;
    this.tracker = tracker;
    this.privacyPolicy = resolvePrivacyPolicy(privacy);
    this.urlRedaction = resolveUrlRedaction(privacy.urls);

    tracker.onSessionUpgrade((previousId, sessionId) => {
      this.sink?.emitLog(sessionStartRecord(previousId, sessionId));
    });

    this.consent = {
      get: () => this.requireRuntime().getConsent(),
      set: (decision) => {
        const runtime = this.requireRuntime();
        const previous = runtime.getConsent();

        runtime.setConsent(decision);

        const current = runtime.getConsent();

        this.tracker.setPersistent(current.analytics);

        if (previous.analytics && !current.analytics) {
          clearExportCarryover();
        } else if (previous.replay && !current.replay) {
          clearExportCarryover("replay:");
        }

        this.startup.releaseDeferred();
      },
    };

    this.identity = {
      clear: () => {
        clearExportCarryover();

        this.tracker.reset();
      },
      get: () => this.tracker.getIdentity(),
      set: async (params) => this.tracker.identify(params),
    };

    this.device = {
      getDeviceId: () => this.tracker.getDeviceId(),
      getFpHash: () => this.tracker.getFpHash(),
    };

    this.session = {
      getId: () => this.tracker.sessionId(),
      getWindowId: () => this.tracker.windowId(),
    };

    this.unsubscribePageHidden = onPageHidden(() => {
      // oxlint-disable-next-line promise/prefer-await-to-then, eslint/no-empty-function
      this.flush().catch(() => {});
    });

    this.unsubscribePageUnloading = onPageUnloading(() => {
      // oxlint-disable-next-line promise/prefer-await-to-then, eslint/no-empty-function
      this.flush().catch(() => {});
    });
  }

  attachRuntime(runtime: PluginRuntime) {
    this.runtime = runtime;

    this.tracker.setPersistent(runtime.getConsent().analytics);
  }

  private requireRuntime() {
    if (!this.runtime) {
      throw new Error(
        "Kernel runtime not attached. createKernel must call attachRuntime before any consent/dispose path runs.",
      );
    }

    return this.runtime;
  }

  /** PluginContext shim: plugins expect a non-null string. */
  getSessionId() {
    return this.tracker.sessionId() ?? "";
  }

  getWindowId() {
    return this.tracker.windowId() ?? "";
  }

  onSessionPause(handler: (reason: SessionPauseReason) => void) {
    return this.tracker.onSessionPause(handler);
  }

  onSessionExceptionWhilePaused(handler: (reason: SessionPauseReason) => void) {
    return this.tracker.onSessionExceptionWhilePaused(handler);
  }

  noteException() {
    this.tracker.noteException();
  }

  onSessionResume(handler: (reason: SessionResumeReason) => void) {
    return this.tracker.onSessionResume(handler);
  }

  getPluginConfigState(key: PluginKey) {
    return this.requireRuntime().getPluginConfigState(key);
  }

  getReplayConfig() {
    return this.requireRuntime().getReplayConfig();
  }

  isInteractionCaptureEnabled() {
    return this.requireRuntime().isInteractionCaptureEnabled();
  }

  subscribePluginConfigState(
    key: PluginKey,
    listener: (state: PluginConfigState) => void,
  ) {
    return this.requireRuntime().subscribePluginConfigState(key, listener);
  }

  // WARNING: No global TracerProvider is installed; trace.getTracer silently returns non-recording spans.
  getTracer(name: string) {
    return this.otel?.tracerProvider.getTracer(name) ?? trace.getTracer(name);
  }

  startPageEventsCapture() {
    const { otelControls: controls } = this;

    if (!controls) {
      return null;
    }

    controls.setPageEventsEnabled?.(true);

    this.pageEventsCaptureEnabled = true;

    controls.recordCurrentView();

    return (() => {
      this.pageEventsCaptureEnabled = false;

      controls.setPageEventsEnabled?.(false);
    }) satisfies (() => void) | null;
  }

  startDeviceInfoCapture() {
    this.tracker.startDeviceIdentity();

    this.deviceInfoCaptureEnabled = true;

    return ((reason) => {
      this.deviceInfoCaptureEnabled = false;

      if (reason === "consent-revoked") {
        this.tracker.discardDeviceIdentity();
      }
    }) satisfies PluginCleanup | null;
  }

  isDeviceInfoCaptureEnabled() {
    return this.deviceInfoCaptureEnabled;
  }

  isPageEventsCaptureEnabled() {
    return this.pageEventsCaptureEnabled;
  }

  /**
   * Records an exception event on the active OTel span. No envelope
   * path — the SDK is OTel-only. Single dedup boundary: drops repeats
   * of the same `Error` instance, plus browser-extension noise and
   * unresolvable stacks. Upstream callers (the errors plugin's
   * `window.onerror` / `console.error` / `unhandledrejection`, the
   * React error boundary) just call this — they don't pre-filter.
   *
   * Marks the span as failed only for unhandled captures. A handled
   * console.error inside a customer-owned `span()` block would otherwise
   * taint the customer's span even when the customer caught the error
   * explicitly. The customer's `span()` helper sets status itself when
   * it observes a throw.
   *
   * Without an active span the kernel opens one named after the throw
   * site (the top in-app stack frame) to carry the event, marked with
   * `interfere.capture.kind` so consumers can recognise it by attribute
   * rather than by name.
   *
   * `appendFrames` are always appended (component-stack supplements);
   * `fallbackFrames` are only adopted when the parsed stack is empty
   * (`window.onerror` source/line/col when the Error has no usable
   * stack). Both ride on the serialised `interfere.exception.chain`
   * attribute when the chain is non-trivial.
   */
  recordException(value: Error | NonErrorException, opts: RecordExceptionOpts) {
    if (!isNonErrorException(value)) {
      if (this.seen.has(value)) {
        return;
      }

      this.seen.add(value);

      if (!this.sameTurn(errorFingerprint(value), opts.mechanism.type)) {
        return;
      }
    }

    if (!isNonErrorException(value)) {
      const stacks: string[] = [];
      let current: Error | undefined = value;

      for (let depth = 0; current && depth < 5; depth += 1) {
        if (current.stack) {
          stacks.push(current.stack);
        }

        current = current.cause instanceof Error ? current.cause : undefined;
      }

      if (
        shouldDropBrowserExtensionNoise(stacks) ||
        shouldDropUnresolvableStack(stacks)
      ) {
        return;
      }
    }

    const exception = filterException(this.filters, value, opts);

    if (!exception) {
      return;
    }

    this.tracker.noteException();

    const attrs = {
      ...buildExceptionEventAttrs(exception, opts),
      ...identityAttributes({
        getActionId: () => this.views.actionId(),
        getViewId: () => this.views.viewId(),
      }),
    };

    if (
      this.startup.holding &&
      this.startup.push({
        attrs,
        handled: opts.mechanism.handled,
        origin: { at: Date.now(), context: context.active() },
        value: exception,
      })
    ) {
      return;
    }

    const active = trace.getActiveSpan();

    if (active?.isRecording()) {
      recordExceptionOnSpan(active, exception, attrs, opts.mechanism.handled);

      return;
    }

    this.sink?.recordException(exception, attrs, opts.mechanism.handled);
  }

  holdStartupExceptions() {
    this.startup.hold();
  }

  /** Ends startup buffering. With tracing on, whose exports are batched anyway, it waits for the consent decision so released exceptions carry the device attributes consent allows, releasing after a short grace period or on the next flush when none arrives. */
  releaseStartupExceptions() {
    this.startup.release(
      this.otel !== null && !this.runtime?.isConfigSettled(),
    );
  }

  private deliverStartupExceptions(pending: readonly PendingException[]) {
    if (this.getPluginConfigState("errors") === "disabled") {
      return;
    }

    for (const { attrs, handled, origin, value } of pending) {
      this.sink?.recordException(value, attrs, handled, origin);
    }
  }

  /**
   * Emits an OTel `LogRecord` through the kernel's sink. Used by
   * `plugins/logs.ts` to capture string-only `console.*` calls (the
   * errors plugin still owns Error-bearing console calls — class
   * boundary). No-ops without a logger provider (`tracing: false`).
   */
  recordLog(input: {
    severityText: string;
    severityNumber: SeverityNumber;
    body: string;
    attributes?: Record<string, string>;
  }) {
    this.sink?.emitLog(
      omitUndefined({
        body: input.body,
        severityNumber: input.severityNumber,
        severityText: input.severityText,
        attributes: input.attributes,
      }),
    );
  }

  /**
   * Force-flushes the OTel exporters. Returns once they've settled
   * (success or failure) so callers driving unload — page-lifecycle
   * handlers, integration test teardown — can actually await
   * completion. The OTel SDK swallows export errors internally; we
   * surface the resolution either way.
   */
  async flush() {
    this.startup.releaseDeferred();

    if (this.sink) {
      await this.sink.flush();
    }
  }

  /**
   * Wires the private OTel providers. `dispose` runs before the providers
   * shut down and must make every patch Interfere installed inert, restore
   * the ones still current and release the API globals Interfere owns.
   */
  attachOtel(
    handle: OtelProviderHandle,
    dispose: () => void,
    controls?: OtelControls,
  ) {
    this.otel = handle;
    this.otelDispose = dispose;
    this.otelControls = controls ?? null;
    this.sink = createOtelSink(handle);
  }

  /** Captures exceptions without tracing: `tracing: false` posts hand-encoded exception spans without a tracer provider, while views and actions rotate as they do with tracing on. */
  attachCaptureOnlySink(
    input: Omit<
      CaptureOnlySinkInput,
      | "getActionId"
      | "getDeviceId"
      | "getSessionId"
      | "getViewId"
      | "urlRedaction"
    >,
  ) {
    const sink: TelemetrySink = createCaptureOnlySink({
      ...input,
      getActionId: () => this.views.actionId(),
      getDeviceId: () => this.device.getDeviceId(),
      getSessionId: () => this.session.getId(),
      getViewId: () => this.views.viewId(),
      urlRedaction: this.urlRedaction,
    });

    const correlation = installViewCorrelation({
      documentContext: null,
      kernel: this,
      loggerProvider: {
        getLogger: () => ({
          emit: (record) => {
            sink.emitLog(record);
          },
        }),
      },
    });

    this.sink = sink;
    this.otelDispose = correlation.dispose;
    this.otelControls = { recordCurrentView: correlation.recordCurrentView };
  }

  syncProviderConfig(config: { plugins?: PluginOverrides | undefined }) {
    this.requireRuntime().applyProviderConfig(config.plugins);

    this.startup.releaseDeferred();
  }

  async dispose() {
    this.startup.releaseDeferred();

    this.startup.discard();

    this.unsubscribePageHidden?.();

    this.unsubscribePageHidden = null;

    this.unsubscribePageUnloading?.();

    this.unsubscribePageUnloading = null;

    if (this.runtime) {
      await this.runtime.dispose();
    }

    this.tracker.dispose();

    this.otelDispose?.();

    this.otelDispose = null;

    this.otel = null;

    if (this.sink) {
      await this.sink.shutdown();

      this.sink = null;
    }
  }
}

interface CreateKernelInput {
  onCreated?: (kernel: Kernel) => void;
  opts?: KernelInternalOptions;
}

export async function createKernel(input: CreateKernelInput = {}) {
  const { opts = {} } = input;
  const fetcher = opts.fetch ?? globalThis.fetch.bind(globalThis);

  const serviceName = opts.serviceName || getStringGlobal(SERVICE_NAME_GLOBAL);
  const service = { serviceName, serviceNamespace: opts.serviceNamespace };

  const forceEnabled = Boolean(getGlobal(FORCE_ENABLE_GLOBAL));

  if (forceEnabled) {
    log.warn(
      "FORCE_ENABLE is active. Events will be dropped by production ingest. This should only be set during local development.",
    );
  }

  const tracker = new SessionTracker({
    fetcher,
    target: {
      ...resolveTargets(opts.apiHost, undefined, service).ingest,
      baseUrl: () => resolveTargets(opts.apiHost).ingest.baseUrl,
    },
  });

  if (opts._wrapperVersions?.length) {
    Object.assign(globalThis, { [SDK_STACK_GLOBAL]: opts._wrapperVersions });
  }

  const kernel = new Kernel(tracker, opts.privacy, service, {
    beforeSend: opts.beforeSend,
    denyUrls: opts.denyUrls,
    errors: opts.errors,
    ignoreErrors: opts.ignoreErrors,
  });

  const baked = readBakedPlugins();
  const runtime = new PluginRuntime(kernel, opts.consent, baked ?? {}, true);

  kernel.attachRuntime(runtime);

  kernel.holdStartupExceptions();

  const disposeStartupNavigation = installViewNavigation(kernel.views);

  try {
    input.onCreated?.(kernel);

    runtime.start();

    const {
      plugins: remotePlugins,
      anonymousReplayPolicy,
      backendHosts,
      protocolVersion,
    } = await resolveRemoteConfig(opts.apiHost, fetcher, baked, service);

    kernel.remoteProtocolVersion = protocolVersion ?? null;

    kernel.anonymousReplayPolicy = anonymousReplayPolicy;

    const targets = resolveTargets(opts.apiHost, undefined, service);

    log.info("ingest: %s", targets.ingest.baseUrl);

    tracker.start();

    const tracingInput = {
      kernel,
      releaseSlug: readReleaseSlugFromGlobals(),
      sdkStack: buildSdkStack(opts._wrapperVersions),
      targets,
    };

    if (opts.tracing === false) {
      disposeStartupNavigation();

      kernel.attachCaptureOnlySink({
        clientOptions: targets.ingest,
        resource: buildResourceAttributes({
          ...tracingInput,
          serviceName,
          serviceNamespace: opts.serviceNamespace,
        }),
      });
    }

    if (opts.tracing !== false) {
      const { wireOtel } = await import("./otel/index.js");

      const tracingOpts =
        // oxlint-disable-next-line anti-slop/no-runtime-typeof -- The public tracing option accepts an instrumentation-options object or false; anything else means defaults.
        typeof opts.tracing === "object" ? opts.tracing : undefined;

      const resolvedTracing: ResolvedTracingOptions = { ...tracingOpts };

      if (serviceName) {
        resolvedTracing.serviceName = serviceName;
      }

      if (opts.serviceNamespace !== undefined) {
        resolvedTracing.serviceNamespace = opts.serviceNamespace;
      }

      if (opts._internalAdditionalSpanProcessors) {
        resolvedTracing.additionalSpanProcessors =
          opts._internalAdditionalSpanProcessors;
      }

      if (opts._internalAdditionalLogRecordProcessors) {
        resolvedTracing.additionalLogRecordProcessors =
          opts._internalAdditionalLogRecordProcessors;
      }

      if (opts._internalAdditionalMetricReaders) {
        resolvedTracing.additionalMetricReaders =
          opts._internalAdditionalMetricReaders;
      }

      if (backendHosts) {
        resolvedTracing.backendHosts = backendHosts;
      }

      disposeStartupNavigation();

      wireOtel({ ...tracingInput, tracing: resolvedTracing });
    }

    runtime.startWithRemoteConfig(remotePlugins ?? {});

    if (remotePlugins) {
      runtime.applyRemoteConfig(remotePlugins);
    } else {
      runtime.applyRemoteConfigDefaults();
    }

    kernel.releaseStartupExceptions();

    return kernel;
  } catch (error) {
    disposeStartupNavigation();

    await kernel.dispose();

    throw error;
  }
}

export { isEnabledByEnvironment } from "./environment/environment.js";

/**
 * Build-time-derived release slug. The build step
 * (`@interfere/next/withInterfere`, `@interfere/vite/plugin`) takes the
 * leading 16 hex chars of the commit SHA (`rel_<16hex>`) and stamps it into
 * the bundle as `__INTERFERE_RELEASE_SLUG__`. The collector derives the same
 * value from the create-release request, so the bundle's runtime slug is
 * guaranteed to match the row without a round-trip.
 *
 * Returns `null` when no slug was injected (dev builds, customers who
 * haven't wired up the plugin).
 */
export function readReleaseSlugFromGlobals() {
  const value = getGlobal(RELEASE_SLUG_GLOBAL);

  // oxlint-disable-next-line anti-slop/no-runtime-typeof
  if (typeof value !== "string") {
    return null;
  }

  return releaseSlugSchema.parse(value);
}
