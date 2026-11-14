import type { Attributes } from "@opentelemetry/api";
import type { ExportResult } from "@opentelemetry/core";
import { JsonMetricsSerializer } from "@opentelemetry/otlp-transformer";
import { resourceFromAttributes } from "@opentelemetry/resources";
import type {
  PushMetricExporter,
  ResourceMetrics,
} from "@opentelemetry/sdk-metrics";
import { AggregationTemporality } from "@opentelemetry/sdk-metrics";

import type { SignalSender } from "./exporter.js";
import { createInFlightExports } from "./transport/in-flight-exports.js";

export interface GatedResourceAttributes {
  attributes: Attributes;
  isEnabled: () => boolean;
}

export class IngestMetricExporter implements PushMetricExporter {
  private readonly gatedResourceAttributes: GatedResourceAttributes | null;

  private readonly inFlight = createInFlightExports();

  private readonly send: SignalSender;

  constructor(
    send: SignalSender,
    gatedResourceAttributes?: GatedResourceAttributes,
  ) {
    this.send = send;

    this.gatedResourceAttributes = gatedResourceAttributes ?? null;
  }

  export(
    metrics: ResourceMetrics,
    resultCallback: (result: ExportResult) => void,
  ) {
    const gated = this.gatedResourceAttributes;

    const payload =
      gated?.isEnabled() === true
        ? {
            ...metrics,
            resource: metrics.resource.merge(
              resourceFromAttributes(gated.attributes),
            ),
          }
        : metrics;

    this.send(
      // oxlint-disable-next-line typescript/no-unsafe-enum-assignment -- SDK and transformer versions use identical OTLP enum values.
      JsonMetricsSerializer.serializeRequest(payload),
      this.inFlight.track(resultCallback),
    );
  }

  selectAggregationTemporality() {
    return AggregationTemporality.DELTA;
  }

  async forceFlush() {
    await this.inFlight.settled();
  }

  async shutdown() {
    await this.inFlight.settled();

    this.send.dispose?.();
  }
}
