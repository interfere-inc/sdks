import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import type { AnonymousReplayPolicy } from "@interfere/types/sdk/remote-config/anonymous-replay";
import {
  API_URL_GLOBAL,
  INGEST_URL_GLOBAL,
  PUBLIC_KEY_GLOBAL,
} from "@interfere/types/sdk/injected-globals";

import { Kernel } from "../../src/internal/kernel.js";
import { PluginRuntime } from "../../src/internal/plugins/plugin-runtime.js";
import type { PluginCleanup } from "../../src/plugins/lib/types.js";
import replayPlugin from "../../src/plugins/replay/index.js";
import { SessionTracker } from "../../src/tracking/api.js";
import { onTestFinished, vi } from "vitest";

import { addBaselineHandlers } from "../msw.js";
import { DISABLE_ALL_PLUGINS } from "./disable-all-plugins.js";
import { replayHandlers, replayUploads } from "./replay-uploads.js";

addBaselineHandlers(...replayHandlers);

const SESSION_STORAGE_KEYS = [
  "interfere:session_id",
  "interfere:last_activity",
  "interfere:session_started",
  "interfere:window_id",
  "__interfere_probe__",
];

interface ReplayHarnessInput {
  anonymousReplayPolicy?: AnonymousReplayPolicy;
  fakeDate?: boolean;
  fakeMonotonicClock?: boolean;
  remoteConfig?: "defaults" | "pending";
  serviceIdentity?: Kernel["serviceIdentity"];
}

export interface ReplayHarness {
  kernel: Kernel;
  runtime: PluginRuntime;
  target: HTMLInputElement;
  cleanup: PluginCleanup | undefined;
}

function saveSessionStorage() {
  const saved = [localStorage, sessionStorage].flatMap((storage) =>
    SESSION_STORAGE_KEYS.map((key) => ({
      storage,
      key,
      value: storage.getItem(key),
    })),
  );

  return () => {
    for (const { storage, key, value } of saved) {
      if (value === null) {
        storage.removeItem(key);
      } else {
        storage.setItem(key, value);
      }
    }
  };
}

export function createReplayHarness({
  anonymousReplayPolicy,
  fakeDate = false,
  fakeMonotonicClock = false,
  remoteConfig = "defaults",
  serviceIdentity,
}: ReplayHarnessInput = {}) {
  const restoreStorage = saveSessionStorage();

  vi.useFakeTimers({
    toFake: [
      "setInterval",
      "clearInterval",
      ...(fakeMonotonicClock ? (["performance"] as const) : []),
      ...(fakeDate ? (["Date"] as const) : []),
    ],
  });

  setInjectedConfiguration(API_URL_GLOBAL, "https://replay.invalid");

  setInjectedConfiguration(INGEST_URL_GLOBAL, "https://replay.invalid");

  setInjectedConfiguration(PUBLIC_KEY_GLOBAL, "interfere_pk_test");

  replayUploads.requests.length = 0;

  replayUploads.uploads.length = 0;

  const tracker = new SessionTracker({
    target: {
      baseUrl: "https://replay.invalid",
      publicKeyHeader: { apiKey: "test-key" },
    },
  });

  const kernel = new Kernel(tracker, undefined, serviceIdentity);

  if (anonymousReplayPolicy) {
    kernel.anonymousReplayPolicy = anonymousReplayPolicy;
  }

  const runtime = new PluginRuntime(kernel);

  kernel.attachRuntime(runtime);

  runtime.applyProviderConfig(DISABLE_ALL_PLUGINS);

  if (remoteConfig === "defaults") {
    runtime.applyRemoteConfigDefaults();
  }

  tracker.start();

  const target = document.createElement("input");

  target.type = "text";

  document.body.append(target);

  const harness: ReplayHarness = {
    kernel,
    runtime,
    target,
    cleanup: undefined,
  };

  onTestFinished(async () => {
    try {
      harness.cleanup?.("consent-revoked");

      await kernel.dispose();
    } finally {
      target.remove();

      vi.useRealTimers();

      vi.unstubAllGlobals();

      vi.restoreAllMocks();

      restoreStorage();
    }
  });

  return harness;
}

export async function startReplayHarness(input: ReplayHarnessInput = {}) {
  const harness = createReplayHarness(input);

  harness.cleanup = replayPlugin.setup(harness.kernel);

  await vi.dynamicImportSettled();

  return harness;
}
