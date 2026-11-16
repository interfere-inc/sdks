import { DISABLE_ALL_PLUGINS } from "../../../test/fixtures/disable-all-plugins.js";
import { SessionTracker } from "../../tracking/api.js";
import { Kernel } from "../kernel.js";
import { PluginRuntime } from "./plugin-runtime.js";
import { afterEach, beforeEach, expect, test } from "vitest";

let kernel: Kernel;

let runtime: PluginRuntime;

beforeEach(() => {
  kernel = new Kernel(
    new SessionTracker({
      target: {
        baseUrl: "https://plugins.invalid",
        publicKeyHeader: { apiKey: "test-key" },
      },
    }),
  );

  runtime = new PluginRuntime(kernel);

  kernel.attachRuntime(runtime);
});

afterEach(async () => {
  await kernel.dispose();
});

test("exposes replay options and clears them when disabled", () => {
  runtime.applyProviderConfig({
    ...DISABLE_ALL_PLUGINS,
    replay: { maskAllInputs: true },
  });

  expect(runtime.getReplayConfig()).toStrictEqual({ maskAllInputs: true });

  runtime.applyProviderConfig(DISABLE_ALL_PLUGINS);

  expect(runtime.getReplayConfig()).toStrictEqual({});
});

test("preserves identical settled consent by identity", () => {
  runtime.applyProviderConfig(DISABLE_ALL_PLUGINS);

  runtime.start();

  runtime.setConsent({ replay: false });

  const consent = runtime.getConsent();

  expect(consent).toStrictEqual({ analytics: true, replay: false });

  runtime.setConsent({ replay: false });

  expect(runtime.getConsent()).toBe(consent);
});

test("publishes disabled remote state and unsubscribes", () => {
  runtime.applyProviderConfig(DISABLE_ALL_PLUGINS);

  const states: string[] = [];

  const unsubscribe = runtime.subscribePluginConfigState("replay", (state) => {
    states.push(state);
  });

  expect(runtime.getPluginConfigState("replay")).toBe("pending");
  expect(states).toStrictEqual([]);

  runtime.applyRemoteConfig({ replay: false });

  expect(runtime.getPluginConfigState("replay")).toBe("disabled");
  expect(states).toStrictEqual(["disabled"]);

  unsubscribe();

  runtime.applyRemoteConfigDefaults();

  expect(states).toStrictEqual(["disabled"]);
});

test("publishes enabled local defaults after remote settlement", () => {
  runtime.applyProviderConfig(DISABLE_ALL_PLUGINS);

  const states: string[] = [];

  runtime.subscribePluginConfigState("replay", (state) => {
    states.push(state);
  });

  runtime.applyRemoteConfigDefaults();

  expect(runtime.getPluginConfigState("replay")).toBe("enabled");
  expect(states).toStrictEqual(["enabled"]);
});

test("disposes a fresh runtime without installing browser handlers", async () => {
  const original = globalThis.onerror;

  await expect(runtime.dispose()).resolves.toBeUndefined();

  expect(globalThis.onerror).toBe(original);
});

test("applies each decision and replaces the previous one", () => {
  runtime.applyProviderConfig(DISABLE_ALL_PLUGINS);

  kernel.consent.set(false);

  runtime.start();

  expect(kernel.consent.get()).toStrictEqual({
    analytics: false,
    replay: false,
  });

  kernel.consent.set(true);

  expect(kernel.consent.get()).toStrictEqual({ analytics: true, replay: true });

  kernel.consent.set({ replay: false });

  expect(kernel.consent.get()).toStrictEqual({
    analytics: true,
    replay: false,
  });

  kernel.consent.set(false);

  expect(kernel.consent.get()).toStrictEqual({
    analytics: false,
    replay: false,
  });

  kernel.consent.set(true);

  expect(kernel.consent.get()).toStrictEqual({ analytics: true, replay: true });
});
