import { omitUndefined } from "@interfere/helpers/omit-undefined";
import type { ReleaseSlug } from "@interfere/types/releases/slug";
import type { Context } from "@opentelemetry/api";
import type { LogRecordProcessor } from "@opentelemetry/sdk-logs";
import type { MetricReader } from "@opentelemetry/sdk-metrics";
import type { SpanProcessor } from "@opentelemetry/sdk-trace";

import {
  requestIdentityIsCurrent,
  snapshotRequestIdentity,
} from "./request-observer/request-identity.js";
import { installViewCorrelation } from "./correlation.js";
import { registerBundledInstrumentations } from "./instrumentations/instrumentations.js";
import { traceContextWriterFor } from "./instrumentations/trace-context-writers.js";
import { readPropagationFromDocument } from "./propagation.js";
import { buildOtelProvider } from "./provider.js";
import { captureWebVitals } from "./web-vitals.js";
import { isPageLeaving } from "../page-lifecycle/unloading.js";

import type { resolveTargets } from "../config/config.js";
import type { Kernel, TracingOptions } from "../kernel.js";
import { INGEST_IGNORE_RE } from "./ingest-ignore.js";

const REPLAY_UPLOAD_RE = /\/v3\/replay\/upload\//u;

export interface ResolvedTracingOptions extends TracingOptions {
  backendHosts?: string[];
  additionalLogRecordProcessors?: LogRecordProcessor[];
  additionalMetricReaders?: MetricReader[];
  additionalSpanProcessors?: SpanProcessor[];
  serviceName?: string;
  serviceNamespace?: string;
}

export interface TracingRuntimeInput {
  kernel: Kernel;
  releaseSlug: ReleaseSlug | null;
  sdkStack: string[];
  targets: ReturnType<typeof resolveTargets>;
}

export function wireOtel(
  input: TracingRuntimeInput & { tracing: ResolvedTracingOptions },
) {
  const { kernel, targets, tracing } = input;

  let endOpenRequests: (() => void) | undefined;

  const handle = buildOtelProvider(
    omitUndefined({
      beforeFlush: () => {
        if (isPageLeaving()) {
          endOpenRequests?.();
        }
      },
      clientOptions: targets.ingest,
      canPersistExports: () => kernel.consent.get().analytics,
      propagateTraceBaggage: tracing.propagateTraceBaggage,
      propagationOwnership: tracing.propagation,
      getDeviceId: () => kernel.device.getDeviceId(),
      getSessionId: () => kernel.session.getId(),
      getViewId: () => kernel.views.viewId(),
      getActionId: () => kernel.views.actionId(),
      isDeviceInfoEnabled: () => kernel.isDeviceInfoCaptureEnabled(),
      onExceptionSpan: () => {
        kernel.noteException();
      },
      releaseSlug: input.releaseSlug,
      sdkStack: input.sdkStack,
      serviceName: tracing.serviceName || undefined,
      serviceNamespace: tracing.serviceNamespace,
      additionalSpanProcessors: [
        ...(tracing.spanProcessors ?? []),
        ...(tracing.additionalSpanProcessors ?? []),
      ],
      additionalLogRecordProcessors: tracing.additionalLogRecordProcessors,
      additionalMetricReaders: tracing.additionalMetricReaders,
      urlRedaction: kernel.urlRedaction,
    }),
  );

  const correlation = installViewCorrelation({
    documentContext: readPropagationFromDocument(),
    kernel,
    loggerProvider: handle.loggerProvider,
    resolveRoute: tracing.resolveRoute,
  });

  const instrumentations = registerBundledInstrumentations(
    omitUndefined({
      tracerProvider: handle.tracerProvider,
      loggerProvider: handle.loggerProvider,
      ignoreUrls: [
        INGEST_IGNORE_RE,
        `${targets.config.baseUrl}/v3/config`,
        `${targets.ingest.baseUrl}/v3/session`,
        REPLAY_UPLOAD_RE,
        ...(tracing.ignoreUrls ?? []),
      ],
      backendHosts: tracing.backendHosts,
      defersTraceContext:
        tracing.propagation === undefined
          ? (url: string) => traceContextWriterFor(url) !== undefined
          : undefined,
      onSessionResume: (handler: Parameters<Kernel["onSessionResume"]>[0]) =>
        kernel.onSessionResume(handler),
      propagateTo: tracing.propagateTo,
      resolveRoute: tracing.resolveRoute,
      captureFailedRequests: tracing.captureFailedRequests,
      redactUrl: kernel.redactUrl,
      headers: tracing.headers,
      propagateTraceBaggage: tracing.propagateTraceBaggage,
      traceHeaders: handle.traceHeaders,
      writesTraceContext: tracing.propagation !== "external",
      canCorrelate: (ctx?: Context) =>
        ctx === undefined || requestIdentityIsCurrent(ctx),
      canPersistDiagnostics: () => kernel.consent.get().analytics,
      captureContext: (ctx: Context) => {
        const consent = kernel.consent.get();

        return snapshotRequestIdentity(
          ctx,
          {
            getDeviceId: () => kernel.device.getDeviceId(),
            getSessionId: () => kernel.session.getId(),
            getViewId: () => kernel.views.viewId(),
            getActionId: () => kernel.views.actionId(),
          },
          () => kernel.consent.get() === consent,
        );
      },
    }),
  );

  ({ endOpenRequests } = instrumentations);

  const stopVitals =
    tracing.webVitals === false
      ? undefined
      : captureWebVitals(
          omitUndefined({
            flush: async () => handle.flush(),
            getDeviceId: () => kernel.device.getDeviceId(),
            meter: handle.meterProvider.getMeter("@interfere/react/web-vitals"),
            resolveRoute: tracing.resolveRoute,
            getIdentity: () => ({
              sessionId: kernel.session.getId(),
              viewId: kernel.views.viewId(),
            }),
          }),
        );

  kernel.attachOtel(
    handle,
    () => {
      stopVitals?.();

      correlation.dispose();

      instrumentations.dispose();
    },
    {
      recordCurrentView: correlation.recordCurrentView,
      setPageEventsEnabled: instrumentations.setPageEventsEnabled,
    },
  );
}
