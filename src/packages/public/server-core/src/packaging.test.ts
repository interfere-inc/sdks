import { execFileSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { afterAll, beforeAll, describe, expect, test } from "vitest";

const root = fileURLToPath(new URL("../../../../../", import.meta.url));
const publicPackages = join(root, "src/packages/public");
const license = readFileSync(join(publicPackages, "next/LICENSE"), "utf8");

const packages = [
  "build-tools",
  "constants",
  "helpers",
  "javascript",
  "next",
  "react",
  "server-core",
  "test-utils",
  "types",
  "vite",
];

test("the repository supplies the SDK MIT license", () => {
  expect(readFileSync(join(root, "LICENSE"), "utf8")).toBe(license);
});

describe.each(packages)("%s published metadata", (directory) => {
  let scratch: string;

  beforeAll(() => {
    scratch = mkdtempSync(join(tmpdir(), "sdk-packaging-"));

    const tarball = join(scratch, "package.tgz");

    execFileSync(
      "bun",
      ["pm", "pack", "--ignore-scripts", "--filename", tarball],
      {
        cwd: join(publicPackages, directory),
        stdio: "pipe",
      },
    );

    execFileSync("tar", ["-xzf", tarball, "-C", scratch]);
  });

  afterAll(() => rmSync(scratch, { recursive: true, force: true }));

  test("ships its MIT license and README", () => {
    const read = (file: string) =>
      readFileSync(join(scratch, "package", file), "utf8");

    expect(read("LICENSE")).toBe(license);
    expect(read("README.md").trim().length).toBeGreaterThan(0);

    expect(JSON.parse(read("package.json"))).toMatchObject({
      license: "MIT",
      repository: {
        type: "git",
        directory: `src/packages/public/${directory}`,
      },
    });
  });
});

test.each([
  "constants",
  "helpers",
  "next",
  "react",
  "types",
  "typescript-config",
  "vite",
])("%s links its license badge to the shipped license", (directory) => {
  const readme = readFileSync(
    join(publicPackages, directory, "README.md"),
    "utf8",
  );

  expect(readme).toMatch(/href="\.\/LICENSE"><img[^>]+alt="License"/u);
});

test.each(packages)(
  "%s declares the Node.js major exercised by SDK CI",
  (directory) => {
    const manifest: unknown = JSON.parse(
      readFileSync(join(publicPackages, directory, "package.json"), "utf8"),
    );

    expect(manifest).toMatchObject({ engines: { node: "24.x" } });

    expect(
      readFileSync(join(publicPackages, directory, "README.md"), "utf8"),
    ).toContain("Node.js 24.x");
  },
);

test.each(["vite", "build-tools", "types", "constants", "test-utils"])(
  "%s declares explicit initialization for tree shaking",
  (directory) => {
    const manifest: unknown = JSON.parse(
      readFileSync(join(publicPackages, directory, "package.json"), "utf8"),
    );

    expect(manifest).toMatchObject({ sideEffects: false });
  },
);

test.each(["next", "react", "vite"])(
  "%s documents the supported framework matrix",
  (directory) => {
    const readme = readFileSync(
      join(publicPackages, directory, "README.md"),
      "utf8",
    );

    expect(readme.replaceAll(String.raw`\|`, "|")).toContain(
      "^15.3.0 || ^16.0.0",
    );

    expect(readme).toContain("React 19");
    expect(readme).toContain("Vite 8.2.4");
  },
);
