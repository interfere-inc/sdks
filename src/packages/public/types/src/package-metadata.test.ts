import { createPackedPackage } from "@interfere/test-utils/vitest/package-contract";

import { execFile } from "node:child_process";
import { promisify } from "node:util";
import { readFile } from "node:fs/promises";
import { beforeAll, describe, expect, test } from "vitest";

const execFileAsync = promisify(execFile);

interface PackageExport {
  import?: { types?: string; default?: string };
  require?: { types?: string; default?: string };
}

interface PackageJson {
  exports?: Record<string, PackageExport | undefined>;
  files?: string[];
  imports?: unknown;
}

const DTS_EXT = /\.d\.mts$/u;

const MJS_EXT = /\.mjs$/u;

async function readPackageJson() {
  return JSON.parse(
    await readFile(new URL("../package.json", import.meta.url), "utf8"),
  ) as PackageJson;
}

describe("@interfere/types package metadata", () => {
  let esm: { stdout: string };
  let cjs: { stdout: string };

  beforeAll(async () => {
    const packed = await createPackedPackage(
      new URL("../package.json", import.meta.url),
    );

    const specifiers = [
      "@interfere/types/data/session",
      "@interfere/types/integrations",
      "@interfere/types/integrations/platforms/vercel",
      "@interfere/types/releases/slug",
      "@interfere/types/sdk/errors",
    ];

    try {
      esm = await execFileAsync(
        process.execPath,
        [
          "--input-type=module",
          "--eval",
          specifiers
            .map((specifier) => `await import(${JSON.stringify(specifier)});`)
            .join("\n"),
        ],
        { cwd: packed.consumerDir },
      );

      cjs = await execFileAsync(
        process.execPath,
        [
          "--eval",
          specifiers
            .map((specifier) => `require(${JSON.stringify(specifier)});`)
            .join("\n"),
        ],
        { cwd: packed.consumerDir },
      );
    } finally {
      await packed.cleanup();
    }
  });

  test("exposes dual ESM and CommonJS package exports", async () => {
    const pkg = await readPackageJson();

    expectDualPackageMap(pkg.exports);
  });

  test("resolves internal modules relatively, with no package imports map", async () => {
    const pkg = await readPackageJson();

    expect(pkg.imports).toBeUndefined();
  });

  test("publishes runtime files and declarations without source maps", async () => {
    const pkg = await readPackageJson();

    expect(pkg.files).toStrictEqual([
      "dist/**/*.cjs",
      "dist/**/*.mjs",
      "dist/**/*.d.cts",
      "dist/**/*.d.mts",
      "README.md",
      "CHANGELOG.md",
    ]);
  });

  test("packs representative built files consumed by ESM and CommonJS exports", async () => {
    const packed = await createPackedPackage(
      new URL("../package.json", import.meta.url),
    );

    try {
      expect(packed.files).toStrictEqual(
        expect.arrayContaining([
          "dist/data/session.cjs",
          "dist/data/session.d.cts",
          "dist/data/session.d.mts",
          "dist/data/session.mjs",
          "dist/integrations/index.cjs",
          "dist/integrations/index.d.cts",
          "dist/integrations/index.d.mts",
          "dist/integrations/index.mjs",
          "dist/integrations/platforms/vercel.cjs",
          "dist/integrations/platforms/vercel.d.cts",
          "dist/integrations/platforms/vercel.d.mts",
          "dist/integrations/platforms/vercel.mjs",
          "dist/releases/slug.cjs",
          "dist/releases/slug.d.cts",
          "dist/releases/slug.d.mts",
          "dist/releases/slug.mjs",
          "dist/sdk/errors.cjs",
          "dist/sdk/errors.d.cts",
          "dist/sdk/errors.d.mts",
          "dist/sdk/errors.mjs",
          "package.json",
        ]),
      );

      expect(
        packed.files.filter((file) => file.startsWith("src/")),
      ).toStrictEqual([]);

      expect(
        packed.files.filter((file) => file.endsWith(".map")),
      ).toStrictEqual([]);

      expect(packed.packageJson.private).toBeUndefined();
    } finally {
      await packed.cleanup();
    }
  });

  test("loads representative ESM and CommonJS subpaths through Node package resolution", () => {
    expect(esm).toHaveProperty("stdout");
    expect(cjs).toHaveProperty("stdout");
  });
});

function expectDualPackageMap(
  map: Record<string, PackageExport | undefined> | undefined,
) {
  expect(map).toStrictEqual(expect.any(Object));

  for (const [subpath, entry] of Object.entries(map ?? {})) {
    expect(entry?.import?.types, subpath).toMatch(DTS_EXT);
    expect(entry?.import?.default, subpath).toMatch(MJS_EXT);

    expect(entry?.require?.types, subpath).toBe(
      entry?.import?.types?.replace(DTS_EXT, ".d.cts"),
    );

    expect(entry?.require?.default, subpath).toBe(
      entry?.import?.default?.replace(MJS_EXT, ".cjs"),
    );
  }
}
