import { isPlainRecord } from "../guards/json.ts";
import { PLUGIN_MANIFEST } from "../plugins/manifest.ts";
import type { RemotePluginConfig } from "../remote-config.ts";

/** Plugin settings resolved for the application at build time. */
export const PLUGINS_GLOBAL = "__INTERFERE_PLUGINS__";

/** Whether a value holds a boolean or no setting for every known plugin, as `remotePluginConfigSchema.partial()` accepts. */
export function isRemotePluginConfig(
  value: unknown,
): value is RemotePluginConfig {
  if (!isPlainRecord(value)) {
    return false;
  }

  return PLUGIN_MANIFEST.every(({ name }) => {
    const setting = value[name];

    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Plugin settings are untrusted values.
    return setting === undefined || typeof setting === "boolean";
  });
}

/** Keeps the known plugin settings of a validated config and drops every other key. */
export function pickRemotePluginConfig(value: RemotePluginConfig) {
  const config: RemotePluginConfig = {};

  for (const { name } of PLUGIN_MANIFEST) {
    if (name in value) {
      config[name] = value[name];
    }
  }

  return config;
}

export function readBakedPlugins() {
  const scope = globalThis;
  const injected = PLUGINS_GLOBAL in scope ? scope[PLUGINS_GLOBAL] : undefined;

  return (
    isRemotePluginConfig(injected) ? pickRemotePluginConfig(injected) : null
  ) satisfies RemotePluginConfig | null;
}
