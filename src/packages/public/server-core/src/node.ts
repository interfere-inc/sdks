import { drainUrl } from "@interfere/constants/api";
import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { correlationSpanProcessor } from "@interfere/types/sdk/correlation-span-processor";
import { VERCEL_EXPORT_TIMEOUT_MS } from "@interfere/types/sdk/server-tracing";
import {
  createUrlRedactionSpanProcessor,
  resolveUrlRedaction,
} from "@interfere/types/sdk/url-redaction";
import type { UrlRedactionOption } from "@interfere/types/sdk/url-redaction";
import type { TracingOwnership } from "@interfere/types/sdk/remote-config/tracing-ownership";
import {
  SeverityNumber,
  context,
  logs,
  propagation,
  trace,
} from "@opentelemetry/api";
import type {
  TracerProvider as ApiTracerProvider,
  Attributes,
  TextMapPropagator,
} from "@opentelemetry/api";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import { getNumberFromEnv } from "@opentelemetry/core";
import {
  JsonLogsSerializer,
  JsonTraceSerializer,
  LogsExporterMetricsHelper,
  TraceExporterMetricsHelper,
} from "@opentelemetry/otlp-transformer";
import { collectorDelegate } from "#otlp-http";
import {
  CompressionAlgorithm,
  OTLPExporterBase,
} from "@opentelemetry/otlp-exporter-base";
import { resourceFromAttributes } from "@opentelemetry/resources";
import {
  BatchLogRecordProcessor,
  LoggerProvider,
} from "@opentelemetry/sdk-logs";
import type { LogRecordProcessor } from "@opentelemetry/sdk-logs";
import {
  AlwaysOnSampler,
  BatchSpanProcessor,
  TracerProvider,
} from "@opentelemetry/sdk-trace";
import type {
  IdGenerator,
  Sampler,
  SpanProcessor,
} from "@opentelemetry/sdk-trace";

import { bridgeConsoleToOtel } from "./console-bridge.js";
import { serverCredentialHeaders } from "./credential.js";
import type { ServerCredential } from "./credential.js";
import { containSpanProcessor } from "./contained-span-processor.js";
import {
  EXISTING_PROPAGATOR_WARNING,
  EXISTING_PROVIDER_WARNING,
  registeredLoggerProvider,
  registeredTracerProvider,
  resolveTracingMode,
} from "./ownership.js";
import {
  ATTRIBUTE_VALUE_LENGTH_LIMIT,
  configuredTraceOptions,
} from "./runtime.js";
import type { ServerPipeline } from "./runtime.js";
import { exceptionSampler } from "./sampler.js";

export interface ServerOptions {
  /** Forward console calls at or above the selected level. Defaults to debug. */
  consoleBridge?: boolean | { level?: "debug" | "info" | "warn" | "error" };
  /** Deliver exceptions through Interfere or the application provider. Defaults to Interfere. */
  errors?: "interfere" | "host";
  /** Privacy controls applied before captured telemetry is exported. */
  privacy?: {
    urls?: UrlRedactionOption;
    /** Capture forwarded client IPs with server errors. Defaults to true. */
    clientIp?: boolean;
  };
  /** Adopt an existing application provider by default; otherwise manage one. */
  tracing?: TracingOwnership | "auto";
  /** OTLP compression. Defaults to none. */
  compression?: "gzip" | "none";
  _internalAdditionalSpanProcessors?: SpanProcessor[];
  _internalAdditionalLogRecordProcessors?: LogRecordProcessor[];
}

export interface ManagedTracingInput {
  resourceAttributes: Attributes;
  spanProcessors: SpanProcessor[];
  sampler: Sampler | undefined;
}

export interface NodeRuntimeInput {
  name: string;
  version: string;
  apiUrl: string;
  credential: ServerCredential;
  resourceAttributes: Attributes;
  options: ServerOptions;
  drain: boolean;
  logDrain: boolean;
  vercel: boolean;
  propagator: TextMapPropagator;
  idGenerator?: IdGenerator;
  registerPlatform?: (input: ManagedTracingInput) => Promise<ApiTracerProvider>;
  instrument?: (
    provider: ApiTracerProvider,
    resource: ReturnType<typeof resourceFromAttributes>,
    requests: boolean,
  ) => Pick<ServerPipeline, "flush" | "shutdown">;
}

function canFlush(
  provider: ApiTracerProvider,
): provider is ApiTracerProvider & Pick<TracerProvider, "forceFlush"> {
  return "forceFlush" in provider && typeof provider.forceFlush === "function";
}

/** Builds the owned server pipeline; framework hooks supply only platform registration and request instrumentation. */
export async function startNodeRuntime(input: NodeRuntimeInput) {
  const { options, resourceAttributes } = input;
  const mode = resolveTracingMode(options.tracing);
  const existing = mode === "managed" ? registeredTracerProvider() : null;
  const ownsTracing = mode === "managed" && existing === null;
  const drain = input.drain && ownsTracing;
  const resource = resourceFromAttributes(resourceAttributes);
  const urlRedaction = resolveUrlRedaction(options.privacy?.urls);
  const attemptTimeout = input.vercel ? VERCEL_EXPORT_TIMEOUT_MS : 65_000;
  const batchTimeout = input.vercel ? VERCEL_EXPORT_TIMEOUT_MS : 70_000;
  const flushTimeout = input.vercel ? VERCEL_EXPORT_TIMEOUT_MS : 75_000;

  const headers = {
    ...serverCredentialHeaders(input.credential),
    "x-interfere-producer-version": input.version,
  };

  const { sampler: headSampler, spanLimits } = configuredTraceOptions(
    ATTRIBUTE_VALUE_LENGTH_LIMIT,
  );

  const compression =
    options.compression === "gzip"
      ? CompressionAlgorithm.GZIP
      : CompressionAlgorithm.NONE;

  const spanProcessors: SpanProcessor[] = [
    containSpanProcessor(createUrlRedactionSpanProcessor(urlRedaction)),
    correlationSpanProcessor(),
    ...(drain
      ? []
      : [
          new BatchSpanProcessor(
            omitUndefined({
              exporter: new OTLPExporterBase(
                collectorDelegate(
                  "traces",
                  {
                    headers,
                    compression,
                    timeoutMillis: attemptTimeout,
                    url: drainUrl(input.apiUrl, "traces"),
                  },
                  JsonTraceSerializer,
                  TraceExporterMetricsHelper,
                ),
              ),
              exportTimeoutMillis: batchTimeout,
              maxExportBatchSize: getNumberFromEnv(
                "OTEL_BSP_MAX_EXPORT_BATCH_SIZE",
              ),
              maxQueueSize: getNumberFromEnv("OTEL_BSP_MAX_QUEUE_SIZE"),
              scheduledDelayMillis: getNumberFromEnv("OTEL_BSP_SCHEDULE_DELAY"),
            }),
          ),
        ]),
    ...(options._internalAdditionalSpanProcessors ?? []),
  ];

  const sampler = exceptionSampler(headSampler);

  let provider: ApiTracerProvider;
  let ownedProvider: TracerProvider | undefined;
  let loggerProvider: LoggerProvider | undefined;
  let ownsPropagator = false;
  let contextManager: AsyncLocalStorageContextManager | undefined;
  let instrumentation: Pick<ServerPipeline, "flush" | "shutdown"> | undefined;
  let restoreConsole: (() => void) | undefined;

  async function shutdown() {
    restoreConsole?.();

    await instrumentation?.shutdown();

    if (ownedProvider && registeredTracerProvider() === ownedProvider) {
      trace.disable();
    }

    if (ownsPropagator) {
      propagation.disable();
    }

    if (contextManager) {
      context.disable();

      contextManager.disable();
    }

    if (loggerProvider && logs.getLoggerProvider() === loggerProvider) {
      logs.disable();
    }

    await Promise.all([ownedProvider?.shutdown(), loggerProvider?.shutdown()]);
  }

  try {
    if (!ownsTracing) {
      if (existing) {
        console.warn(EXISTING_PROVIDER_WARNING);
      }

      ownedProvider = new TracerProvider(
        omitUndefined({
          resource,
          idGenerator: input.idGenerator,
          sampler: new AlwaysOnSampler(),
          spanLimits,
          spanProcessors,
        }),
      );

      provider = ownedProvider;
    } else if (drain && input.registerPlatform) {
      provider = await input.registerPlatform({
        resourceAttributes,
        spanProcessors,
        sampler,
      });

      ownsPropagator = false;
    } else {
      const nodeProvider = new TracerProvider(
        omitUndefined({
          resource,
          idGenerator: input.idGenerator,
          sampler,
          spanLimits,
          spanProcessors,
        }),
      );

      ownedProvider = nodeProvider;

      provider = nodeProvider;

      ownsPropagator = propagation.setGlobalPropagator(input.propagator);

      if (!ownsPropagator) {
        console.warn(EXISTING_PROPAGATOR_WARNING);
      }

      const manager = new AsyncLocalStorageContextManager().enable();

      if (context.setGlobalContextManager(manager)) {
        contextManager = manager;
      } else {
        manager.disable();
      }

      trace.setGlobalTracerProvider(nodeProvider);
    }

    const hostLogger =
      !ownsTracing && mode === "application"
        ? registeredLoggerProvider()
        : null;

    const bridgeTarget = input.logDrain
      ? logs
      : (hostLogger ??
        (loggerProvider = new LoggerProvider({
          resource,
          processors: [
            new BatchLogRecordProcessor({
              exportTimeoutMillis: batchTimeout,
              exporter: new OTLPExporterBase(
                collectorDelegate(
                  "logs",
                  {
                    headers,
                    compression,
                    timeoutMillis: attemptTimeout,
                    url: drainUrl(input.apiUrl, "logs"),
                  },
                  JsonLogsSerializer,
                  LogsExporterMetricsHelper,
                ),
              ),
            }),
            ...(options._internalAdditionalLogRecordProcessors ?? []),
          ],
        })));

    if (ownsTracing && loggerProvider) {
      logs.setGlobalLoggerProvider(loggerProvider);
    }

    if (ownsTracing) {
      instrumentation = input.instrument?.(
        provider,
        resource,
        ownsPropagator && !drain,
      );
    }

    if (!input.logDrain && options.consoleBridge !== false) {
      const level =
        options.consoleBridge !== undefined && options.consoleBridge !== true
          ? (options.consoleBridge.level ?? "debug")
          : "debug";

      restoreConsole = bridgeConsoleToOtel(
        bridgeTarget,
        {
          debug: SeverityNumber.DEBUG,
          info: SeverityNumber.INFO,
          warn: SeverityNumber.WARN,
          error: SeverityNumber.ERROR,
        }[level],
        `${input.name}/console`,
      );
    }

    return {
      mode,
      provider,
      isolated: !ownsTracing,
      capture: {
        errors: options.errors ?? "interfere",
        recordOnHostSpan: mode === "application",
        urlRedaction,
        clientIp: options.privacy?.clientIp !== false,
      },
      flush: async () => {
        await Promise.all([
          canFlush(provider)
            ? provider.forceFlush({ timeoutMillis: flushTimeout })
            : undefined,
          loggerProvider?.forceFlush({ timeoutMillis: flushTimeout }),
          instrumentation?.flush(),
        ]);
      },
      shutdown,
    } satisfies ServerPipeline;
  } catch (error) {
    await shutdown();

    throw error;
  }
}
