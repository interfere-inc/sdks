import { createHash } from "node:crypto";
import { readFile, realpath } from "node:fs/promises";
import { dirname, isAbsolute, join, relative, resolve, sep } from "node:path";
import { fileURLToPath } from "node:url";

import { z } from "zod";

import type { SourceMapFile } from "./discover.js";

const PROJECT_PREFIX = "turbopack:///[project]/";

const URL_SCHEME_RE = /^[a-z][a-z\d+.-]*:/iu;

const requiredServerFilesSchema = z.object({
  config: z
    .object({
      outputFileTracingRoot: z.string().min(1).optional(),
      turbopack: z.object({ root: z.string().min(1).optional() }).nullish(),
    })
    .optional(),
});

const rawMapSchema = z
  .object({
    sourceRoot: z.string().optional(),
    sources: z.array(z.string().nullable()),
  })
  .loose();

const indexMapSchema = z
  .object({
    sections: z.array(z.object({ map: rawMapSchema }).loose()),
  })
  .loose();

type RawMap = z.infer<typeof rawMapSchema>;

type IndexMap = z.infer<typeof indexMapSchema>;

interface Roots {
  readonly distDirs: readonly string[];
  readonly roots: readonly string[];
}

async function aliases(path: string) {
  const resolved = resolve(path);

  const real = await realpath(resolved).catch(() => resolved);

  return [...new Set([real, resolved])];
}

function isInside(root: string, candidate: string) {
  const rel = relative(root, candidate);

  return rel !== "" && !rel.startsWith("..") && !isAbsolute(rel);
}

async function projectRoots(projectDir: string, recorded: string) {
  const projects = await aliases(projectDir);

  const roots = await aliases(resolve(projectDir, recorded));

  const containsProject = roots.some((root) =>
    projects.some((project) => root === project || isInside(root, project)),
  );

  return containsProject ? roots : projects;
}

function parseJson(content: string) {
  try {
    return { ok: true as const, value: JSON.parse(content) as unknown };
  } catch {
    return { ok: false as const };
  }
}

async function readTurbopackRoot(absDistDir: string, projectDir: string) {
  const content = await readFile(
    join(absDistDir, "required-server-files.json"),
    "utf8",
  ).catch(() => null);

  const json = content === null ? null : parseJson(content);

  const parsed = json?.ok
    ? requiredServerFilesSchema.safeParse(json.value)
    : null;

  const config = parsed?.success ? parsed.data.config : undefined;

  return config?.turbopack?.root ?? config?.outputFileTracingRoot ?? projectDir;
}

function withSourceRoot(source: string, sourceRoot: string) {
  if (sourceRoot === "" || URL_SCHEME_RE.test(source) || isAbsolute(source)) {
    return source;
  }

  return sourceRoot.endsWith("/")
    ? `${sourceRoot}${source}`
    : `${sourceRoot}/${source}`;
}

function toFilePath(source: string, mapDir: string) {
  if (source.startsWith("file:")) {
    try {
      return fileURLToPath(source);
    } catch {
      return null;
    }
  }

  if (isAbsolute(source)) {
    return source;
  }

  if (URL_SCHEME_RE.test(source)) {
    return null;
  }

  return resolve(mapDir, decodeURIComponent(source));
}

function projectSource(source: string, mapDir: string, roots: Roots) {
  const path = toFilePath(source, mapDir);

  if (path === null || roots.distDirs.some((dir) => isInside(dir, path))) {
    return null;
  }

  const root = roots.roots.find((candidate) => isInside(candidate, path));

  return root === undefined
    ? null
    : `${PROJECT_PREFIX}${relative(root, path).split(sep).join("/")}`;
}

function normalizeRawMap(map: RawMap, mapDir: string, roots: Roots) {
  const sourceRoot = map.sourceRoot ?? "";

  const rewritten = map.sources.map((source) => {
    if (source === null) {
      return { changed: false, source };
    }

    const rooted = withSourceRoot(source, sourceRoot);
    const normalized = projectSource(rooted, mapDir, roots);

    return normalized === null
      ? { changed: false, source: rooted }
      : { changed: true, source: normalized };
  });

  if (!rewritten.some((entry) => entry.changed)) {
    return null;
  }

  const { sourceRoot: _sourceRoot, ...rest } = map;

  return { ...rest, sources: rewritten.map((entry) => entry.source) };
}

function normalizeIndexMap(map: IndexMap, mapDir: string, roots: Roots) {
  const sections = map.sections.map((section) => {
    const normalized = normalizeRawMap(section.map, mapDir, roots);

    return normalized === null ? null : { ...section, map: normalized };
  });

  return sections.every((section) => section === null)
    ? null
    : {
        ...map,
        sections: sections.map(
          (section, index) => section ?? map.sections[index],
        ),
      };
}

function normalizeMap(content: string, mapDir: string, roots: Roots) {
  const json = parseJson(content);

  if (!json.ok) {
    return null;
  }

  const index = indexMapSchema.safeParse(json.value);

  if (index.success) {
    return normalizeIndexMap(index.data, mapDir, roots);
  }

  const raw = rawMapSchema.safeParse(json.value);

  return raw.success ? normalizeRawMap(raw.data, mapDir, roots) : null;
}

/** Rewrite server source-map sources inside the Turbopack root to the `turbopack:///[project]/` URLs browser maps use. */
export async function normalizeTurbopackServerSources(
  files: SourceMapFile[],
  opts: { absDistDir: string; projectDir: string },
) {
  const serverDir = join(opts.absDistDir, "server");

  const roots: Roots = {
    distDirs: await aliases(opts.absDistDir),
    roots: await projectRoots(
      opts.projectDir,
      await readTurbopackRoot(opts.absDistDir, opts.projectDir),
    ),
  };

  return Promise.all(
    files.map(async (file) => {
      if (!isInside(serverDir, file.absolute)) {
        return file;
      }

      const [mapDir = dirname(file.absolute)] = await aliases(
        dirname(file.absolute),
      );

      const normalized = normalizeMap(file.content, mapDir, roots);

      if (normalized === null) {
        return file;
      }

      const content = JSON.stringify(normalized);

      return {
        ...file,
        content,
        hash: createHash("sha256").update(content).digest("hex"),
      } satisfies SourceMapFile;
    }),
  );
}
