/* oxlint-disable typescript/require-await, eslint/class-methods-use-this -- Each RPC call gets a fresh entrypoint, so the runtime under test lives in module state shared by both requests. */
import { ServerRuntime } from "@interfere/server-core/runtime";
import { resolveUrlRedaction } from "@interfere/types/sdk/url-redaction";
import { trace } from "@opentelemetry/api";
import { WorkerEntrypoint } from "cloudflare:workers";

import { ensureRemoteConfig } from "../../src/internal/server/remote-config";
import { runtime as viteRuntime } from "../../src/internal/server/runtime";

const runtime = new ServerRuntime("@interfere/vite/workerd-test");

const startupRuntime = new ServerRuntime("@interfere/vite/workerd-startup");

let flushes = 0;

let registeredFlushes = 0;

let exporting: (() => void) | null = null;

const capture = {
  errors: "interfere",
  recordOnHostSpan: false,
  urlRedaction: resolveUrlRedaction(),
} as const;

const pipeline = {
  mode: "managed",
  provider: trace.getTracerProvider(),
  isolated: false,
  capture,
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

const registeredPipeline = {
  mode: "managed",
  provider: trace.getTracerProvider(),
  isolated: false,
  capture,
  flush: async () => {
    registeredFlushes += 1;
  },
  shutdown: async () => undefined,
} as const;

async function registerVite() {
  await viteRuntime.register(async () => registeredPipeline);

  await ensureRemoteConfig({ serviceName: "workerd-test" });
}

// oxlint-disable-next-line import/no-default-export -- Native Worker entrypoints require a default export.
export default class ServerTests extends WorkerEntrypoint {
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

  async abandonStartup() {
    const starting = Promise.withResolvers<undefined>();

    void startupRuntime.register(async () => {
      starting.resolve(undefined);

      await Promise.withResolvers<undefined>().promise;

      return pipeline;
    });

    await starting.promise;
  }

  async registerDuringStartup() {
    await startupRuntime.register(async () => pipeline);
  }

  async abandonRegister() {
    void registerVite();

    await new Promise((resolve) => {
      setTimeout(resolve, 0);
    });
  }

  async register() {
    await registerVite();

    await viteRuntime.flush();

    return registeredFlushes;
  }
}
