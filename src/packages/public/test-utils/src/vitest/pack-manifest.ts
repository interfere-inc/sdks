import { execFile } from "node:child_process";
import { mkdtemp, readFile, rename, rm, writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import { promisify } from "node:util";

const exec = promisify(execFile);

type Condition = string | null | Condition[] | { [key: string]: Condition };

interface PackageManifest {
  exports?: Condition;
  imports?: Condition;
}

const stripConditions: (value: Condition) => Condition = (value) => {
  if (Array.isArray(value)) {
    return value.map((entry) => stripConditions(entry));
  }

  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Conditional exports are a recursive JSON union, with strings and null as leaves.
  if (value === null || typeof value !== "object") {
    return value;
  }

  return Object.fromEntries(
    Object.entries(value)
      .filter(([key]) => key !== "monorepo")
      .map(([key, entry]) => [key, stripConditions(entry)]),
  );
};

/** Removes workspace conditions from a disposable package manifest, preserving condition order. */
export async function stripPackageConditions(file: string) {
  const original = await readFile(file, "utf8");

  // oxlint-disable-next-line typescript/no-unsafe-type-assertion -- The package manager validates these local package manifests; keep this release-automation helper dependency-free.
  const manifest = JSON.parse(original) as PackageManifest;
  let changed = false;

  for (const field of ["exports", "imports"] as const) {
    const value = manifest[field];

    if (value === undefined) {
      continue;
    }

    const stripped = stripConditions(value);

    if (JSON.stringify(value) !== JSON.stringify(stripped)) {
      manifest[field] = stripped;

      changed = true;
    }
  }

  if (changed) {
    await writeFile(file, `${JSON.stringify(manifest, null, 2)}\n`);
  }

  return changed;
}

/** Sanitizes a locally packed tarball without modifying its workspace source. */
export async function stripTarballConditions(tarball: string) {
  const scratch = await mkdtemp(join(dirname(tarball), ".pack-conditions-"));

  try {
    await exec("tar", ["-xzf", tarball, "-C", scratch]);

    const changed = await stripPackageConditions(
      join(scratch, "package/package.json"),
    );

    if (changed) {
      const sanitized = join(scratch, "sanitized.tgz");

      await exec("tar", ["-czf", sanitized, "-C", scratch, "package"]);

      await rename(sanitized, tarball);
    }
  } finally {
    await rm(scratch, { recursive: true, force: true });
  }
}
