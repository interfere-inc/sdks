import type { ExportResult } from "@opentelemetry/core";

/** Tracks exports whose result callback has not fired yet, so flush and shutdown can wait for requests already on the wire. */
export function createInFlightExports() {
  const pending = new Set<Promise<void>>();

  return {
    track(resultCallback: (result: ExportResult) => void) {
      let finish: (() => void) | undefined;

      const settled = new Promise<void>((resolve) => {
        finish = resolve;
      });

      pending.add(settled);

      return (result: ExportResult) => {
        pending.delete(settled);

        finish?.();

        resultCallback(result);
      };
    },
    async settled() {
      await Promise.allSettled(pending);
    },
  };
}
