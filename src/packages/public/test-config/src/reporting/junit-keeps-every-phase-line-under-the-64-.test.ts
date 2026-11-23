import {
  mkdir,
  mkdtemp,
  readFile,
  realpath,
  rm,
  symlink,
  writeFile,
} from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, relative } from "node:path";
import { fileURLToPath } from "node:url";
import { afterAll, beforeAll, expect, test } from "vitest";

import type { Vitest } from "vitest/node";
import { createVitest } from "vitest/node";
import RepositoryJUnitReporter from "./junit";
import ModuleStartReporter from "./module-start";

async function collect(
  source: string,
  { root, data = {} }: { root?: string; data?: Record<string, string> } = {},
) {
  const directory = await realpath(
    await mkdtemp(join(tmpdir(), "junit-identities-")),
  );

  const report = join(directory, "results.xml");
  const phases: string[] = [];

  try {
    await symlink(
      fileURLToPath(new URL("../../node_modules", import.meta.url)),
      join(directory, "node_modules"),
      "dir",
    );

    await mkdir(join(directory, "src"));

    await writeFile(join(directory, "src/cases.test.ts"), source);

    await writeFile(join(directory, "src/data.json"), JSON.stringify(data));

    const { exitCode } = process;

    const runner = await createVitest({
      root: root ?? directory,
      config: false,
      watch: false,
      reporters: [
        new RepositoryJUnitReporter({ outputFile: report }),
        new ModuleStartReporter({
          write: (line) => {
            if (line.startsWith("vitest-phase ")) {
              phases.push(line);
            }
          },
        }),
      ],
      fsModuleCache: false,
      coverage: { enabled: false },
      projects: [
        {
          test: {
            root: directory,
            name: "first",
            include: ["src/cases.test.ts"],
            fsModuleCache: false,
          },
        },
        {
          test: {
            root: directory,
            name: "second",
            include: ["src/cases.test.ts"],
            fsModuleCache: false,
          },
        },
      ],
    });

    try {
      const result = await runner.start();

      return {
        phases,
        xml: await readFile(report, "utf8"),
        result,
        file: relative(
          fileURLToPath(new URL("../../../../../..", import.meta.url)),
          join(directory, "src/cases.test.ts"),
        ).replaceAll("\\", "/"),
      };
    } finally {
      await runner.close();

      process.exitCode = exitCode;
    }
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
}

let runner: Vitest | undefined;

afterAll(async () => {
  await runner?.close();
});

type Collected = Awaited<ReturnType<typeof collect>>;

const collected = new Map<string, Collected>();

const longName = "long ".repeat(220);
const quotedName = '"é'.repeat(180);
const computedLimitName = "near ".repeat(190);
const oversizedMessage = "unhandled detail ".repeat(100);
const oversizedSuite = '"é suite '.repeat(160);

beforeAll(async () => {
  runner = await createVitest({
    config: false,
    watch: false,
    name: "large-package (worker)",
  });

  collected.set(
    "distinct",
    await collect(
      'import { test, expect } from "vitest"; test.each([false, true])("duplicate", (value) => { expect(value).toBe(true); }); test.skip("skipped", () => {}); test("unique", () => { expect(true).toBe(true); }); test("duplicate [case 1]", () => { expect(true).toBe(true); });',
    ),
  );

  collected.set("imports", await collect('throw new Error("broken-import");'));

  collected.set(
    "rejections",
    await collect(
      'import { test } from "vitest"; test("native rejection", async () => { Promise.reject(new Error("unhandled-fixture")); Promise.reject(new Error("unhandled-fixture")); await new Promise((resolve) => setImmediate(resolve)); });',
    ),
  );

  collected.set(
    "long-names",
    await collect(
      'import { test, expect } from "vitest"; import data from "./data.json"; test.each([false, true])(data.longName, (value) => { expect(value).toBe(true); }); test(data.longName + "different", () => {}); test(data.quotedName, () => {}); test(data.computedLimitName, () => {});',
      { data: { longName, quotedName, computedLimitName } },
    ),
  );

  collected.set(
    "oversized",
    await collect(
      'import { test } from "vitest"; import data from "./data.json"; test("native rejection", async () => { Promise.reject(new Error(data.message)); await new Promise((resolve) => setImmediate(resolve)); });',
      { data: { message: oversizedMessage } },
    ),
  );

  collected.set(
    "hooks",
    await collect(
      'import { describe, beforeAll, test } from "vitest"; import data from "./data.json"; describe(data.suite, () => { beforeAll(() => { throw new Error("suite setup failed"); }); test("child", () => {}); });',
      { data: { suite: oversizedSuite } },
    ),
  );

  const cwd = process.cwd();

  try {
    process.chdir(fileURLToPath(new URL("../..", import.meta.url)));

    collected.set(
      "fileless",
      await collect(
        'import { test } from "vitest"; test("native fileless rejection", async () => { Promise.reject("fileless-fixture"); await new Promise((resolve) => setImmediate(resolve)); });',
        { root: fileURLToPath(new URL("../../../../../..", import.meta.url)) },
      ),
    );
  } finally {
    process.chdir(cwd);
  }
});

test("keeps every phase line under the 64 KiB line limit of log readers however many modules a run discovers", () => {
  const [project] = runner?.projects ?? [];

  if (project === undefined) {
    throw new Error("Vitest created no project");
  }

  const lines: string[] = [];

  const reporter = new ModuleStartReporter({
    write: (line) => {
      lines.push(line);
    },
  });

  reporter.onTestRunStart(
    Array.from({ length: 1000 }, (_, index) =>
      project.createSpecification(
        join(
          project.config.root,
          `src/investigation/lifecycle/test/module-${index}.worker.test.ts`,
        ),
      ),
    ),
  );

  expect(
    lines.filter((line) => Buffer.byteLength(line) >= 64 * 1024),
  ).toStrictEqual([]);

  expect(lines).toHaveLength(1000);
});
