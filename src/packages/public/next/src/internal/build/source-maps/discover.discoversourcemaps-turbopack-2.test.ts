import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { discoverSourceMaps } from "./discover.js";
import { afterAll, beforeAll, describe, expect, test, vi } from "vitest";

const writeHook = vi.hoisted(() => ({
  onFirstWrite: undefined as (() => void) | undefined,
}));

// oxlint-disable-next-line anti-slop/no-module-mocking, landmines/no-test-doubles -- Every write still reaches the real writeFile unchanged; the wrapper only measures the heap once every map has been prepared and before the first write.
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
