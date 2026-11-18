import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { setFlagsFromString } from "node:v8";
import { runInNewContext } from "node:vm";

import { discoverByPairing } from "./source-maps";
import { afterEach, beforeEach, expect, test, vi } from "vitest";

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

let projectDir: string;

beforeEach(async () => {
  projectDir = await mkdtemp(join(tmpdir(), "interfere-vite-discover-memory-"));
});

afterEach(async () => {
  writeHook.onFirstWrite = undefined;

  await rm(projectDir, { force: true, recursive: true });
});

test("holds about one copy of every map in memory when the first debug-ID write starts", async () => {
  setFlagsFromString("--expose-gc");

  const collectGarbage = runInNewContext("gc") as () => void;
  const mapCount = 8;
  const mapBytes = 4 * 1024 * 1024;

  await mkdir(join(projectDir, "dist"), { recursive: true });

  await Promise.all(
    Array.from({ length: mapCount }, async (_, index) => {
      await writeFile(
        join(projectDir, "dist", `c${index}.js`),
        `x;\n//# sourceMappingURL=c${index}.js.map`,
        "utf8",
      );

      await writeFile(
        join(projectDir, "dist", `c${index}.js.map`),
        JSON.stringify({
          mappings: "AAAA",
          sources: ["a.ts"],
          sourcesContent: [`${"a".repeat(mapBytes)}${index}`],
          version: 3,
        }),
        "utf8",
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

  const result = await discoverByPairing({
    distDir: join(projectDir, "dist"),
    projectDir,
    publicPathRewrites: [],
    subtrees: ["."],
  });

  expect(result.files).toHaveLength(mapCount);
  expect(atFirstWrite - baseline).toBeLessThan(mapCount * mapBytes * 1.5);
});
