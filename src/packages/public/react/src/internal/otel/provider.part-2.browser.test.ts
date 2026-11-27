import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";

import { context, propagation, trace } from "@opentelemetry/api";
import {
  InMemoryLogRecordExporter,
  SimpleLogRecordProcessor,
} from "@opentelemetry/sdk-logs";
import {
  AggregationTemporality,
  InMemoryMetricExporter,
  PeriodicExportingMetricReader,
} from "@opentelemetry/sdk-metrics";
import type { TracerProvider } from "@opentelemetry/sdk-trace";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace";

import { buildOtelProvider } from "./provider.js";
import type { OtelProviderHandle } from "./provider.js";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

import { worker } from "../../../test/msw.js";

function buildReader() {
  const exporter = new InMemoryMetricExporter(
    AggregationTemporality.CUMULATIVE,
  );

  const shutdownSpy = vi.spyOn(exporter, "shutdown");

  const reader = new PeriodicExportingMetricReader({
    exportIntervalMillis: 60_000,
    exporter,
  });

  return { reader, shutdownSpy };
}

function captureSpans() {
  const exporter = new InMemorySpanExporter();
  const processor = new SimpleSpanProcessor({ exporter });

  return { processor, spans: exporter.getFinishedSpans() };
}

function captureLogs() {
  const exporter = new InMemoryLogRecordExporter();
  const processor = new SimpleLogRecordProcessor({ exporter });

  return { processor, logs: exporter.getFinishedLogRecords() };
}

const SDK_STACK = ["@interfere/next@10.0.0", "@interfere/react@10.0.0"];

function buildProvider(
  options: Partial<Parameters<typeof buildOtelProvider>[0]> = {},
) {
  return buildOtelProvider({
    clientOptions: {
      baseUrl: "https://collector.test",
      releaseTokenHeader: { apiKey: "test-key" },
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

  test("re-reads device.id per span so mid-page rotation lands on subsequent spans", () => {
    let current: string | null = "dev_first";
    const captured = captureSpans();

    handle = buildProvider({
      additionalSpanProcessors: [captured.processor],
      getDeviceId: () => current,
    });

    const tracer = handle.tracerProvider.getTracer("test");

    tracer.startSpan("before").end();

    current = "dev_second";

    tracer.startSpan("after").end();

    expect(captured.spans).toHaveLength(2);
    expect(captured.spans[0]?.attributes["device.id"]).toBe("dev_first");
    expect(captured.spans[1]?.attributes["device.id"]).toBe("dev_second");
  });

  describe("device-info (browser-detector) consent gating", () => {
    test("keeps browser attrs off the shared resource so metrics can never leak them", () => {
      const captured = captureSpans();

      handle = buildProvider({
        additionalSpanProcessors: [captured.processor],
        isDeviceInfoEnabled: () => true,
      });

      handle.tracerProvider.getTracer("test").startSpan("probe").end();

      const resourceAttrs = captured.spans[0]?.resource.attributes ?? {};

      expect(resourceAttrs["user_agent.original"]).toBeUndefined();
      expect(resourceAttrs["browser.language"]).toBeUndefined();
    });

    test("stamps browser attrs per-span only while the getter allows", () => {
      let allowed = false;
      const captured = captureSpans();

      handle = buildProvider({
        additionalSpanProcessors: [captured.processor],
        isDeviceInfoEnabled: () => allowed,
      });

      const tracer = handle.tracerProvider.getTracer("test");

      tracer.startSpan("denied").end();

      allowed = true;

      tracer.startSpan("granted").end();

      allowed = false;

      tracer.startSpan("revoked").end();

      expect(captured.spans).toHaveLength(3);

      expect(
        captured.spans[0]?.attributes["user_agent.original"],
      ).toBeUndefined();

      expect(captured.spans[1]?.attributes["user_agent.original"]).toBe(
        navigator.userAgent,
      );

      expect(
        captured.spans[2]?.attributes["user_agent.original"],
      ).toBeUndefined();
    });

    test("stamps browser attrs per-log only while the getter allows", () => {
      let allowed = true;
      const captured = captureLogs();

      handle = buildProvider({
        additionalLogRecordProcessors: [captured.processor],
        isDeviceInfoEnabled: () => allowed,
      });

      const logger = handle.loggerProvider.getLogger("test");

      logger.emit({ body: "granted" });

      allowed = false;

      logger.emit({ body: "revoked" });

      expect(captured.logs).toHaveLength(2);

      expect(captured.logs[0]?.attributes["user_agent.original"]).toBe(
        navigator.userAgent,
      );

      expect(
        captured.logs[1]?.attributes["user_agent.original"],
      ).toBeUndefined();
    });

    test("denies by default when no getter is supplied", () => {
      const captured = captureSpans();

      handle = buildProvider({
        additionalSpanProcessors: [captured.processor],
      });

      handle.tracerProvider.getTracer("test").startSpan("probe").end();

      expect(
        captured.spans[0]?.attributes["user_agent.original"],
      ).toBeUndefined();
    });
  });

  test("omits `device.id` when the getter returns null (SSR boot / pre-hydrate)", () => {
    const captured = captureSpans();

    handle = buildProvider({
      additionalSpanProcessors: [captured.processor],
    });

    handle.tracerProvider.getTracer("test").startSpan("probe").end();

    expect(captured.spans).toHaveLength(1);
    expect(captured.spans[0]?.attributes["device.id"]).toBeUndefined();
  });

  test("prefers the stamped __INTERFERE_ENVIRONMENT__ global for `deployment.environment.name`", () => {
    setInjectedConfiguration("__INTERFERE_ENVIRONMENT__", "staging");

    try {
      const captured = captureSpans();

      handle = buildProvider({
        additionalSpanProcessors: [captured.processor],
      });

      handle.tracerProvider.getTracer("test").startSpan("probe").end();

      expect(captured.spans).toHaveLength(1);

      expect(
        captured.spans[0]?.resource.attributes["deployment.environment.name"],
      ).toBe("staging");
    } finally {
      vi.unstubAllGlobals();
    }
  });

  test("reports development from a dev-server page without a stamped environment global", () => {
    const captured = captureSpans();

    handle = buildProvider({
      additionalSpanProcessors: [captured.processor],
    });

    handle.tracerProvider.getTracer("test").startSpan("probe").end();

    expect(
      captured.spans[0]?.resource.attributes["deployment.environment.name"],
    ).toBe("development");
  });

  test("fans additional metric readers into the MeterProvider", async () => {
    const { reader, shutdownSpy } = buildReader();

    handle = buildProvider({
      additionalMetricReaders: [reader],
    });

    await handle.shutdown();

    handle = null;

    expect(shutdownSpy).toHaveBeenCalledWith();
  });
});
