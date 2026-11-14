import { createHash } from "node:crypto";
import {
  mkdir,
  mkdtemp,
  readFile,
  realpath,
  rm,
  symlink,
  writeFile,
} from "node:fs/promises";
import { SourceMap } from "node:module";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";

import type { SourceMapFile } from "./discover.js";
import { discoverSourceMaps } from "./discover.js";
import { afterEach, beforeEach, describe, expect, test } from "vitest";

const FIXTURE_DIR = join(
  import.meta.dirname,
  "../../../../test/fixtures/turbopack-server",
);

const CHUNK_NAME = "[root-of-the-server]__0l3q8v7ytmj_p._.js";

interface RawMap {
  debugId?: string;
  ignoreList?: number[];
  mappings: string;
  names: string[];
  sourceRoot?: string;
  sources: string[];
  sourcesContent?: (string | null)[];
}

interface IndexMap {
  sections: { map: RawMap; offset: { column: number; line: number } }[];
  version: 3;
}

let workspace: string;

beforeEach(async () => {
  workspace = await mkdtemp(join(tmpdir(), "turbopack-server-sources-"));
});

afterEach(async () => {
  await rm(workspace, { force: true, recursive: true });
});

async function readFixture() {
  const [chunk, map] = await Promise.all([
    readFile(join(FIXTURE_DIR, "chunk.js.txt"), "utf8"),
    readFile(join(FIXTURE_DIR, "chunk.js.map"), "utf8"),
  ]);

  return { chunk, map };
}

async function writeBuild(opts: {
  chunk: string;
  chunkDir?: string;
  map: string;
  projectDir: string;
  root?: string;
}) {
  const dir = join(
    opts.projectDir,
    ".next",
    opts.chunkDir ?? "server/chunks/ssr",
  );

  await mkdir(dir, { recursive: true });

  await Promise.all([
    writeFile(join(dir, CHUNK_NAME), opts.chunk),
    writeFile(join(dir, `${CHUNK_NAME}.map`), opts.map),
    writeFile(
      join(opts.projectDir, ".next/required-server-files.json"),
      JSON.stringify({
        version: 1,
        appDir: opts.projectDir,
        relativeAppDir: "",
        config: {
          outputFileTracingRoot: opts.root ?? opts.projectDir,
          turbopack: { root: opts.root ?? opts.projectDir },
        },
        files: [],
        ignore: [],
      }),
    ),
  ]);

  return join(dir, `${CHUNK_NAME}.map`);
}

async function discover(projectDir: string) {
  const result = await discoverSourceMaps({
    bundler: "turbopack",
    distDir: ".next",
    projectDir,
  });

  return result.files;
}

function onlyFile(files: SourceMapFile[]) {
  expect(files).toHaveLength(1);

  const [file] = files;

  if (file === undefined) {
    throw new Error("expected one discovered source map");
  }

  return file;
}

function originOf(map: RawMap, chunk: string, needle: string) {
  const offset = chunk.indexOf(needle);

  expect(offset).toBeGreaterThan(-1);

  const before = chunk.slice(0, offset).split("\n");

  return new SourceMap({
    file: "",
    mappings: map.mappings,
    names: map.names,
    sourceRoot: "",
    sources: map.sources,
    sourcesContent: [],
    version: 3,
  }).findEntry(before.length - 1, (before.at(-1) ?? "").length);
}

describe("Turbopack server source-map sources", () => {
  test("rewrites map-relative server sources to project-relative turbopack URLs", async () => {
    expect.hasAssertions();

    const { chunk, map } = await readFixture();

    const projectDir = join(workspace, "app");

    await writeBuild({ chunk, map, projectDir });

    const file = onlyFile(await discover(projectDir));

    const uploaded = JSON.parse(file.content) as RawMap;
    const original = JSON.parse(map) as RawMap;

    expect(uploaded.sources).toStrictEqual([
      "turbopack:///[project]/app/layout.js",
      "turbopack:///[project]/node_modules/next/src/server/route-modules/app-page/module.compiled.js",
      "turbopack:///[project]/node_modules/next/src/server/route-modules/app-page/vendored/rsc/react-jsx-runtime.ts",
    ]);

    expect(uploaded.sourcesContent).toStrictEqual(original.sourcesContent);
    expect(uploaded.mappings).toBe(original.mappings);
    expect(uploaded.names).toStrictEqual(original.names);
    expect(uploaded.ignoreList).toStrictEqual(original.ignoreList);
    expect(uploaded.debugId).toBe(file.debugId);

    expect(file.hash).toBe(
      createHash("sha256").update(file.content).digest("hex"),
    );

    expect(originOf(uploaded, chunk, '"html"')).toMatchObject({
      originalLine: 0,
      originalSource: "turbopack:///[project]/app/layout.js",
    });
  });

  test("never uploads a build-machine path in any server source", async () => {
    expect.hasAssertions();

    const { chunk, map } = await readFixture();

    const projectDir = join(workspace, "app");

    await writeBuild({ chunk, map, projectDir });

    const file = onlyFile(await discover(projectDir));

    const { sources } = JSON.parse(file.content) as RawMap;

    for (const source of sources) {
      expect(source).not.toContain(workspace);
      expect(source).not.toContain("../");
    }
  });

  test("leaves the on-disk server map untouched for the Node runtime", async () => {
    expect.hasAssertions();

    const { chunk, map } = await readFixture();

    const projectDir = join(workspace, "app");

    const mapPath = await writeBuild({ chunk, map, projectDir });

    await discover(projectDir);

    const onDisk = JSON.parse(await readFile(mapPath, "utf8")) as RawMap;

    expect(onDisk.sources).toStrictEqual((JSON.parse(map) as RawMap).sources);
  });

  test("matches the recorded root through a symlinked project path", async () => {
    expect.hasAssertions();

    const { chunk, map } = await readFixture();

    const projectDir = join(workspace, "app");

    await mkdir(projectDir, { recursive: true });

    const linked = join(workspace, "linked");

    await symlink(projectDir, linked);

    await writeBuild({
      chunk,
      map,
      projectDir,
      root: await realpath(projectDir),
    });

    const file = onlyFile(await discover(linked));

    expect((JSON.parse(file.content) as RawMap).sources[0]).toBe(
      "turbopack:///[project]/app/layout.js",
    );
  });

  test("falls back to the project when the recorded root does not contain it", async () => {
    expect.hasAssertions();

    const { chunk, map } = await readFixture();

    const projectDir = join(workspace, "app");

    await writeBuild({
      chunk,
      map,
      projectDir,
      root: join(workspace, "moved-build"),
    });

    const file = onlyFile(await discover(projectDir));

    expect((JSON.parse(file.content) as RawMap).sources[0]).toBe(
      "turbopack:///[project]/app/layout.js",
    );
  });

  test("names monorepo sources relative to the Turbopack root, like browser maps", async () => {
    expect.hasAssertions();

    const { chunk, map } = await readFixture();

    const root = join(workspace, "repo");
    const projectDir = join(root, "apps/web");

    await writeBuild({ chunk, map, projectDir, root });

    const file = onlyFile(await discover(projectDir));

    expect((JSON.parse(file.content) as RawMap).sources[0]).toBe(
      "turbopack:///[project]/apps/web/app/layout.js",
    );
  });

  test("normalises absolute and file:// sources, honouring sourceRoot", async () => {
    expect.hasAssertions();

    const { chunk, map } = await readFixture();

    const projectDir = join(workspace, "app");
    const original = JSON.parse(map) as RawMap;

    await writeBuild({
      chunk,
      map: JSON.stringify({
        ...original,
        sources: [
          join(projectDir, "app/layout.js"),
          `file://${join(projectDir, "node_modules/next/src/server/route-modules/app-page/module.compiled.js")}`,
          "react-jsx-runtime.ts",
        ],
        sourceRoot: `file://${join(projectDir, "node_modules/next/src/server/route-modules/app-page/vendored/rsc")}/`,
      }),
      projectDir,
    });

    const uploaded = JSON.parse(
      onlyFile(await discover(projectDir)).content,
    ) as RawMap;

    expect(uploaded.sources).toStrictEqual([
      "turbopack:///[project]/app/layout.js",
      "turbopack:///[project]/node_modules/next/src/server/route-modules/app-page/module.compiled.js",
      "turbopack:///[project]/node_modules/next/src/server/route-modules/app-page/vendored/rsc/react-jsx-runtime.ts",
    ]);

    expect(uploaded.sourceRoot ?? "").toBe("");
  });

  test("normalises every section of an index map", async () => {
    expect.hasAssertions();

    const { chunk, map } = await readFixture();

    const projectDir = join(workspace, "app");

    const index: IndexMap = {
      version: 3,
      sections: [
        { offset: { line: 0, column: 0 }, map: JSON.parse(map) as RawMap },
      ],
    };

    await writeBuild({ chunk, map: JSON.stringify(index), projectDir });

    const uploaded = JSON.parse(
      onlyFile(await discover(projectDir)).content,
    ) as IndexMap;

    expect(uploaded.sections[0]?.map.sources[0]).toBe(
      "turbopack:///[project]/app/layout.js",
    );

    expect(uploaded.sections[0]?.map.sourcesContent).toStrictEqual(
      (JSON.parse(map) as RawMap).sourcesContent,
    );
  });

  test("keeps Turbopack runtime and foreign-scheme URLs as they are", async () => {
    expect.hasAssertions();

    const { chunk, map } = await readFixture();

    const projectDir = join(workspace, "app");
    const original = JSON.parse(map) as RawMap;

    const sources = [
      "turbopack:///[turbopack]/shared/runtime/runtime-utils.ts",
      "../../../../app/layout.js",
      "webpack://next/external.js",
    ];

    await writeBuild({
      chunk,
      map: JSON.stringify({ ...original, sources }),
      projectDir,
    });

    const uploaded = JSON.parse(
      onlyFile(await discover(projectDir)).content,
    ) as RawMap;

    expect(uploaded.sources).toStrictEqual([
      "turbopack:///[turbopack]/shared/runtime/runtime-utils.ts",
      "turbopack:///[project]/app/layout.js",
      "webpack://next/external.js",
    ]);
  });

  test("leaves browser maps byte-identical apart from debug-ID injection", async () => {
    expect.hasAssertions();

    const { chunk, map } = await readFixture();

    const projectDir = join(workspace, "app");

    const browserMap = JSON.stringify({
      ...(JSON.parse(map) as RawMap),
      sources: ["turbopack:///[project]/app/layout.js"],
    });

    const mapPath = await writeBuild({
      chunk,
      chunkDir: "static/chunks",
      map: browserMap,
      projectDir,
    });

    const file = onlyFile(await discover(projectDir));

    expect(dirname(file.absolute)).toBe(dirname(mapPath));

    expect((JSON.parse(file.content) as RawMap).sources).toStrictEqual([
      "turbopack:///[project]/app/layout.js",
    ]);
  });
});
