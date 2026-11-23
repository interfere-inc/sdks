// oxlint-disable-next-line typescript/triple-slash-reference
/// <reference path="../../vite-env.d.ts" />
import { drainUrl } from "@interfere/constants/api";
import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { installGlobalHandlers } from "@interfere/server-core/global-handlers";
import {
  JsonMetricsSerializer,
  MetricsExporterMetricsHelper,
} from "@opentelemetry/otlp-transformer";
import { collectorDelegate } from "@interfere/server-core/otlp-http";
import { startNodeRuntime } from "@interfere/server-core/node";
import {
  serverCredential,
  serverCredentialHeaders,
} from "@interfere/server-core/credential";
import { resolveLogSourcingMode } from "@interfere/types/sdk/log-sourcing";
import { resolveServerTracing } from "@interfere/types/sdk/server-tracing";
import { metrics } from "@opentelemetry/api";
import {
  CompositePropagator,
  W3CBaggagePropagator,
  W3CTraceContextPropagator,
  getStringFromEnv,
} from "@opentelemetry/core";
import {
  AggregationTemporalityPreference,
  OTLPMetricExporterBase,
} from "@opentelemetry/exporter-metrics-otlp-http";
import { registerInstrumentations } from "@opentelemetry/instrumentation";
import { UndiciInstrumentation } from "@opentelemetry/instrumentation-undici";
import { defaultServiceName } from "@opentelemetry/resources";
import {
  MeterProvider,
  PeriodicExportingMetricReader,
} from "@opentelemetry/sdk-metrics";

import { PRODUCER_VERSION } from "../../version";
import { captureError } from "./capture";
import { resolveReleaseSlug } from "../release-slug";
import { readInterfereServerEnv } from "./env";
import { FreshnessStampingPropagator } from "./freshness-propagator";
import { RequestPropagation } from "./request-propagation";
import type { ServerInstrumentationOptions } from "./instrumentation-options";
import { ensureRemoteConfig } from "./remote-config";
import { runtime } from "./runtime";

function resolveServiceName(opts: ServerInstrumentationOptions) {
  return (
    opts.serviceName ||
    getStringFromEnv("OTEL_SERVICE_NAME") ||
    import.meta.env.VITE_INTERFERE_SERVICE_NAME ||
    defaultServiceName()
  );
}

/** Flushes Interfere's server providers within the platform export budget. */
export async function flush() {
  await runtime.flush();
}

/** Initializes one Node server runtime; concurrent callers share startup and failures never reject. */
export async function register(opts: ServerInstrumentationOptions = {}) {
  await runtime.register(async () => {
    const env = readInterfereServerEnv();

    const credential = serverCredential(
      env.apiKey,
      env.releaseToken,
      env.publicKey,
    );

    if (!credential) {
      return null;
    }

    const { slug } = resolveReleaseSlug();

    if (!slug) {
      console.warn(
        "[interfere] No commit SHA available; server spans will ship without `release.slug`. Set `INTERFERE_SOURCE_ID` (or any of `VERCEL_GIT_COMMIT_SHA`, `GITHUB_SHA`) on the runtime env.",
      );
    }

    const tracing = resolveServerTracing({
      injected: import.meta.env.VITE_INTERFERE_SERVER_TRACING,
      platform: import.meta.env.VITE_INTERFERE_PLATFORM,
    });

    const serviceName = resolveServiceName(opts);

    const requestPropagation = new RequestPropagation(
      new CompositePropagator({
        propagators: [
          new W3CTraceContextPropagator(),
          new FreshnessStampingPropagator(),
          new W3CBaggagePropagator(),
        ],
      }),
      opts.propagateContextUrls ?? [],
    );

    const ignoreUrls = [
      ...(["traces", "logs", "metrics"] as const).map((signal) =>
        drainUrl(env.apiUrl, signal),
      ),
      `${env.apiUrl}/v3/config`,
      ...(opts.ignoreUrls ?? []),
    ];

    const pipeline = await startNodeRuntime({
      name: "@interfere/vite",
      version: PRODUCER_VERSION,
      apiUrl: env.apiUrl,
      credential,
      options: opts,
      drain: tracing.mode === "drain",
      logDrain:
        resolveLogSourcingMode(import.meta.env.VITE_INTERFERE_LOG_SOURCING) ===
        "drain",
      vercel: tracing.vercel,
      propagator: requestPropagation,
      resourceAttributes: omitUndefined({
        "deployment.environment.name": env.nodeEnvironment,
        "interfere.sdk.name": "@interfere/vite",
        "interfere.sdk.version": PRODUCER_VERSION,
        "service.name": serviceName,
        "service.namespace": opts.serviceNamespace,
        "telemetry.sdk.language": "nodejs",
        "release.slug": slug || undefined,
      }),
      instrument: (tracerProvider, resource, requests) => {
        const meterProvider = new MeterProvider({
          resource,
          readers: [
            new PeriodicExportingMetricReader({
              exporter: new OTLPMetricExporterBase(
                collectorDelegate(
                  "metrics",
                  {
                    url: drainUrl(env.apiUrl, "metrics"),
                    headers: {
                      ...serverCredentialHeaders(credential),
                      "x-interfere-producer-version": PRODUCER_VERSION,
                    },
                  },
                  JsonMetricsSerializer,
                  MetricsExporterMetricsHelper,
                ),
                {
                  temporalityPreference: AggregationTemporalityPreference.DELTA,
                },
              ),
              exportIntervalMillis: 30_000,
            }),
          ],
        });

        metrics.setGlobalMeterProvider(meterProvider);

        const stop = requests
          ? registerInstrumentations({
              tracerProvider,
              meterProvider,
              instrumentations: [
                new UndiciInstrumentation({
                  ignoreRequestHook: (request) =>
                    ignoreUrls.some((pattern) =>
                      pattern instanceof RegExp
                        ? new RegExp(pattern.source, pattern.flags).test(
                            `${request.origin}${request.path}`,
                          )
                        : `${request.origin}${request.path}`.includes(pattern),
                    ),
                  requestHook: (span, request) =>
                    requestPropagation.observe(
                      span,
                      `${request.origin}${request.path}`,
                    ),
                }),
              ],
            })
          : undefined;

        return {
          flush: async () => meterProvider.forceFlush(),
          shutdown: async () => {
            stop?.();

            if (metrics.getMeterProvider() === meterProvider) {
              metrics.disable();
            }

            await meterProvider.shutdown();
          },
        };
      },
    });

    installGlobalHandlers({
      captureUncaughtException: opts.captureUncaughtException ?? true,
      captureUnhandledRejection: opts.captureUnhandledRejection ?? true,
      capture: (error, mechanism) => {
        captureError(error, { mechanism });
      },
      flush,
    });

    return pipeline;
  });

  await ensureRemoteConfig({
    serviceName: resolveServiceName(opts),
    serviceNamespace: opts.serviceNamespace,
  });
}
