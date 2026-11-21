/* oxlint-disable typescript/require-await */
import { exceptions } from "@interfere/test-utils/vitest/otlp";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import { context, propagation, trace } from "@opentelemetry/api";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import { W3CTraceContextPropagator } from "@opentelemetry/core";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
  TracerProvider,
} from "@opentelemetry/sdk-trace";
import { afterEach, expect, test, vi } from "vitest";

await import("./register");

await import("./runtime");

await import("../../tanstack-start");

afterEach(() => {
  trace.disable();

  context.disable();

  propagation.disable();

  vi.unstubAllEnvs();

  vi.restoreAllMocks();
});

test("managed mode next to a registered provider delivers a throwing request middleware error to Interfere", async () => {
  const collector = await receiver();

  vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test");

  vi.stubEnv("INTERFERE_API_URL", collector.url);

  vi.stubEnv("INTERFERE_SOURCE_ID", "a".repeat(40));

  vi.stubEnv("OTEL_BSP_SCHEDULE_DELAY", "20");

  const host = new TracerProvider({
    spanProcessors: [
      new SimpleSpanProcessor({ exporter: new InMemorySpanExporter() }),
    ],
  });

  trace.setGlobalTracerProvider(host);

  context.setGlobalContextManager(
    new AsyncLocalStorageContextManager().enable(),
  );

  propagation.setGlobalPropagator(new W3CTraceContextPropagator());

  vi.resetModules();

  const { register } = await import("./register");

  const { runtime } = await import("./runtime");

  const { interfereRequestMiddleware } = await import("../../tanstack-start");

  try {
    await register({ consoleBridge: false, tracing: "managed" });

    await expect(
      interfereRequestMiddleware.options.server?.({
        context: undefined,
        next: async () => {
          throw new Error("boom-in-middleware");
        },
        pathname: "/page",
        request: new Request("https://app.test/page"),
        serverFnMeta: undefined,
      } as never),
    ).rejects.toThrow("boom-in-middleware");

    await expect
      .poll(() => exceptions(collector.requests, "boom-in-middleware"), {
        timeout: 3000,
      })
      .toHaveLength(1);
  } finally {
    await runtime.shutdown();

    await host.shutdown();

    await collector.close();
  }
});
