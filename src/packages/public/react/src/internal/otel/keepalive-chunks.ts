import type { ExportResult } from "@opentelemetry/core";
import { ExportResultCode } from "@opentelemetry/core";

import { isPageHidden } from "../page-lifecycle/page-lifecycle.js";
import { isPageUnloading } from "../page-lifecycle/unloading.js";
import type { SignalSender } from "./exporter.js";

const UNLOAD_REQUEST_MAX_BYTES = 16_000;
const REQUEST_MAX_BYTES = 4 * 1024 * 1024;

function splitPayloads<T>(
  items: T[],
  serialize: (items: T[]) => Uint8Array | undefined,
  maxBytes: number,
) {
  const chunks: (Uint8Array | undefined)[] = [];
  const pending = [items];

  for (let next = pending.pop(); next; next = pending.pop()) {
    const payload = serialize(next);

    if (next.length <= 1 || (payload?.byteLength ?? 0) <= maxBytes) {
      chunks.push(payload);

      continue;
    }

    const middle = Math.ceil(next.length / 2);

    pending.push(next.slice(middle), next.slice(0, middle));
  }

  return chunks;
}

/** Splits exports below the ingest body cap, using smaller requests while hidden, and reports one aggregate result. */
export function sendInKeepaliveChunks<T>(
  items: T[],
  serialize: (items: T[]) => Uint8Array | undefined,
  send: SignalSender,
  resultCallback: (result: ExportResult) => void,
) {
  const chunks = splitPayloads(
    items,
    serialize,
    isPageHidden() || isPageUnloading()
      ? UNLOAD_REQUEST_MAX_BYTES
      : REQUEST_MAX_BYTES,
  );

  let pending = chunks.length;
  let failure: ExportResult | undefined;

  const settle = (result: ExportResult) => {
    if (result.code !== ExportResultCode.SUCCESS) {
      failure ??= result;
    }

    pending -= 1;

    if (pending === 0) {
      resultCallback(failure ?? { code: ExportResultCode.SUCCESS });
    }
  };

  for (const payload of chunks) {
    if ((payload?.byteLength ?? 0) > REQUEST_MAX_BYTES) {
      settle({
        code: ExportResultCode.FAILED,
        error: new Error("A telemetry record exceeds the export body limit"),
      });
    } else {
      send(payload, settle);
    }
  }
}
