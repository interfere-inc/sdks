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
import { context, propagation, trace } from "@opentelemetry/api";
import type {
  Attributes,
  TextMapPropagator,
  TracerProvider,
} from "@opentelemetry/api";
import { SeverityNumber, logs } from "@opentelemetry/api-logs";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import { getStringFromEnv } from "@opentelemetry/core";
import { OTLPTraceExporter } from "@opentelemetry/exporter-trace-otlp-http";
import { OTLPLogExporter } from "@opentelemetry/exporter-logs-otlp-http";
import { CompressionAlgorithm } from "@opentelemetry/otlp-exporter-base";
import { resourceFromAttributes } from "@opentelemetry/resources";
import {
  BatchLogRecordProcessor,
  LoggerProvider,
} from "@opentelemetry/sdk-logs";
import type { LogRecordProcessor } from "@opentelemetry/sdk-logs";
import {
  AlwaysOnSampler,
  BasicTracerProvider,
  BatchSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import type {
  IdGenerator,
  Sampler,
  SpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import { NodeTracerProvider } from "@opentelemetry/sdk-trace-node";

import { bridgeConsoleToOtel } from "./console-bridge.js";
import { containSpanProcessor } from "./contained-span-processor.js";
import {
  EXISTING_PROPAGATOR_WARNING,
  EXISTING_PROVIDER_WARNING,
  registeredLoggerProvider,
  registeredTracerProvider,
  resolveTracingMode,
} from "./ownership.js";
import { ATTRIBUTE_VALUE_LENGTH_LIMIT } from "./runtime.js";
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
  publicKey: string;
  resourceAttributes: Attributes;
  options: ServerOptions;
  drain: boolean;
  logDrain: boolean;
  vercel: boolean;
  propagator: TextMapPropagator;
  idGenerator?: IdGenerator;
  registerPlatform?: (input: ManagedTracingInput) => Promise<TracerProvider>;
  instrument?: (
    provider: TracerProvider,
    resource: ReturnType<typeof resourceFromAttributes>,
    requests: boolean,
  ) => Pick<ServerPipeline, "flush" | "shutdown">;
}

function canFlush(
  provider: TracerProvider,
): provider is TracerProvider & Pick<BasicTracerProvider, "forceFlush"> {
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
  const headers = { "x-interfere-producer-version": input.version };

  const spanLimits =
    (getStringFromEnv("OTEL_SPAN_ATTRIBUTE_VALUE_LENGTH_LIMIT") ??
    getStringFromEnv("OTEL_ATTRIBUTE_VALUE_LENGTH_LIMIT"))
      ? undefined
      : { attributeValueLengthLimit: ATTRIBUTE_VALUE_LENGTH_LIMIT };

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
            new OTLPTraceExporter({
              headers,
              compression,
              timeoutMillis: attemptTimeout,
              url: drainUrl(input.apiUrl, "traces", input.publicKey),
            }),
            { exportTimeoutMillis: batchTimeout },
          ),
        ]),
    ...(options._internalAdditionalSpanProcessors ?? []),
  ];

  const sampler = exceptionSampler();

  let provider: TracerProvider;
  let ownedProvider: BasicTracerProvider | undefined;
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

      ownedProvider = new BasicTracerProvider(
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
      const nodeProvider = new NodeTracerProvider(
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

      nodeProvider.register({ contextManager: null, propagator: null });
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
              exporter: new OTLPLogExporter({
                headers,
                compression,
                timeoutMillis: attemptTimeout,
                url: drainUrl(input.apiUrl, "logs", input.publicKey),
              }),
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
