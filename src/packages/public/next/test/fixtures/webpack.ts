import { PHASE_DEVELOPMENT_SERVER } from "next/constants";
import loadConfig from "next/dist/server/config";
import type { WebpackConfigContext } from "next/dist/server/config-shared";
import webpack from "webpack";

export async function webpackContext(projectDir: string) {
  const config = await loadConfig(PHASE_DEVELOPMENT_SERVER, projectDir, {
    customConfig: {},
    silent: true,
  });

  return {
    dir: projectDir,
    dev: true,
    isServer: false,
    buildId: "test-build",
    config,
    defaultLoaders: { babel: { loader: "next/dist/build/babel/loader/index" } },
    totalPages: 1,
    webpack,
  } satisfies WebpackConfigContext;
}
