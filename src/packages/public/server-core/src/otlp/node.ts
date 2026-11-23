import type { CollectorConfig } from "./options.js";
import { exporterComponents } from "./options.js";
import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { promisify } from "node:util";
import { gzip } from "node:zlib";
import {
  OTLPExporterError,
  createOtlpNetworkExportDelegate,
} from "@opentelemetry/otlp-exporter-base";
import {
  convertLegacyHttpOptions,
  createOtlpHttpExporterMetrics,
} from "@opentelemetry/otlp-exporter-base/node-http";
import type { createOtlpHttpExportDelegate } from "@opentelemetry/otlp-exporter-base/node-http";
import type {
  IExporterMetricsHelper,
  ISerializer,
} from "@opentelemetry/otlp-transformer";

const gzipPayload = promisify(gzip);
const maxResponseSize = 4 * 1024 * 1024;

type CollectorOptions = Parameters<typeof createOtlpHttpExportDelegate>[0];

async function collectorClient(url: URL, options: CollectorOptions) {
  const [{ Pool, RetryAgent }, { Agent }, agent] = await Promise.all([
    import("undici/index.js"),
    import("node:https"),
    options.agentFactory(url.protocol),
  ]);

  const { ca, cert, key } = agent instanceof Agent ? agent.options : {};
  const connect = omitUndefined({ ca, cert, key });

  return new RetryAgent(
    new Pool(url.origin, {
      connect,
      connections: options.concurrencyLimit,
      maxResponseSize,
    }),
    {
      methods: ["POST"],
      statusCodes: [429, 502, 503, 504],
      errorCodes: [
        "ECONNRESET",
        "ECONNREFUSED",
        "EPIPE",
        "ETIMEDOUT",
        "EAI_AGAIN",
        "ENOTFOUND",
        "ENETUNREACH",
        "EHOSTUNREACH",
        "UND_ERR_SOCKET",
        "UND_ERR_CONNECT_TIMEOUT",
      ],
      minTimeout: 1000,
      timeoutFactor: 1.5,
      maxTimeout: options.timeoutMillis,
      throwOnError: false,
    },
  );
}

/** Sends OTLP/HTTP JSON to the collector on an exporter-owned, protocol-negotiated connection. */
export function collectorDelegate<Items, Response>(
  signal: keyof typeof exporterComponents,
  config: CollectorConfig,
  serializer: ISerializer<Items, Response>,
  metrics: IExporterMetricsHelper<Items>,
) {
  // oxlint-disable-next-line typescript/no-deprecated -- The installed OTel Node exporters use this factory for signal-specific headers, certificates, compression and timeouts.
  const options = convertLegacyHttpOptions(
    config,
    signal.toUpperCase(),
    `v1/${signal}`,
    {
      "Content-Type": "application/json",
    },
  );

  const url = new URL(options.url);
  const path = url.pathname + url.search;
  let opening: ReturnType<typeof collectorClient> | undefined;
  let client: Awaited<ReturnType<typeof collectorClient>> | undefined;

  async function open() {
    try {
      client = await collectorClient(url, options);

      return client;
    } finally {
      opening = undefined;
    }
  }

  let closing: Promise<void> | undefined;

  const delegate = createOtlpNetworkExportDelegate(
    options,
    serializer,
    createOtlpHttpExporterMetrics(
      exporterComponents[signal],
      metrics,
      options.url,
      undefined,
    ),
    {
      async send(data, timeoutMillis) {
        const deadline = AbortSignal.timeout(
          Math.max(0, Math.ceil(timeoutMillis)),
        );

        const [dispatcher, configuredHeaders, body] = await Promise.all([
          client ?? (opening ??= open()),
          options.headers(),
          options.compression === "gzip" ? gzipPayload(data) : data,
        ]);

        const headers = Object.fromEntries(
          Object.entries(configuredHeaders).map(([key, value]) => [
            key.toLowerCase(),
            value,
          ]),
        );

        if (options.compression === "gzip") {
          headers["content-encoding"] = "gzip";
        }

        headers["user-agent"] =
          `Interfere-OTLP-Exporter-JavaScript/${config.headers["x-interfere-producer-version"]}`;

        const response = await dispatcher.request({
          path,
          origin: url.origin,
          method: "POST",
          headers,
          body,
          signal: deadline,
        });

        const bytes = await response.body.bytes();

        return response.statusCode < 300
          ? { status: "success", data: bytes }
          : {
              status: "failure",
              error: new OTLPExporterError(
                `Collector returned HTTP ${response.statusCode}`,
                response.statusCode,
                Buffer.from(bytes).toString(),
              ),
            };
      },
      shutdown() {
        closing = client?.close();
      },
    },
  );

  return {
    export: delegate.export.bind(delegate),
    forceFlush: delegate.forceFlush.bind(delegate),
    setMetrics: delegate.setMetrics.bind(delegate),
    async shutdown() {
      await delegate.shutdown();

      await closing;
    },
  };
}
