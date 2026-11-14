import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { mapWithConcurrency } from "@interfere/helpers/map-with-concurrency";

import { createHash } from "node:crypto";
import { open, readFile, stat } from "node:fs/promises";
import { relative } from "node:path";

import type { DiscoveryResult, SourceMapFile } from "./discover.js";
import { discoverWebpack } from "./discover-webpack.js";
import { resolveDistDir, toPublicPath, walkDistTrees } from "./paths.js";
import { normalizeTurbopackServerSources } from "./turbopack-server-sources.js";

const DEBUG_ID_RE = /\/\/[#@]\s*debugId=(?<debugId>[0-9a-f-]+)\s*$/mu;

const SOURCE_MAPPING_URL_RE = /\/\/[#@]\s*sourceMappingURL=\S+\s*$/mu;

const DISCOVERY_CONCURRENCY = 16;

const DEBUG_ID_TAIL_BYTES = 64 * 1024;

function readChunkMarkers(text: string) {
  return {
    debugId: DEBUG_ID_RE.exec(text)?.groups?.["debugId"] ?? null,
    referencesMap: SOURCE_MAPPING_URL_RE.test(text),
  };
}

async function readChunk(jsAbs: string) {
  const tail = await readTail(jsAbs).catch(() => null);

  const fromTail = tail === null ? null : readChunkMarkers(tail);

  if (fromTail?.debugId) {
    return fromTail;
  }

  // WARNING: A debugId can occur outside the tail window; truncating the scan loses source-map identity.
  const content = await readFile(jsAbs, "utf8").catch(() => null);

  return content === null
    ? { debugId: null, referencesMap: false }
    : readChunkMarkers(content);
}

async function readTail(jsAbs: string) {
  const { size } = await stat(jsAbs);

  if (size <= DEBUG_ID_TAIL_BYTES) {
    return null;
  }

  const handle = await open(jsAbs, "r");

  try {
    const buf = Buffer.allocUnsafe(DEBUG_ID_TAIL_BYTES);

    const { bytesRead } = await handle.read(
      buf,
      0,
      DEBUG_ID_TAIL_BYTES,
      size - DEBUG_ID_TAIL_BYTES,
    );

    return buf.subarray(0, bytesRead).toString("utf8");
  } finally {
    await handle.close();
  }
}

/** Pair native debug IDs, then resolve contained sourceMappingURL references for chunks without IDs. */
export async function discoverTurbopack(opts: {
  projectDir: string;
  distDir: string;
}) {
  const absDistDir = resolveDistDir(opts.projectDir, opts.distDir);
  const relDistDir = relative(opts.projectDir, absDistDir);

  const [mapPaths, jsPaths] = await Promise.all([
    walkDistTrees(absDistDir, ".js.map"),
    walkDistTrees(absDistDir, ".js"),
  ]);

  const chunksByDebugId = new Map<
    string,
    {
      chunkAbsolutes: string[];
      chunkUrl: string;
      chunkUrls: string[];
      referencesMap: boolean;
    }
  >();

  const chunksWithoutDebugId: string[] = [];

  await mapWithConcurrency(jsPaths, DISCOVERY_CONCURRENCY, async (jsAbs) => {
    const { debugId, referencesMap } = await readChunk(jsAbs);

    if (!debugId) {
      chunksWithoutDebugId.push(jsAbs);

      return;
    }

    const chunkUrl = toPublicPath(relative(opts.projectDir, jsAbs), relDistDir);
    const chunk = chunksByDebugId.get(debugId);

    if (chunk) {
      chunk.chunkAbsolutes.push(jsAbs);

      chunk.chunkUrls.push(chunkUrl);

      chunk.referencesMap ||= referencesMap;
    } else {
      chunksByDebugId.set(debugId, {
        chunkAbsolutes: [jsAbs],
        chunkUrl,
        chunkUrls: [chunkUrl],
        referencesMap,
      });
    }
  });

  const files = await mapWithConcurrency(
    mapPaths,
    DISCOVERY_CONCURRENCY,
    async (mapAbs) => {
      const content = await readFile(mapAbs, "utf8").catch(() => null);

      if (content === null) {
        return null;
      }

      let debugId: string | undefined;

      try {
        const parsed: unknown = JSON.parse(content);

        if (
          parsed !== null && // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Turbopack map files may contain malformed JSON values; only objects can supply debugId metadata.
          typeof parsed === "object" &&
          "debugId" in parsed &&
          // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Only nonempty string debug IDs can join map metadata to IDs extracted from chunk comments.
          typeof parsed.debugId === "string" &&
          parsed.debugId.length > 0
        ) {
          ({ debugId } = parsed);
        }
      } catch {
        return null;
      }

      if (!debugId) {
        return null;
      }

      const chunk = chunksByDebugId.get(debugId);

      if (!chunk) {
        return null;
      }

      const chunkUrls = [...new Set(chunk.chunkUrls)].toSorted();

      return omitUndefined({
        absolute: mapAbs,
        chunkAbsolutes: chunk.chunkAbsolutes.toSorted(),
        chunkUrl: chunkUrls[0] ?? chunk.chunkUrl,
        chunkUrls: chunkUrls.length > 1 ? chunkUrls : undefined,
        content,
        debugId,
        hash: createHash("sha256").update(content).digest("hex"),
        path: toPublicPath(relative(opts.projectDir, mapAbs), relDistDir),
      }) satisfies SourceMapFile;
    },
  );

  const loaded = files.filter((f): f is SourceMapFile => f !== null);
  const loadedDebugIds = new Set(loaded.map((file) => file.debugId));

  const missingSourceMapCount = [...chunksByDebugId].filter(
    ([debugId, chunk]) => chunk.referencesMap && !loadedDebugIds.has(debugId),
  ).length;

  const referenced = await discoverWebpack({
    ...opts,
    chunkPaths: chunksWithoutDebugId,
  });

  const byPath = new Map(loaded.map((file) => [file.absolute, file]));

  for (const file of referenced.files) {
    const existing = byPath.get(file.absolute);

    if (!existing) {
      byPath.set(file.absolute, file);

      continue;
    }

    const chunkUrls = [
      ...new Set([
        ...(existing.chunkUrls ?? [existing.chunkUrl]),
        ...(file.chunkUrls ?? [file.chunkUrl]),
      ]),
    ].toSorted();

    byPath.set(file.absolute, {
      ...existing,
      chunkAbsolutes: [
        ...new Set([...existing.chunkAbsolutes, ...file.chunkAbsolutes]),
      ].toSorted(),
      chunkUrl: chunkUrls[0] ?? existing.chunkUrl,
      chunkUrls,
    });
  }

  return {
    files: await normalizeTurbopackServerSources([...byPath.values()], {
      absDistDir,
      projectDir: opts.projectDir,
    }),
    missingSourceMapCount:
      missingSourceMapCount + referenced.missingSourceMapCount,
    sourceFileCount: byPath.size + referenced.missingSourceMapCount,
  } satisfies DiscoveryResult;
}
