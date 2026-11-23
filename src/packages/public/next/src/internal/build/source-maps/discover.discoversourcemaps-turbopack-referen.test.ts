import { mkdir, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
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
