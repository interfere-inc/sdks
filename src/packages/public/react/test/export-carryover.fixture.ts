import { InterfereError } from "@interfere/javascript";
import { ExportResultCode } from "@opentelemetry/core";
import {
  clearExportCarryover,
  withExportCarryover,
} from "../src/internal/otel/export/carryover.js";

const delivered: string[] = [];
const pending: (() => void)[] = [];
let aborted = 0;
let hold = false;
let allowed = true;

const scope =
  new URL(location.href).searchParams.get("scope") ?? "public-test-key";

const carryover = withExportCarryover(
  scope,
  (payload, callback, abortSignal, retained) => {
    const complete = () => {
      const status = Number(new URL(location.href).searchParams.get("status"));

      if (status) {
        callback({
          code: ExportResultCode.FAILED,
          error: new InterfereError({ statusCode: status }),
        });

        return;
      }

      delivered.push(new TextDecoder().decode(payload));

      callback({ code: ExportResultCode.SUCCESS });
    };

    if (hold) {
      retained?.onKeepalive();

      pending.push(complete);

      abortSignal?.addEventListener(
        "abort",
        () => {
          aborted += 1;

          callback({
            code: ExportResultCode.FAILED,
            error: new Error("aborted"),
          });
        },
        { once: true },
      );

      return;
    }

    complete();
  },
  () => allowed,
);

const fixture = {
  delivered,
  pending,
  aborted: () => aborted,
  release() {
    hold = false;

    for (const complete of pending.splice(0)) {
      complete();
    }
  },
  clear: clearExportCarryover,
  deny() {
    allowed = false;

    clearExportCarryover();
  },
  enqueue(body: string) {
    hold = true;

    globalThis.dispatchEvent(new PageTransitionEvent("pagehide"));

    carryover.send(new TextEncoder().encode(body), () => undefined);
  },
};

declare global {
  var exportCarryoverFixture: typeof fixture;
}

globalThis.exportCarryoverFixture = fixture;
