import {
  ROOT_CONTEXT,
  defaultTextMapGetter,
  defaultTextMapSetter,
  trace,
} from "@opentelemetry/api";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
  TracerProvider,
} from "@opentelemetry/sdk-trace";
import { expect, onTestFinished, test } from "vitest";

import { CacheSafeTraceContextPropagator } from "./cache-safe-trace-context.js";

test.each([
  "00000000000000001234567890abcdef",
  "0af7651916cd43dd8448eb211c80319c",
])(
  "continues a valid inbound W3C trace %s without inventing its missing parent",
  (traceId) => {
    const parentSpanId = "b7ad6b7169203331";

    const propagator = new CacheSafeTraceContextPropagator({
      isStaticRender: () => false,
    });

    const parent = propagator.extract(
      ROOT_CONTEXT,
      {
        traceparent: `00-${traceId}-${parentSpanId}-01`,
      },
      defaultTextMapGetter,
    );

    const exporter = new InMemorySpanExporter();

    const provider = new TracerProvider({
      spanProcessors: [new SimpleSpanProcessor({ exporter })],
    });

    onTestFinished(async () => provider.shutdown());

    const span = provider
      .getTracer("inbound-context")
      .startSpan("GET /items/[id]", {}, parent);

    const carrier: Record<string, string> = {};

    propagator.inject(
      trace.setSpan(parent, span),
      carrier,
      defaultTextMapSetter,
    );

    span.end();

    expect(exporter.getFinishedSpans()).toHaveLength(1);
    expect(span.spanContext().traceId).toBe(traceId);

    expect(exporter.getFinishedSpans()[0]?.parentSpanContext).toMatchObject({
      traceId,
      spanId: parentSpanId,
      isRemote: true,
    });

    expect(carrier["traceparent"]).toBe(
      `00-${traceId}-${span.spanContext().spanId}-01`,
    );
  },
);
