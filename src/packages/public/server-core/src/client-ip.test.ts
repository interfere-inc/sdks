import { attribute, exportedSpans } from "@interfere/test-utils/vitest/otlp";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import { trace } from "@opentelemetry/api";
import { W3CTraceContextPropagator } from "@opentelemetry/core";
import { expect, onTestFinished, test } from "vitest";

import { startNodeRuntime } from "./node.js";
import { ServerRuntime } from "./runtime.js";

test.each([
  [undefined, "203.0.113.7"],
  [true, "203.0.113.7"],
  [false, undefined],
] as const)(
  "applies clientIp=%s before writing exception events to the host span",
  async (clientIp, expectedIp) => {
    const collector = await receiver();

    const runtime = new ServerRuntime("client-ip");

    onTestFinished(async () => {
      await runtime.shutdown();

      await collector.close();
    });

    await runtime.register(async () =>
      startNodeRuntime({
        name: "client-ip",
        version: "1.0.0",
        apiUrl: collector.url,
        publicKey: "test",
        resourceAttributes: {},
        options: {
          consoleBridge: false,
          privacy: clientIp === undefined ? {} : { clientIp },
        },
        drain: false,
        logDrain: false,
        vercel: false,
        propagator: new W3CTraceContextPropagator(),
      }),
    );

    trace.getTracer("request").startActiveSpan("GET /orders", (span) => {
      runtime.recordException(
        new Error("request failed"),
        { type: "manual.api.capture", handled: true },
        {
          "http.request.header.x-forwarded-for": "203.0.113.7",
          "http.request.header.x-forwarded-host": "app.example.com",
        },
      );

      span.end();
    });

    await runtime.flush();

    const spans = exportedSpans(collector.requests);

    expect(spans).toHaveLength(1);
    expect(spans[0]?.events).toHaveLength(1);

    const attrs = spans[0]?.events[0]?.attributes ?? [];

    expect(attribute(attrs, "http.request.header.x-forwarded-for")).toBe(
      expectedIp,
    );

    expect(attribute(attrs, "http.request.header.x-forwarded-host")).toBe(
      "app.example.com",
    );
  },
);
