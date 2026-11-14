import { startNodeRuntime } from "@interfere/server-core/node";
import { attribute, exportedSpans } from "@interfere/test-utils/vitest/otlp";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import { W3CTraceContextPropagator } from "@opentelemetry/core";
import { expect, onTestFinished, test, vi } from "vitest";

import { withRequestSpan } from "./request-trace.js";
import { runtime } from "./runtime.js";

test.each([
  [undefined, "203.0.113.7, 10.0.0.1"],
  [true, "203.0.113.7, 10.0.0.1"],
  [false, undefined],
] as const)(
  "captures request client IP unless clientIp=%s opts out",
  async (clientIp, expectedIp) => {
    const collector = await receiver();

    vi.stubEnv("VITE_INTERFERE_PUBLIC_KEY", "interfere_pk_test");

    onTestFinished(async () => {
      await runtime.shutdown();

      await collector.close();

      vi.unstubAllEnvs();
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

    await expect(
      withRequestSpan(
        new Request("https://app.example.com/orders", {
          headers: {
            "x-forwarded-for": "203.0.113.7, 10.0.0.1",
            authorization: "secret-credential",
          },
        }),
        async () => {
          await Promise.resolve();

          throw new Error("request failed");
        },
      ),
    ).rejects.toThrow("request failed");

    await runtime.flush();

    const spans = exportedSpans(collector.requests);

    expect(spans).toHaveLength(1);
    expect(spans[0]?.events).toHaveLength(1);

    const attrs = spans[0]?.events[0]?.attributes ?? [];

    expect(attribute(attrs, "http.request.header.x-forwarded-for")).toBe(
      expectedIp,
    );

    expect(JSON.stringify(spans)).not.toContain("secret-credential");
  },
);
