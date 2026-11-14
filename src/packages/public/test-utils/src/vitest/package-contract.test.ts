import { execFile } from "node:child_process";
import { mkdir, mkdtemp, readdir, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { pathToFileURL } from "node:url";
import { promisify } from "node:util";

import { createPackedPackage } from "./package-contract.js";
import { afterEach, beforeEach, expect, test, vi } from "vitest";

const execFileAsync = promisify(execFile);

let directory: string;

let scratch: string;

let packageJsonUrl: URL;

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

test.each(["package-contract-fixture", "@fixture/package-contract"])(
  "packs %s concurrently into isolated consumers and cleans them",
  async (name) => {
    expect.hasAssertions();

    await writeFile(
      packageJsonUrl,
      JSON.stringify({
        name,
        version: "1.0.0",
        files: ["index.cjs"],
        exports: "./index.cjs",
      }),
    );

    await writeFile(path.join(directory, "index.cjs"), "module.exports = 42;");

    const results = await Promise.allSettled([
      createPackedPackage(packageJsonUrl),
      createPackedPackage(packageJsonUrl),
      createPackedPackage(packageJsonUrl),
    ]);

    const packages = results.flatMap((result) =>
      result.status === "fulfilled" ? [result.value] : [],
    );

    try {
      expect(results.map((result) => result.status)).toStrictEqual([
        "fulfilled",
        "fulfilled",
        "fulfilled",
      ]);

      expect(new Set(packages.map((packed) => packed.consumerDir)).size).toBe(
        3,
      );

      for (const packed of packages) {
        expect(packed.files).toStrictEqual(["index.cjs", "package.json"]);
        expect(packed.packageJson.name).toBe(name);
      }

      await Promise.all(
        packages.map(async (packed) => {
          const esm = await execFileAsync(
            process.execPath,
            [
              "--input-type=module",
              "--eval",
              `console.log((await import(${JSON.stringify(name)})).default)`,
            ],
            { cwd: packed.consumerDir },
          );

          const cjs = await execFileAsync(
            process.execPath,
            ["--eval", `console.log(require(${JSON.stringify(name)}))`],
            { cwd: packed.consumerDir },
          );

          expect(esm.stdout.trim()).toBe("42");
          expect(cjs.stdout.trim()).toBe("42");
        }),
      );
    } finally {
      await Promise.all(packages.map(async (packed) => packed.cleanup()));
    }

    await expect(readdir(scratch)).resolves.toStrictEqual([]);

    expect((await readdir(directory)).toSorted()).toStrictEqual([
      "index.cjs",
      "package.json",
      "scratch",
    ]);
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
