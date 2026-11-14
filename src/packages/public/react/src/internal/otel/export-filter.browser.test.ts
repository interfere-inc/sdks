import {
  BasicTracerProvider,
  BatchSpanProcessor,
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";

import { createExportFilterProcessor } from "./export-filter.js";
import { describe, expect, test, vi } from "vitest";

function exportedNames(record: (provider: BasicTracerProvider) => void) {
  const exporter = new InMemorySpanExporter();

  const provider = new BasicTracerProvider({
    spanProcessors: [
      createExportFilterProcessor(new SimpleSpanProcessor(exporter)),
    ],
  });

  record(provider);

  return exporter.getFinishedSpans().map((span) => span.name);
}

function endAfter(
  provider: BasicTracerProvider,
  scope: string,
  name: string,
  durationMs: number,
) {
  const start = Date.now();

  provider
    .getTracer(scope)
    .startSpan(name, { startTime: start })
    .end(start + durationMs);
}

describe(createExportFilterProcessor, () => {
  test("drops long tasks shorter than 500 ms and keeps longer ones", () => {
    const names = exportedNames((provider) => {
      endAfter(
        provider,
        "@opentelemetry/instrumentation-long-task",
        "short",
        120,
      );

      endAfter(
        provider,
        "@opentelemetry/instrumentation-long-task",
        "long",
        650,
      );
    });

    expect(names).toStrictEqual(["long"]);
  });

  test("keeps short spans from every other scope", () => {
    const names = exportedNames((provider) => {
      endAfter(provider, "@opentelemetry/instrumentation-fetch", "GET /api", 5);
    });

    expect(names).toStrictEqual(["GET /api"]);
  });
});

describe("exception delivery", () => {
  test("exports a span carrying an exception without waiting for the batch delay", async () => {
    expect.hasAssertions();

    const exporter = new InMemorySpanExporter();

    const provider = new BasicTracerProvider({
      spanProcessors: [
        createExportFilterProcessor(
          new BatchSpanProcessor(exporter, { scheduledDelayMillis: 60_000 }),
        ),
      ],
    });

    const tracer = provider.getTracer("test");

    tracer.startSpan("routine").end();

    const failing = tracer.startSpan("failing");

    failing.recordException(new Error("boom"));

    failing.end();

    await vi.waitFor(() => {
      expect(
        exporter.getFinishedSpans().map((span) => span.name),
      ).toStrictEqual(["routine", "failing"]);
    });
  });
});

describe("exception span notification", () => {
  test("reports spans whose exception event was added directly, as network failures are", () => {
    const onExceptionSpan = vi.fn<() => void>();

    const provider = new BasicTracerProvider({
      spanProcessors: [
        createExportFilterProcessor(
          new SimpleSpanProcessor(new InMemorySpanExporter()),
          onExceptionSpan,
        ),
      ],
    });

    const tracer = provider.getTracer("@opentelemetry/instrumentation-fetch");

    tracer.startSpan("GET /api/ok").end();

    expect(onExceptionSpan).not.toHaveBeenCalled();

    const failing = tracer.startSpan("GET /api/poll");

    failing.addEvent("exception", {
      "exception.message": "Network network GET https://example.test/api/poll",
      "exception.type": "NetworkError",
    });

    failing.end();

    expect(onExceptionSpan).toHaveBeenCalledTimes(1);
  });
});
