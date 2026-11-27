import { SourceMap } from "node:module";
import { mkdtemp, readFile, readdir, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { runInNewContext } from "node:vm";
import { build } from "vite";
import {
  afterAll,
  afterEach,
  beforeAll,
  expect,
  onTestFinished,
  test,
  vi,
} from "vitest";

import { interfere } from "../../plugin";
import {
  RELEASE_API,
  finalized,
  resetRecordedRequests,
  startReleaseApi,
  stopReleaseApi,
  uploads,
} from "./release-api.fixture";

beforeAll(() => {
  startReleaseApi();
});

afterEach(() => {
  vi.unstubAllEnvs();
});

afterAll(() => {
  stopReleaseApi();
});

test.each([
  { nativeIds: false, ssr: false, minify: false },
  { nativeIds: true, ssr: false, minify: true },
  { nativeIds: false, ssr: true, minify: true },
  { nativeIds: true, ssr: true, minify: false },
])(
  "keeps chunk bytes, runtime identity and mapped throws stable: %j",
  async ({ nativeIds, ssr, minify }) => {
    expect.hasAssertions();

    const root = await mkdtemp(path.join(tmpdir(), "vite-debug-id-build-"));

    onTestFinished(async () => rm(root, { recursive: true, force: true }));

    vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test");

    vi.stubEnv("INTERFERE_API_URL", RELEASE_API);

    vi.stubEnv("INTERFERE_ENVIRONMENT", "production");

    async function compile(message: string) {
      resetRecordedRequests();

      await writeFile(
        path.join(root, "entry.js"),
        `globalThis.fail = () => {\n  throw new Error(${JSON.stringify(message)});\n};`,
      );

      await build({
        configFile: false,
        root,
        envDir: root,
        logLevel: "silent",
        plugins: [
          interfere({
            commitSha: "deadbeef12345678",
            serverSourcemapsDir: "dist",
          }),
        ],
        build: {
          minify,
          ssr,
          sourcemap: true,
          rolldownOptions: {
            input: path.join(root, "entry.js"),
            output: {
              entryFileNames: "[name]-[hash].js",
              format: "iife",
              sourcemapDebugIds: nativeIds,
            },
          },
        },
      });

      const distDir = path.join(root, "dist");

      const files = await readdir(distDir);

      const filename = files.find((file) => file.endsWith(".js"));

      if (!filename) {
        throw new Error("Built chunk missing");
      }

      expect(files).not.toContain(`${filename}.map`);

      const code = await readFile(path.join(distDir, filename), "utf8");

      const uploaded = uploads.get(`${filename}.map`);

      if (!uploaded) {
        throw new Error("Uploaded map missing");
      }

      const map = JSON.parse(uploaded) as ConstructorParameters<
        typeof SourceMap
      >[0] & { debugId: string };

      const registry: unknown = runInNewContext(
        `${code}\nObject.values(globalThis._debugIds ?? {})`,
        {},
        { filename },
      );

      const stack = String(
        runInNewContext(
          `${code}\ntry { globalThis.fail(); } catch (error) { error.stack; }`,
          {},
          { filename },
        ),
      );

      const frame = stack
        .split("\n")
        .find((line) => line.includes(`${filename}:`));

      const position = frame?.match(/:(?<line>\d+):(?<column>\d+)\)/u)?.groups;

      if (!position) {
        throw new Error("Generated throw position missing");
      }

      const original = new SourceMap(map).findEntry(
        Number(position["line"]) - 1,
        Number(position["column"]) - 1,
      );

      expect(original).toHaveProperty("originalLine", 1);
      expect(original).toHaveProperty("originalSource");

      if (!("originalSource" in original)) {
        throw new Error("Original source missing");
      }

      expect(original.originalSource).toMatch(/entry\.js$/u);

      expect(registry).toContain(map.debugId);

      expect(finalized).toContainEqual(
        expect.objectContaining({ debugId: map.debugId }),
      );

      return { code, filename, debugId: map.debugId };
    }

    const first = await compile("original");

    await expect(compile("original")).resolves.toStrictEqual(first);

    const changed = await compile("changed");

    expect(changed.filename).not.toBe(first.filename);
    expect(changed.debugId).not.toBe(first.debugId);
  },
);
