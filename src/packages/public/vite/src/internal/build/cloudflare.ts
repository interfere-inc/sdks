import path from "node:path";
import type { BuildEnvironment, PluginOption } from "vite";

export const CLOUDFLARE_PLUGIN = "vite-plugin-cloudflare";

export function hasCloudflarePlugin(plugins: readonly PluginOption[] = []) {
  const pending: PluginOption[] = [...plugins];

  while (pending.length > 0) {
    const plugin = pending.pop();

    if (Array.isArray(plugin)) {
      pending.push(...plugin);
    } else if (
      plugin instanceof Object &&
      "name" in plugin &&
      plugin.name === CLOUDFLARE_PLUGIN
    ) {
      return true;
    }
  }

  return false;
}

export function environmentOutDir(environment: BuildEnvironment) {
  return path.resolve(environment.config.root, environment.config.build.outDir);
}
