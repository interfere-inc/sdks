import {
  API_URL_GLOBAL,
  INGEST_URL_GLOBAL,
} from "@interfere/types/sdk/injected-globals";
import { RELEASE_TOKEN_GLOBAL } from "@interfere/types/sdk/release/token";
import { record } from "rrweb";
import { Kernel } from "../src/internal/kernel.js";
import { PluginRuntime } from "../src/internal/plugins/plugin-runtime.js";
import replayPlugin from "../src/plugins/replay/index.js";
import { SessionTracker } from "../src/tracking/api.js";
import { DISABLE_ALL_PLUGINS } from "./fixtures/disable-all-plugins.js";

Object.assign(globalThis, {
  [API_URL_GLOBAL]: location.origin,
  [INGEST_URL_GLOBAL]: location.origin,
  [RELEASE_TOKEN_GLOBAL]: "interfere_pk_visibility",
});

const tracker = new SessionTracker({
  target: {
    baseUrl: location.origin,
    releaseTokenHeader: { apiKey: "test-key" },
  },
});

const kernel = new Kernel(tracker);
const runtime = new PluginRuntime(kernel);

kernel.attachRuntime(runtime);

runtime.applyProviderConfig(DISABLE_ALL_PLUGINS);

runtime.applyRemoteConfigDefaults();

tracker.start();

const target = document.createElement("input");

target.type = "text";

document.body.append(target);

const lifecycle: { type: string; persisted: boolean }[] = [];

window.addEventListener("pageshow", (event) => {
  lifecycle.push({ type: "pageshow", persisted: event.persisted });
});

const cleanup = replayPlugin.setup(kernel);

const resumes: string[] = [];

kernel.onSessionResume((reason) => {
  resumes.push(reason);
});

const hiddenFaults: string[] = [];

const fixture = {
  target,
  lifecycle,
  resumes,
  hiddenFaults,
  observeHidden() {
    globalThis.addEventListener("visibilitychange", () => {
      if (document.visibilityState !== "hidden") {
        return;
      }

      try {
        record.addCustomEvent("hidden-event", true);
      } catch (error) {
        hiddenFaults.push(
          error instanceof Error ? error.message : String(error),
        );
      }

      target.dataset["rebaseline"] = "changed-while-hidden";
    });
  },
  setupAndCancel() {
    cleanup?.("consent-revoked");

    const lateCleanup = replayPlugin.setup(kernel);

    lateCleanup?.("consent-revoked");
  },
  addMarker: (name: string) => record.addCustomEvent(name, true),
  firePageHideAndShow: () => {
    globalThis.dispatchEvent(
      new PageTransitionEvent("pagehide", { persisted: true }),
    );

    globalThis.dispatchEvent(
      new PageTransitionEvent("pageshow", { persisted: false }),
    );
  },
  isRecording: () => record.mirror.getId(target) > -1,
  cleanup,
};

declare global {
  var replayVisibilityFixture: typeof fixture;
}

globalThis.replayVisibilityFixture = fixture;
