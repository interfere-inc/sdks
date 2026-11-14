/* oxlint-disable typescript/no-unnecessary-condition */
import { omitUndefined } from "@interfere/helpers/omit-undefined";
import type { TracingOwnership } from "@interfere/types/sdk/remote-config/tracing-ownership";

import { ROOT_CONTEXT, context, trace } from "@opentelemetry/api";
import type {
  Context,
  SpanOptions,
  Tracer,
  TracerProvider,
} from "@opentelemetry/api";
import { isUrlIgnored } from "@opentelemetry/core";
import { registerInstrumentations } from "@opentelemetry/instrumentation";
import { DocumentLoadInstrumentation } from "@opentelemetry/instrumentation-document-load";
import { LongTaskInstrumentation } from "@opentelemetry/instrumentation-long-task";
import type { ObserverCallbackInformation } from "@opentelemetry/instrumentation-long-task";
import type { LoggerProvider } from "@opentelemetry/sdk-logs";
import type { WebTracerProvider } from "@opentelemetry/sdk-trace-web";

import { THIRD_PARTY_IGNORE } from "../../third-party-hosts.js";
import { excludeFromExport } from "../export-filter.js";
import { requestObservationLogger } from "../request-observer/request-observer-events.js";
import {
  REQUEST_TRACER_NAME,
  installRequestObserver,
} from "../request-observer/request-observer.js";
import type { RequestObserverInput } from "../request-observer/request-observer.js";
import type { TraceHeaderWriter } from "../trace-headers.js";
import { ATTR_URL_FULL, ATTR_URL_PATH, stampUrlAttrs } from "../url-attrs.js";
import {
  capturesApplicationErrors,
  resolveFailureRanges,
} from "./failure-policy.js";
import type { CaptureFailedRequestsOption } from "./failure-policy.js";
import { propagationTargets } from "./propagation-targets.js";
import type { PropagateToEntry, TraceHeadersOption } from "./header-policy.js";

// WARNING: Document load extracts the traceparent meta through the global propagator, bypassing the freshness guard; remote parents must become ROOT_CONTEXT.
function browserRootedTracerProvider(provider: TracerProvider) {
  return {
    getTracer: (...args: Parameters<TracerProvider["getTracer"]>) => {
      const tracer = provider.getTracer(...args);

      return {
        startSpan: (name: string, options?: SpanOptions, ctx?: Context) => {
          const parent = ctx ?? context.active();

          return tracer.startSpan(
            name,
            options,
            trace.getSpanContext(parent)?.isRemote ? ROOT_CONTEXT : parent,
          );
        },
        startActiveSpan: tracer.startActiveSpan.bind(tracer),
      } satisfies Tracer;
    },
  } satisfies TracerProvider;
}

export type {
  CaptureFailedRequestsOption,
  CaptureFailedRequestsOptions,
  HttpStatusCodeRange,
} from "./failure-policy.js";

export interface InstrumentationsInput {
  /** The organization's backend hostnames from Interfere remote config. */
  backendHosts?: string[];
  /** See {@link CaptureFailedRequestsOption}. On by default. */
  captureFailedRequests?: CaptureFailedRequestsOption;
  /** Which headers allowed origins receive; `false` writes none anywhere. Defaults to every header. */
  headers?: TraceHeadersOption;
  /** Defaults to true. False disables Interfere baggage injection for every origin. */
  propagateTraceBaggage?: boolean;
  /**
   * URLs the SDK exempts from `fetch` and `XHR` instrumentation —
   * typically our own ingest/OTLP endpoints, so the SDK doesn't trace
   * its own export requests in an infinite loop. Combined with the
   * `THIRD_PARTY_IGNORE` defaults and the customer's `ignoreUrls`.
   */
  ignoreUrls: (string | RegExp)[];
  loggerProvider?: LoggerProvider;
  /** Who owns OpenTelemetry on this page, as the ownership controller resolved it: `managed` records request spans, `application` records request observations. Request headers do not depend on it. Defaults to `managed`. */
  mode?: TracingOwnership;
  onSessionResume?: RequestObserverInput["onSessionResume"];
  /** Explicit HTTP(S) origins allowed to receive trace headers; an entry object carries its own header list. */
  propagateTo?: (PropagateToEntry | string)[];
  /** Redacts URLs in request observations. Defaults to leaving them unchanged. */
  redactUrl?: (url: string) => string;
  /**
   * Pathname → low-cardinality route template (e.g. `/blog/[slug]`).
   * Drives span renaming + `url.path` / `http.route` attrs across
   * fetch, document-load and long-task.
   */
  resolveRoute?: (pathname: string) => string | undefined;
  /** Writes the headers the policy allows to requests for allowed origins in every mode; `null` or absent writes none. */
  traceHeaders?: TraceHeaderWriter | null;
  /** `false` when a tracing vendor on the page propagates W3C trace context itself; removes `traceparent` and `tracestate` from every origin. Defaults to `true`. */
  writesTraceContext?: boolean;
  captureContext?: RequestObserverInput["captureContext"];
  canCorrelate?: RequestObserverInput["canCorrelate"];
  /**
   * The kernel's private provider — instrumentations register against
   * it, not the global one, so customer OTel setups stay untouched.
   */
  tracerProvider: WebTracerProvider;
}

export interface BundledInstrumentations {
  dispose: () => void;
  /** Idempotent toggle for the analytics-consent-gated pageEvents signals. */
  setPageEventsEnabled: (enabled: boolean) => void;
}

/**
 * Registers the browser auto-instrumentations against the kernel's
 * private provider. Every instrumentation is enriched with the same
 * route-template-aware URL attributes so dashboards can slice fetch /
 * interaction / long-task / resource-load by `http.route` without
 * cardinality blow-up from raw pathnames.
 *
 * Requests go through the single request observer: a managed page gets
 * request spans on the private provider, an application-mode page gets
 * request observation events; both write the headers `traceHeaders`
 * decides on.
 */
export function registerBundledInstrumentations(input: InstrumentationsInput) {
  const {
    tracerProvider,
    loggerProvider,
    ignoreUrls,
    backendHosts = [],
    propagateTo = [],
    redactUrl = (url: string) => url,
    mode = "managed",
    traceHeaders = null,
  } = input;

  const { resolveRoute } = input;

  const managed = mode === "managed";

  const targets = propagationTargets(
    omitUndefined({
      pageOrigin: globalThis.location.origin,
      backendHosts,
      headers: input.headers,
      propagateTraceBaggage: input.propagateTraceBaggage,
      propagateTo,
      writesTraceContext: input.writesTraceContext,
    }),
  );

  const requestObserver = installRequestObserver(
    omitUndefined({
      applicationErrors: capturesApplicationErrors(input.captureFailedRequests),
      failureRanges: resolveFailureRanges(input.captureFailedRequests),
      ignoreUrls: [...ignoreUrls, ...THIRD_PARTY_IGNORE],
      onObservation: loggerProvider
        ? requestObservationLogger(loggerProvider, redactUrl)
        : undefined,
      onSessionResume: input.onSessionResume,
      captureContext: input.captureContext,
      canCorrelate: input.canCorrelate,
      redactUrl,
      resolveRoute,
      targets,
      traceHeaders,
      tracer: managed ? tracerProvider.getTracer(REQUEST_TRACER_NAME) : null,
    }),
  );

  const coreInstrumentations = [
    new LongTaskInstrumentation({
      observerCallback: (span, info: ObserverCallbackInformation) => {
        stampUrlAttrs(span, resolveRoute);

        const containerType = info.longtaskEntry.attribution[0]?.containerType;

        if (containerType) {
          span.setAttribute("longtask.container_type", containerType);
        }
      },
    }),
  ];

  const pageEventsInstrumentations = [
    new DocumentLoadInstrumentation({
      applyCustomAttributesOnSpan: {
        resourceFetch: (span, entry) => {
          const name = entry?.name;

          // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Resource timing callbacks can receive incomplete host entries; only string names can be parsed as URLs.
          if (typeof name !== "string") {
            return;
          }

          if (isUrlIgnored(name, ignoreUrls)) {
            excludeFromExport(span);

            return;
          }

          let url: URL;

          try {
            url = new URL(name);
          } catch {
            return;
          }

          const isCrossOrigin =
            typeof window !== "undefined" &&
            url.origin !== globalThis.location.origin;

          span.updateName(
            isCrossOrigin
              ? `resource: ${url.host}${url.pathname}`
              : `resource: ${url.pathname}`,
          );

          span.setAttribute(ATTR_URL_FULL, name);

          span.setAttribute(ATTR_URL_PATH, url.pathname);

          span.setAttribute("server.address", url.host);
        },
      },
      enabled: false,
    }),
  ];

  const unregisterCore = registerInstrumentations(
    omitUndefined({
      tracerProvider,
      loggerProvider,
      instrumentations: coreInstrumentations,
    }),
  );

  let unregisterPageEvents: (() => void) | null = null;

  const setPageEventsEnabled = (enabled: boolean) => {
    if (enabled) {
      unregisterPageEvents ??= registerInstrumentations(
        omitUndefined({
          tracerProvider: browserRootedTracerProvider(tracerProvider),
          loggerProvider,
          instrumentations: pageEventsInstrumentations,
        }),
      );

      return;
    }

    unregisterPageEvents?.();

    unregisterPageEvents = null;
  };

  return {
    dispose: () => {
      setPageEventsEnabled(false);

      requestObserver.dispose();

      unregisterCore();
    },
    setPageEventsEnabled,
  } satisfies BundledInstrumentations;
}
