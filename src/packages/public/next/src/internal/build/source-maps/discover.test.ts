import { mkdir, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { setFlagsFromString } from "node:v8";
import { runInNewContext } from "node:vm";

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
});
