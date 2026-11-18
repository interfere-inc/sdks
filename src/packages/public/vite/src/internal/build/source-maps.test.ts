import { runInNewContext } from "node:vm";
import {
  mkdir,
  mkdtemp,
  readFile,
  rm,
  symlink,
  writeFile,
} from "node:fs/promises";
import { tmpdir } from "node:os";
import { basename, dirname, join, win32 } from "node:path";

import { discoverByPairing, injectDebugIdIntoJs } from "./source-maps";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

let projectDir: string;

beforeEach(async () => {
  projectDir = await mkdtemp(join(tmpdir(), "interfere-vite-sourcemaps-"));
});

afterEach(async () => {
  vi.unstubAllGlobals();

  await rm(projectDir, { force: true, recursive: true });
});

async function writeChunk(relativePath: string) {
  const jsPath = join(projectDir, "dist", relativePath);
  const mapPath = `${jsPath}.map`;

  await mkdir(dirname(jsPath), { recursive: true });

  await writeFile(
    jsPath,
    `console.log("chunk");\n//# sourceMappingURL=${basename(relativePath)}.map`,
    "utf8",
  );

  await writeFile(
    mapPath,
    JSON.stringify({
      mappings: "AAAA",
      names: [],
      sources: ["src/app.tsx"],
      sourcesContent: ["export const app = true;"],
      version: 3,
    }),
    "utf8",
  );
}

async function discover() {
  return discoverByPairing({
    distDir: join(projectDir, "dist"),
    projectDir,
    publicPathRewrites: [{ from: "public", to: "" }],
    subtrees: ["."],
  });
}

describe(discoverByPairing, () => {
  test("discovers a Windows output path and publishes slash-separated URLs", async () => {
    await writeChunk("nested/assets/app.js");

    const result = await discoverByPairing({
      distDir: win32.join("dist", "nested"),
      projectDir,
      publicPathRewrites: [],
      subtrees: ["."],
    });

    expect(result.files).toHaveLength(1);

    expect(result.files[0]).toMatchObject({
      chunkUrl: "assets/app.js",
      path: "assets/app.js.map",
      absolute: join(projectDir, "dist", "nested", "assets", "app.js.map"),
    });
  });

  test.each([
    { content: "{", expectedError: SyntaxError },
    { content: "null", expectedError: TypeError },
    { content: "false", expectedError: TypeError },
    { content: "42", expectedError: TypeError },
    { content: '"map"', expectedError: TypeError },
    { content: "[]", expectedError: TypeError },
  ])(
    "throws on malformed source map $content behind more chunks than the worker limit without rewriting any chunk or map, leaving cleanup to the caller",
    async ({ content, expectedError }) => {
      const chunks = [
        ...Array.from({ length: 40 }, (_, index) => `assets/${index + 10}.js`),
        "assets/zz-app.js",
      ];

      await Promise.all(chunks.map(async (chunk) => writeChunk(chunk)));

      await writeFile(join(projectDir, "dist/assets/zz-app.js.map"), content);

      const snapshot = async () =>
        Promise.all(
          chunks.flatMap((chunk) =>
            [chunk, `${chunk}.map`].map(async (file) =>
              readFile(join(projectDir, "dist", file), "utf8"),
            ),
          ),
        );

      const before = await snapshot();

      await expect(discover()).rejects.toThrow(expectedError);

      await expect(snapshot()).resolves.toStrictEqual(before);
    },
  );

  test("processes more files than the worker limit and preserves discovery order", async () => {
    const chunks = Array.from(
      { length: 40 },
      (_, index) => `assets/chunk-${String(index).padStart(2, "0")}.js`,
    );

    await Promise.all(chunks.map(async (element) => writeChunk(element)));

    const result = await discover();

    expect(result.sourceFileCount).toBe(chunks.length);
    expect(result.files.map((file) => file.chunkUrl)).toStrictEqual(chunks);

    expect(new Set(result.files.map((file) => file.debugId)).size).toBe(
      chunks.length,
    );
  });

  test("preserves every chunk URL referencing one map", async () => {
    await writeChunk("assets/shared.js");

    const debugId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";

    await writeFile(
      join(projectDir, "dist/assets/z.js"),
      injectDebugIdIntoJs(
        "console.log('last');\n//# sourceMappingURL=shared.js.map",
        debugId,
      ),
    );

    const result = await discover();

    expect(result.sourceFileCount).toBe(1);
    expect(result.files).toHaveLength(1);

    expect(result.files[0]).toMatchObject({
      chunkAbsolutes: [
        join(projectDir, "dist/assets/shared.js"),
        join(projectDir, "dist/assets/z.js"),
      ],
      chunkUrl: "assets/shared.js",
      chunkUrls: ["assets/shared.js", "assets/z.js"],
      path: "assets/shared.js.map",
      debugId,
    });

    await expect(
      readFile(join(projectDir, "dist/assets/shared.js"), "utf8"),
    ).resolves.toContain(`//# debugId=${debugId}`);

    await expect(
      readFile(join(projectDir, "dist/assets/z.js"), "utf8"),
    ).resolves.toContain(`//# debugId=${debugId}`);

    expect(JSON.parse(result.files[0]?.content ?? "null")).toMatchObject({
      debugId,
    });
  });

  test("skips missing maps without changing their chunks", async () => {
    await writeChunk("assets/missing.js");

    const jsPath = join(projectDir, "dist/assets/missing.js");

    const original = await readFile(jsPath, "utf8");

    await rm(`${jsPath}.map`);

    await expect(discover()).resolves.toStrictEqual({
      files: [],
      missingSourceMapCount: 1,
      sourceFileCount: 1,
    });

    await expect(readFile(jsPath, "utf8")).resolves.toBe(original);
  });

  test("counts stamped chunks whose mapping comments were removed alongside partial coverage", async () => {
    await writeChunk("assets/covered.js");

    await writeChunk("server/action.js");

    const missing = join(projectDir, "dist/server/action.js");

    const content = injectDebugIdIntoJs(
      "function action(){}",
      "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
    );

    await writeFile(missing, content);

    await rm(`${missing}.map`);

    const result = await discover();

    expect(result.files).toHaveLength(1);

    expect(result).toMatchObject({
      missingSourceMapCount: 1,
      sourceFileCount: 2,
    });

    await expect(readFile(missing, "utf8")).resolves.toBe(content);
  });

  test("uses deploy-relative paths for vanilla Vite assets", async () => {
    await writeChunk("assets/app.123.js");

    const result = await discover();

    expect(result.files).toHaveLength(1);
    expect(result.files[0]?.chunkUrl).toBe("assets/app.123.js");
    expect(result.files[0]?.path).toBe("assets/app.123.js.map");
  });

  test("injects matching debug IDs into the JS runtime and source map", async () => {
    await writeChunk("assets/with-debug-id.js");

    const result = await discover();

    const jsContent = await readFile(
      join(projectDir, "dist", "assets/with-debug-id.js"),
      "utf8",
    );

    const mapJson: unknown = JSON.parse(
      await readFile(
        join(projectDir, "dist", "assets/with-debug-id.js.map"),
        "utf8",
      ),
    );

    const debugId = result.files[0]?.debugId;

    expect(result.files).toHaveLength(1);
    expect(debugId).toStrictEqual(expect.stringMatching(/[a-f0-9-]{36}/u));
    expect(mapJson).toHaveProperty("debugId", debugId);
    expect(result.files[0]?.content).toContain(`"debugId":"${debugId}"`);

    expect(jsContent.split("\n").at(-1)).toBe(
      "//# sourceMappingURL=with-debug-id.js.map",
    );

    expect(jsContent).toContain("_debugIds[n]=");
    expect(jsContent).toContain(`//# debugId=${debugId}`);

    const registeredIds: unknown = runInNewContext(
      `${jsContent}\nObject.values(globalThis._debugIds ?? {})`,
      {},
    );

    expect(registeredIds).toContain(debugId);
  });

  test("strips Nitro's public output folder for client chunks", async () => {
    await writeChunk("public/_build/client.456.js");

    const result = await discover();

    expect(result.files).toHaveLength(1);
    expect(result.files[0]?.chunkUrl).toBe("_build/client.456.js");
    expect(result.files[0]?.path).toBe("_build/client.456.js.map");
  });

  test("keeps Nitro server output under the server prefix", async () => {
    await writeChunk("server/chunks/handler.789.mjs");

    const result = await discover();

    expect(result.files).toHaveLength(1);
    expect(result.files[0]?.chunkUrl).toBe("server/chunks/handler.789.mjs");
    expect(result.files[0]?.path).toBe("server/chunks/handler.789.mjs.map");
  });

  test("ignores a sourceMappingURL that resolves outside the output directory", async () => {
    const jsPath = join(projectDir, "dist", "assets", "vendor-worker.js");
    const secretPath = join(projectDir, "outside", "secret.json");

    const jsContent =
      "self.onmessage=function(){};\n//# sourceMappingURL=../../../outside/secret.json\n";

    const secret = '{"auths":{"registry.example.test":{"auth":"ZHVtbXk="}}}';

    await mkdir(dirname(jsPath), { recursive: true });

    await mkdir(dirname(secretPath), { recursive: true });

    await writeFile(jsPath, jsContent, "utf8");

    await writeFile(secretPath, secret, "utf8");

    await expect(discover()).resolves.toStrictEqual({
      files: [],
      missingSourceMapCount: 0,
      sourceFileCount: 0,
    });

    await expect(readFile(jsPath, "utf8")).resolves.toBe(jsContent);

    await expect(readFile(secretPath, "utf8")).resolves.toBe(secret);
  });

  test("ignores a map inside the output directory that links outside it", async () => {
    const jsPath = join(projectDir, "dist", "assets", "linked.js");
    const secretPath = join(projectDir, "outside", "secret.json");
    const secret = '{"auths":{}}';

    await mkdir(dirname(jsPath), { recursive: true });

    await mkdir(dirname(secretPath), { recursive: true });

    await writeFile(
      jsPath,
      "console.log('linked');\n//# sourceMappingURL=linked.js.map",
      "utf8",
    );

    await writeFile(secretPath, secret, "utf8");

    await symlink(secretPath, `${jsPath}.map`);

    await expect(discover()).resolves.toStrictEqual({
      files: [],
      missingSourceMapCount: 0,
      sourceFileCount: 0,
    });

    await expect(readFile(secretPath, "utf8")).resolves.toBe(secret);
  });

  test("pairs a map referenced through a sibling directory inside the output directory", async () => {
    const jsPath = join(projectDir, "dist", "assets", "sibling.js");
    const mapPath = join(projectDir, "dist", "maps", "sibling.js.map");

    await mkdir(dirname(jsPath), { recursive: true });

    await mkdir(dirname(mapPath), { recursive: true });

    await writeFile(
      jsPath,
      "console.log('sibling');\n//# sourceMappingURL=../maps/sibling.js.map",
      "utf8",
    );

    await writeFile(mapPath, '{"version":3,"mappings":"AAAA"}', "utf8");

    const result = await discover();

    expect(result.files).toHaveLength(1);
    expect(result.files[0]?.chunkUrl).toBe("assets/sibling.js");
    expect(result.files[0]?.path).toBe("maps/sibling.js.map");
    expect(result.files[0]?.absolute).toBe(mapPath);
  });

  test("skips chunks without a sourceMappingURL", async () => {
    const jsPath = join(projectDir, "dist", "assets", "orphan.js");

    await mkdir(dirname(jsPath), { recursive: true });

    await writeFile(jsPath, "console.log('no map');", "utf8");

    const result = await discover();

    expect(result.files).toHaveLength(0);

    await expect(readFile(jsPath, "utf8")).resolves.toBe(
      "console.log('no map');",
    );
  });
});

describe(injectDebugIdIntoJs, () => {
  const debugId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";

  test("appends the runtime carrier when a sourcemap comment is absent", () => {
    const output = injectDebugIdIntoJs("console.log('chunk');", debugId);

    expect(output).toContain("console.log('chunk');");
    expect(output).toContain("_debugIds");
    expect(output).toContain(`//# debugId=${debugId}`);
  });
});
