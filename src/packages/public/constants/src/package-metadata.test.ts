import { createPackedPackage } from "@interfere/test-utils/vitest/package-contract";

import { execFile } from "node:child_process";
import { promisify } from "node:util";
import { readFile } from "node:fs/promises";
import { describe, expect, test } from "vitest";

const execFileAsync = promisify(execFile);

interface ConditionalExport {
  import?: { types?: string; default?: string };
  monorepo?: string;
  require?: { types?: string; default?: string };
}

interface PackageJson {
  exports?: Record<string, ConditionalExport | string | undefined>;
  files?: string[];
}

const DTS_EXT = /\.d\.mts$/u;

const MJS_EXT = /\.mjs$/u;

async function readPackageJson() {
  return JSON.parse(
    await readFile(new URL("../package.json", import.meta.url), "utf8"),
  ) as PackageJson;
}

function expectDualExport(entry: ConditionalExport | string | undefined) {
  expect(entry).toStrictEqual(expect.any(Object));

  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Package export entries can be strings or conditional objects; these assertions require the conditional-object contract.
  if (typeof entry !== "object") {
    throw new TypeError("expected conditional export object");
  }

  expect(entry.import?.types).toMatch(DTS_EXT);
  expect(entry.import?.default).toMatch(MJS_EXT);

  expect(entry.require?.types).toBe(
    entry.import?.types?.replace(DTS_EXT, ".d.cts"),
  );

  expect(entry.require?.default).toBe(
    entry.import?.default?.replace(MJS_EXT, ".cjs"),
  );
}

describe("@interfere/constants package metadata", () => {
  test.each([undefined, "./dist/api.mjs"])(
    "rejects non-conditional export entry %j",
    (entry) => {
      expect(() => expectDualExport(entry)).toThrow(Error);
    },
  );

  test("exposes dual ESM and CommonJS runtime exports", async () => {
    const pkg = await readPackageJson();

    expectDualExport(pkg.exports?.["./api"]);

    expectDualExport(pkg.exports?.["./http"]);

    expectDualExport(pkg.exports?.["./route-prefix"]);

    expectDualExport(pkg.exports?.["./service-names"]);

    expect(pkg.exports?.["./package.json"]).toBe("./package.json");
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

  test("packs the built files consumed by ESM and CommonJS exports", async () => {
    const packed = await createPackedPackage(
      new URL("../package.json", import.meta.url),
    );

    try {
      expect(packed.files).toStrictEqual(
        expect.arrayContaining([
          "dist/api.cjs",
          "dist/api.d.cts",
          "dist/api.d.mts",
          "dist/api.mjs",
          "dist/http.cjs",
          "dist/http.d.cts",
          "dist/http.d.mts",
          "dist/http.mjs",
          "dist/route-prefix.cjs",
          "dist/route-prefix.d.cts",
          "dist/route-prefix.d.mts",
          "dist/route-prefix.mjs",
          "dist/service-names.cjs",
          "dist/service-names.d.cts",
          "dist/service-names.d.mts",
          "dist/service-names.mjs",
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

  test("loads ESM and CommonJS subpaths through Node package resolution", async () => {
    const packed = await createPackedPackage(
      new URL("../package.json", import.meta.url),
    );

    const specifiers = [
      "@interfere/constants/api",
      "@interfere/constants/http",
      "@interfere/constants/route-prefix",
      "@interfere/constants/service-names",
    ];

    try {
      await expect(
        execFileAsync(
          process.execPath,
          [
            "--input-type=module",
            "--eval",
            specifiers
              .map((specifier) => `await import(${JSON.stringify(specifier)});`)
              .join("\n"),
          ],
          { cwd: packed.consumerDir },
        ),
      ).resolves.toHaveProperty("stdout");

      await expect(
        execFileAsync(
          process.execPath,
          [
            "--eval",
            specifiers
              .map((specifier) => `require(${JSON.stringify(specifier)});`)
              .join("\n"),
          ],
          { cwd: packed.consumerDir },
        ),
      ).resolves.toHaveProperty("stdout");
    } finally {
      await packed.cleanup();
    }
  });
});
