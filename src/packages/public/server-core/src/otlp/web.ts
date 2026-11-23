import type { CollectorConfig } from "./options.js";
import { createLegacyOtlpBrowserExportDelegate } from "@opentelemetry/otlp-exporter-base/browser-http";
import type {
  IExporterMetricsHelper,
  ISerializer,
} from "@opentelemetry/otlp-transformer";
import { exporterComponents } from "./options.js";

/** Uses OpenTelemetry's fetch transport for the server adapter in a Web runtime. */
export function collectorDelegate<Items, Response>(
  signal: keyof typeof exporterComponents,
  config: CollectorConfig,
  serializer: ISerializer<Items, Response>,
  metrics: IExporterMetricsHelper<Items>,
) {
  // oxlint-disable-next-line typescript/no-deprecated -- This is OpenTelemetry's browser OTLP/HTTP exporter factory for the installed version.
  return createLegacyOtlpBrowserExportDelegate(
    config,
    serializer,
    exporterComponents[signal],
    metrics,
    undefined,
    `v1/${signal}`,
    { "Content-Type": "application/json" },
  );
}
