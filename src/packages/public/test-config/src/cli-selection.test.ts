import { spawnSync } from "node:child_process";
import {
  mkdirSync,
  mkdtempSync,
  readFileSync,
  realpathSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterAll, beforeAll, expect, test, vi } from "vitest";

const repository = realpathSync(mkdtempSync(join(tmpdir(), "selection-")));
const local = join(repository, "pkg");
const selectionFile = join(repository, "selection.txt");
const output = join(local, "arguments.json");

async function start(entries: string[]) {
  writeFileSync(selectionFile, `${entries.join("\n")}\n`);

  rmSync(output, { force: true });

  const originalDirectory = process.cwd();
  const originalArgv = process.argv;
  const write = vi.spyOn(process.stdout, "write");

  vi.stubEnv("VITEST_ARGS", '[".browser.test.","--project=*(browser)"]');

  vi.stubEnv("VITEST_SELECTION", selectionFile);

  vi.resetModules();

  process.argv = [process.execPath, "cli.ts", "run"];

  try {
    process.chdir(local);

    await import("./cli.ts");

    const started: Started = {
      stdout: write.mock.calls.map(([chunk]) => String(chunk)).join(""),
      args: readFileSync(output, { encoding: "utf8", flag: "a+" }),
    };

    return started;
  } finally {
    write.mockRestore();

    process.argv = originalArgv;

    process.chdir(originalDirectory);

    vi.unstubAllEnvs();

    vi.resetModules();
  }
}

interface Started {
  stdout: string;
  args: string;
}

let matching: Started;
let unmatched: Started;

beforeAll(async () => {
  const installation = join(local, "node_modules/vitest");

  mkdirSync(installation, { recursive: true });

  mkdirSync(join(local, "sub"));

  writeFileSync(
    join(installation, "package.json"),
    JSON.stringify({
      name: "vitest",
      exports: { "./package.json": "./package.json" },
    }),
  );

  writeFileSync(
    join(installation, "vitest.mjs"),
    `import { writeFileSync } from "node:fs";
writeFileSync(${JSON.stringify(output)}, JSON.stringify(process.argv.slice(2)));`,
  );

  writeFileSync(join(local, "package.json"), '{ "name": "@fixture/pkg" }');

  for (const file of [
    "a.browser.test.ts",
    "b.test.ts",
    "sub/c.browser.test.ts",
  ]) {
    writeFileSync(join(local, file), "");
  }

  spawnSync("git", ["init", "-q"], { cwd: repository });

  spawnSync(
    "git",
    [
      "add",
      "pkg/package.json",
      "pkg/a.browser.test.ts",
      "pkg/b.test.ts",
      "pkg/sub",
    ],
    {
      cwd: repository,
    },
  );

  matching = await start([
    "pkg/a.browser.test.ts",
    "pkg/b.test.ts",
    "pkg/sub/",
  ]);

  unmatched = await start(["pkg/b.test.ts", "other/x.browser.test.ts"]);
}, 30_000);

afterAll(() => rmSync(repository, { recursive: true, force: true }));

test("passes the matching package files and drops the suite patterns", () => {
  expect(JSON.parse(matching.args)).toStrictEqual([
    "run",
    "--project=*(browser)",
    join(local, "a.browser.test.ts"),
    join(local, "sub/c.browser.test.ts"),
  ]);
});

test("starts no Vitest when no selected file matches", () => {
  expect(unmatched).toStrictEqual({
    stdout:
      "The selected test files include none for this package and suite.\n",
    args: "",
  });
});
