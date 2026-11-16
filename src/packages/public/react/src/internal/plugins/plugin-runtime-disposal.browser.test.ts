import { context, propagation } from "@opentelemetry/api";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import type { RemotePluginConfig } from "@interfere/types/sdk/remote-config";
import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import {
  API_URL_GLOBAL,
  INGEST_URL_GLOBAL,
  PUBLIC_KEY_GLOBAL,
} from "@interfere/types/sdk/injected-globals";
import { record } from "rrweb";

import { DISABLE_ALL_PLUGINS } from "../../../test/fixtures/disable-all-plugins.js";
import { pluginTransport } from "../../../test/fixtures/plugin-runtime-transport.js";
import { addBaselineHandlers } from "../../../test/msw.js";
import { SessionTracker } from "../../tracking/api.js";
import { Kernel } from "../kernel.js";
import { buildOtelProvider } from "../otel/provider.js";
import { PluginRuntime } from "./plugin-runtime.js";
import { beforeAll, describe, expect, onTestFinished, test, vi } from "vitest";

beforeAll(() => {
  addBaselineHandlers(...pluginTransport.handlers);
});

const ORIGIN = "https://plugin-runtime.invalid";

const LOCAL = { ...DISABLE_ALL_PLUGINS, rageClick: true, replay: true };

function createRuntime(consent?: "required", remote?: RemotePluginConfig) {
  vi.useFakeTimers({ toFake: ["setInterval", "clearInterval"] });

  setInjectedConfiguration(API_URL_GLOBAL, ORIGIN);

  setInjectedConfiguration(INGEST_URL_GLOBAL, ORIGIN);

  setInjectedConfiguration(PUBLIC_KEY_GLOBAL, "interfere_pk_test");

  const tracker = new SessionTracker({
    target: { baseUrl: ORIGIN, publicKeyHeader: { apiKey: "test-key" } },
  });

  const kernel = new Kernel(tracker);
  const runtime = new PluginRuntime(kernel, consent, remote);

  kernel.attachRuntime(runtime);

  const exporter = new InMemorySpanExporter();

  const otel = buildOtelProvider({
    clientOptions: { baseUrl: ORIGIN, publicKeyHeader: { apiKey: "test-key" } },
    getDeviceId: () => kernel.device.getDeviceId(),
    getSessionId: () => kernel.session.getId(),
    sdkStack: [],
    additionalSpanProcessors: [new SimpleSpanProcessor(exporter)],
  });

  kernel.attachOtel(otel, () => undefined);

  const target = document.createElement("button");

  target.textContent = "Audit target";

  document.body.append(target);

  onTestFinished(async () => {
    try {
      await kernel.dispose();
    } finally {
      target.remove();

      context.disable();

      propagation.disable();

      vi.useRealTimers();
    }
  });

  tracker.start();

  const rageClickSpans = () => {
    const before = exporter.getFinishedSpans().length;

    target.click();

    target.click();

    target.click();

    return exporter.getFinishedSpans().length - before;
  };

  return { kernel, rageClickSpans, runtime };
}

function isRecording() {
  try {
    record.addCustomEvent("audit-probe", {});

    return true;
  } catch {
    return false;
  }
}

describe("D-PL-01 analytics and replay wait for provider config", () => {
  test("nothing records before applyProviderConfig, then replay and rageClick start", async () => {
    const { rageClickSpans, runtime } = createRuntime();

    runtime.start();

    runtime.applyRemoteConfigDefaults();

    await vi.dynamicImportSettled();

    expect(isRecording()).toBe(false);
    expect(rageClickSpans()).toBe(0);
    expect(runtime.getPluginConfigState("logs")).toBe("enabled");

    runtime.applyProviderConfig(LOCAL);

    await vi.dynamicImportSettled();

    expect(isRecording()).toBe(true);
    expect(rageClickSpans()).toBe(1);
  });
});

describe("D-PL-02 remote false disables a plugin regardless of local options", () => {
  test("local replay and rageClick on, remote false keeps both off", async () => {
    const { rageClickSpans, runtime } = createRuntime(undefined, {
      rageClick: false,
      replay: false,
    });

    runtime.start();

    runtime.applyProviderConfig({ ...LOCAL, replay: { maskAllInputs: true } });

    runtime.applyRemoteConfig({ rageClick: false, replay: false });

    await vi.dynamicImportSettled();

    expect(isRecording()).toBe(false);
    expect(rageClickSpans()).toBe(0);
    expect(runtime.getPluginConfigState("replay")).toBe("disabled");
  });
});

describe("D-PL-03 and D-PL-04 a later remote disable stops a running plugin and a re-enable restarts it", () => {
  test("replay stops on remote false and restarts when the disable is cleared", async () => {
    const { runtime } = createRuntime();

    runtime.start();

    runtime.applyRemoteConfigDefaults();

    runtime.applyProviderConfig(LOCAL);

    await vi.dynamicImportSettled();

    expect(isRecording()).toBe(true);

    runtime.applyRemoteConfig({ replay: false });

    expect(isRecording()).toBe(false);
    expect(runtime.getPluginConfigState("replay")).toBe("disabled");

    runtime.applyRemoteConfig({});

    await vi.dynamicImportSettled();

    expect(isRecording()).toBe(true);
    expect(runtime.getPluginConfigState("replay")).toBe("enabled");
  });
});

describe("D-PL-05 a remote disable racing the replay lazy load", () => {
  test("replay never ends up recording", async () => {
    const { runtime } = createRuntime();

    runtime.start();

    runtime.applyProviderConfig(LOCAL);

    runtime.applyRemoteConfig({ replay: false });

    await vi.dynamicImportSettled();

    expect(isRecording()).toBe(false);
  });
});

describe("D-PL-06 consent revocation racing the replay lazy load", () => {
  test("replay never ends up recording and uploads nothing", async () => {
    const { runtime } = createRuntime();

    const uploadsBefore = pluginTransport.uploads.length;

    runtime.start();

    runtime.applyRemoteConfigDefaults();

    runtime.applyProviderConfig(LOCAL);

    runtime.setConsent({ replay: false });

    await vi.dynamicImportSettled();

    expect(isRecording()).toBe(false);

    await vi.advanceTimersByTimeAsync(10_000);

    expect(pluginTransport.uploads).toHaveLength(uploadsBefore);
  });
});

describe("D-PL-07 dispose racing the replay lazy load", () => {
  test("replay never ends up recording after dispose", async () => {
    const { kernel, runtime } = createRuntime();

    runtime.start();

    runtime.applyRemoteConfigDefaults();

    runtime.applyProviderConfig(LOCAL);

    await kernel.dispose();

    await vi.dynamicImportSettled();

    expect(isRecording()).toBe(false);
  });
});

describe("D-CS-01 consent categories gate plugins independently", () => {
  test("analytics granted with replay denied records rage clicks but no replay; revoke and re-grant follow", async () => {
    const { rageClickSpans, runtime } = createRuntime();

    runtime.start();

    runtime.applyRemoteConfigDefaults();

    runtime.applyProviderConfig(LOCAL);

    runtime.setConsent({ replay: false });

    await vi.dynamicImportSettled();

    expect(isRecording()).toBe(false);
    expect(rageClickSpans()).toBe(1);

    runtime.setConsent(false);

    expect(isRecording()).toBe(false);

    runtime.setConsent(true);

    await vi.dynamicImportSettled();

    expect(isRecording()).toBe(true);
    expect(rageClickSpans()).toBe(1);
  });
});
