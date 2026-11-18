import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { mapWithConcurrency } from "@interfere/helpers/map-with-concurrency";
import {
  injectDebugIdIntoJs,
  readDebugIdFromJs,
} from "@interfere/types/data/source-maps";

import { createHash } from "node:crypto";
import { readFile, readdir, realpath, writeFile } from "node:fs/promises";
import { dirname, isAbsolute, join, relative, resolve, sep } from "node:path";

export { injectDebugIdIntoJs } from "@interfere/types/data/source-maps";

const DISCOVERY_CONCURRENCY = 16;

const LEADING_DOT_SLASH_RE = /^(?:\.\/)+/u;

const SOURCEMAPPING_RE = /\/\/[#@]\s*sourceMappingURL=(?<sourceMap>\S+)\s*$/u;

const RUNTIME_DEBUG_ID_RE =
  /_debugIds\[[^\]]+\]\s*=\s*["'`](?<id>[a-fA-F0-9-]{36})["'`]/gu;

const SCRIPT_SUFFIXES = [".js", ".mjs", ".cjs"] as const;

interface SourceMapFile {
  readonly absolute: string;
  readonly chunkUrl: string;
  readonly chunkUrls?: string[];
  readonly chunkAbsolutes: string[];
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

interface PublicPathRewrite {
  from: string;
  to: string;
}

export interface DiscoverByPairingOptions {
  distDir: string;
  projectDir: string;
  publicPathRewrites: readonly PublicPathRewrite[];
  subtrees: readonly string[];
}

function normalizeDistDir(distDir: string) {
  return distDir.replaceAll("\\", "/").replace(LEADING_DOT_SLASH_RE, "");
}

function resolveDistDir(projectDir: string, distDir: string) {
  return isAbsolute(distDir) ? distDir : resolve(projectDir, distDir);
}

function isInsideDir(root: string, candidate: string) {
  const rel = relative(root, candidate);

  return rel !== "" && !isAbsolute(rel) && rel.split(sep)[0] !== "..";
}

async function resolveContainedMap(
  absDistDir: string,
  realDistDir: string,
  jsAbs: string,
  sourceMappingUrl: string,
) {
  const mapAbs = resolve(dirname(jsAbs), decodeURIComponent(sourceMappingUrl));

  if (!isInsideDir(absDistDir, mapAbs)) {
    return null;
  }

  const mapReal = await realpath(mapAbs).catch(() => null);

  return mapReal === null || isInsideDir(realDistDir, mapReal) ? mapAbs : null;
}

async function walkDistTrees(
  absDistDir: string,
  suffixes: readonly string[],
  subtrees: readonly string[],
) {
  const results = await Promise.all(
    subtrees.map(async (subtree) => {
      const root = subtree === "." ? absDistDir : join(absDistDir, subtree);

      const entries = await readdir(root, { recursive: true }).catch(
        () => [] as string[],
      );

      const out: string[] = [];

      for (const entry of entries) {
        if (suffixes.some((suffix) => entry.endsWith(suffix))) {
          out.push(join(root, entry));
        }
      }

      return out;
    }),
  );

  return results.flat();
}

function toPublicPath(rel: string, rewrites: readonly PublicPathRewrite[]) {
  const normalized = rel.replaceAll("\\", "/");

  for (const { from, to } of rewrites) {
    const prefix = from.endsWith("/") ? from : `${from}/`;

    if (normalized.startsWith(prefix)) {
      return `${to}${normalized.slice(prefix.length)}`;
    }
  }

  return normalized;
}

function injectDebugIdIntoMap(content: string, debugId: string) {
  const json: unknown = JSON.parse(content);

  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Source-map files contain untrusted JSON; primitives and arrays cannot carry debug-ID fields.
  if (json === null || typeof json !== "object" || Array.isArray(json)) {
    throw new TypeError("Source map must be a JSON object");
  }

  Reflect.set(json, "debugId", debugId);

  return JSON.stringify(json);
}

export async function discoverByPairing(opts: DiscoverByPairingOptions) {
  const absDistDir = resolveDistDir(
    opts.projectDir,
    normalizeDistDir(opts.distDir),
  );

  const realDistDir = await realpath(absDistDir).catch(() => absDistDir);

  const jsPaths = await walkDistTrees(
    absDistDir,
    SCRIPT_SUFFIXES,
    opts.subtrees,
  );

  let unpairedChunkCount = 0;

  const discovered = await mapWithConcurrency(
    jsPaths.toSorted(),
    DISCOVERY_CONCURRENCY,
    async (jsAbs) => {
      const content = await readFile(jsAbs, "utf8");

      const match = SOURCEMAPPING_RE.exec(content);

      if (
        !match?.groups?.["sourceMap"] ||
        match.groups["sourceMap"].startsWith("data:")
      ) {
        if (
          readDebugIdFromJs(content) !== null ||
          [...content.matchAll(RUNTIME_DEBUG_ID_RE)].length > 0
        ) {
          unpairedChunkCount += 1;
        }

        return null;
      }

      const mapAbs = await resolveContainedMap(
        absDistDir,
        realDistDir,
        jsAbs,
        match.groups["sourceMap"],
      );

      if (mapAbs === null) {
        return null;
      }

      const jsPublic = toPublicPath(
        relative(absDistDir, jsAbs),
        opts.publicPathRewrites,
      );

      const mapPublic = toPublicPath(
        relative(absDistDir, mapAbs),
        opts.publicPathRewrites,
      );

      return [
        mapPublic,
        {
          existingDebugId:
            readDebugIdFromJs(content) ??
            [...content.matchAll(RUNTIME_DEBUG_ID_RE)].at(-1)?.groups?.["id"],
          jsAbs,
          jsContent: content,
          jsPublic,
          mapAbs,
        },
      ] as const;
    },
  );

  const pairs = new Map<
    string,
    NonNullable<(typeof discovered)[number]>[1][]
  >();

  for (const [mapPublic, pair] of discovered.filter(
    (entry) => entry !== null,
  )) {
    const chunks = pairs.get(mapPublic) ?? [];

    chunks.push(pair);

    pairs.set(mapPublic, chunks);
  }

  const prepared = await mapWithConcurrency(
    [...pairs],
    DISCOVERY_CONCURRENCY,
    async ([mapPublic, chunks]) => {
      const [pair] = chunks;

      if (!pair) {
        return null;
      }

      const mapContent = await readFile(pair.mapAbs, "utf8").catch(() => null);

      if (mapContent === null) {
        return null;
      }

      const existingIds = new Set(
        chunks.flatMap((chunk) =>
          chunk.existingDebugId ? [chunk.existingDebugId] : [],
        ),
      );

      const [existingDebugId] = existingIds;

      const digest = createHash("sha256")
        .update(mapPublic)
        .update("\0")
        .update(mapContent)
        .digest("hex");

      const debugId =
        existingIds.size === 1 && existingDebugId
          ? existingDebugId
          : [
              digest.slice(0, 8),
              digest.slice(8, 12),
              `8${digest.slice(13, 16)}`,
              `8${digest.slice(17, 20)}`,
              digest.slice(20, 32),
            ].join("-");

      const mapJson: unknown = JSON.parse(mapContent);

      const finalMap =
        mapJson !== null && // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Only a parsed object with the matching debug-ID field can bypass map rewriting.
        typeof mapJson === "object" &&
        "debugId" in mapJson &&
        mapJson.debugId === debugId
          ? mapContent
          : injectDebugIdIntoMap(mapContent, debugId);

      return {
        chunks,
        debugId,
        finalMap,
        mapPublic,
        pair,
        rewritesMap: finalMap !== mapContent,
      };
    },
  );

  const parsed = prepared.filter((entry) => entry !== null);

  // WARNING: A failed discovery is followed at once by a cleanup that strips and deletes every map, so no debug-ID write may start before every map has parsed or still be running when discovery fails.
  const failedWrites = await mapWithConcurrency(
    parsed,
    DISCOVERY_CONCURRENCY,
    async ({ chunks, debugId, finalMap, pair, rewritesMap }) => {
      const writes = await Promise.allSettled([
        ...chunks.flatMap((chunk) =>
          chunk.existingDebugId === debugId
            ? []
            : [
                writeFile(
                  chunk.jsAbs,
                  injectDebugIdIntoJs(chunk.jsContent, debugId),
                  "utf8",
                ),
              ],
        ),
        ...(rewritesMap ? [writeFile(pair.mapAbs, finalMap, "utf8")] : []),
      ]);

      return writes.find((write) => write.status === "rejected");
    },
  );

  const failedWrite = failedWrites.find((write) => write !== undefined);

  if (failedWrite) {
    throw failedWrite.reason;
  }

  const loaded = parsed.map(
    ({ chunks, debugId, finalMap, mapPublic, pair }) => {
      const chunkUrls = [
        ...new Set(chunks.map((chunk) => chunk.jsPublic)),
      ].toSorted();

      return omitUndefined({
        absolute: pair.mapAbs,
        chunkAbsolutes: [
          ...new Set(chunks.map((chunk) => chunk.jsAbs)),
        ].toSorted(),
        chunkUrl: chunkUrls[0] ?? pair.jsPublic,
        chunkUrls: chunkUrls.length > 1 ? chunkUrls : undefined,
        content: finalMap,
        debugId,
        hash: createHash("sha256").update(finalMap).digest("hex"),
        path: mapPublic,
      }) satisfies SourceMapFile;
    },
  );

  return {
    files: loaded,
    missingSourceMapCount: pairs.size - loaded.length + unpairedChunkCount,
    sourceFileCount: pairs.size + unpairedChunkCount,
  } satisfies DiscoveryResult;
}

/**
 * Pairs every chunk with the contained source map its `sourceMappingURL`
 * names, without parsing or rewriting either file, so a build can remove
 * maps that discovery could not parse.
 */
export async function pairSourceMapsForCleanup(distDir: string) {
  const absDistDir = resolve(distDir);

  const [realDistDir, jsPaths] = await Promise.all([
    realpath(absDistDir).catch(() => absDistDir),
    walkDistTrees(absDistDir, SCRIPT_SUFFIXES, ["."]),
  ]);

  const pairs = await mapWithConcurrency(
    jsPaths,
    DISCOVERY_CONCURRENCY,
    async (jsAbs) => {
      const content = await readFile(jsAbs, "utf8").catch(() => "");

      const sourceMap = SOURCEMAPPING_RE.exec(content)?.groups?.["sourceMap"];

      if (!sourceMap || sourceMap.startsWith("data:")) {
        return null;
      }

      const mapAbs = await resolveContainedMap(
        absDistDir,
        realDistDir,
        jsAbs,
        sourceMap,
      );

      return mapAbs === null ? null : ([mapAbs, jsAbs] as const);
    },
  );

  const chunksByMap = new Map<string, string[]>();

  for (const [mapAbs, jsAbs] of pairs.filter((pair) => pair !== null)) {
    chunksByMap.set(mapAbs, [...(chunksByMap.get(mapAbs) ?? []), jsAbs]);
  }

  return [...chunksByMap].map(([absolute, chunkAbsolutes]) => ({
    absolute,
    chunkAbsolutes,
  }));
}
