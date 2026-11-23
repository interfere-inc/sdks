import { mkdir, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";

import type { DiscoveryResult } from "./discover.js";
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
