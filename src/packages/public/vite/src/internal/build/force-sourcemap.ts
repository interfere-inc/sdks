import type { Logger, UserConfig } from "vite";

import { pickEnvValue } from "../plugin-env";
import type { InterfereViteOptions } from "../plugin-options";

export type SourcemapSetting = boolean | "inline" | "hidden";

export function willUploadSourceMaps(
  userConfig: UserConfig,
  options: InterfereViteOptions,
  command: "build" | "serve",
  env: Record<string, string | undefined>,
) {
  return (
    command === "build" &&
    (userConfig.build?.lib === undefined ||
      userConfig.build.lib === false ||
      options.sourceMaps?.upload === true ||
      options.sourceMaps?.retain === true) &&
    (options.sourceMaps?.retain === true ||
      (options.sourceMaps?.upload !== false &&
        Boolean(pickEnvValue(env, "INTERFERE_API_KEY"))))
  );
}

export function forceSourcemapForUpload(userConfig: UserConfig) {
  const previous = userConfig.build?.sourcemap;

  userConfig.build = { ...userConfig.build, sourcemap: true };

  return (
    previous === undefined || previous === true ? undefined : previous
  ) satisfies SourcemapSetting | undefined;
}

export function warnForcedSourcemap(
  logger: Logger,
  overridden: SourcemapSetting | undefined,
  option = "build.sourcemap",
) {
  if (overridden === undefined) {
    return;
  }

  logger.warn(
    `[interfere] Forced ${option} to true (was ${JSON.stringify(overridden)}). Source maps are required to symbolicate errors; they are uploaded then deleted from the build output, so they are never served. Pass interfere({ sourceMaps: { upload: false } }) to opt out.`,
  );
}
