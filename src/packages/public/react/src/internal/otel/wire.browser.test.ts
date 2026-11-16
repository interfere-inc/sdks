import {
  AggregationTemporality,
  DataPointType,
  InMemoryMetricExporter,
  PeriodicExportingMetricReader,
} from "@opentelemetry/sdk-metrics";
import { onCLS } from "web-vitals";
import { expect, onTestFinished, test, vi } from "vitest";

import { DISABLE_ALL_PLUGINS } from "../../../test/fixtures/disable-all-plugins.js";
import { buildKernel } from "../../../test/fixtures/kernel.js";
import { resolveTargets } from "../config/config.js";
import { wireOtel } from "./wire.js";

vi.mock(import("web-vitals"), { spy: true });

test("stamps device consent and rotation on buffered vital points at record time", async () => {
  const { kernel, runtime } = buildKernel({
    pluginOverrides: DISABLE_ALL_PLUGINS,
  });

  onTestFinished(async () => kernel.dispose());

  runtime.applyProviderConfig({ ...DISABLE_ALL_PLUGINS, device: true });

  kernel.consent.set(false);

  const exporter = new InMemoryMetricExporter(AggregationTemporality.DELTA);

  const reader = new PeriodicExportingMetricReader({
    exporter,
    exportIntervalMillis: 60_000,
  });

  wireOtel({
    kernel,
    releaseSlug: null,
    sdkStack: [],
    targets: resolveTargets("https://test.local"),
    tracing: { additionalMetricReaders: [reader] },
  });

  const report = vi.mocked(onCLS).mock.lastCall?.[0];

  expect(report).toBeDefined();

  function record(value: number) {
    report?.({
      name: "CLS",
      value,
      delta: value,
      rating: "good",
      id: "vital",
      navigationType: "navigate",
      navigationId: 1,
      entries: [],
    });
  }

  expect(kernel.device.getDeviceId()).toBeNull();

  record(0.01);

  kernel.consent.set(true);

  runtime.start();

  const firstDevice = kernel.device.getDeviceId();

  expect(firstDevice).toBeTypeOf("string");

  record(0.02);

  kernel.identity.clear();

  const rotatedDevice = kernel.device.getDeviceId();

  expect(rotatedDevice).toBeTypeOf("string");
  expect(rotatedDevice).not.toBe(firstDevice);

  record(0.03);

  kernel.consent.set(false);

  expect(kernel.device.getDeviceId()).toBeNull();

  record(0.04);

  await reader.forceFlush();

  const metrics = exporter
    .getMetrics()
    .flatMap((resource) => resource.scopeMetrics)
    .flatMap((scope) => scope.metrics)
    .filter((entry) => entry.descriptor.name === "web_vitals.cls");

  expect(metrics).toHaveLength(1);
  expect(metrics[0]?.aggregationTemporality).toBe(AggregationTemporality.DELTA);

  const points = metrics.flatMap((metric) =>
    metric.dataPointType === DataPointType.HISTOGRAM ? metric.dataPoints : [],
  );

  expect(
    points.map((point) => ({
      device: point.attributes["device.id"],
      sum: point.value.sum,
    })),
  ).toStrictEqual([
    { device: undefined, sum: 0.05 },
    { device: firstDevice, sum: 0.02 },
    { device: rotatedDevice, sum: 0.03 },
  ]);
});
