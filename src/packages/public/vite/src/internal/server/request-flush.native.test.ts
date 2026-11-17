/* oxlint-disable typescript/require-await */
import { context, propagation, trace } from "@opentelemetry/api";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import { W3CTraceContextPropagator } from "@opentelemetry/core";
import { resolveUrlRedaction } from "@interfere/types/sdk/url-redaction";
import {
  BasicTracerProvider,
  BatchSpanProcessor,
  InMemorySpanExporter,
} from "@opentelemetry/sdk-trace-base";
import { expect, test, vi } from "vitest";

import { withRequestSpan } from "./request-trace";
import { runtime } from "./runtime";

test("on workerd, a finished request exports its server span in the browser's trace without a batch tick", async () => {
  expect.hasAssertions();

  const exporter = new InMemorySpanExporter();

  const provider = new BasicTracerProvider({
    spanProcessors: [
      new BatchSpanProcessor(exporter, { scheduledDelayMillis: 60_000 }),
    ],
  });

  trace.setGlobalTracerProvider(provider);

  context.setGlobalContextManager(
    new AsyncLocalStorageContextManager().enable(),
  );

  propagation.setGlobalPropagator(new W3CTraceContextPropagator());

  await runtime.register(async () => ({
    mode: "managed",
    provider,
    isolated: false,
    capture: {
      errors: "interfere",
      recordOnHostSpan: false,
      urlRedaction: resolveUrlRedaction(),
    },
    flush: async () => provider.forceFlush(),
    shutdown: async () => provider.shutdown(),
  }));

  const traceId = "4bf92f3577b34da6a3ce929d0e0e4736";

  const response = await withRequestSpan(
    new Request("https://interfere.com/_serverFn/abc", {
      headers: { traceparent: `00-${traceId}-00f067aa0ba902b7-01` },
    }),
    async () => new Response("streamed body"),
  );

  await expect(response.text()).resolves.toBe("streamed body");

  await vi.waitFor(() => {
    expect(exporter.getFinishedSpans()).toHaveLength(1);
  });

  const [span] = exporter.getFinishedSpans();

  expect(span?.spanContext().traceId).toBe(traceId);

  expect(span?.parentSpanContext?.spanId).toBe("00f067aa0ba902b7");
});
