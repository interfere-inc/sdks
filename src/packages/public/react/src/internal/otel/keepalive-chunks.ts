import type { ExportResult } from "@opentelemetry/core";
import { ExportResultCode } from "@opentelemetry/core";

import { isPageHidden } from "../page-lifecycle/page-lifecycle.js";
import { isPageLeaving } from "../page-lifecycle/unloading.js";
import type { SignalSender } from "./exporter.js";

const UNLOAD_REQUEST_MAX_BYTES = 16_000;
const REQUEST_MAX_BYTES = 4 * 1024 * 1024;

function splitPayloads<T>(
  items: T[],
  serialize: (items: T[]) => Uint8Array | undefined,
  maxBytes: number,
) {
  const chunks: (Uint8Array | undefined)[] = [];

  const fits = (payload: Uint8Array | undefined) =>
    (payload?.byteLength ?? 0) <= maxBytes;

  let start = 0;

  while (start < items.length) {
    let end = start + 1;
    let payload = serialize(items.slice(start, end));
    let high = items.length;

    while (fits(payload) && end < high) {
      const middle = Math.ceil((end + high) / 2);
      const candidate = serialize(items.slice(start, middle));

      if (fits(candidate)) {
        end = middle;

        payload = candidate;
      } else {
        high = middle - 1;
      }
    }

    chunks.push(payload);

    start = end;
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
    isPageHidden() || isPageLeaving()
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
