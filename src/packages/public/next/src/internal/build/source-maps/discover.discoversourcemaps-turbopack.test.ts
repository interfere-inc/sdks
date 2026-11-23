import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { discoverSourceMaps } from "./discover.js";
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
});
