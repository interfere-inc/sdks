import type { ExportResult } from "@opentelemetry/core";
import { ExportResultCode } from "@opentelemetry/core";

import {
  isPageHidden,
  onPageUnloading,
} from "../page-lifecycle/page-lifecycle.js";
import { isPageLeaving } from "../page-lifecycle/unloading.js";
import type { SignalSender } from "./exporter.js";

const UNLOAD_REQUEST_MAX_BYTES = 16_000;
const REQUEST_MAX_BYTES = 4 * 1024 * 1024;

function splitPayloads<T>(
  items: T[],
  serialize: (items: T[]) => Uint8Array | undefined,
  maxBytes: number,
) {
  const chunks: { items: T[]; payload: Uint8Array | undefined }[] = [];

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

    chunks.push({ items: items.slice(start, end), payload });

    start = end;
  }

  return chunks;
}

/** Splits exports below the ingest body cap, using smaller requests in `leaveOrder` while hidden or leaving, and reports one aggregate result. A visible request still unacknowledged and not sent with keepalive when the page starts leaving is cancelled and its items are sent again through the leave path. */
export function sendInKeepaliveChunks<T>(
  items: T[],
  serialize: (items: T[]) => Uint8Array | undefined,
  send: SignalSender,
  resultCallback: (result: ExportResult) => void,
  leaveOrder: (items: T[]) => T[] = (unordered) => unordered,
) {
  const leaving = isPageHidden() || isPageLeaving();

  const chunks = splitPayloads(
    leaving ? leaveOrder(items) : items,
    serialize,
    leaving ? UNLOAD_REQUEST_MAX_BYTES : REQUEST_MAX_BYTES,
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

  for (const chunk of chunks) {
    if ((chunk.payload?.byteLength ?? 0) > REQUEST_MAX_BYTES) {
      settle({
        code: ExportResultCode.FAILED,
        error: new Error("A telemetry record exceeds the export body limit"),
      });
    } else if (leaving) {
      send(chunk.payload, settle);
    } else {
      const controller = new AbortController();
      let handedOver = false;

      // WARNING: a navigation cancels a request that is not keepalive, and the leave-time flush no longer holds these items, so the page's unloading signal must resend them through the leave path.
      const unsubscribe = onPageUnloading(() => {
        unsubscribe();

        handedOver = true;

        controller.abort();

        sendInKeepaliveChunks(chunk.items, serialize, send, settle, leaveOrder);
      });

      send(
        chunk.payload,
        (result) => {
          unsubscribe();

          if (!handedOver) {
            settle(result);
          }
        },
        controller.signal,
        // oxlint-disable-next-line eslint/no-empty-function -- A visible export is not retained, so there is no custody to release when the keepalive budget is exhausted.
        { onKeepalive: unsubscribe, onBudgetExhausted: () => {} },
      );
    }
  }
}
