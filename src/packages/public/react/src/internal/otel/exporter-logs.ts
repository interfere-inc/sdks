import type { ExportResult } from "@opentelemetry/core";
import { ExportResultCode } from "@opentelemetry/core";
import { JsonLogsSerializer } from "@opentelemetry/otlp-transformer";
import type {
  LogRecordExporter,
  ReadableLogRecord,
} from "@opentelemetry/sdk-logs";

import type { SignalSender } from "./exporter.js";
import { createInFlightExports } from "./transport/in-flight-exports.js";
import { sendInKeepaliveChunks } from "./keepalive-chunks.js";
import { noteLogsDecided, noteLogsExporting } from "./transport/leave-gate.js";

export class IngestLogExporter implements LogRecordExporter {
  private readonly send: SignalSender;

  private readonly inFlight = createInFlightExports();

  constructor(send: SignalSender) {
    this.send = send;
  }

  export(
    logs: ReadableLogRecord[],
    resultCallback: (result: ExportResult) => void,
  ) {
    if (logs.length === 0) {
      resultCallback({ code: ExportResultCode.SUCCESS });

      return;
    }

    noteLogsExporting();

    sendInKeepaliveChunks(
      logs,
      // oxlint-disable-next-line typescript/no-unsafe-enum-assignment -- SDK and transformer versions use identical OTLP enum values.
      (chunk) => JsonLogsSerializer.serializeRequest(chunk),
      this.send,
      this.inFlight.track((result) => {
        noteLogsDecided();

        resultCallback(result);
      }),
    );
  }

  async shutdown() {
    await this.inFlight.settled();

    this.send.dispose?.();
  }

  async forceFlush() {
    await this.inFlight.settled();
  }
}
