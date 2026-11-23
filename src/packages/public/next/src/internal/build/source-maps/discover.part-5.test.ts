import { mkdir, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";

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
