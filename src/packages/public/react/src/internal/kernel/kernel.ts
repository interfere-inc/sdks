import {
  createKernelConsent,
  createKernelDevice,
  createKernelIdentity,
  createKernelSession,
} from "./facades.js";
import { omitUndefined } from "@interfere/helpers/omit-undefined";
import type { NonErrorException } from "@interfere/types/sdk/errors";
import type { ErrorFilterOptions } from "@interfere/types/sdk/options/filter-options";
import type { PluginKey } from "@interfere/types/sdk/plugins/manifest";
import type { AnonymousReplayPolicy } from "@interfere/types/sdk/remote-config/anonymous-replay";
import {
  redactUrl,
  resolveUrlRedaction,
} from "@interfere/types/sdk/url-redaction";
import type { UrlRedactionPolicy } from "@interfere/types/sdk/url-redaction";

import { trace } from "@opentelemetry/api";
import type { SeverityNumber } from "@opentelemetry/api";

import type { PluginOverrides } from "../../plugins/lib/loader.js";
import type {
  PluginCleanup,
  PluginConfigState,
  PluginContext,
} from "../../plugins/lib/types.js";

import type { SessionTracker } from "../../tracking/api.js";
import { sessionStartRecord } from "../../tracking/session.js";
import { ViewIds } from "../../tracking/view.js";
import { createLogger } from "../../util/log.js";
import type {
  SessionPauseReason,
  SessionResumeReason,
} from "../activity/activity.js";
import { resolvePrivacyPolicy } from "../dom/privacy.js";
import type { PrivacyOptions, PrivacyPolicy } from "../dom/privacy.js";
import type { BundledInstrumentations } from "../otel/instrumentations/instrumentations.js";
import type { OtelProviderHandle } from "../otel/provider.js";
import {
  onPageHidden,
  onPageUnloading,
} from "../page-lifecycle/page-lifecycle.js";
import type { RecordExceptionOpts } from "../capture/exception-span.js";
import { createOtelSink } from "../capture/otel-sink.js";
import { StartupExceptionBuffer } from "../capture/startup-buffer.js";
import { createTurnDedupe } from "../capture/turn-dedupe.js";
import type { PendingException } from "../capture/startup-buffer.js";
import type { CaptureOnlySinkInput } from "../capture/capture-only-sink.js";
import type { TelemetrySink } from "../capture/sink.js";
import type { PluginRuntime } from "../plugins/plugin-runtime.js";
import { createCaptureOnlyCorrelation } from "./capture-only.js";
import type {
  KernelConsent,
  KernelDevice,
  KernelIdentity,
  KernelSession,
} from "./handles.js";
import { recordKernelException } from "./record-exception.js";

const log = createLogger("kernel");

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
  private readonly heldStartRecords: ReturnType<typeof sessionStartRecord>[] =
    [];
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
      const record = sessionStartRecord(previousId, sessionId);

      if (this.sink) {
        this.sink.emitLog(record);
      } else {
        this.heldStartRecords.push(record);
      }
    });

    this.consent = createKernelConsent({
      requireRuntime: () => this.requireRuntime(),
      startup: this.startup,
      tracker: this.tracker,
    });

    this.identity = createKernelIdentity({ tracker: this.tracker });

    this.device = createKernelDevice({ tracker: this.tracker });

    this.session = createKernelSession({ tracker: this.tracker });

    this.unsubscribePageHidden = onPageHidden(() => {
      // oxlint-disable-next-line promise/prefer-await-to-then
      this.flush().catch(log.debug);
    });

    this.unsubscribePageUnloading = onPageUnloading(() => {
      // oxlint-disable-next-line promise/prefer-await-to-then
      this.flush().catch(log.debug);
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
    recordKernelException(
      {
        filters: this.filters,
        sameTurn: this.sameTurn,
        seen: this.seen,
        sink: this.sink,
        startup: this.startup,
        tracker: this.tracker,
        views: this.views,
      },
      value,
      opts,
    );
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

    this.adoptSink(createOtelSink(handle));
  }

  private adoptSink(sink: TelemetrySink) {
    this.sink = sink;

    for (const record of this.heldStartRecords.splice(0)) {
      sink.emitLog(record);
    }
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
    const { correlation, sink } = createCaptureOnlyCorrelation(this, input);

    this.adoptSink(sink);

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
