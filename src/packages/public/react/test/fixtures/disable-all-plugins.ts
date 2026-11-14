import type { PluginOverrides } from "../../src/plugins/lib/loader.js";

export const DISABLE_ALL_PLUGINS = {
  device: false,
  errors: false,
  logs: false,
  pageEvents: false,
  rageClick: false,
  replay: false,
} as const satisfies PluginOverrides;
