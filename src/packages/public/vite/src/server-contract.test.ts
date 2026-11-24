import { serverContract } from "@interfere/test-utils/vitest/server-contract";
import { context, propagation, trace } from "@opentelemetry/api";
import { AsyncLocalStorageContextManager } from "@interfere/server-core/otel/context-async-hooks";
import { W3CTraceContextPropagator } from "@interfere/server-core/otel/core/propagation";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@interfere/server-core/otel/sdk/trace/processing";
import { TracerProvider } from "@interfere/server-core/otel/sdk/trace/provider";
import { describe, vi } from "vitest";

describe("shared server contract", () => {
  // oxlint-disable-next-line vitest/require-hook -- Registers the shared tests and their setup/teardown hooks.
  serverContract(async (collectorUrl) => {
    vi.stubEnv("INTERFERE_API_KEY", "server-contract");

    vi.stubEnv("INTERFERE_API_URL", collectorUrl);

    vi.stubEnv("INTERFERE_SOURCE_ID", "a".repeat(40));

    const sdk = await import("./server.js");

    const { runtime } = await import("./internal/server/runtime.js");

    let host: TracerProvider | undefined;
    let manager: AsyncLocalStorageContextManager | undefined;

    return {
      register: async () => sdk.register(),
      capture: (error) => sdk.captureError(error),
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

        const provider = new TracerProvider({
          spanProcessors: [new SimpleSpanProcessor({ exporter })],
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
