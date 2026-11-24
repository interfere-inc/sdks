import type { InterfereClient } from "@interfere/javascript";
/* oxlint-disable unicorn/no-useless-promise-resolve-reject */
import {
  INTERFERE_DEVICE_ID_KEY,
  INTERFERE_SESSION_ID_KEY,
} from "@interfere/types/data/correlation-baggage";
import type { PageSessionId, SessionId } from "@interfere/types/data/session";
import type { ReleaseSlug } from "@interfere/types/releases/slug";
import {
  createUrlRedactionSpanProcessor,
  resolveUrlRedaction,
} from "@interfere/types/sdk/url-redaction";
import type { UrlRedactionPolicy } from "@interfere/types/sdk/url-redaction";

import { propagation } from "@opentelemetry/api";
import type {
  Attributes,
  Baggage,
  TextMapPropagator,
} from "@opentelemetry/api";
import {
  W3CBaggagePropagator,
  W3CTraceContextPropagator,
} from "@opentelemetry/core";
import { browserDetector } from "@opentelemetry/opentelemetry-browser-detector";
import {
  detectResources,
  resourceFromAttributes,
} from "@opentelemetry/resources";
import {
  BatchLogRecordProcessor,
  LoggerProvider,
} from "@opentelemetry/sdk-logs";
import type { LogRecordProcessor } from "@opentelemetry/sdk-logs";
import {
  MeterProvider,
  PeriodicExportingMetricReader,
} from "@opentelemetry/sdk-metrics";
import type { MetricReader } from "@opentelemetry/sdk-metrics";
import {
  AlwaysOnSampler,
  BatchSpanProcessor,
  ParentBasedSampler,
  TracerProvider,
} from "@opentelemetry/sdk-trace";
import type { SpanProcessor } from "@opentelemetry/sdk-trace";

import { requestIdentity } from "./request-observer/request-identity.js";
import { identityAttributes } from "../../tracking/view.js";
import type { IdentityIds } from "../../tracking/view.js";
import { buildResourceAttributes } from "../capture/resource-attributes.js";
import {
  createLogExporter,
  createMetricExporter,
  createTraceExporter,
} from "./exporter.js";
import { createExportFilterProcessor } from "./export/filter.js";
import { isPageHidden } from "../page-lifecycle/page-lifecycle.js";
import { isPageLeaving } from "../page-lifecycle/unloading.js";
import { holdSpansForLogs } from "./transport/leave-gate.js";
import { createTraceHeaderWriter } from "./trace-headers.js";
import type { TraceHeaderWriter } from "./trace-headers.js";

const BROWSER_LOG_BATCH_OPTIONS = {
  exportTimeoutMillis: 70_000,
  maxExportBatchSize: 200,
  maxQueueSize: 2048,
  scheduledDelayMillis: 5000,
} as const;

const BROWSER_TRACE_BATCH_OPTIONS = {
  ...BROWSER_LOG_BATCH_OPTIONS,
  scheduledDelayMillis: 60_000,
} as const;

function withBaggageEntry(baggage: Baggage, key: string, value: string | null) {
  return value ? baggage.setEntry(key, { value }) : baggage.removeEntry(key);
}

function createSessionBaggagePropagator(opts: {
  propagateTraceBaggage: boolean;
  getDeviceId: () => string | null;
  getSessionId: () => string | null;
}) {
  const propagator = new W3CBaggagePropagator();

  return {
    fields: () => (opts.propagateTraceBaggage ? propagator.fields() : []),
    extract: (ctx, carrier, getter) => propagator.extract(ctx, carrier, getter),
    inject: (ctx, carrier, setter) => {
      if (!opts.propagateTraceBaggage) {
        return;
      }

      const baggage =
        propagation.getBaggage(ctx) ?? propagation.createBaggage();

      const updated = withBaggageEntry(
        withBaggageEntry(
          baggage,
          INTERFERE_SESSION_ID_KEY,
          opts.getSessionId(),
        ),
        INTERFERE_DEVICE_ID_KEY,
        opts.getDeviceId(),
      );

      propagator.inject(propagation.setBaggage(ctx, updated), carrier, setter);
    },
  } satisfies TextMapPropagator;
}

function createIdentitySpanProcessor(ids: IdentityIds) {
  return {
    forceFlush: async () => Promise.resolve(),
    // oxlint-disable-next-line eslint/no-empty-function
    onEnd() {},
    onStart(span) {
      span.setAttributes(identityAttributes(ids));
    },
    shutdown: async () => Promise.resolve(),
  } satisfies SpanProcessor;
}

function createIdentityLogRecordProcessor(ids: IdentityIds) {
  return {
    forceFlush: async () => Promise.resolve(),
    onEmit(logRecord, ctx) {
      logRecord.setAttributes(requestIdentity(ctx) ?? identityAttributes(ids));
    },
    shutdown: async () => Promise.resolve(),
  } satisfies LogRecordProcessor;
}

interface GatedAttributesInput {
  attributes: Attributes;
  isEnabled: () => boolean;
}

function stampGatedAttributes(
  target: {
    setAttribute: (key: string, value: Attributes[string]) => void;
  },
  opts: GatedAttributesInput,
) {
  if (!opts.isEnabled()) {
    return;
  }

  for (const [key, value] of Object.entries(opts.attributes)) {
    if (value !== undefined) {
      target.setAttribute(key, value);
    }
  }
}

function createDeviceInfoSpanProcessor(opts: GatedAttributesInput) {
  return {
    forceFlush: async () => Promise.resolve(),
    // oxlint-disable-next-line eslint/no-empty-function
    onEnd() {},
    onStart: (span) => {
      stampGatedAttributes(span, opts);
    },
    shutdown: async () => Promise.resolve(),
  } satisfies SpanProcessor;
}

function createDeviceInfoLogRecordProcessor(opts: GatedAttributesInput) {
  return {
    forceFlush: async () => Promise.resolve(),
    onEmit: (logRecord) => {
      stampGatedAttributes(logRecord, opts);
    },
    shutdown: async () => Promise.resolve(),
  } satisfies LogRecordProcessor;
}

export interface OtelProviderInput {
  additionalLogRecordProcessors?: LogRecordProcessor[];
  additionalMetricReaders?: MetricReader[];
  additionalSpanProcessors?: SpanProcessor[];
  /**
   * Auth + content headers for OTLP exports. Sourced from the kernel's
   * already-resolved ingest target so the OTLP path inherits the same
   * transport metadata the rest of the SDK uses — no separate auth knob.
   */
  clientOptions: InterfereClient.Options;
  canPersistExports?: () => boolean;
  /** Defaults to true. False disables outgoing baggage injection; extraction is unchanged. */
  propagateTraceBaggage?: boolean;
  propagationOwnership?: "interfere" | "external";
  /**
   * Returns the current `DeviceManager` device id. Wired into per-span
   * and per-log processors so a mid-page device rotation
   * (`identity.clear()` after logout) lands on subsequent spans / logs
   * without the kernel rebuilding the providers. Returns `null` before
   * the device manager has bootstrapped, or in SSR / test boot paths.
   *
   * Not applied to metrics: the MeterProvider takes a static resource
   * and rebuilding it mid-page would destroy the periodic reader; metrics
   * remain stamped with the boot-time device id.
   */
  getDeviceId: () => string | null;
  /**
   * Returns the kernel's current session id. Wired into per-span /
   * per-log session processors so a mid-page session rotation (30-min
   * idle expiry, manual reset) lands on subsequent spans without the
   * kernel having to rebuild the providers. Returns `null` before the
   * `SessionTracker` has bootstrapped.
   */
  getSessionId: () => PageSessionId | SessionId | null;
  /**
   * Consent gate for the browser-detector attributes (`browser.*`,
   * `user_agent.original`), read per span / log / metric export so a
   * mid-page consent change needs no provider rebuild. Defaults to deny;
   * the kernel wires it to the `device` plugin's activation state.
   */
  isDeviceInfoEnabled?: () => boolean;
  /** Returns the action id while a click or submit dispatch runs; stamped as `interfere.action.id`. */
  getActionId?: () => string | null;
  /** Returns the current page view id; stamped as `interfere.view.id` on every span and log record. */
  getViewId?: () => string | null;
  /** Called when a finished span carries an `exception` event, whichever instrumentation recorded it. */
  onExceptionSpan?: () => void;
  /** Runs before each flush, so work that must be ended first joins it. */
  beforeFlush?: () => void;
  /** Optional release slug — top-level `release.slug` attr (Phase 6 wire identity). */
  releaseSlug?: ReleaseSlug | null | undefined;
  /**
   * Producer chain for the `interfere.sdk.stack` resource attribute —
   * e.g. `["@interfere/next@10.0.0", "@interfere/react@10.0.0"]`.
   * Wrapper-injected; the kernel just forwards.
   */
  sdkStack: string[];
  /**
   * Override the OTel `service.name` resource attribute. Defaults to
   * `"unknown_service"`.
   */
  serviceName?: string;
  /** Optional application namespace for grouping related services. */
  serviceNamespace?: string;
  /** URL redaction applied in `onEnding`, before customer processors and export. Defaults to built-in sensitive names. */
  urlRedaction?: UrlRedactionPolicy;
}

export interface OtelProviderHandle {
  /** Force-flush all three providers — call from kernel.flush() / on visibility hidden. */
  flush: () => Promise<void>;
  loggerProvider: LoggerProvider;
  meterProvider: MeterProvider;
  metricReader: MetricReader;
  /** Tear down all three providers — called from kernel.dispose(). */
  shutdown: () => Promise<void>;
  /** Writes the headers the propagation policy allows for a request: `traceparent` where absent plus merged session baggage. */
  traceHeaders: TraceHeaderWriter;
  tracerProvider: TracerProvider;
}

/**
 * Constructs the SDK's private OTel providers and trace-header writer.
 * Nothing here touches an OpenTelemetry API global: the
 * `TracerProvider` is passed to `registerInstrumentations(...)`
 * explicitly, and Interfere never registers a global propagator or
 * context manager.
 *
 * Kept out of `Kernel` so the error-only bundle path never imports the
 * OTel SDK.
 */
export function buildOtelProvider(input: OtelProviderInput) {
  const resource = resourceFromAttributes(buildResourceAttributes(input));

  const deviceInfoAttributes = detectResources({
    detectors: [browserDetector],
  }).attributes;

  const isDeviceInfoEnabled = input.isDeviceInfoEnabled ?? (() => false);

  const traceExporter = createTraceExporter({
    clientOptions: input.clientOptions,
    canPersistExports: input.canPersistExports,
  });

  const { getSessionId } = input;
  const { getDeviceId } = input;

  const identity = {
    getActionId: input.getActionId,
    getDeviceId,
    getSessionId,
    getViewId: input.getViewId,
  } satisfies IdentityIds;

  const spanProcessors: SpanProcessor[] = [
    createUrlRedactionSpanProcessor(
      input.urlRedaction ?? resolveUrlRedaction(),
    ),
    createIdentitySpanProcessor(identity),

    createDeviceInfoSpanProcessor({
      attributes: deviceInfoAttributes,
      isEnabled: isDeviceInfoEnabled,
    }),

    ...(input.additionalSpanProcessors ?? []),

    createExportFilterProcessor(
      new BatchSpanProcessor({
        exporter: traceExporter,
        ...BROWSER_TRACE_BATCH_OPTIONS,
      }),
      input.onExceptionSpan,
    ),
  ];

  const tracerProvider = new TracerProvider({
    resource,
    sampler: new ParentBasedSampler({
      localParentNotSampled: new AlwaysOnSampler(),
      remoteParentNotSampled: new AlwaysOnSampler(),
      root: new AlwaysOnSampler(),
    }),
    spanProcessors,
  });

  let isShutdown = false;

  const w3c = new W3CTraceContextPropagator();

  const traceContext: TextMapPropagator = {
    fields: () =>
      input.propagationOwnership === "external" ? [] : w3c.fields(),
    extract: (ctx, carrier, getter) => w3c.extract(ctx, carrier, getter),
    inject: (ctx, carrier, setter) => {
      if (input.propagationOwnership !== "external") {
        w3c.inject(ctx, carrier, setter);
      }
    },
  };

  const sessionBaggage = createSessionBaggagePropagator({
    propagateTraceBaggage: input.propagateTraceBaggage !== false,
    getDeviceId: () => (isShutdown ? null : getDeviceId()),
    getSessionId: () => (isShutdown ? null : getSessionId()),
  });

  const metricExporter = createMetricExporter(
    {
      clientOptions: input.clientOptions,
      canPersistExports: input.canPersistExports,
    },
    {
      attributes: deviceInfoAttributes,
      isEnabled: isDeviceInfoEnabled,
    },
  );

  const metricReader = new PeriodicExportingMetricReader({
    exportIntervalMillis: 75_000,
    exportTimeoutMillis: 70_000,
    exporter: metricExporter,
  });

  const meterProvider = new MeterProvider({
    readers: [metricReader, ...(input.additionalMetricReaders ?? [])],
    resource,
  });

  const logExporter = createLogExporter({
    clientOptions: input.clientOptions,
    canPersistExports: input.canPersistExports,
  });

  const loggerProvider = new LoggerProvider({
    processors: [
      createIdentityLogRecordProcessor(identity),

      createDeviceInfoLogRecordProcessor({
        attributes: deviceInfoAttributes,
        isEnabled: isDeviceInfoEnabled,
      }),

      new BatchLogRecordProcessor({
        exporter: logExporter,
        ...BROWSER_LOG_BATCH_OPTIONS,
      }),
      ...(input.additionalLogRecordProcessors ?? []),
    ],
    resource,
  });

  return {
    flush: async () => {
      input.beforeFlush?.();

      if (isPageHidden() || isPageLeaving()) {
        holdSpansForLogs();
      }

      const flushes = [
        tracerProvider.forceFlush({ timeoutMillis: 75_000 }),
        metricReader.forceFlush({ timeoutMillis: 75_000 }),
        loggerProvider.forceFlush({ timeoutMillis: 75_000 }),
      ];

      await Promise.allSettled(flushes);
    },
    loggerProvider,
    meterProvider,
    metricReader,
    shutdown: async () => {
      isShutdown = true;

      await Promise.allSettled([
        tracerProvider.shutdown(),
        meterProvider.shutdown(),
        loggerProvider.shutdown(),
      ]);
    },
    traceHeaders: createTraceHeaderWriter({
      baggage: input.propagateTraceBaggage === false ? null : sessionBaggage,
      traceContext,
    }),
    tracerProvider,
  } satisfies OtelProviderHandle;
}
