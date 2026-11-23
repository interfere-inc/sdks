import { context, propagation, trace } from "@opentelemetry/api";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
  TracerProvider,
} from "@opentelemetry/sdk-trace";

import { buildOtelProvider } from "./provider.js";
import type { OtelProviderHandle } from "./provider.js";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

import { worker } from "../../../test/msw.js";

function captureSpans() {
  const exporter = new InMemorySpanExporter();
  const processor = new SimpleSpanProcessor({ exporter });

  return { processor, spans: exporter.getFinishedSpans() };
}

const SDK_STACK = ["@interfere/next@10.0.0", "@interfere/react@10.0.0"];

function buildProvider(
  options: Partial<Parameters<typeof buildOtelProvider>[0]> = {},
) {
  return buildOtelProvider({
    clientOptions: {
      baseUrl: "https://collector.test",
      publicKeyHeader: { apiKey: "test-key" },
    },
    getDeviceId: () => null,
    getSessionId: () => null,
    sdkStack: SDK_STACK,
    ...options,
  });
}

describe(buildOtelProvider, () => {
  let handle: OtelProviderHandle | null = null;
  const foreignProviders: TracerProvider[] = [];

  beforeEach(() => {
    worker.use(
      ...["traces", "logs", "metrics"].map((signal) =>
        http.post(
          `https://collector.test/drain/v1/${signal}`,
          () => new HttpResponse(null, { status: 204 }),
        ),
      ),
    );

    context.disable();

    trace.disable();

    propagation.disable();
  });

  afterEach(async () => {
    try {
      const results = await Promise.allSettled([
        handle?.shutdown(),
        ...foreignProviders
          .splice(0)
          .map(async (provider) => provider.shutdown()),
      ]);

      const failures = results.filter((result) => result.status === "rejected");

      if (failures.length > 0) {
        throw new AggregateError(
          failures.map((result) => {
            const decodedValue: unknown = result.reason;

            return decodedValue;
          }),
          "Provider cleanup failed",
        );
      }
    } finally {
      handle = null;

      context.disable();

      trace.disable();

      propagation.disable();

      vi.restoreAllMocks();

      vi.unstubAllGlobals();
    }
  });

  describe("global-provider isolation", () => {
    beforeEach(() => {
      trace.disable();

      context.disable();
    });

    afterEach(() => {
      trace.disable();

      context.disable();
    });

    test("does not stomp a foreign global tracer provider installed before us", () => {
      const foreignExporter = new InMemorySpanExporter();

      const foreignProvider = new TracerProvider({
        spanProcessors: [
          new SimpleSpanProcessor({ exporter: foreignExporter }),
        ],
      });

      trace.setGlobalTracerProvider(foreignProvider);

      foreignProviders.push(foreignProvider);

      const ours = captureSpans();

      handle = buildProvider({
        additionalSpanProcessors: [ours.processor],
      });

      trace.getTracer("global-consumer").startSpan("via-global").end();

      handle.tracerProvider.getTracer("private").startSpan("via-private").end();

      expect(foreignExporter.getFinishedSpans()).toHaveLength(1);
      expect(foreignExporter.getFinishedSpans()[0]?.name).toBe("via-global");
      expect(ours.spans).toHaveLength(1);
      expect(ours.spans[0]?.name).toBe("via-private");
    });

    test("installs our tracer provider ONLY on the private handle, never as global", () => {
      const ours = captureSpans();

      handle = buildProvider({
        additionalSpanProcessors: [ours.processor],
      });

      const foreignExporter = new InMemorySpanExporter();

      const foreignProvider = new TracerProvider({
        spanProcessors: [
          new SimpleSpanProcessor({ exporter: foreignExporter }),
        ],
      });

      expect(trace.setGlobalTracerProvider(foreignProvider)).toBe(true);

      foreignProviders.push(foreignProvider);

      trace.getTracer("global-consumer").startSpan("via-global").end();

      handle.tracerProvider.getTracer("private").startSpan("via-private").end();

      expect(foreignExporter.getFinishedSpans()).toHaveLength(1);
      expect(ours.spans).toHaveLength(1);
    });
  });
});
