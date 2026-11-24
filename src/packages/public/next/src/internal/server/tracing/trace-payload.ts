import type { InterfereClient } from "@interfere/javascript";
import { toError } from "@interfere/types/sdk/errors";
import { ExportResultCode } from "@interfere/server-core/otel/core/export";
import type { ExportResult } from "@interfere/server-core/otel/core/export";
import { z } from "zod";

export function exportTracePayload(
  payload: Uint8Array | null | undefined,
  client: InterfereClient,
  resultCallback: (result: ExportResult) => void,
) {
  if (!payload || payload.byteLength === 0) {
    resultCallback({ code: ExportResultCode.SUCCESS });

    return;
  }

  void (async () => {
    try {
      const body = z
        .record(z.string(), z.unknown())
        .parse(JSON.parse(new TextDecoder().decode(payload)));

      await client.drain.otlpExport({ signal: "traces", body });

      resultCallback({ code: ExportResultCode.SUCCESS });
    } catch (error) {
      resultCallback({ code: ExportResultCode.FAILED, error: toError(error) });
    }
  })();
}
