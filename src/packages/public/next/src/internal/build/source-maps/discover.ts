import type { NextBuildBundler } from "../detect-bundler.js";
import { discoverTurbopack } from "./discover-turbopack.js";
import { discoverWebpack } from "./discover-webpack.js";
import { normalizeDistDir } from "./paths.js";

export interface SourceMapFile {
  readonly absolute: string;
  readonly chunkAbsolutes: string[];
  readonly chunkUrl: string;
  readonly chunkUrls?: string[];
  readonly content: string;
  readonly debugId: string;
  readonly hash: string;
  readonly path: string;
}

export interface DiscoveryResult {
  readonly files: SourceMapFile[];
  readonly missingSourceMapCount: number;
  readonly sourceFileCount: number;
}

export async function discoverSourceMaps(opts: {
  bundler: NextBuildBundler;
  projectDir: string;
  distDir: string;
}) {
  const distDir = normalizeDistDir(opts.distDir);

  const discover = {
    webpack: discoverWebpack,
    turbopack: discoverTurbopack,
  } satisfies Record<NextBuildBundler, typeof discoverWebpack>;

  return discover[opts.bundler]({ projectDir: opts.projectDir, distDir });
}

export { injectDebugIdIntoJs } from "@interfere/types/data/source-maps";

export { injectDebugIdIntoMap } from "./discover-webpack.js";
