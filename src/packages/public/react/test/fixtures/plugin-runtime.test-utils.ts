import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { omitUndefined } from "@interfere/helpers/omit-undefined";
import {
  API_URL_GLOBAL,
  INGEST_URL_GLOBAL,
  PUBLIC_KEY_GLOBAL,
} from "@interfere/types/sdk/injected-globals";

import { context, propagation } from "@opentelemetry/api";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";

import { Kernel } from "../../src/internal/kernel.js";
import { buildOtelProvider } from "../../src/internal/otel/provider.js";
import { PluginRuntime } from "../../src/internal/plugins/plugin-runtime.js";
import { SessionTracker } from "../../src/tracking/api.js";
import type { DeviceManager } from "../../src/tracking/device.js";
import { addBaselineHandlers } from "../msw.js";
import { DISABLE_ALL_PLUGINS } from "./disable-all-plugins.js";
import { pluginTransport } from "./plugin-runtime-transport.js";
import { onTestFinished, vi } from "vitest";

addBaselineHandlers(...pluginTransport.handlers);

export async function startRecordingPluginRuntime(options: {
  device?: DeviceManager;
  devicePlugin: boolean;
}) {
  vi.useFakeTimers({ toFake: ["setInterval", "clearInterval"] });

  setInjectedConfiguration(API_URL_GLOBAL, "https://plugin-runtime.invalid");

  setInjectedConfiguration(INGEST_URL_GLOBAL, "https://plugin-runtime.invalid");

  setInjectedConfiguration(PUBLIC_KEY_GLOBAL, "interfere_pk_test");

  pluginTransport.uploads.length = 0;

  const tracker = new SessionTracker(
    omitUndefined({
      device: options.device,
      target: {
        baseUrl: "https://plugin-runtime.invalid",
        publicKeyHeader: { apiKey: "test-key" },
      },
    }),
  );

  const kernel = new Kernel(tracker);

  const runtime = new PluginRuntime(kernel);

  kernel.attachRuntime(runtime);

  const exporter = new InMemorySpanExporter();

  const otel = buildOtelProvider({
    clientOptions: {
      baseUrl: "https://plugin-runtime.invalid",
      publicKeyHeader: { apiKey: "test-key" },
    },
    getDeviceId: () => kernel.device.getDeviceId(),
    getSessionId: () => kernel.session.getId(),
    sdkStack: [],
    additionalSpanProcessors: [new SimpleSpanProcessor(exporter)],
  });

  kernel.attachOtel(otel, () => undefined);

  const target = document.createElement("button");

  target.id = "teardown-target";
  target.textContent = "Record interaction";

  document.body.append(target);

  onTestFinished(async () => {
    try {
      runtime.setConsent(false);

      await kernel.dispose();
    } finally {
      target.remove();

      context.disable();

      propagation.disable();

      vi.useRealTimers();

      vi.unstubAllGlobals();

      await cookieStore.delete("interfere_did");
    }
  });

  tracker.start();

  runtime.applyRemoteConfigDefaults();

  runtime.applyProviderConfig({
    ...DISABLE_ALL_PLUGINS,
    rageClick: true,
    replay: true,
    device: options.devicePlugin,
  });

  await vi.dynamicImportSettled();

  return { kernel, runtime, exporter, target };
}
