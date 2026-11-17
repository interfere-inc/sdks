import { serverContract } from "@interfere/test-utils/vitest/server-contract";
import { context, propagation, trace } from "@opentelemetry/api";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import { W3CTraceContextPropagator } from "@opentelemetry/core";
import {
  BasicTracerProvider,
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import { describe, vi } from "vitest";

describe("shared server contract", () => {
  // oxlint-disable-next-line vitest/require-hook -- Registers the shared tests and their setup/teardown hooks.
  serverContract(async (collectorUrl) => {
    vi.stubEnv("INTERFERE_API_KEY", "server-contract");

    vi.stubEnv("INTERFERE_API_URL", collectorUrl);

    vi.stubEnv("INTERFERE_SOURCE_ID", "a".repeat(40));

    vi.stubEnv("NEXT_PUBLIC_INTERFERE_FORCE_ENABLE", "1");

    const sdk = await import("./instrumentation.js");

    const { captureError } = await import("./server.js");

    const { runtime } = await import("./internal/server/runtime.js");

    let host: BasicTracerProvider | undefined;
    let manager: AsyncLocalStorageContextManager | undefined;

    return {
      register: async () =>
        sdk.register({
          captureUncaughtException: false,
          captureUnhandledRejection: false,
        }),
      capture: (error) => captureError(error),
      recordUrl: (url) =>
        trace
          .getTracer("contract")
          .startSpan("request", {
            attributes: {
              "url.full": url,
              "url.query": "?token=contract-secret&sort=asc",
            },
          })
          .end(),
      installHost: () => {
        const exporter = new InMemorySpanExporter();

        const provider = new BasicTracerProvider({
          spanProcessors: [new SimpleSpanProcessor(exporter)],
        });

        host = provider;

        trace.setGlobalTracerProvider(provider);

        manager = new AsyncLocalStorageContextManager().enable();

        context.setGlobalContextManager(manager);

        const propagator = new W3CTraceContextPropagator();

        propagation.setGlobalPropagator(propagator);

        return {
          run: (capture) =>
            provider.getTracer("host").startActiveSpan("host", (span) => {
              capture();

              span.end();
            }),
          captured: () =>
            exporter
              .getFinishedSpans()
              .flatMap((span) => span.events)
              .filter((event) => event.name === "exception").length,
          ownsTracing: () =>
            trace.getTracerProvider().getTracer("host") ===
              provider.getTracer("host") &&
            propagation.fields().join(",") === propagator.fields().join(","),
        };
      },
      flush: async () => sdk.flush(),
      dispose: async () => {
        await runtime.shutdown();

        await host?.shutdown();

        manager?.disable();

        trace.disable();

        context.disable();

        propagation.disable();
      },
    };
  });
});
