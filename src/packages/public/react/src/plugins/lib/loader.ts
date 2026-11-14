/* oxlint-disable typescript/no-unnecessary-condition */
import { PLUGIN_MANIFEST } from "@interfere/types/sdk/plugins/manifest";
import type { PluginKey } from "@interfere/types/sdk/plugins/manifest";

import { createLogger } from "../../util/log.js";
import errorsPlugin from "../errors.js";
import type { PageEventsOption } from "../page-events-config.js";
import type { ReplayOption } from "../replay/config.js";
import type { Plugin, PluginContext } from "./types.js";

const log = createLogger("plugins");

type PluginLoader = () => Promise<{ default: Plugin } | Plugin> | Plugin;

export const LOADERS: Record<PluginKey, PluginLoader> = {
  device: async () => import("../device.js"),
  errors: () => errorsPlugin,
  logs: async () => import("../logs.js"),
  pageEvents: async () => import("../page-events.js"),
  rageClick: async () => import("../rage-clicks.js"),
  replay: async () => import("../replay/index.js"),
};

// oxlint-disable-next-line typescript/no-unsafe-type-assertion
const DEFAULTS: Record<PluginKey, boolean> = Object.fromEntries(
  PLUGIN_MANIFEST.map((p) => [p.name, p.defaultEnabled]),
) as Record<PluginKey, boolean>;

export type PluginOverrides = Partial<
  Record<Exclude<PluginKey, "pageEvents" | "replay">, boolean>
> & {
  pageEvents?: PageEventsOption;
  replay?: ReplayOption;
};

export function resolveFeatures(overrides?: PluginOverrides) {
  if (!overrides) {
    return { ...DEFAULTS } satisfies Record<PluginKey, boolean>;
  }

  const { pageEvents, replay, ...rest } = overrides;

  return {
    ...DEFAULTS,
    ...rest,
    pageEvents:
      pageEvents === undefined ? DEFAULTS.pageEvents : pageEvents !== false,
    replay: replay === undefined ? DEFAULTS.replay : replay !== false,
  } satisfies Record<PluginKey, boolean>;
}

function resolvePlugin(mod: { default: Plugin } | Plugin) {
  return ("default" in mod ? mod.default : mod) satisfies Plugin;
}

export async function loadPlugin(key: PluginKey, context: PluginContext) {
  const loader = LOADERS[key];

  if (loader === undefined) {
    return null;
  }

  try {
    const mod = await loader();

    const plugin = resolvePlugin(mod);
    const cleanup = plugin.setup(context);

    log.debug("loaded %s", key);

    return cleanup ?? null;
  } catch {
    log.error("failed to load plugin %s", key);

    return null;
  }
}
