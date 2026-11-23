import { execFile } from "node:child_process";
import { mkdir, mkdtemp, readdir, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { pathToFileURL } from "node:url";
import { promisify } from "node:util";

import { createPackedPackage } from "./package-contract.js";
import { afterEach, beforeAll, beforeEach, expect, test, vi } from "vitest";

const execFileAsync = promisify(execFile);

let directory: string;

let scratch: string;

let packageJsonUrl: URL;

const concurrentNames = [
  "package-contract-fixture",
  "@fixture/package-contract",
] as const;

interface ConcurrentPack {
  cjs: string[];
  directories: number;
  esm: string[];
  names: string[];
  root: string[];
  scratch: string[];
  statuses: string[];
}

const concurrentPacks = new Map<string, ConcurrentPack>();

async function packConcurrently(name: string) {
  const root = await mkdtemp(path.join(tmpdir(), "package-contract-owned-"));

  const ownedScratch = path.join(root, "scratch");

  await mkdir(ownedScratch);

  vi.stubEnv("TMPDIR", ownedScratch);

  const manifest = pathToFileURL(path.join(root, "package.json"));

  try {
    await writeFile(
      manifest,
      JSON.stringify({
        name,
        version: "1.0.0",
        files: ["index.cjs"],
        exports: "./index.cjs",
      }),
    );

    await writeFile(path.join(root, "index.cjs"), "module.exports = 42;");

    const results = await Promise.allSettled([
      createPackedPackage(manifest),
      createPackedPackage(manifest),
      createPackedPackage(manifest),
    ]);

    const packages = results.flatMap((result) =>
      result.status === "fulfilled" ? [result.value] : [],
    );

    let loaded: { cjs: string; esm: string }[] = [];

    try {
      loaded = await Promise.all(
        packages.map(async (packed) => {
          const [esmResult, cjsResult] = await Promise.all([
            execFileAsync(
              process.execPath,
              [
                "--input-type=module",
                "--eval",
                `console.log((await import(${JSON.stringify(name)})).default)`,
              ],
              { cwd: packed.consumerDir },
            ),
            execFileAsync(
              process.execPath,
              ["--eval", `console.log(require(${JSON.stringify(name)}))`],
              { cwd: packed.consumerDir },
            ),
          ]);

          return {
            cjs: cjsResult.stdout.trim(),
            esm: esmResult.stdout.trim(),
          };
        }),
      );
    } finally {
      await Promise.all(packages.map(async (packed) => packed.cleanup()));
    }

    return {
      cjs: loaded.map((entry) => entry.cjs),
      directories: new Set(packages.map((packed) => packed.consumerDir)).size,
      esm: loaded.map((entry) => entry.esm),
      names: packages.map((packed) => packed.packageJson.name),
      root: (await readdir(root)).toSorted(),
      scratch: await readdir(ownedScratch),
      statuses: results.map((result) => result.status),
    };
  } finally {
    vi.unstubAllEnvs();

    await rm(root, { recursive: true, force: true });
  }
}

beforeAll(async () => {
  const [plainName, scopedName] = concurrentNames;

  concurrentPacks.set(plainName, await packConcurrently(plainName));

  concurrentPacks.set(scopedName, await packConcurrently(scopedName));
});

beforeEach(async () => {
  directory = await mkdtemp(path.join(tmpdir(), "package-contract-owned-"));

  scratch = path.join(directory, "scratch");

  await mkdir(scratch);

  vi.stubEnv("TMPDIR", scratch);

  packageJsonUrl = pathToFileURL(path.join(directory, "package.json"));
});

afterEach(async () => {
  vi.unstubAllEnvs();

  await rm(directory, { recursive: true, force: true });
});

test("rejects nested dependency links after packing without writing through source dependencies", async () => {
  await writeFile(
    packageJsonUrl,
    JSON.stringify({
      name: "package-contract-fixture",
      version: "1.0.0",
      files: ["index.cjs"],
      dependencies: { foo: "1.0.0", "foo/nested": "1.0.0" },
    }),
  );

  await writeFile(path.join(directory, "index.cjs"), "module.exports = 42;");

  const dependencyDir = path.join(directory, "node_modules", "foo");

  await mkdir(dependencyDir, { recursive: true });

  await writeFile(path.join(dependencyDir, "index.cjs"), "module.exports = 7;");

  await expect(createPackedPackage(packageJsonUrl)).rejects.toThrow(
    "Package path must remain inside the owned node_modules directory.",
  );

  await expect(readdir(scratch)).resolves.toStrictEqual([]);

  await expect(readdir(dependencyDir)).resolves.toStrictEqual(["index.cjs"]);

  expect((await readdir(directory)).toSorted()).toStrictEqual([
    "index.cjs",
    "node_modules",
    "package.json",
    "scratch",
  ]);
});

test.each(concurrentNames)(
  "packs %s concurrently into isolated consumers and cleans them",
  (name) => {
    const packed = concurrentPacks.get(name);

    if (!packed) {
      throw new Error(name);
    }

    expect.hasAssertions();

    expect(packed.statuses).toStrictEqual([
      "fulfilled",
      "fulfilled",
      "fulfilled",
    ]);

    expect(packed.directories).toBe(3);

    expect(packed.names).toStrictEqual([name, name, name]);

    expect(packed.esm).toStrictEqual(["42", "42", "42"]);

    expect(packed.cjs).toStrictEqual(["42", "42", "42"]);

    expect(packed.scratch).toStrictEqual([]);

    expect(packed.root).toStrictEqual(["index.cjs", "package.json", "scratch"]);
  },
);

test.each([
  "../outside",
  "../../outside",
  "/outside",
  ".",
  "@fixture",
  "@/package",
  "@fixture/../outside",
  "@fixture/",
  String.raw`fixture\outside`,
])(
  "rejects escaping package path %s and cleans its owned directory",
  async (name) => {
    await writeFile(packageJsonUrl, JSON.stringify({ name, version: "1.0.0" }));

    await expect(createPackedPackage(packageJsonUrl)).rejects.toThrow(
      "Package path must remain inside the owned node_modules directory.",
    );

    await expect(readdir(scratch)).resolves.toStrictEqual([]);

    expect((await readdir(directory)).toSorted()).toStrictEqual([
      "package.json",
      "scratch",
    ]);
  },
);

test.each([
  null,
  false,
  42,
  "package",
  [],
  {},
  { name: 42 },
  { name: "fixture", private: "true" },
  ...["dependencies", "devDependencies", "peerDependencies"].flatMap((field) =>
    [
      null,
      [],
      "dependency",
      { example: 42 },
      { example: null },
      { example: false },
    ].map((value) => ({ name: "fixture", [field]: value })),
  ),
])("rejects malformed manifests before packing: %j", async (manifest) => {
  await writeFile(packageJsonUrl, JSON.stringify(manifest));

  await expect(createPackedPackage(packageJsonUrl)).rejects.toThrow(
    "Package manifest has an invalid name, privacy flag, or dependency map.",
  );

  await expect(readdir(scratch)).resolves.toStrictEqual([]);
});
