import { mapWithConcurrency } from "@interfere/helpers/map-with-concurrency";

import { execFile } from "node:child_process";
import {
  mkdir,
  mkdtemp,
  readFile,
  rename,
  rm,
  symlink,
} from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { promisify } from "node:util";

const execFileAsync = promisify(execFile);

const PACKED_LINE = /^packed\s+\S+\s+(?<file>.+)$/u;

const TGZ_LINE = /^(?<filename>\S+\.tgz)$/u;

interface PackageJson {
  dependencies?: Record<string, string>;
  devDependencies?: Record<string, string>;
  name: string;
  peerDependencies?: Record<string, string>;
  private?: boolean;
}

function isDependencyMap(value: unknown): value is Record<string, string> {
  return (
    value !== null &&
    typeof value === "object" &&
    !Array.isArray(value) &&
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- JSON manifest dependency values must be strings before they become package specifiers.
    Object.values(value).every((version) => typeof version === "string")
  );
}

function isPackageJson(value: unknown): value is PackageJson {
  return (
    value !== null &&
    typeof value === "object" &&
    "name" in value &&
    typeof value.name === "string" &&
    (!("private" in value) || typeof value.private === "boolean") &&
    (!("dependencies" in value) || isDependencyMap(value.dependencies)) &&
    (!("devDependencies" in value) || isDependencyMap(value.devDependencies)) &&
    (!("peerDependencies" in value) || isDependencyMap(value.peerDependencies))
  );
}

async function readPackageJson(file: string | URL) {
  const value: unknown = JSON.parse(await readFile(file, "utf8"));

  if (!isPackageJson(value)) {
    throw new TypeError(
      "Package manifest has an invalid name, privacy flag, or dependency map.",
    );
  }

  return value;
}

export async function createPackedPackage(packageJsonUrl: URL) {
  const packageDir = path.dirname(fileURLToPath(packageJsonUrl));

  const packageJson = await readPackageJson(packageJsonUrl);

  const tempDir = await mkdtemp(
    path.join(tmpdir(), "interfere-package-contract-"),
  );

  const consumerDir = path.join(tempDir, "consumer");
  const nodeModulesDir = path.join(consumerDir, "node_modules");

  try {
    const installedPackageDir = joinPackageName(
      nodeModulesDir,
      packageJson.name,
    );

    await mkdir(path.dirname(installedPackageDir), { recursive: true });

    const { stdout } = await execFileAsync(
      "bun",
      ["pm", "pack", "--ignore-scripts", "--destination", tempDir],
      { cwd: packageDir },
    );

    const { filename, files } = parseBunPackOutput(stdout);
    const tarballPath = path.join(tempDir, path.basename(filename));
    const extractedDir = path.join(tempDir, "extracted");

    await mkdir(extractedDir);

    await execFileAsync("tar", ["-xzf", tarballPath, "-C", extractedDir]);

    await rm(tarballPath);

    await rename(path.join(extractedDir, "package"), installedPackageDir);

    await linkRuntimeDependencies({
      nodeModulesDir,
      packageDir,
      packageJson,
    });

    const packedPackageJson = await readPackageJson(
      path.join(installedPackageDir, "package.json"),
    );

    return {
      cleanup: async () => rm(tempDir, { force: true, recursive: true }),
      consumerDir,
      files: files.toSorted(),
      packageJson: packedPackageJson,
    };
  } catch (error) {
    await rm(tempDir, { force: true, recursive: true });

    throw error;
  }
}

function parseBunPackOutput(stdout: string) {
  const files: string[] = [];
  let filename = "";

  for (const line of stdout.split("\n")) {
    const packMatch = PACKED_LINE.exec(line);

    if (packMatch?.[1]) {
      files.push(packMatch[1]);

      continue;
    }

    const tgzName = TGZ_LINE.exec(line)?.groups?.["filename"];

    if (tgzName) {
      filename = tgzName;
    }
  }

  if (!filename) {
    throw new Error(`bun pm pack did not produce a tarball:\n${stdout}`);
  }

  return { filename, files };
}

async function linkRuntimeDependencies({
  nodeModulesDir,
  packageDir,
  packageJson,
}: {
  nodeModulesDir: string;
  packageDir: string;
  packageJson: PackageJson;
}) {
  const dependencies = new Set([
    ...Object.keys(packageJson.dependencies ?? {}),
    ...Object.keys(packageJson.peerDependencies ?? {}),
  ]);

  const results = await mapWithConcurrency(
    [...dependencies],
    16,
    async (dependencyName) => {
      if (dependencyName === packageJson.name) {
        return { ok: true } as const;
      }

      try {
        const source = joinPackageName(
          path.join(packageDir, "node_modules"),
          dependencyName,
        );

        const target = joinPackageName(nodeModulesDir, dependencyName);

        await mkdir(path.dirname(target), { recursive: true });

        await symlink(source, target, "junction");

        return { ok: true } as const;
      } catch (error) {
        return { ok: false, error } as const;
      }
    },
  );

  for (const result of results) {
    if (!result.ok) {
      throw result.error;
    }
  }
}

function joinPackageName(nodeModulesDir: string, packageName: string) {
  const segments = packageName.split("/");

  const validSegments = packageName.startsWith("@")
    ? segments.length === 2 && segments[0] !== "@"
    : segments.length === 1;

  const target = path.resolve(nodeModulesDir, packageName);
  const relative = path.relative(nodeModulesDir, target);

  if (
    !validSegments ||
    packageName.includes("\\") ||
    segments.some(
      (segment) => !segment || segment === "." || segment === "..",
    ) ||
    !relative ||
    relative === ".." ||
    relative.startsWith(`..${path.sep}`) ||
    path.isAbsolute(relative)
  ) {
    throw new Error(
      "Package path must remain inside the owned node_modules directory.",
    );
  }

  return target;
}
