import { cleanupSourceMaps } from "@interfere/build/cleanup-source-maps";
import { mapWithConcurrency } from "@interfere/helpers/map-with-concurrency";
import { readFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import type { SourceMapFile } from "./discover.js";
import { resolveDistDir, walkDistTrees } from "./paths.js";

export async function cleanupBuildSourceMaps(
  context: { projectDir: string; distDir: string },
  discovered: readonly SourceMapFile[],
) {
  const distDir = resolveDistDir(context.projectDir, context.distDir);

  const [maps, chunks] = await Promise.all([
    walkDistTrees(distDir, ".map"),
    walkDistTrees(distDir, ".js"),
  ]);

  const inventory = new Map(
    maps.map((absolute) => [absolute, new Set<string>()]),
  );

  for (const file of discovered) {
    inventory.set(file.absolute, new Set(file.chunkAbsolutes));
  }

  await mapWithConcurrency(chunks, 16, async (chunk) => {
    const content = await readFile(chunk, "utf8");

    const reference = /\/\/[#@]\s*sourceMappingURL=(?<sourceMap>\S+)\s*$/u.exec(
      content,
    )?.groups?.["sourceMap"];

    if (!reference || reference.startsWith("data:")) {
      return;
    }

    let decoded: string;

    try {
      decoded = decodeURIComponent(reference);
    } catch {
      return;
    }

    inventory.get(resolve(dirname(chunk), decoded))?.add(chunk);
  });

  return cleanupSourceMaps(
    [...inventory].map(([absolute, chunkAbsolutes]) => ({
      absolute,
      chunkAbsolutes: [...chunkAbsolutes],
    })),
  );
}
