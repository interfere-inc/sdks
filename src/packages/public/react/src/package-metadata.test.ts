import {
  mkdir,
  mkdtemp,
  readFile,
  rm,
  symlink,
  writeFile,
} from "node:fs/promises";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { expect, test } from "vitest";
import { packageFiles } from "../test/package-files";

test("shipped JavaScript supports the Safari 16.4 and Chrome 111 builtin baseline", async () => {
  const directory = fileURLToPath(new URL("../", import.meta.url));

  const files = packageFiles(directory, ["dist/**/*.mjs"]).filter((file) =>
    file.endsWith(".mjs"),
  );

  const unsupportedBuiltins =
    /\.(?:withResolvers|toSorted|toReversed|toSpliced|groupBy)\s*\(/u;

  expect(files.length).toBeGreaterThan(0);

  const sources = await Promise.all(
    files.map(async (file) => readFile(join(directory, file), "utf8")),
  );

  const incompatible = files.filter((_, index) =>
    unsupportedBuiltins.test(sources[index] ?? ""),
  );

  expect(incompatible).toStrictEqual([]);
});

test("packs every exported entry without internal test utilities", async () => {
  expect.hasAssertions();

  const packageDirectory = new URL("../", import.meta.url);

  const metadata = JSON.parse(
    await readFile(new URL("package.json", packageDirectory), "utf8"),
  ) as {
    files: string[];
    exports: Record<string, { types: string; default: string }>;
  };

  const files = packageFiles(fileURLToPath(packageDirectory), metadata.files);

  expect(files).toContain("CHANGELOG.md");

  expect(
    files.filter((file) => file.startsWith("dist/internal/test/")),
  ).toStrictEqual([]);

  for (const entry of Object.values(metadata.exports)) {
    expect(files).toContain(entry.default.slice(2));
    expect(files).toContain(entry.types.slice(2));
  }
});

test("evaluates publish globs within the package without following dependencies or links", async () => {
  const fixture = await mkdtemp(join(tmpdir(), "react-package-files-"));

  const directory = join(fixture, "package");

  const paths = [
    "package.json",
    "README.md",
    "LICENSE",
    "CHANGELOG.md",
    "dist/entry.mjs",
    "dist/.hidden.mjs",
    "dist/internal/test/helper.mjs",
    "dist/.npmrc",
    "dist/.git/config",
    "dist/node_modules/dependency/index.mjs",
    "node_modules/dependency/index.mjs",
    "src/entry.ts",
    "notes.txt",
  ];

  try {
    await Promise.all(
      paths.map(async (path) => {
        await mkdir(dirname(join(directory, path)), { recursive: true });

        await writeFile(join(directory, path), "");
      }),
    );

    await writeFile(join(fixture, "outside.mjs"), "");

    await symlink(fixture, join(directory, "dist/linked"), "dir");

    await symlink(
      join(fixture, "outside.mjs"),
      join(directory, "dist/linked.mjs"),
    );

    expect(
      packageFiles(directory, [
        "dist/**/*.mjs",
        "!dist/internal/test",
        "!README.md",
        "../*",
      ]),
    ).toStrictEqual([
      "CHANGELOG.md",
      "LICENSE",
      "README.md",
      "dist/.hidden.mjs",
      "dist/entry.mjs",
      "package.json",
    ]);

    expect(
      packageFiles(directory, ["./dist/", "!dist/internal/**"]),
    ).toStrictEqual([
      "CHANGELOG.md",
      "LICENSE",
      "README.md",
      "dist/.hidden.mjs",
      "dist/entry.mjs",
      "package.json",
    ]);
  } finally {
    await rm(fixture, { recursive: true, force: true });
  }
});
