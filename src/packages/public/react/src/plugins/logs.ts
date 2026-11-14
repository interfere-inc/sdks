/* oxlint-disable import/no-default-export */
import { SeverityNumber } from "@opentelemetry/api-logs";

import { onConsoleCall } from "../internal/console-patch/console-patch.js";
import type { ConsoleLevel } from "../internal/console-patch/console-patch.js";
import { activeKernel } from "../internal/kernel-registry.js";
import { safeStringify } from "../util/stringify.js";
import type { Plugin } from "./lib/types.js";

const LEVEL_TO_SEVERITY: Record<
  ConsoleLevel,
  { number: SeverityNumber; text: string }
> = {
  debug: { number: SeverityNumber.DEBUG, text: "DEBUG" },
  error: { number: SeverityNumber.ERROR, text: "ERROR" },
  info: { number: SeverityNumber.INFO, text: "INFO" },
  log: { number: SeverityNumber.INFO, text: "INFO" },
  warn: { number: SeverityNumber.WARN, text: "WARN" },
};

const MAX_BODY_BYTES = 16_384;
const MAX_RECORDS_PER_WINDOW = 100;
const WINDOW_MS = 60_000;
const TRUNCATED = "[Truncated]";
const encoder = new TextEncoder();
const decoder = new TextDecoder();

function capBody(body: string) {
  const buffer = new Uint8Array(MAX_BODY_BYTES);
  const result = encoder.encodeInto(body, buffer);

  if (result.read === body.length) {
    return body;
  }

  const { written } = encoder.encodeInto(
    body,
    buffer.subarray(0, MAX_BODY_BYTES - TRUNCATED.length),
  );

  return decoder.decode(buffer.subarray(0, written)) + TRUNCATED;
}

const logsPlugin: Plugin = {
  name: "logs",

  setup() {
    let windowStartedAt: number | undefined;
    let recordCount = 0;

    const unsubscribe = onConsoleCall((level, args) => {
      if (args.some((a) => a instanceof Error)) {
        return;
      }

      const kernel = activeKernel();

      if (!kernel) {
        return;
      }

      const now = Date.now();

      if (windowStartedAt === undefined || now - windowStartedAt >= WINDOW_MS) {
        windowStartedAt = now;

        recordCount = 0;
      }

      if (recordCount >= MAX_RECORDS_PER_WINDOW) {
        return;
      }

      recordCount += 1;

      const sev = LEVEL_TO_SEVERITY[level];
      const body = args.map((element) => safeStringify(element)).join(" ");

      const redacted = body.replaceAll(
        /https?:\/\/[^\s"'<>]+/giu,
        kernel.redactUrl,
      );

      kernel.recordLog({
        attributes: { "console.level": level },
        body: capBody(redacted),
        severityNumber: sev.number,
        severityText: sev.text,
      });
    });

    return unsubscribe;
  },
};

export default logsPlugin;
