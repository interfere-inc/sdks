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
import { dirname, join, relative } from "node:path";
import { fileURLToPath } from "node:url";
import { beforeAll, describe, expect, test } from "vitest";
import { createVitest } from "vitest/node";
import { reporting } from "./reporting.ts";

const repository = fileURLToPath(new URL("../../../../../..", import.meta.url));

interface Execution {
  unhandledErrors: unknown[];
  states: string[];
}

describe.each([false, true])(
  "produces authoritative LCOV with project selection %s",
  (projectSelected) => {
    let execution: Execution;
    let records: Map<string, string>;
    let directory: string;
    let includedPaths: string[];
    let uncoveredPaths: string[];

    beforeAll(async () => {
      directory = await realpath(
        await mkdtemp(join(tmpdir(), "coverage-scope-")),
      );

      const { exitCode } = process;
      const source = "export const value = () => 42;\n";
      const covered = "src/covered.ts";

      const uncovered = [
        "src/uncovered.ts",
        "src/workers.ts",
        "src/cloudflare.ts",
        "src/raw/row-context.ts",
        "src/schema/shared/retention.ts",
        "src/routes/__root.tsx",
        "src/routes/-examples.tsx",
        "src/packages/internal/database/src/workers.ts",
        "src/packages/internal/database/src/schema/tables/example.ts",
        "src/packages/internal/observability/lib/workers/observe.ts",
        "src/packages/internal/flags/src/vite.tsx",
        "src/apps/website/src/components/article/shadow-contract/diagrams.tsx",
        "src/packages/public/test-utils/src/helper.ts",
      ];

      const excluded = [
        "src/route-tree.gen.ts",
        "src/client.gen.mts",
        "src/component.gen.tsx",
        "src/example.fixture.ts",
        "src/example.fixture.mts",
        "src/example.mock.cts",
        "src/test/helper.ts",
        "src/dist/output.ts",
        "src/vite.config.ts",
      ];

      const support = [
        "src/packages/internal/internal-test-utils/src/helper.ts",
        "src/packages/internal/test-data/src/factory.ts",
      ];

      try {
        await symlink(
          fileURLToPath(new URL("../../node_modules", import.meta.url)),
          join(directory, "node_modules"),
          "dir",
        );

        await Promise.all(
          [covered, ...uncovered, ...excluded, ...support].map(async (file) => {
            await mkdir(dirname(join(directory, file)), { recursive: true });

            await writeFile(join(directory, file), source);
          }),
        );

        await writeFile(
          join(directory, "src/types.d.ts"),
          "export declare const value: number;\n",
        );

        await writeFile(
          join(directory, "src/cases.test.ts"),
          [
            'import { test, expect } from "vitest";',
            'import { value } from "./covered.ts";',
            ...[...excluded, ...support].map(
              (file) =>
                `import ${JSON.stringify(`./${relative("src", file)}`)};`,
            ),
            'test("executes source, fixtures and support", () => { expect(value()).toBe(42); });',
          ].join("\n"),
        );

        const runner = await createVitest({
          root: projectSelected ? dirname(directory) : directory,
          config: false,
          watch: false,
          reporters: [],
          maxWorkers: 1,
          fsModuleCache: false,
          project: projectSelected ? ["scope"] : [],
          projects: [
            {
              root: directory,
              test: {
                name: "scope",
                include: ["src/cases.test.ts"],
                fsModuleCache: false,
              },
            },
          ],
          coverage: {
            ...reporting.test?.coverage,
            enabled: true,
            reportsDirectory: join(directory, "coverage"),
          },
        });

        try {
          const result = await runner.start();

          execution = {
            unhandledErrors: result.unhandledErrors,
            states: result.testModules.map((module) => module.state()),
          };

          const lcov = await readFile(
            join(directory, "coverage/lcov.info"),
            "utf8",
          );

          records = new Map(
            lcov.split("end_of_record").flatMap((record) => {
              const file = /^SF:(?<file>.+)$/mu.exec(record)?.groups?.["file"];

              return file ? [[file, record] as const] : [];
            }),
          );

          const reportPath = (file: string) =>
            relative(repository, join(directory, file)).replaceAll("\\", "/");

          const included = [covered, ...uncovered, ...support];

          includedPaths = included.map((file) => reportPath(file)).toSorted();

          uncoveredPaths = uncovered.map((file) => reportPath(file)).toSorted();
        } finally {
          await runner.close();
        }
      } finally {
        process.exitCode = exitCode;

        await rm(directory, { recursive: true, force: true });
      }
    });

    test("reports complete source coverage", () => {
      expect(execution.unhandledErrors).toStrictEqual([]);

      expect(execution.states).toStrictEqual(["passed"]);

      expect([...records.keys()].toSorted()).toStrictEqual(includedPaths);

      expect(
        /^DA:\d+,[1-9]\d*$/mu.test(
          records.get(
            includedPaths.find((file) => file.endsWith("/src/covered.ts")) ??
              "",
          ) ?? "",
        ),
      ).toBe(true);

      expect(
        [...records]
          .flatMap(([file, record]) =>
            /^DA:\d+,0$/mu.test(record) && !/^DA:\d+,[1-9]\d*$/mu.test(record)
              ? [file]
              : [],
          )
          .toSorted(),
      ).toStrictEqual(uncoveredPaths);
    });
  },
);
