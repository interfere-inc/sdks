import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { startNodeRuntime } from "@interfere/server-core/node";
import { serverCredential } from "@interfere/server-core/credential";
import { containAsync } from "@interfere/server-core/contain";
import { registeredTracerProvider } from "@interfere/server-core/ownership";
import { readLogSourcingMode } from "@interfere/types/sdk/log-sourcing";
import { readBakedBackendHosts } from "@interfere/types/sdk/remote-config/backend-hosts";
import { readServerTracing } from "@interfere/types/sdk/server-tracing";
import { CompositePropagator, getStringFromEnv } from "@opentelemetry/core";
import { defaultServiceName } from "@opentelemetry/resources";
import type { Configuration } from "@vercel/otel";

import { readInterfereEnv } from "./internal/env.js";
import { resolveReleaseSlug } from "./internal/release-slug.js";
import { managedPropagators } from "./internal/server/tracing/managed-propagators.js";
import { serverPropagationTargets } from "./internal/server/tracing/propagation-targets.js";
import { installGlobalHandlers } from "./internal/server/capture/global-handlers.js";
import { nextSpanExceptionProcessor } from "./internal/server/capture/next-span-exceptions.js";
import { PrerenderSafeIdGenerator as ManagedIdGenerator } from "./internal/server/tracing/id-generator.js";
import type { ServerInstrumentationOptions } from "./internal/server/instrumentation-options.js";
import { setServerFlush } from "./internal/server/exporters/server-flush.js";
import { PRODUCER_VERSION } from "./internal/version.js";
import { fetchAndCacheRemoteConfig } from "./internal/server/remote-config.js";
import { runtime } from "./internal/server/runtime.js";

export { onRequestError } from "./internal/server/capture/capture.js";

export { correlationSpanProcessor } from "@interfere/types/sdk/correlation-span-processor";

export type { ServerInstrumentationOptions } from "./internal/server/instrumentation-options.js";

/** Flushes Interfere's server providers within the platform export budget. */
export async function flush() {
  await runtime.flush();
}

/** Initializes one server runtime; concurrent callers share startup and SDK failures never reject. */
export async function register(opts: ServerInstrumentationOptions = {}) {
  await runtime.register(async () => {
    if (getStringFromEnv("NEXT_PHASE") === "phase-production-build") {
      return null;
    }

    const env = readInterfereEnv();
    const tracing = readServerTracing();
    const credential = serverCredential(env.apiKey, env.publicKey);

    if (
      !credential ||
      (tracing.mode === "drain" &&
        getStringFromEnv("OTEL_SDK_DISABLED")?.trim())
    ) {
      return null;
    }

    const { slug } = resolveReleaseSlug();

    if (!slug) {
      console.warn(
        "[interfere] No commit SHA available; server spans will ship without `release.slug`. Set `INTERFERE_SOURCE_ID` (or any of `VERCEL_GIT_COMMIT_SHA`, `GITHUB_SHA`) on the runtime env.",
      );
    }

    const idGenerator = new ManagedIdGenerator();
    const propagators = managedPropagators();

    const pipeline = await startNodeRuntime({
      name: "@interfere/next",
      version: PRODUCER_VERSION,
      apiUrl: env.apiUrl,
      credential,
      options: {
        ...opts,
        _internalAdditionalSpanProcessors: [
          nextSpanExceptionProcessor,
          ...(opts._internalAdditionalSpanProcessors ?? []),
        ],
      },
      drain: tracing.mode === "drain",
      logDrain: readLogSourcingMode() === "drain",
      vercel: tracing.vercel,
      idGenerator,
      propagator: new CompositePropagator({ propagators }),
      resourceAttributes: omitUndefined({
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
        "release.slug": slug || undefined,
      }),
      registerPlatform: async ({
        resourceAttributes,
        spanProcessors,
        sampler,
      }) => {
        const { registerOTel } = await import("@vercel/otel");

        const config: Configuration = {
          serviceName: String(resourceAttributes["service.name"]),
          attributes: resourceAttributes,
          idGenerator,
          instrumentations: [],
          propagators: ["auto", ...propagators],
          spanProcessors: ["auto", ...spanProcessors],
          traceSampler: sampler ?? "auto",
        };

        const targets = serverPropagationTargets([
          ...(readBakedBackendHosts() ?? []).map(
            (host) =>
              `https://${host.trim().toLowerCase().replace(/\.$/u, "")}`,
          ),
          ...(opts.propagateTo ?? []),
        ]);

        if (targets.length > 0 || opts.propagateTo !== undefined) {
          config.instrumentations = ["fetch"];

          config.instrumentationConfig = {
            fetch: { propagateContextUrls: targets },
          };
        }

        registerOTel(config);

        const provider = registeredTracerProvider();

        if (!provider) {
          throw new Error("Platform tracing did not register a provider");
        }

        return provider;
      },
    });

    installGlobalHandlers({
      captureUncaughtException: opts.captureUncaughtException ?? true,
      captureUnhandledRejection: opts.captureUnhandledRejection ?? true,
      flush,
    });

    setServerFlush(flush);

    void containAsync("remote config", fetchAndCacheRemoteConfig);

    return pipeline;
  });
}
