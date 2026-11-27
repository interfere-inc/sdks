import { getGlobal } from "@interfere/react/util/global";
import { FORCE_ENABLE_GLOBAL } from "@interfere/types/sdk/injected-globals";
import { getStringFromEnv } from "@interfere/server-core/otel/core/environment";
/* oxlint-disable unicorn/no-useless-promise-resolve-reject */
import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { LOCAL_DEV_RELEASE_SLUG } from "@interfere/types/releases/slug";
import {
  createUrlRedactionSpanProcessor,
  resolveUrlRedaction,
} from "@interfere/types/sdk/url-redaction";

import { trace } from "@opentelemetry/api";
import {
  defaultServiceName,
  resourceFromAttributes,
} from "@interfere/server-core/otel/resources";
import { SimpleSpanProcessor } from "@interfere/server-core/otel/sdk/trace/processing";
import { TracerProvider } from "@interfere/server-core/otel/sdk/trace/provider";
import { AlwaysOnSampler } from "@interfere/server-core/otel/sdk/trace/sampling";

import { resolveReleaseSlug } from "./internal/release-slug.js";
import { readInterfereEnv } from "./internal/env.js";
import { runtime } from "./internal/server/runtime.js";
import { configuredTraceOptions } from "@interfere/server-core/runtime";
import { contain } from "@interfere/server-core/contain";
import { FetchTraceExporter } from "./internal/server/exporters/edge-exporter.js";
import type { ServerInstrumentationOptions } from "./internal/server/instrumentation-options.js";
import { setServerFlush } from "./internal/server/exporters/server-flush.js";
import { containSpanProcessor } from "@interfere/server-core/contained-span-processor";
import { nextSpanExceptionProcessor } from "./internal/server/capture/next-span-exceptions.js";
import {
  EXISTING_PROVIDER_WARNING,
  registeredTracerProvider,
  resolveTracingMode,
} from "@interfere/server-core/ownership";
import { PRODUCER_VERSION } from "./internal/version.js";

export { onRequestError } from "./internal/server/capture/capture.js";

export { correlationSpanProcessor } from "@interfere/types/sdk/correlation-span-processor";

export type { ServerInstrumentationOptions } from "./internal/server/instrumentation-options.js";

let registered = false;

function edgeExportCredential(env: {
  readonly apiKey: string | null;
  readonly releaseToken: string | null;
}) {
  if (env.apiKey) {
    return { workspaceTelemetryKeyHeader: { apiKey: env.apiKey } };
  }

  return env.releaseToken
    ? { releaseTokenHeader: { apiKey: env.releaseToken } }
    : null;
}

/**
 * Edge-runtime bootstrap for `@interfere/next/instrumentation`. Selected by
 * `package.json`'s `edge-light` / `workerd` / `worker` export conditions when
 * Next.js compiles `instrumentation.ts` for the Edge runtime.
 *
 * The Node entry's pipeline (`TracerProvider`, `AsyncLocalStorage`, undici,
 * console bridge) can't run on edge, so this builds an edge-safe pipeline
 * instead: a
 * `TracerProvider` + `SimpleSpanProcessor` exporting via {@link FetchTraceExporter}.
 * Edge always exports directly because Vercel Trace Drains do not forward custom Edge spans.
 * `tracing: "auto"` (default) adopts a registered provider, otherwise uses managed tracing.
 * `tracing: "managed"` registers that provider globally unless
 * another provider is already registered; `tracing: "application"` keeps it
 * as an isolated capture provider and writes no OpenTelemetry global.
 * Never rejects: an Interfere failure logs one warning and the app runs without Interfere.
 */
export async function register(opts: ServerInstrumentationOptions = {}) {
  contain("register()", () => {
    start(opts);
  });

  return Promise.resolve();
}

function start(opts: ServerInstrumentationOptions) {
  if (registered) {
    return;
  }

  const env = readInterfereEnv();

  const credential = edgeExportCredential(env);

  if (!credential) {
    return;
  }

  const releaseSlug =
    getGlobal(FORCE_ENABLE_GLOBAL) === true
      ? LOCAL_DEV_RELEASE_SLUG
      : resolveReleaseSlug().slug;

  if (!releaseSlug) {
    console.warn(
      "[interfere] edge runtime has no release.slug; configure a build release or INTERFERE_SOURCE_ID.",
    );
  }

  const mode = resolveTracingMode(opts.tracing);

  const existingProvider =
    mode === "managed" ? registeredTracerProvider() : null;

  const isolated = mode === "application" || existingProvider !== null;
  const urlRedaction = resolveUrlRedaction(opts.privacy?.urls);

  const { sampler, spanLimits } = configuredTraceOptions();

  const provider = new TracerProvider(
    omitUndefined({
      sampler: isolated ? new AlwaysOnSampler() : sampler,
      spanLimits,
      resource: resourceFromAttributes(
        omitUndefined({
          "deployment.environment.name": env.nodeEnvironment,
          "interfere.sdk.name": "@interfere/next",
          "interfere.sdk.version": PRODUCER_VERSION,
          "service.name":
            opts.serviceName ||
            getStringFromEnv("OTEL_SERVICE_NAME") ||
            env.serviceName ||
            defaultServiceName(),
          "service.namespace": opts.serviceNamespace,
          "telemetry.sdk.language": "nodejs",
          "release.slug": releaseSlug || undefined,
        }),
      ),
      spanProcessors: [
        containSpanProcessor(createUrlRedactionSpanProcessor(urlRedaction)),
        nextSpanExceptionProcessor,
        new SimpleSpanProcessor({
          exporter: new FetchTraceExporter({
            baseUrl: env.apiUrl,
            ...credential,
            headers: {
              "x-interfere-producer-version": PRODUCER_VERSION,
            },
          }),
        }),
      ],
    }),
  );

  if (isolated) {
    if (existingProvider !== null) {
      console.warn(EXISTING_PROVIDER_WARNING);
    }
  } else {
    trace.setGlobalTracerProvider(provider);
  }

  runtime.configureCapture(isolated ? provider : null, {
    errors: opts.errors ?? "interfere",
    recordOnHostSpan: mode === "application",
    urlRedaction,
    clientIp: opts.privacy?.clientIp !== false,
  });

  setServerFlush(async () => provider.forceFlush({ timeoutMillis: 75_000 }));

  registered = true;
}
