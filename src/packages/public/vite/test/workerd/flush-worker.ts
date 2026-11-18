/* oxlint-disable typescript/require-await, eslint/class-methods-use-this -- Each RPC call gets a fresh entrypoint, so the runtime under test lives in module state shared by both requests. */
import { ServerRuntime } from "@interfere/server-core/runtime";
import { resolveUrlRedaction } from "@interfere/types/sdk/url-redaction";
import { trace } from "@opentelemetry/api";
import { WorkerEntrypoint } from "cloudflare:workers";

const runtime = new ServerRuntime("@interfere/vite/workerd-test");

let flushes = 0;

let exporting: (() => void) | null = null;

const pipeline = {
  mode: "managed",
  provider: trace.getTracerProvider(),
  isolated: false,
  capture: {
    errors: "interfere",
    recordOnHostSpan: false,
    urlRedaction: resolveUrlRedaction(),
  },
  flush: async () => {
    flushes += 1;

    if (flushes === 1) {
      exporting?.();

      await new Promise((resolve) => {
        setTimeout(resolve, 60_000);
      });
    }
  },
  shutdown: async () => undefined,
} as const;

// oxlint-disable-next-line import/no-default-export -- Native Worker entrypoints require a default export.
export default class ServerFlushTests extends WorkerEntrypoint {
  async abandon() {
    await runtime.register(async () => pipeline);

    const started = Promise.withResolvers<undefined>();

    exporting = () => {
      started.resolve(undefined);
    };

    void runtime.flush();

    await started.promise;
  }

  async flush() {
    await runtime.flush();
  }
}
