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

describe("discoverSourceMaps (webpack)", () => {
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
