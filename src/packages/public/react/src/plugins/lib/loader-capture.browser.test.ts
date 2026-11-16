import { DISABLE_ALL_PLUGINS } from "../../../test/fixtures/disable-all-plugins.js";
import { PluginRuntime } from "../../internal/plugins/plugin-runtime.js";
import { context, propagation } from "@opentelemetry/api";
import {
  InMemoryLogRecordExporter,
  SimpleLogRecordProcessor,
} from "@opentelemetry/sdk-logs";

import { Kernel } from "../../internal/kernel.js";
import { registerBundledInstrumentations } from "../../internal/otel/instrumentations/instrumentations.js";
import type { BundledInstrumentations } from "../../internal/otel/instrumentations/instrumentations.js";
import { installViewTracker } from "../../internal/otel/navigation.js";
import { buildOtelProvider } from "../../internal/otel/provider.js";
import type { OtelProviderHandle } from "../../internal/otel/provider.js";
import { SessionTracker } from "../../tracking/api.js";
import { DeviceManager } from "../../tracking/device.js";
import { loadPlugin } from "./loader.js";
import type { PluginCleanup } from "./types.js";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, expect, test, vi } from "vitest";

import { worker } from "../../../test/msw.js";

let kernel: Kernel;

let otel: OtelProviderHandle;

let controls: BundledInstrumentations & { recordCurrentView: () => void };

let exporter: InMemoryLogRecordExporter;

let cleanups: PluginCleanup[];

let originalUrl: string;

let originalState: unknown;

beforeEach(() => {
  originalUrl = location.href;

  originalState = history.state;

  worker.use(
    ...[
      "https://plugins.invalid/v3/session",
      "https://plugins.invalid/drain/v1/logs",
      "https://plugins.invalid/drain/v1/traces",
    ].map((url) =>
      http.post(url, () => new HttpResponse(null, { status: 204 })),
    ),
  );

  const tracker = new SessionTracker({
    device: new DeviceManager(),
    target: {
      baseUrl: "https://plugins.invalid",
      publicKeyHeader: { apiKey: "test-key" },
    },
  });

  kernel = new Kernel(tracker);

  exporter = new InMemoryLogRecordExporter();

  cleanups = [];

  otel = buildOtelProvider({
    additionalLogRecordProcessors: [new SimpleLogRecordProcessor({ exporter })],

    getDeviceId: () => kernel.device.getDeviceId(),
    getSessionId: () => kernel.session.getId(),
    getViewId: () => kernel.views.viewId(),
    clientOptions: {
      baseUrl: "https://plugins.invalid",
      publicKeyHeader: { apiKey: "test-key" },
    },
    sdkStack: [],
  });

  const instrumentations = registerBundledInstrumentations({
    tracerProvider: otel.tracerProvider,
    loggerProvider: otel.loggerProvider,
    ignoreUrls: [/.*/u],
  });

  const views = installViewTracker({
    documentContext: null,
    isEnabled: () => kernel.isPageEventsCaptureEnabled(),
    loggerProvider: otel.loggerProvider,
    redactUrl: kernel.redactUrl,
    views: kernel.views,
  });

  controls = {
    dispose: () => {
      views.dispose();

      instrumentations.dispose();
    },
    endOpenRequests: instrumentations.endOpenRequests,
    recordCurrentView: views.recordCurrentView,
    setPageEventsEnabled: instrumentations.setPageEventsEnabled,
  };

  kernel.attachOtel(otel, controls.dispose);

  tracker.start();
});

afterEach(async () => {
  try {
    for (const cleanup of cleanups) {
      cleanup();
    }

    await kernel.dispose();
  } finally {
    context.disable();

    propagation.disable();

    history.replaceState(originalState, "", originalUrl);

    await cookieStore.delete("interfere_did");
  }
});

test("loads device identity and page navigation through native capture hooks", async () => {
  kernel.attachOtel(otel, controls.dispose, controls);

  const device = await loadPlugin("device", kernel);

  expect(device).toBeTypeOf("function");

  if (device) {
    cleanups.push(device);
  }

  const id = kernel.device.getDeviceId();

  expect(id).toStrictEqual(expect.any(String));

  const page = await loadPlugin("pageEvents", kernel);

  expect(page).toBeTypeOf("function");

  if (page) {
    cleanups.push(page);
  }

  expect(exporter.getFinishedLogRecords().at(-1)?.attributes).toMatchObject({
    "browser.navigation.type": "pageload",
    "url.full": kernel.redactUrl(location.href),
    "device.id": id,
    "interfere.view.id": kernel.views.viewId(),
    "session.id": kernel.session.getId(),
  });
});

test("can retry a page-events activation after native telemetry attaches", async () => {
  await expect(loadPlugin("pageEvents", kernel)).resolves.toBeNull();

  expect(exporter.getFinishedLogRecords()).toStrictEqual([]);

  kernel.attachOtel(otel, controls.dispose, controls);

  const cleanup = await loadPlugin("pageEvents", kernel);

  expect(cleanup).toBeTypeOf("function");

  if (cleanup) {
    cleanups.push(cleanup);
  }

  expect(exporter.getFinishedLogRecords().at(-1)?.attributes).toMatchObject({
    "browser.navigation.type": "pageload",
    "url.full": kernel.redactUrl(location.href),
  });

  const baseline = exporter.getFinishedLogRecords().length;

  history.replaceState(history.state, "", "/loader-before-cleanup");

  await expect
    .poll(() => exporter.getFinishedLogRecords().length)
    .toBe(baseline + 1);

  cleanup?.();

  cleanups = [];

  const before = exporter.getFinishedLogRecords();

  history.replaceState(history.state, "", "/loader-after-cleanup");

  await vi.dynamicImportSettled();

  expect(exporter.getFinishedLogRecords()).toStrictEqual(before);
});

test("keeps page events capturing when a second sync starts while the plugin loads", async () => {
  const calls: boolean[] = [];
  const settled = Promise.withResolvers<true>();

  const observed = {
    ...controls,
    setPageEventsEnabled(enabled: boolean) {
      controls.setPageEventsEnabled(enabled);

      calls.push(enabled);

      if (calls.length === 3) {
        settled.resolve(true);
      }
    },
  };

  kernel.attachOtel(otel, controls.dispose, observed);

  const pageEventsOnly = { ...DISABLE_ALL_PLUGINS, pageEvents: true };
  const overlapping = new PluginRuntime(kernel, undefined, pageEventsOnly);

  kernel.attachRuntime(overlapping);

  overlapping.setConsent(true);

  overlapping.start();

  await Promise.resolve();

  overlapping.applyRemoteConfig(pageEventsOnly);

  await settled.promise;

  expect(calls).toStrictEqual([true, false, true]);

  await overlapping.dispose();

  expect(calls).toStrictEqual([true, false, true, false]);
});
