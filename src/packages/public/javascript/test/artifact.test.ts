import { execFileSync } from "node:child_process";
import {
  copyFile,
  mkdir,
  mkdtemp,
  readFile,
  readdir,
  rm,
  symlink,
  writeFile,
} from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { build } from "tsdown";
import { afterAll, beforeAll, expect, test } from "vitest";

let temporary: string;
let published: string;

const portableConditions = [
  "browser",
  "edge-light",
  "workerd",
  "worker",
] as const;

const portableOutputs = new Map<string, string[]>();

const nodeOutputs: string[] = [];

beforeAll(async () => {
  temporary = await mkdtemp(join(tmpdir(), "javascript-artifact-"));

  const packageRoot = resolve(import.meta.dirname, "..");

  execFileSync(
    "bun",
    ["pm", "pack", "--ignore-scripts", "--quiet", "--destination", temporary],
    {
      cwd: packageRoot,
      timeout: 10_000,
    },
  );

  const tarball = (await readdir(temporary)).find((name) =>
    name.endsWith(".tgz"),
  );

  if (!tarball) {
    throw new Error("Package archive was not created");
  }

  execFileSync("tar", ["-xzf", join(temporary, tarball), "-C", temporary]);

  published = join(temporary, "package");

  await mkdir(join(temporary, "node_modules/@interfere"), { recursive: true });

  await symlink(
    published,
    join(temporary, "node_modules/@interfere/javascript"),
  );

  await copyFile(
    join(import.meta.dirname, "runtime-probe.mts"),
    join(temporary, "runtime-probe.mts"),
  );

  await writeFile(join(temporary, "upload.map"), "source-map-content");

  for (const condition of portableConditions) {
    const outputs: string[] = [];

    for (const format of ["import", "require"]) {
      outputs.push(
        execFileSync(
          process.execPath,
          [
            `--conditions=${condition}`,
            join(temporary, "runtime-probe.mts"),
            format,
            "portable",
            join(temporary, "upload.map"),
          ],
          { timeout: 10_000, encoding: "utf8" },
        ),
      );
    }

    portableOutputs.set(condition, outputs);
  }

  for (const format of ["import", "require"]) {
    nodeOutputs.push(
      execFileSync(
        process.execPath,
        [
          join(temporary, "runtime-probe.mts"),
          format,
          "node",
          join(temporary, "upload.map"),
        ],
        { timeout: 10_000, encoding: "utf8" },
      ),
    );
  }
});

afterAll(async () => {
  if (temporary) {
    await rm(temporary, { recursive: true, force: true });
  }
});

test.each(portableConditions)(
  "%s selects a portable client that uploads Blobs and rejects file paths",
  (condition) => {
    expect.assertions(2);

    for (const output of portableOutputs.get(condition) ?? []) {
      expect(output).toBe("uploads verified");
    }
  },
);

test("node uploads Blobs and file paths through ESM and CommonJS", () => {
  expect.assertions(2);

  for (const output of nodeOutputs) {
    expect(output).toBe("uploads verified");
  }
});

test.each(["mjs", "cjs"])(
  "the portable %s artifact has no filesystem imports",
  async (extension) => {
    const source = await readFile(
      join(published, `dist/portable/index.${extension}`),
      "utf8",
    );

    expect(source).not.toMatch(
      /(?:import|require)\s*\(\s*["'`](?:node:)?fs(?:\/[^"'`]*)?["'`]/u,
    );
  },
);

test.each(["esm", "cjs"] as const)(
  "unknown runtimes bundle the portable %s default without filesystem imports",
  async (format) => {
    const extension = format === "esm" ? "mjs" : "cjs";
    const entry = join(temporary, `consumer.${extension}`);
    const outDir = join(temporary, "consumer-dist");

    await writeFile(
      entry,
      format === "esm"
        ? 'export { InterfereClient } from "@interfere/javascript";'
        : 'module.exports = require("@interfere/javascript");',
    );

    await build({
      config: false,
      entry: { index: entry },
      outDir,
      platform: "neutral",
      format,
      fixedExtension: true,
      dts: false,
      inputOptions: {
        platform: "neutral",
        resolve: {
          conditionNames: ["default", format === "esm" ? "import" : "require"],
        },
      },
      deps: { alwaysBundle: ["@interfere/javascript"] },
      logLevel: "silent",
    });

    const source = await readFile(join(outDir, `index.${extension}`), "utf8");

    expect(source).not.toMatch(
      /(?:import|require)\s*\(\s*["'`](?:node:)?fs(?:\/[^"'`]*)?["'`]/u,
    );

    expect(source).toContain(
      "File path uploads are not supported in this environment.",
    );
  },
);
