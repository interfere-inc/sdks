import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { mapWithConcurrency } from "@interfere/helpers/map-with-concurrency";
import {
  injectDebugIdIntoJs,
  readDebugIdFromJs,
} from "@interfere/types/data/source-maps";

import { createHash } from "node:crypto";
import { readFile, realpath, writeFile } from "node:fs/promises";
import { dirname, isAbsolute, relative, resolve, sep } from "node:path";

import type { DiscoveryResult, SourceMapFile } from "./discover.js";
import { resolveDistDir, toPublicPath, walkDistTrees } from "./paths.js";

const SOURCEMAPPING_RE = /\/\/[#@]\s*sourceMappingURL=(?<sourceMap>\S+)\s*$/u;

const DISCOVERY_CONCURRENCY = 16;

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

export function injectDebugIdIntoMap(content: string, debugId: string) {
  const json: unknown = JSON.parse(content);

  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Source-map JSON must be an object before debugId can be attached; primitives and arrays are invalid maps.
  if (json === null || typeof json !== "object" || Array.isArray(json)) {
    throw new TypeError("Source map must be a JSON object");
  }

  Reflect.set(json, "debugId", debugId);

  return JSON.stringify(json);
}

export async function discoverWebpack(opts: {
  projectDir: string;
  distDir: string;
  chunkPaths?: readonly string[];
}) {
  const absDistDir = resolveDistDir(opts.projectDir, opts.distDir);
  const relDistDir = relative(opts.projectDir, absDistDir);

  const realDistDir = await realpath(absDistDir).catch(() => absDistDir);

  const jsPaths = opts.chunkPaths ?? (await walkDistTrees(absDistDir, ".js"));

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
        relative(opts.projectDir, jsAbs),
        relDistDir,
      );

      const mapPublic = toPublicPath(
        relative(opts.projectDir, mapAbs),
        relDistDir,
      );

      return [
        mapPublic,
        {
          existingDebugId: readDebugIdFromJs(content),
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

  const prepared = await Promise.all(
    Array.from(pairs.entries(), async ([mapPublic, chunks]) => {
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
      const mapJson: unknown = JSON.parse(mapContent);

      const mapDebugId =
        mapJson !== null &&
        // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Untrusted build output must expose a valid string identity before it can be injected into JavaScript.
        typeof mapJson === "object" &&
        "debugId" in mapJson &&
        // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Preserve only source-map identities safe for the runtime registry carrier.
        typeof mapJson.debugId === "string" &&
        /^[a-f0-9-]+$/iu.test(mapJson.debugId)
          ? mapJson.debugId
          : undefined;

      const digest = createHash("sha256")
        .update(mapPublic)
        .update("\0")
        .update(mapContent)
        .digest("hex");

      const inheritedDebugId =
        existingIds.size === 0 ? mapDebugId : existingDebugId;

      const debugId =
        existingIds.size <= 1 && inheritedDebugId
          ? inheritedDebugId
          : [
              digest.slice(0, 8),
              digest.slice(8, 12),
              `8${digest.slice(13, 16)}`,
              `8${digest.slice(17, 20)}`,
              digest.slice(20, 32),
            ].join("-");

      const finalMap =
        mapJson !== null && // oxlint-disable-next-line anti-slop/no-runtime-typeof -- A parsed map can be any JSON value; only objects can carry an existing debugId.
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
    }),
  );

  const parsed = prepared.filter((entry) => entry !== null);

  // WARNING: A failed discovery is followed at once by a cleanup that strips and deletes every map, so no debug-ID write may start before every map has parsed or still be running when discovery fails.
  const writes = await Promise.allSettled(
    parsed.flatMap(({ chunks, debugId, finalMap, pair, rewritesMap }) => [
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
    ]),
  );

  const failedWrite = writes.find((write) => write.status === "rejected");

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
    missingSourceMapCount: pairs.size - loaded.length,
    sourceFileCount: pairs.size,
  } satisfies DiscoveryResult;
}
