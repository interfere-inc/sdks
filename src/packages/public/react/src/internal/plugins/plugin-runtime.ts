import { PLUGIN_MANIFEST } from "@interfere/types/sdk/plugins/manifest";
import type {
  ConsentInput,
  ConsentState,
  PluginKey,
} from "@interfere/types/sdk/plugins/manifest";
import type { RemotePluginConfig } from "@interfere/types/sdk/remote-config";

import devicePlugin from "../../plugins/device.js";
import errorsPlugin from "../../plugins/errors.js";
import { loadPlugin, resolveFeatures } from "../../plugins/lib/loader.js";
import type { PluginOverrides } from "../../plugins/lib/loader.js";
import type {
  PluginCleanup,
  PluginConfigState,
  PluginContext,
  PluginTeardownReason,
} from "../../plugins/lib/types.js";
import type { PageEventsConfig } from "../../plugins/page-events-config.js";
import type { ReplayConfig } from "../../plugins/replay/config.js";
import { createLogger } from "../../util/log.js";
import {
  getPluginConsentCategory,
  hasConsentChanged,
  isConsentAllowed,
  resolveGrantedConsent,
} from "../consent/consent.js";

const log = createLogger("plugin-runtime");

const BUNDLED_PLUGINS = {
  device: devicePlugin,
  errors: errorsPlugin,
} as const;

type BundledPluginKey = keyof typeof BUNDLED_PLUGINS;

function isBundledPlugin(key: PluginKey): key is BundledPluginKey {
  return key === "device" || key === "errors";
}

export class PluginRuntime {
  private readonly activeCleanups = new Map<PluginKey, PluginCleanup>();
  private readonly activations = new Map<PluginKey, Promise<void>>();

  private features: Record<PluginKey, boolean> = resolveFeatures();
  private readonly pluginConfigListeners = new Map<
    PluginKey,
    Set<(state: PluginConfigState) => void>
  >();
  private remoteConfig: RemotePluginConfig = {};
  private remoteConfigSettled = false;
  private awaitingRemoteConfig: boolean;
  private consentState: ConsentState | null = null;
  private configSettled = false;
  private replayConfig: ReplayConfig = {};
  private pageEventsConfig: PageEventsConfig = {};
  private syncVersion = 0;
  private disposed = false;

  private readonly context: PluginContext;

  constructor(
    context: PluginContext,
    consent?: ConsentInput,
    remoteConfig: RemotePluginConfig = {},
    awaitingRemoteConfig = false,
  ) {
    this.context = context;

    this.consentState =
      consent === undefined ? null : resolveGrantedConsent(consent);

    this.configSettled = consent !== undefined;
    this.remoteConfig = remoteConfig;
    this.awaitingRemoteConfig = awaitingRemoteConfig;
  }

  getConsent() {
    return this.consentState;
  }

  getReplayConfig() {
    return this.replayConfig;
  }

  isInteractionCaptureEnabled() {
    return this.pageEventsConfig.interactions !== false;
  }

  applyProviderConfig(
    overrides: PluginOverrides | undefined,
    consent: ConsentInput | undefined,
  ) {
    const pageEvents = overrides?.pageEvents;
    const replay = overrides?.replay;

    this.features = resolveFeatures(overrides);
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- The public pageEvents override accepts boolean enablement or a page-events options object.
    this.pageEventsConfig = typeof pageEvents === "object" ? pageEvents : {};
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- The public replay override accepts boolean enablement or a replay-options object.
    this.replayConfig = typeof replay === "object" ? replay : {};

    if (consent !== undefined) {
      this.consentState = resolveGrantedConsent(consent);
    }

    this.configSettled = true;

    this.sync();
  }

  setConsent(nextConsent?: ConsentInput) {
    const nextState = resolveGrantedConsent(nextConsent);

    if (
      this.configSettled &&
      !hasConsentChanged(this.consentState, nextState)
    ) {
      return;
    }

    this.consentState = nextState;
    this.configSettled = true;

    this.sync();
  }

  resetConsent() {
    if (this.configSettled && !hasConsentChanged(this.consentState, null)) {
      return;
    }

    this.consentState = null;
    this.configSettled = true;

    this.sync();
  }

  applyRemoteConfig(config: RemotePluginConfig) {
    this.remoteConfig = config;
    this.remoteConfigSettled = true;

    this.emitPluginConfigState();

    this.sync();
  }

  applyRemoteConfigDefaults() {
    this.remoteConfigSettled = true;

    this.emitPluginConfigState();

    this.sync();
  }

  /** Ends the pre-config phase of a runtime constructed with `awaitingRemoteConfig` and starts the plugins it held back. */
  startWithRemoteConfig(config: RemotePluginConfig) {
    this.remoteConfig = config;
    this.awaitingRemoteConfig = false;

    this.start();
  }

  /** True once consent and remote config have settled, so consent-gated plugins have their current state. */
  isConfigSettled() {
    return this.configSettled && !this.awaitingRemoteConfig;
  }

  start() {
    for (const key of ["errors", "device"] as const) {
      if (this.shouldEnablePlugin(key)) {
        this.setupBundledPlugin(key);
      }
    }

    this.sync();
  }

  private setupBundledPlugin(key: BundledPluginKey) {
    if (this.activeCleanups.has(key)) {
      return;
    }

    const cleanup = BUNDLED_PLUGINS[key].setup(this.context);

    if (cleanup) {
      this.activeCleanups.set(key, cleanup);
    }
  }

  async dispose() {
    this.disposed = true;
    this.syncVersion += 1;

    for (const key of this.activeCleanups.keys()) {
      this.deactivate(key, "dispose");
    }

    await Promise.allSettled(this.activations.values());
  }

  private shouldEnablePlugin(key: PluginKey) {
    if (this.disposed) {
      return false;
    }

    const category = getPluginConsentCategory(key);

    if (
      category !== "necessary" &&
      (!this.configSettled || this.awaitingRemoteConfig)
    ) {
      return false;
    }

    return (
      this.features[key] &&
      this.remoteConfig[key] !== false &&
      isConsentAllowed(category, this.consentState)
    );
  }

  getPluginConfigState(key: PluginKey) {
    if (!this.remoteConfigSettled) {
      return "pending";
    }

    return (
      this.remoteConfig[key] === false ? "disabled" : "enabled"
    ) satisfies PluginConfigState;
  }

  subscribePluginConfigState(
    key: PluginKey,
    listener: (state: PluginConfigState) => void,
  ) {
    const listeners = this.pluginConfigListeners.get(key) ?? new Set();

    listeners.add(listener);

    this.pluginConfigListeners.set(key, listeners);

    return (() => {
      listeners.delete(listener);

      if (listeners.size === 0) {
        this.pluginConfigListeners.delete(key);
      }
    }) satisfies () => void;
  }

  private emitPluginConfigState() {
    for (const [key, listeners] of this.pluginConfigListeners) {
      const state = this.getPluginConfigState(key);

      for (const listener of listeners) {
        listener(state);
      }
    }
  }

  private teardownReason(key: PluginKey) {
    const category = getPluginConsentCategory(key);

    return (
      isConsentAllowed(category, this.consentState)
        ? "disabled"
        : "consent-revoked"
    ) satisfies PluginTeardownReason;
  }

  private deactivate(key: PluginKey, reason: PluginTeardownReason) {
    const cleanup = this.activeCleanups.get(key);

    if (!cleanup) {
      return;
    }

    try {
      cleanup(reason);
    } catch {
      log.warn("cleanup failed for %s", key);
    }

    this.activeCleanups.delete(key);
  }

  private async activate(key: PluginKey) {
    if (this.activeCleanups.has(key) || !this.shouldEnablePlugin(key)) {
      return;
    }

    const version = this.syncVersion;

    const cleanup = await loadPlugin(key, this.context);

    if (!cleanup) {
      return;
    }

    const staleSync = version !== this.syncVersion;

    if (staleSync || !this.shouldEnablePlugin(key)) {
      cleanup(this.teardownReason(key));

      return;
    }

    this.activeCleanups.set(key, cleanup);
  }

  private sync() {
    if (this.disposed) {
      return;
    }

    this.syncVersion += 1;

    for (const plugin of PLUGIN_MANIFEST) {
      if (this.shouldEnablePlugin(plugin.name)) {
        if (isBundledPlugin(plugin.name)) {
          this.setupBundledPlugin(plugin.name);

          continue;
        }

        const previous = this.activations.get(plugin.name);

        this.activations.set(
          plugin.name,
          (async () => {
            await previous;

            try {
              await this.activate(plugin.name);
            } catch {
              log.warn("non-critical plugin loading failed");
            }
          })(),
        );

        continue;
      }

      this.deactivate(plugin.name, this.teardownReason(plugin.name));
    }
  }
}
