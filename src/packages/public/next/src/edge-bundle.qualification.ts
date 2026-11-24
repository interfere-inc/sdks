import { execFile } from "node:child_process";
import { fileURLToPath } from "node:url";
import { promisify } from "node:util";
import { beforeAll, expect, test } from "vitest";

const execute = promisify(execFile);
let output: string;
let errors: string;

beforeAll(async () => {
  const bundle = await execute(
    "bun",
    [
      "build",
      fileURLToPath(new URL("../test/edge-entry.ts", import.meta.url)),
      "--target=browser",
      "--format=iife",
      "--conditions=edge-light,browser",
      "--define",
      `process.env=${JSON.stringify({
        INTERFERE_API_KEY: "interfere_pk_test",
        INTERFERE_API_URL: "https://collector.test",
        INTERFERE_ENVIRONMENT: "preview",
        INTERFERE_SOURCE_ID: "abcdef1234567890abcdef1234567890abcdef1234",
      })}`,
    ],
    { maxBuffer: 10 * 1024 * 1024 },
  );

  const runner = execute("node", [
    fileURLToPath(new URL("../test/edge-vm.ts", import.meta.url)),
  ]);

  runner.child.stdin?.end(bundle.stdout);

  const result = await runner;

  output = result.stdout;

  errors = result.stderr;
});

test("edge instrumentation bundles and initializes without Node globals", () => {
  expect(errors).toBe("");

  expect(JSON.parse(output)).toStrictEqual({
    globals: ["undefined", "undefined", "undefined"],
    recording: true,
  });
});
