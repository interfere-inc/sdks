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
import { expect, test, vi } from "vitest";

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

test("keeps native failures, skips and duplicate cases distinct across projects", async () => {
  const { xml, result, file, phases } = await collect(
    'import { test, expect } from "vitest"; test.each([false, true])("duplicate", (value) => { expect(value).toBe(true); }); test.skip("skipped", () => {}); test("unique", () => { expect(true).toBe(true); }); test("duplicate [case 1]", () => { expect(true).toBe(true); });',
  );

  expect(result.testModules).toHaveLength(2);
  expect(result.unhandledErrors).toStrictEqual([]);
  expect(xml.match(/<failure /gu)).toHaveLength(2);
  expect(xml.match(/<skipped\/>/gu)).toHaveLength(2);
  expect(xml.match(/name="duplicate \[case 1\]"/gu)).toHaveLength(2);
  expect(xml.match(/name="duplicate \[case 2\]"/gu)).toHaveLength(2);
  expect(xml.match(/name="duplicate \[case 3\]"/gu)).toHaveLength(2);
  expect(xml).toContain("src/cases.test.ts [first]");
  expect(xml).toContain("src/cases.test.ts [second]");
  expect(xml.match(/name="unique"/gu)).toHaveLength(2);
  expect(xml.match(/file="[^"]+src\/cases.test.ts"/gu)).toHaveLength(10);
  expect(xml).toContain(`file="${file}"`);
  expect(xml).toContain(`classname="${file} [first]"`);
  expect(xml).toContain(`classname="${file} [second]"`);

  expect(phases[0]).toContain(
    '"phase":"discovered","files":[{"project":"first","file":"src/cases.test.ts"},{"project":"second","file":"src/cases.test.ts"}]',
  );

  const modules = phases.filter((event) => event.includes('"phase":"module"'));

  expect(modules).toHaveLength(2);
  expect(modules.join("\n")).toContain('"project":"first"');
  expect(modules.join("\n")).toContain('"project":"second"');

  expect(
    modules.every((event) =>
      /"milliseconds":\{"environment":[\d.]+,"prepare":[\d.]+,"collect":[\d.]+,"setup":[\d.]+,"testsAndHooks":[\d.]+\}/u.test(
        event,
      ),
    ),
  ).toBe(true);

  expect(phases.at(-1)).toContain(
    '"phase":"finished","files":2,"unhandledErrors":0',
  );
});

test("retains import failures separately for each native project", async () => {
  const { xml, result, phases } = await collect(
    'throw new Error("broken-import");',
  );

  expect(
    phases.filter((event) => event.includes('"phase":"module"')),
  ).toHaveLength(2);

  expect(result.testModules).toHaveLength(2);
  expect(xml.match(/<failure /gu)).toHaveLength(2);
  expect(xml).toContain("broken-import");
  expect(phases.at(-1)).toContain('"phase":"finished","files":2');
  expect(xml).toContain("src/cases.test.ts [first]");
  expect(xml).toContain("src/cases.test.ts [second]");
});

test("scopes repeated native unhandled rejections without dropping error nodes", async () => {
  const { xml, result } = await collect(
    'import { test } from "vitest"; test("native rejection", async () => { Promise.reject(new Error("unhandled-fixture")); Promise.reject(new Error("unhandled-fixture")); await new Promise((resolve) => setImmediate(resolve)); });',
  );

  expect(result.unhandledErrors).toHaveLength(4);
  expect(xml.match(/<error /gu)).toHaveLength(4);
  expect(xml).toContain("unhandled-fixture");
  expect(xml).not.toContain('classname="vitest unhandled errors"');
  expect(xml).not.toContain('name="vitest unhandled errors"');
  expect(xml).toContain("src/cases.test.ts &gt; vitest unhandled errors");
  expect(xml).toContain("[occurrence 1]");
  expect(xml).toContain("[occurrence 4]");
});

test("keeps long and XML-escaped names within the parser limit without losing case names", async () => {
  const longName = "long ".repeat(220);
  const quotedName = '"é'.repeat(180);
  const computedLimitName = "near ".repeat(190);

  const { xml, result } = await collect(
    'import { test, expect } from "vitest"; import data from "./data.json"; test.each([false, true])(data.longName, (value) => { expect(value).toBe(true); }); test(data.longName + "different", () => {}); test(data.quotedName, () => {}); test(data.computedLimitName, () => {});',
    { data: { longName, quotedName, computedLimitName } },
  );

  const names = Array.from(
    xml.matchAll(/<testcase[^>]* name="(?<name>[^"]+)"/gu),
    (match) => match.groups?.["name"] ?? "",
  );

  expect(result.testModules).toHaveLength(2);
  expect(names).toHaveLength(10);
  expect(new Set(names.slice(0, 5)).size).toBe(5);
  expect(new Set(names.slice(5)).size).toBe(5);
  expect(names.every((name) => Buffer.byteLength(name) <= 1000)).toBe(true);
  expect(xml.match(/<failure /gu)).toHaveLength(2);
  expect(xml.match(/name="vitest.fullName"/gu)).toHaveLength(10);
  expect(xml).toContain(`value="${longName} [case 1]"`);
  expect(xml).toContain(`value="${longName} [case 2]"`);
  expect(xml).toContain(`value="${quotedName.replaceAll('"', "&quot;")}"`);
});

test("retains oversized native unhandled errors and their original names", async () => {
  const message = "unhandled detail ".repeat(100);

  const { xml, result } = await collect(
    'import { test } from "vitest"; import data from "./data.json"; test("native rejection", async () => { Promise.reject(new Error(data.message)); await new Promise((resolve) => setImmediate(resolve)); });',
    { data: { message } },
  );

  expect(result.unhandledErrors).toHaveLength(2);
  expect(xml.match(/<error /gu)).toHaveLength(2);
  expect(xml.match(/name="vitest.fullName"/gu)).toHaveLength(2);
  expect(xml).toContain(`${message} [occurrence 1]`);
  expect(xml).toContain(`${message} [occurrence 2]`);

  const names = Array.from(
    xml.matchAll(/<testcase[^>]* name="(?<name>[^"]+)"/gu),
    (match) => match.groups?.["name"] ?? "",
  );

  expect(names.every((name) => Buffer.byteLength(name) <= 1000)).toBe(true);
});

test("retains failed hooks belonging to oversized suite names", async () => {
  const suite = '"é suite '.repeat(160);

  const { xml, result } = await collect(
    'import { describe, beforeAll, test } from "vitest"; import data from "./data.json"; describe(data.suite, () => { beforeAll(() => { throw new Error("suite setup failed"); }); test("child", () => {}); });',
    { data: { suite } },
  );

  expect(result.testModules).toHaveLength(2);
  expect(xml).toContain("suite setup failed");
  expect(xml.match(/<failure /gu)).toHaveLength(2);
  expect(xml.match(/<skipped\/>/gu)).toHaveLength(2);
  expect(xml.match(/name="vitest.fullName"/gu)).toHaveLength(4);
  expect(xml).toContain(`value="${suite.replaceAll('"', "&quot;")}"`);

  const names = Array.from(
    xml.matchAll(/<testcase[^>]* name="(?<name>[^"]+)"/gu),
    (match) => match.groups?.["name"] ?? "",
  );

  expect(names).toHaveLength(4);
  expect(names.every((name) => Buffer.byteLength(name) <= 1000)).toBe(true);
});

test("scopes native fileless rejections to the invoking workspace with the repository root", async () => {
  const cwd = process.cwd();

  try {
    process.chdir(fileURLToPath(new URL("../..", import.meta.url)));

    const { xml, result } = await collect(
      'import { test } from "vitest"; test("native fileless rejection", async () => { Promise.reject("fileless-fixture"); await new Promise((resolve) => setImmediate(resolve)); });',
      { root: fileURLToPath(new URL("../../../../../..", import.meta.url)) },
    );

    expect(result.unhandledErrors).toHaveLength(2);
    expect(xml.match(/<error /gu)).toHaveLength(2);

    expect(xml).toContain(
      'classname="src/packages/public/test-config &gt; vitest unhandled errors"',
    );

    expect(xml).toContain(
      'name="src/packages/public/test-config &gt; vitest unhandled errors"',
    );

    expect(xml).toContain("fileless-fixture [occurrence 1]");
    expect(xml).toContain("fileless-fixture [occurrence 2]");
  } finally {
    process.chdir(cwd);
  }
});

test.each([
  {
    name: "local",
    runId: undefined,
    attempt: undefined,
    suite: undefined,
    expected: '"runId":null,"runAttempt":null,"suite":null',
  },
  {
    name: "CI",
    runId: "source-run",
    attempt: "2",
    suite: "workers-1",
    expected: '"runId":"source-run","runAttempt":"2","suite":"workers-1"',
  },
])(
  "writes phase provenance to stdout for $name",
  ({ runId, attempt, suite, expected }) => {
    vi.stubEnv("GITHUB_RUN_ID", runId);

    vi.stubEnv("GITHUB_RUN_ATTEMPT", attempt);

    vi.stubEnv("TEST_SUITE", suite);

    const write = vi.spyOn(process.stdout, "write");

    try {
      const reporter = new ModuleStartReporter();

      reporter.onTestRunStart([]);

      reporter.onTestRunEnd([], [], "passed");

      expect(write).toHaveBeenCalledTimes(2);
      expect(write.mock.calls[0]?.[0]).toContain(expected);

      expect(write.mock.calls[0]?.[0]).toContain(
        '"phase":"discovered","files":[]',
      );

      expect(write.mock.calls[1]?.[0]).toContain(
        '"phase":"finished","files":0,"unhandledErrors":0,"reason":"passed"',
      );
    } finally {
      write.mockRestore();

      vi.unstubAllEnvs();
    }
  },
);
