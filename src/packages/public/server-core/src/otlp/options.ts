import type { OTLPExporterNodeConfigBase } from "@opentelemetry/otlp-exporter-base/node-http";

export type CollectorConfig = Pick<
  OTLPExporterNodeConfigBase,
  "url" | "compression" | "timeoutMillis"
> & {
  headers: Record<string, string> & { "x-interfere-producer-version": string };
};

export const exporterComponents = {
  traces: "otlp_http_span_exporter",
  logs: "otlp_http_log_exporter",
  metrics: "otlp_http_metric_exporter",
} as const;
