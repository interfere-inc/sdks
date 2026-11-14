import { readDebugIdFromJs } from "@interfere/types/data/source-maps";
import { mapWithConcurrency } from "@interfere/helpers/map-with-concurrency";

import { readFile } from "node:fs/promises";
import { relative, resolve } from "node:path";

import { log } from "../../logger.js";
import type { SourceMapFile } from "./discover.js";
import { resolveDistDir, walkDistTrees } from "./paths.js";

export async function reportUnmappedChunks(opts: {
  projectDir: string;
  distDir: string;
  discovered: readonly SourceMapFile[];
}) {
  const covered = new Set(
    opts.discovered.flatMap((file) =>
      file.chunkAbsolutes.map((chunk) => resolve(chunk)),
    ),
  );

  const absDistDir = resolveDistDir(opts.projectDir, opts.distDir);

  const chunks = await walkDistTrees(absDistDir, ".js");

  const missing = await mapWithConcurrency(
    chunks.toSorted(),
    16,
    async (jsAbs) => {
      if (covered.has(resolve(jsAbs))) {
        return null;
      }

      const content = await readFile(jsAbs, "utf8").catch(() => "");

      return readDebugIdFromJs(content) !== null ||
        /(?:^|\n)[\t ]*\/\/[#@]\s*sourceMappingURL=\S+\s*$/u.test(content)
        ? relative(absDistDir, jsAbs).replaceAll("\\", "/")
        : null;
    },
  );

  const paths = missing.filter((path) => path !== null);

  if (paths.length === 0) {
    return;
  }

  const clientCount = paths.filter((path) => path.startsWith("static/")).length;

  log.error("Source-map coverage is incomplete", [
    `${clientCount} client chunk(s) and ${paths.length - clientCount} server chunk(s) have source-map markers but no discovered map. Errors in those chunks will not resolve to source.`,
    ...paths.slice(0, 20),
    ...(paths.length > 20
      ? [`${paths.length - 20} more chunk(s) omitted.`]
      : []),
    "Ensure source maps are generated and configure coexisting build tools to preserve them until Interfere has discovered them.",
    "The build continues.",
  ]);
}
