import {
  AggregationTemporality,
  InMemoryMetricExporter,
  MeterProvider,
  PeriodicExportingMetricReader,
} from "@opentelemetry/sdk-metrics";
import { onCLS } from "web-vitals";
import { expect, onTestFinished, test, vi } from "vitest";

import { captureWebVitals } from "./web-vitals.js";

vi.mock(import("web-vitals"), { spy: true });

test("records vitals without device identity when no getter is supplied", async () => {
  const exporter = new InMemoryMetricExporter(AggregationTemporality.DELTA);

  const reader = new PeriodicExportingMetricReader({
    exporter,
    exportIntervalMillis: 60_000,
  });

  const provider = new MeterProvider({ readers: [reader] });

  onTestFinished(async () => provider.shutdown());

  const stop = captureWebVitals({
    meter: provider.getMeter("vitals-test"),
    flush: async () => provider.forceFlush(),
    getIdentity: () => ({ sessionId: "session", viewId: "view" }),
  });

  onTestFinished(stop);

  const report = vi.mocked(onCLS).mock.lastCall?.[0];

  expect(report).toBeDefined();

  report?.({
    name: "CLS",
    value: 0.1,
    delta: 0.1,
    rating: "good",
    id: "vital",
    navigationType: "navigate",
    navigationId: 1,
    entries: [],
  });

  await provider.forceFlush();

  const metric = exporter
    .getMetrics()
    .flatMap((resource) => resource.scopeMetrics)
    .flatMap((scope) => scope.metrics)
    .find((entry) => entry.descriptor.name === "web_vitals.cls");

  expect(metric?.aggregationTemporality).toBe(AggregationTemporality.DELTA);
  expect(metric?.dataPoints).toHaveLength(1);

  expect(metric?.dataPoints[0]?.attributes).toStrictEqual({
    "url.path": location.pathname,
    "web_vital.rating": "good",
    "web_vital.navigation_type": "navigate",
    "session.id": "session",
    "interfere.view.id": "view",
  });
});
