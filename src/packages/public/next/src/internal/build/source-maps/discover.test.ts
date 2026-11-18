import { mkdir, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { setFlagsFromString } from "node:v8";
import { runInNewContext } from "node:vm";

import type { DiscoveryResult } from "./discover.js";
import {
  discoverSourceMaps,
  injectDebugIdIntoJs,
  injectDebugIdIntoMap,
} from "./discover.js";
import { afterAll, beforeAll, describe, expect, test, vi } from "vitest";

const writeHook = vi.hoisted(() => ({
  onFirstWrite: undefined as (() => void) | undefined,
}));

// oxlint-disable-next-line anti-slop/no-module-mocking, interfere-testing/no-test-doubles -- Every write still reaches the real writeFile unchanged; the wrapper only measures the heap once every map has been prepared and before the first write.
vi.mock(import("node:fs/promises"), async (importOriginal) => {
  const actual = await importOriginal();

  return {
    ...actual,
    writeFile: (async (...args: Parameters<typeof actual.writeFile>) => {
      const hook = writeHook.onFirstWrite;

      writeHook.onFirstWrite = undefined;

      hook?.();

      return actual.writeFile(...args);
    }) as typeof actual.writeFile,
  };
});

const JS_WITH_MAP = (mapName: string) =>
  `function hello(){}\n//# sourceMappingURL=${mapName}`;

const SOURCE_MAP = '{"version":3,"sources":["src/app.ts"],"mappings":"AAAA"}';

const APP_JS_SOURCE_MAP_URL_SUFFIX_RE = /\/\/# sourceMappingURL=app\.js\.map$/u;

const DEBUG_ID_COMMENT_RE = /\/\/# debugId=[a-f0-9-]{36}/u;

const DEBUG_ID_COMMENT_CAPTURE_RE = /\/\/# debugId=(?<debugId>[a-f0-9-]{36})/u;

const UUID_RE = /[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}/u;

let projectDir: string;

beforeAll(async () => {
  projectDir = await mkdtemp(join(tmpdir(), "discover-test-"));

  await mkdir(join(projectDir, ".next/static/chunks"), { recursive: true });

  await mkdir(join(projectDir, ".next/server/chunks"), { recursive: true });

  await writeFile(
    join(projectDir, ".next/static/chunks/abc123.js"),
    JS_WITH_MAP("abc123.js.map"),
  );

  await writeFile(
    join(projectDir, ".next/static/chunks/abc123.js.map"),
    SOURCE_MAP,
  );

  await writeFile(
    join(projectDir, ".next/server/chunks/[root-of-the-server]__2e816af5._.js"),
    JS_WITH_MAP("%5Broot-of-the-server%5D__2e816af5._.js.map"),
  );

  await writeFile(
    join(
      projectDir,
      ".next/server/chunks/[root-of-the-server]__2e816af5._.js.map",
    ),
    SOURCE_MAP,
  );

  await writeFile(
    join(projectDir, ".next/server/chunks/src_app_route_abc123.js"),
    JS_WITH_MAP("src_app_route_abc123.js.map"),
  );

  await writeFile(
    join(projectDir, ".next/server/chunks/src_app_route_abc123.js.map"),
    SOURCE_MAP,
  );
});

afterAll(async () => {
  await rm(projectDir, { force: true, recursive: true });
});

describe("discoverSourceMaps (webpack)", () => {
  test.each([
    { mapContent: "{", expectedError: SyntaxError },
    { mapContent: "null", expectedError: TypeError },
    { mapContent: "false", expectedError: TypeError },
    { mapContent: "42", expectedError: TypeError },
    { mapContent: '"map"', expectedError: TypeError },
    { mapContent: "[]", expectedError: TypeError },
  ])(
    "leaves every chunk and map unchanged when one of many maps is invalid: $mapContent",
    async ({ mapContent, expectedError }) => {
      const dir = await mkdtemp(join(tmpdir(), "discover-invalid-map-"));

      const chunks = join(dir, ".next/static/chunks");

      const names = [
        ...Array.from({ length: 40 }, (_, index) => `${index + 10}.js`),
        "zz-app.js",
      ];

      const files = names.flatMap((name) => [
        join(chunks, name),
        join(chunks, `${name}.map`),
      ]);

      const snapshot = async () =>
        Promise.all(files.map(async (file) => readFile(file, "utf8")));

      try {
        await mkdir(chunks, { recursive: true });

        await Promise.all(
          names.map(async (name) => {
            await writeFile(join(chunks, name), JS_WITH_MAP(`${name}.map`));

            await writeFile(
              join(chunks, `${name}.map`),
              name === "zz-app.js" ? mapContent : SOURCE_MAP,
            );
          }),
        );

        const before = await snapshot();

        await expect(
          discoverSourceMaps({
            bundler: "webpack",
            distDir: ".next",
            projectDir: dir,
          }),
        ).rejects.toThrow(expectedError);

        await expect(snapshot()).resolves.toStrictEqual(before);
      } finally {
        await rm(dir, { recursive: true, force: true });
      }
    },
  );

  test("preserves every URL when chunks share one source map", async () => {
    const dir = await mkdtemp(join(tmpdir(), "discover-shared-map-"));

    const chunks = join(dir, ".next/static/chunks");
    const firstId = "11111111-1111-4111-8111-111111111111";
    const lastId = "22222222-2222-4222-8222-222222222222";

    try {
      await mkdir(chunks, { recursive: true });

      await Promise.all([
        writeFile(
          join(chunks, "a.js"),
          `${" ".repeat(128 * 1024)}\n//# debugId=${firstId}\n//# sourceMappingURL=shared.js.map`,
        ),
        writeFile(
          join(chunks, "z.js"),
          `//# debugId=${lastId}\n//# sourceMappingURL=shared.js.map`,
        ),
        writeFile(join(chunks, "shared.js.map"), SOURCE_MAP),
      ]);

      const result = await discoverSourceMaps({
        bundler: "webpack",
        distDir: ".next",
        projectDir: dir,
      });

      expect(result.sourceFileCount).toBe(1);
      expect(result.files).toHaveLength(1);

      expect(result.files[0]).toMatchObject({
        chunkUrl: "_next/static/chunks/a.js",
        chunkUrls: ["_next/static/chunks/a.js", "_next/static/chunks/z.js"],
      });

      const [file] = result.files;

      expect(file?.debugId).toMatch(UUID_RE);
      expect(file?.debugId).not.toBe(firstId);
      expect(file?.debugId).not.toBe(lastId);

      const rewritten = await Promise.all(
        ["a.js", "z.js"].map(async (name) =>
          readFile(join(chunks, name), "utf8"),
        ),
      );

      expect(
        rewritten.map(
          (content) =>
            DEBUG_ID_COMMENT_CAPTURE_RE.exec(content)?.groups?.["debugId"],
        ),
      ).toStrictEqual([file?.debugId, file?.debugId]);

      expect(JSON.parse(file?.content ?? "null")).toMatchObject({
        debugId: file?.debugId,
      });

      const repeated = await discoverSourceMaps({
        bundler: "webpack",
        distDir: ".next",
        projectDir: dir,
      });

      expect(repeated.files).toStrictEqual(result.files);
    } finally {
      await rm(dir, { force: true, recursive: true });
    }
  });

  test("ignores a sourceMappingURL that resolves outside the dist directory", async () => {
    const dir = await mkdtemp(join(tmpdir(), "discover-traversal-"));

    const chunks = join(dir, ".next/static/chunks");
    const jsPath = join(chunks, "vendor-worker.js");
    const secretPath = join(dir, "outside", "secret.json");

    const jsContent =
      "self.onmessage=function(){};\n//# sourceMappingURL=../../../../outside/secret.json\n";

    const secret = '{"auths":{"registry.example.test":{"auth":"ZHVtbXk="}}}';

    try {
      await mkdir(chunks, { recursive: true });

      await mkdir(join(dir, "outside"), { recursive: true });

      await writeFile(jsPath, jsContent);

      await writeFile(secretPath, secret);

      await expect(
        discoverSourceMaps({
          bundler: "webpack",
          distDir: ".next",
          projectDir: dir,
        }),
      ).resolves.toStrictEqual({
        files: [],
        missingSourceMapCount: 0,
        sourceFileCount: 0,
      });

      await expect(readFile(jsPath, "utf8")).resolves.toBe(jsContent);

      await expect(readFile(secretPath, "utf8")).resolves.toBe(secret);
    } finally {
      await rm(dir, { force: true, recursive: true });
    }
  });

  test("finds source maps for files with bracket characters in their names", async () => {
    const result = await discoverSourceMaps({
      bundler: "webpack",
      distDir: ".next",
      projectDir,
    });

    const paths = result.files.map((f) => f.path);

    expect(paths).toContain(
      "_next/server/chunks/[root-of-the-server]__2e816af5._.js.map",
    );
  });

  test("finds standard source maps without special characters", async () => {
    const result = await discoverSourceMaps({
      bundler: "webpack",
      distDir: ".next",
      projectDir,
    });

    const paths = result.files.map((f) => f.path);

    expect(paths).toContain("_next/static/chunks/abc123.js.map");
    expect(paths).toContain("_next/server/chunks/src_app_route_abc123.js.map");
  });

  test("discovers all source maps in the fixture", async () => {
    const result = await discoverSourceMaps({
      bundler: "webpack",
      distDir: ".next",
      projectDir,
    });

    expect(result.files).toHaveLength(3);
  });

  test("populates chunkUrl on each discovered file", async () => {
    const result = await discoverSourceMaps({
      bundler: "webpack",
      distDir: ".next",
      projectDir,
    });

    const byPath = new Map(result.files.map((f) => [f.path, f]));

    expect(byPath.get("_next/static/chunks/abc123.js.map")?.chunkUrl).toBe(
      "_next/static/chunks/abc123.js",
    );

    expect(
      byPath.get("_next/server/chunks/[root-of-the-server]__2e816af5._.js.map")
        ?.chunkUrl,
    ).toBe("_next/server/chunks/[root-of-the-server]__2e816af5._.js");
  });

  test("holds about one copy of every map in memory when the first debug-ID write starts", async () => {
    setFlagsFromString("--expose-gc");

    const collectGarbage = runInNewContext("gc") as () => void;

    const dir = await mkdtemp(join(tmpdir(), "discover-peak-memory-"));

    const chunks = join(dir, ".next/static/chunks");
    const mapCount = 8;
    const mapBytes = 4 * 1024 * 1024;

    try {
      await mkdir(chunks, { recursive: true });

      await Promise.all(
        Array.from({ length: mapCount }, async (_, index) => {
          await writeFile(
            join(chunks, `c${index}.js`),
            JS_WITH_MAP(`c${index}.js.map`),
          );

          await writeFile(
            join(chunks, `c${index}.js.map`),
            JSON.stringify({
              mappings: "AAAA",
              sources: ["src/app.ts"],
              sourcesContent: [`${"a".repeat(mapBytes)}${index}`],
              version: 3,
            }),
          );
        }),
      );

      collectGarbage();

      const baseline = process.memoryUsage().heapUsed;
      let atFirstWrite = baseline;

      writeHook.onFirstWrite = () => {
        collectGarbage();

        atFirstWrite = process.memoryUsage().heapUsed;
      };

      const result = await discoverSourceMaps({
        bundler: "webpack",
        distDir: ".next",
        projectDir: dir,
      });

      expect(result.files).toHaveLength(mapCount);
      expect(atFirstWrite - baseline).toBeLessThan(mapCount * mapBytes * 1.5);
    } finally {
      writeHook.onFirstWrite = undefined;

      await rm(dir, { recursive: true, force: true });
    }
  });

  test("returns empty when distDir does not exist", async () => {
    const result = await discoverSourceMaps({
      bundler: "webpack",
      distDir: "nonexistent",
      projectDir,
    });

    expect(result.files).toHaveLength(0);
  });

  test("skips JS files without a sourceMappingURL comment", async () => {
    await writeFile(
      join(projectDir, ".next/static/chunks/no-map.js"),
      "function noMap(){}",
    );

    const result = await discoverSourceMaps({
      bundler: "webpack",
      distDir: ".next",
      projectDir,
    });

    const paths = result.files.map((f) => f.path);

    expect(paths).not.toContain("_next/static/chunks/no-map.js.map");
  });

  test("works when distDir is an absolute path", async () => {
    const absDistDir = join(projectDir, ".next");

    const result = await discoverSourceMaps({
      bundler: "webpack",
      distDir: absDistDir,
      projectDir,
    });

    expect(result.files.length).toBeGreaterThanOrEqual(3);

    const paths = result.files.map((f) => f.path);

    expect(paths).toContain("_next/static/chunks/abc123.js.map");

    expect(paths).toContain(
      "_next/server/chunks/[root-of-the-server]__2e816af5._.js.map",
    );
  });
});

describe("discoverSourceMaps (turbopack references)", () => {
  test("publishes no-ID runtime and Edge maps without replacing native ID authority", async () => {
    expect.hasAssertions();

    const project = await mkdtemp(
      join(tmpdir(), "discover-turbopack-references-"),
    );

    const chunks = join(project, ".next/server/edge/chunks");
    const nativeId = "11111111-2222-3333-4444-555555555555";
    const missingId = "99999999-2222-3333-4444-555555555555";

    const nativeMap = JSON.stringify({
      version: 3,
      debugId: nativeId,
      sources: ["native.ts"],
      mappings: "AAAA",
    });

    try {
      await mkdir(chunks, { recursive: true });

      await Promise.all([
        writeFile(
          join(chunks, "native.js"),
          `//# debugId=${nativeId}\n//# sourceMappingURL=wrong.js.map`,
        ),
        writeFile(join(chunks, "native-map.js.map"), nativeMap),
        writeFile(join(chunks, "alias.js"), JS_WITH_MAP("native-map.js.map")),
        writeFile(
          join(chunks, "[runtime].js"),
          JS_WITH_MAP("%5Bruntime%5D.js.map"),
        ),
        writeFile(join(chunks, "[runtime].js.map"), SOURCE_MAP),
        writeFile(join(chunks, "edge.js"), JS_WITH_MAP("edge.js.map")),
        writeFile(join(chunks, "edge.js.map"), SOURCE_MAP),
        writeFile(
          join(chunks, "unmatched.js"),
          `//# debugId=${missingId}\n//# sourceMappingURL=edge.js.map`,
        ),
      ]);

      const result = await discoverSourceMaps({
        bundler: "turbopack",
        distDir: ".next",
        projectDir: project,
      });

      expect(result.files).toHaveLength(3);
      expect(result.missingSourceMapCount).toBe(1);

      expect(
        result.files.find((file) => file.debugId === nativeId),
      ).toMatchObject({
        content: nativeMap,
        chunkUrls: [
          "_next/server/edge/chunks/alias.js",
          "_next/server/edge/chunks/native.js",
        ],
      });

      for (const name of ["[runtime]", "edge"]) {
        const file = result.files.find(
          (entry) => entry.chunkUrl === `_next/server/edge/chunks/${name}.js`,
        );

        expect(file?.debugId).toMatch(UUID_RE);

        expect(JSON.parse(file?.content ?? "null")).toMatchObject({
          debugId: file?.debugId,
        });
      }

      await expect(
        readFile(join(chunks, "unmatched.js"), "utf8"),
      ).resolves.toContain(`debugId=${missingId}`);

      await expect(
        readFile(join(chunks, "native-map.js.map"), "utf8"),
      ).resolves.toBe(nativeMap);

      const repeated = await discoverSourceMaps({
        bundler: "turbopack",
        distDir: ".next",
        projectDir: project,
      });

      expect(
        repeated.files.toSorted((a, b) => a.absolute.localeCompare(b.absolute)),
      ).toStrictEqual(
        result.files.toSorted((a, b) => a.absolute.localeCompare(b.absolute)),
      );

      expect(repeated.missingSourceMapCount).toBe(1);
    } finally {
      await rm(project, { recursive: true, force: true });
    }
  });
});

describe("discoverSourceMaps (turbopack)", () => {
  let dir: string;

  const TBP_DEBUG_ID_1 = "11111111-2222-3333-4444-555555555555";
  const TBP_DEBUG_ID_3 = "33333333-3333-3333-3333-333333333333";
  const TBP_BIG_TAIL_ID = "44444444-4444-4444-4444-444444444444";
  const TBP_BIG_HEAD_ID = "55555555-5555-5555-5555-555555555555";
  const TBP_DUPE_ID = "66666666-6666-6666-6666-666666666666";

  beforeAll(async () => {
    dir = await mkdtemp(join(tmpdir(), "discover-turbopack-"));

    await mkdir(join(dir, ".next/static/chunks"), { recursive: true });

    await mkdir(join(dir, ".next/server/chunks"), { recursive: true });

    await writeFile(
      join(dir, ".next/static/chunks/with-debug-id.js"),
      `function a(){}\n//# debugId=${TBP_DEBUG_ID_1}\n//# sourceMappingURL=ignored.js.map`,
    );

    await writeFile(
      join(dir, ".next/static/chunks/with-debug-id.js.map"),
      JSON.stringify({
        debugId: TBP_DEBUG_ID_1,
        mappings: "AAAA",
        sources: ["a.ts"],
        version: 3,
      }),
    );

    await writeFile(
      join(dir, ".next/static/chunks/no-debug-id.js.map"),
      JSON.stringify({ mappings: "AAAA", sources: ["b.ts"], version: 3 }),
    );

    await writeFile(
      join(dir, ".next/server/chunks/empty-debug-id.js.map"),
      JSON.stringify({
        debugId: "",
        mappings: "AAAA",
        sources: ["c.ts"],
        version: 3,
      }),
    );

    await writeFile(
      join(dir, ".next/server/chunks/garbage.js.map"),
      "not json",
    );

    await writeFile(
      join(dir, ".next/server/chunks/orphan.js.map"),
      JSON.stringify({
        debugId: "99999999-9999-9999-9999-999999999999",
        mappings: "AAAA",
        sources: ["d.ts"],
        version: 3,
      }),
    );

    await writeFile(
      join(dir, ".next/server/chunks/server-page.js"),
      `module.exports={};\n//# debugId=${TBP_DEBUG_ID_3}`,
    );

    await writeFile(
      join(dir, ".next/server/chunks/server-page.js.map"),
      JSON.stringify({
        debugId: TBP_DEBUG_ID_3,
        mappings: "AAAA",
        sources: ["page.tsx"],
        version: 3,
      }),
    );

    await writeFile(
      join(dir, ".next/static/chunks/big-tail.js"),
      `${"//".padEnd(80 * 1024, "x")}\n//# debugId=${TBP_BIG_TAIL_ID}\n//# sourceMappingURL=ignored.js.map`,
    );

    await writeFile(
      join(dir, ".next/static/chunks/big-tail.js.map"),
      JSON.stringify({
        debugId: TBP_BIG_TAIL_ID,
        mappings: "AAAA",
        sources: ["big-tail.ts"],
        version: 3,
      }),
    );

    await writeFile(
      join(dir, ".next/static/chunks/big-head.js"),
      `//# debugId=${TBP_BIG_HEAD_ID}\n${"//".padEnd(80 * 1024, "y")}\n`,
    );

    await writeFile(
      join(dir, ".next/static/chunks/big-head.js.map"),
      JSON.stringify({
        debugId: TBP_BIG_HEAD_ID,
        mappings: "AAAA",
        sources: ["big-head.ts"],
        version: 3,
      }),
    );

    await writeFile(
      join(dir, ".next/static/chunks/plain.js"),
      "function plain(){}\n//# sourceMappingURL=plain.js.map",
    );

    await writeFile(
      join(dir, ".next/server/chunks/dupe-a.js"),
      `function a(){}\n//# debugId=${TBP_DUPE_ID}`,
    );

    await writeFile(
      join(dir, ".next/server/chunks/dupe-b.js"),
      `function b(){}\n//# debugId=${TBP_DUPE_ID}`,
    );

    await writeFile(
      join(dir, ".next/server/chunks/dupe.js.map"),
      JSON.stringify({
        debugId: TBP_DUPE_ID,
        mappings: "AAAA",
        sources: ["dupe.ts"],
        version: 3,
      }),
    );

    await mkdir(join(dir, ".next/static/chunks/oddly-named.js"), {
      recursive: true,
    });

    await mkdir(join(dir, ".next/static/chunks/oddly-named.js.map"), {
      recursive: true,
    });
  });

  afterAll(async () => {
    await rm(dir, { force: true, recursive: true });
  });

  test.each([
    "null",
    "false",
    "42",
    '"map"',
    "[]",
    '{"debugId":42}',
    '{"debugId":null}',
    '{"debugId":{}}',
  ])("skips maps without a string debug ID: %s", async (content) => {
    const project = await mkdtemp(
      join(tmpdir(), "discover-invalid-turbopack-"),
    );

    const chunks = join(project, ".next/static/chunks");

    try {
      await mkdir(chunks, { recursive: true });

      await writeFile(join(chunks, "app.js.map"), content);

      await expect(
        discoverSourceMaps({
          bundler: "turbopack",
          distDir: ".next",
          projectDir: project,
        }),
      ).resolves.toStrictEqual({
        files: [],
        missingSourceMapCount: 0,
        sourceFileCount: 0,
      });
    } finally {
      await rm(project, { recursive: true, force: true });
    }
  });

  test("counts chunks that reference a map that is absent, but not chunks without a map reference", async () => {
    const DEBUG_ID_A = "aaaaaaaa-0000-4000-8000-000000000001";
    const DEBUG_ID_B = "bbbbbbbb-0000-4000-8000-000000000002";
    const DEBUG_ID_C = "cccccccc-0000-4000-8000-000000000003";

    const project = await mkdtemp(
      join(tmpdir(), "discover-missing-turbopack-"),
    );

    const chunks = join(project, ".next/static/chunks");
    const server = join(project, ".next/server/chunks");

    try {
      await mkdir(chunks, { recursive: true });

      await mkdir(server, { recursive: true });

      await writeFile(
        join(chunks, "mapped.js"),
        `console.log(1);\n//# sourceMappingURL=mapped.js.map\n//# debugId=${DEBUG_ID_A}\n`,
      );

      await writeFile(
        join(chunks, "mapped.js.map"),
        JSON.stringify({ version: 3, debugId: DEBUG_ID_A, mappings: "" }),
      );

      await writeFile(
        join(chunks, "deleted.js"),
        `console.log(2);\n//# sourceMappingURL=deleted.js.map\n//# debugId=${DEBUG_ID_B}\n`,
      );

      await writeFile(
        join(server, "no-maps.js"),
        `console.log(3);\n//# debugId=${DEBUG_ID_C}\n`,
      );

      const result = await discoverSourceMaps({
        bundler: "turbopack",
        distDir: ".next",
        projectDir: project,
      });

      expect(result.files.map((file) => file.debugId)).toStrictEqual([
        DEBUG_ID_A,
      ]);

      expect(result.missingSourceMapCount).toBe(1);
    } finally {
      await rm(project, { recursive: true, force: true });
    }
  });

  test("includes maps whose debugId matches a chunk", async () => {
    const result = await discoverSourceMaps({
      bundler: "turbopack",
      distDir: ".next",
      projectDir: dir,
    });

    const paths = result.files.map((f) => f.path).toSorted();

    expect(paths).toStrictEqual([
      "_next/server/chunks/dupe.js.map",
      "_next/server/chunks/server-page.js.map",
      "_next/static/chunks/big-head.js.map",
      "_next/static/chunks/big-tail.js.map",
      "_next/static/chunks/with-debug-id.js.map",
    ]);
  });

  test("populates chunkUrl from the matching JS chunk", async () => {
    const result = await discoverSourceMaps({
      bundler: "turbopack",
      distDir: ".next",
      projectDir: dir,
    });

    const byDebugId = new Map(result.files.map((f) => [f.debugId, f]));

    expect(byDebugId.get(TBP_DEBUG_ID_1)?.chunkUrl).toBe(
      "_next/static/chunks/with-debug-id.js",
    );

    expect(byDebugId.get(TBP_DEBUG_ID_3)?.chunkUrl).toBe(
      "_next/server/chunks/server-page.js",
    );
  });

  test("reads the debugId from a chunk larger than the tail window", async () => {
    const result = await discoverSourceMaps({
      bundler: "turbopack",
      distDir: ".next",
      projectDir: dir,
    });

    const byDebugId = new Map(result.files.map((f) => [f.debugId, f]));

    expect(byDebugId.get(TBP_BIG_TAIL_ID)?.chunkUrl).toBe(
      "_next/static/chunks/big-tail.js",
    );
  });

  test("falls back to the whole chunk when the debugId is outside the tail window", async () => {
    const result = await discoverSourceMaps({
      bundler: "turbopack",
      distDir: ".next",
      projectDir: dir,
    });

    const byDebugId = new Map(result.files.map((f) => [f.debugId, f]));

    expect(byDebugId.get(TBP_BIG_HEAD_ID)?.chunkUrl).toBe(
      "_next/static/chunks/big-head.js",
    );
  });

  test("pairs distinct server chunk aliases to one source map", async () => {
    const result = await discoverSourceMaps({
      bundler: "turbopack",
      distDir: ".next",
      projectDir: dir,
    });

    const matches = result.files.filter((f) => f.debugId === TBP_DUPE_ID);

    expect(matches).toHaveLength(1);

    expect(matches[0]?.chunkUrl).toBe("_next/server/chunks/dupe-a.js");

    expect(matches[0]?.chunkUrls).toStrictEqual([
      "_next/server/chunks/dupe-a.js",
      "_next/server/chunks/dupe-b.js",
    ]);
  });

  test("leaves an unresolved no-ID chunk out of the published inventory", async () => {
    const result = await discoverSourceMaps({
      bundler: "turbopack",
      distDir: ".next",
      projectDir: dir,
    });

    const chunkUrls = result.files.map((f) => f.chunkUrl);

    expect(chunkUrls).not.toContain("_next/static/chunks/plain.js");
    expect(result.missingSourceMapCount).toBe(1);

    expect(result.files.map((f) => f.path)).not.toContain(
      "_next/static/chunks/oddly-named.js.map",
    );
  });

  test("skips maps missing a debugId, with empty debugId, with invalid JSON, or whose id has no chunk", async () => {
    const result = await discoverSourceMaps({
      bundler: "turbopack",
      distDir: ".next",
      projectDir: dir,
    });

    expect(result.files).toHaveLength(5);
    expect(result.sourceFileCount).toBe(6);
  });
});

describe(injectDebugIdIntoJs, () => {
  const id = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";

  test("preserves sourceMappingURL at the end of the file", () => {
    const input = "function a(){}\n//# sourceMappingURL=app.js.map";
    const output = injectDebugIdIntoJs(input, id);

    expect(output).toMatch(APP_JS_SOURCE_MAP_URL_SUFFIX_RE);
  });

  test("places debugId comment before sourceMappingURL", () => {
    const input = "function a(){}\n//# sourceMappingURL=app.js.map";
    const output = injectDebugIdIntoJs(input, id);
    const lines = output.split("\n");
    const debugLine = lines.findIndex((l) => l.includes("//# debugId="));
    const mapLine = lines.findIndex((l) => l.includes("sourceMappingURL"));

    expect(debugLine).toBeGreaterThan(-1);
    expect(mapLine).toBeGreaterThan(debugLine);
  });

  test("injects the _debugIds runtime snippet before the debugId comment", () => {
    const input = "function a(){}\n//# sourceMappingURL=app.js.map";
    const output = injectDebugIdIntoJs(input, id);
    const lines = output.split("\n");
    const snippetLine = lines.findIndex((l) => l.includes("_debugIds="));
    const debugLine = lines.findIndex((l) => l.includes("//# debugId="));
    const mapLine = lines.findIndex((l) => l.includes("sourceMappingURL"));

    expect(output).toContain("new e.Error");
    expect(output).toContain(`_debugIds[n]="${id}"`);
    expect(snippetLine).toBeGreaterThan(-1);
    expect(debugLine).toBeGreaterThan(snippetLine);
    expect(mapLine).toBeGreaterThan(debugLine);
  });

  test("handles JS without sourceMappingURL", () => {
    const input = "function a(){}";
    const output = injectDebugIdIntoJs(input, id);

    expect(output).toContain(`//# debugId=${id}`);
    expect(output).toContain("function a(){}");
  });

  test("handles empty JS content", () => {
    const output = injectDebugIdIntoJs("", id);

    expect(output).toContain(`//# debugId=${id}`);
  });

  test("handles sourceMappingURL with @ prefix", () => {
    const input = "function a(){}\n//@ sourceMappingURL=old.js.map";
    const output = injectDebugIdIntoJs(input, id);

    expect(output).toContain("sourceMappingURL=old.js.map");
    expect(output).toContain(`//# debugId=${id}`);
  });
});

describe(injectDebugIdIntoMap, () => {
  const id = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";

  test("adds debugId to the source map JSON", () => {
    const input = '{"version":3,"sources":["a.ts"],"mappings":"AAAA"}';
    const output: unknown = JSON.parse(injectDebugIdIntoMap(input, id));

    expect(output).toHaveProperty("debugId", id);
  });

  test("does not shift mappings (no runtime snippet to compensate for)", () => {
    const input = '{"version":3,"sources":["a.ts"],"mappings":"AAAA"}';
    const output: unknown = JSON.parse(injectDebugIdIntoMap(input, id));

    expect(output).toHaveProperty("mappings", "AAAA");
  });

  test("handles source map with empty mappings", () => {
    const input = '{"version":3,"sources":[],"mappings":""}';
    const output: unknown = JSON.parse(injectDebugIdIntoMap(input, id));

    expect(output).toHaveProperty("debugId", id);
    expect(output).toHaveProperty("mappings", "");
  });

  test("handles source map without mappings field", () => {
    const input = '{"version":3,"sources":[]}';
    const output: unknown = JSON.parse(injectDebugIdIntoMap(input, id));

    expect(output).toHaveProperty("debugId", id);
    expect(output).not.toHaveProperty("mappings");
  });

  test("preserves all other source map fields", () => {
    const input =
      '{"version":3,"sources":["a.ts"],"sourcesContent":["x"],"names":["foo"],"mappings":"AAAA"}';

    const output: unknown = JSON.parse(injectDebugIdIntoMap(input, id));

    expect(output).toHaveProperty("version", 3);
    expect(output).toHaveProperty("sources", ["a.ts"]);
    expect(output).toHaveProperty("sourcesContent", ["x"]);
    expect(output).toHaveProperty("names", ["foo"]);
  });
});

describe("discover debug ID injection", () => {
  const JS_CONTENT = (mapName: string) =>
    `function hello(){}\n//# sourceMappingURL=${mapName}`;

  const MAP_CONTENT =
    '{"version":3,"sources":["src/app.ts"],"mappings":"AAAA"}';

  let dir: string;
  let result: DiscoveryResult;
  let jsContent: string;
  let mapJson: unknown;

  beforeAll(async () => {
    dir = await mkdtemp(join(tmpdir(), "discover-debugid-"));

    await mkdir(join(dir, ".next/static/chunks"), { recursive: true });

    await writeFile(
      join(dir, ".next/static/chunks/app.js"),
      JS_CONTENT("app.js.map"),
    );

    await writeFile(join(dir, ".next/static/chunks/app.js.map"), MAP_CONTENT);

    result = await discoverSourceMaps({
      bundler: "webpack",
      distDir: ".next",
      projectDir: dir,
    });

    jsContent = await readFile(join(dir, ".next/static/chunks/app.js"), "utf8");

    mapJson = JSON.parse(
      await readFile(join(dir, ".next/static/chunks/app.js.map"), "utf8"),
    );
  });

  afterAll(async () => {
    await rm(dir, { force: true, recursive: true });
  });

  test("returns a debugId on each discovered file", () => {
    expect(result.files).toHaveLength(1);
    expect(result.files[0]?.debugId).toMatch(UUID_RE);
  });

  test("injects the runtime carrier and the debugId comment into the JS file", () => {
    expect(jsContent).toContain("new e.Error");
    expect(jsContent).toContain("_debugIds[n]=");
    expect(jsContent).toMatch(DEBUG_ID_COMMENT_RE);
    expect(jsContent).toContain("//# sourceMappingURL=app.js.map");
  });

  test("injects debugId into the source map without shifting mappings", () => {
    expect(mapJson).toHaveProperty("debugId", expect.stringMatching(UUID_RE));
    expect(mapJson).toHaveProperty("mappings", "AAAA");
  });

  test("uses the same debugId in the JS comment and the source map", () => {
    const jsDebugId =
      DEBUG_ID_COMMENT_CAPTURE_RE.exec(jsContent)?.groups?.["debugId"];

    expect(mapJson).toHaveProperty("debugId", jsDebugId);
  });
});

describe("discoverWebpack: respect existing debugId comments", () => {
  const PRE_INJECTED_ID = "12345678-90ab-cdef-1234-567890abcdef";

  let dir: string;
  let result: DiscoveryResult;
  let jsContentBefore: string;
  let jsContentAfter: string;

  beforeAll(async () => {
    dir = await mkdtemp(join(tmpdir(), "discover-existing-debugid-"));

    await mkdir(join(dir, ".next/static/chunks"), { recursive: true });

    jsContentBefore = [
      "function hello(){}",
      `//# debugId=${PRE_INJECTED_ID}`,
      "//# sourceMappingURL=app.js.map",
    ].join("\n");

    await writeFile(join(dir, ".next/static/chunks/app.js"), jsContentBefore);

    await writeFile(
      join(dir, ".next/static/chunks/app.js.map"),
      JSON.stringify({
        debugId: PRE_INJECTED_ID,
        mappings: "AAAA",
        sources: ["src/app.ts"],
        version: 3,
      }),
    );

    result = await discoverSourceMaps({
      bundler: "webpack",
      distDir: ".next",
      projectDir: dir,
    });

    jsContentAfter = await readFile(
      join(dir, ".next/static/chunks/app.js"),
      "utf8",
    );
  });

  afterAll(async () => {
    await rm(dir, { force: true, recursive: true });
  });

  test("reuses the existing debugId from the chunk", () => {
    expect(result.files).toHaveLength(1);
    expect(result.files[0]?.debugId).toBe(PRE_INJECTED_ID);
  });

  test("does not rewrite the chunk when its existing debugId is canonical", () => {
    expect(jsContentAfter).toBe(jsContentBefore);
  });

  test("does not duplicate the debugId comment when one is already present", () => {
    const matches = jsContentAfter.match(/\/\/# debugId=/gu) ?? [];

    expect(matches).toHaveLength(1);
  });
});

describe("discoverWebpack: coexists with foreign debug-id tooling", () => {
  const FOREIGN_ID = "abcdef12-3456-7890-abcd-ef1234567890";
  const FOREIGN_CARRIER = `;!function(){try{var e=window;e._sentryDebugIds=e._sentryDebugIds||{},e._sentryDebugIds[(new e.Error).stack]="${FOREIGN_ID}"}catch(e){}}();`;

  let dir: string;
  let result: DiscoveryResult;
  let jsBefore: string;
  let jsAfter: string;
  let mapBefore: string;

  beforeAll(async () => {
    dir = await mkdtemp(join(tmpdir(), "discover-foreign-debugid-"));

    await mkdir(join(dir, ".next/static/chunks"), { recursive: true });

    jsBefore = [
      "function hello(){}",
      FOREIGN_CARRIER,
      `//# debugId=${FOREIGN_ID}`,
      "//# sourceMappingURL=app.js.map",
    ].join("\n");

    mapBefore = JSON.stringify({
      debugId: FOREIGN_ID,
      mappings: "AAAA",
      sources: ["src/app.ts"],
      version: 3,
    });

    await writeFile(join(dir, ".next/static/chunks/app.js"), jsBefore);

    await writeFile(join(dir, ".next/static/chunks/app.js.map"), mapBefore);

    result = await discoverSourceMaps({
      bundler: "webpack",
      distDir: ".next",
      projectDir: dir,
    });

    jsAfter = await readFile(join(dir, ".next/static/chunks/app.js"), "utf8");
  });

  afterAll(async () => {
    await rm(dir, { force: true, recursive: true });
  });

  test("adopts the foreign debugId instead of minting its own", () => {
    expect(result.files).toHaveLength(1);
    expect(result.files[0]?.debugId).toBe(FOREIGN_ID);
  });

  test("leaves the foreign chunk byte-identical", () => {
    expect(jsAfter).toBe(jsBefore);
  });

  test("adds neither a second comment nor its own runtime carrier", () => {
    expect(jsAfter.match(/\/\/# debugId=/gu) ?? []).toHaveLength(1);
    expect(jsAfter).not.toContain('_debugIds[n]="');
    expect(jsAfter).toContain("_sentryDebugIds");
  });

  test("does not rewrite the source map when its debugId already matches", () => {
    expect(result.files[0]?.content).toBe(mapBefore);
  });
});
