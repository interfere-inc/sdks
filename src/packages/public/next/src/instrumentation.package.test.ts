import { createPackedPackage } from "@interfere/test-utils/vitest/package-contract";
import { execFile } from "node:child_process";
import { writeFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { basename, dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { promisify } from "node:util";
import { beforeAll, expect, test } from "vitest";

const exec = promisify(execFile);
const require = createRequire(import.meta.url);
let packed: Awaited<ReturnType<typeof createPackedPackage>>;

const typeModules = ["CommonJS", "ESNext", "NodeNext"] as const;

const runtimeCases = [
  { condition: "node", expectedEntry: "instrumentation.mjs" },
  { condition: "edge-light", expectedEntry: "instrumentation.edge.mjs" },
  { condition: "workerd", expectedEntry: "instrumentation.edge.mjs" },
  { condition: "worker", expectedEntry: "instrumentation.edge.mjs" },
] as const;

const typeConditions = ["edge-light", "workerd", "worker"] as const;

const typeResults = new Map<string, { stderr: string; stdout: string }>();
const runtimeEntries = new Map<string, string>();
const conditionResults = new Map<string, { stderr: string; stdout: string }>();

const tsc = () =>
  join(dirname(require.resolve("typescript/package.json")), "bin/tsc");

beforeAll(async () => {
  packed = await createPackedPackage(
    new URL("../package.json", import.meta.url),
  );

  await Promise.all(
    typeModules.map(async (module) => {
      const source = join(packed.consumerDir, `consumer-${module}.ts`);

      await writeFile(
        source,
        `
import { correlationSpanProcessor, flush, onRequestError, register } from "@interfere/next/instrumentation";
import type { ServerInstrumentationOptions } from "@interfere/next/instrumentation";
import { registerOTel } from "@vercel/otel";
registerOTel({ serviceName: "user-dashboard", spanProcessors: [correlationSpanProcessor(), "auto"] });
const options: ServerInstrumentationOptions = {
  consoleBridge: false,
  serviceName: "user-dashboard",
  tracing: "auto",
  propagateTo: ["https://api.example.com"],
};
void register(options);
void flush();
void onRequestError;
`,
      );

      const result = await exec(
        process.execPath,
        [
          tsc(),
          "--ignoreConfig",
          "--noEmit",
          "--strict",
          "--skipLibCheck",
          "--target",
          "ES2022",
          "--module",
          module,
          "--moduleResolution",
          module === "NodeNext" ? "NodeNext" : "Bundler",
          source,
        ],
        { cwd: packed.consumerDir },
      );

      typeResults.set(module, { stderr: result.stderr, stdout: result.stdout });
    }),
  );

  await Promise.all(
    runtimeCases.map(async ({ condition }) => {
      const result = await exec(
        process.execPath,
        [
          `--conditions=${condition}`,
          "--input-type=module",
          "--eval",
          'console.log(import.meta.resolve("@interfere/next/instrumentation"))',
        ],
        { cwd: packed.consumerDir },
      );

      runtimeEntries.set(
        condition,
        basename(fileURLToPath(result.stdout.trim())),
      );
    }),
  );

  await Promise.all(
    typeConditions.map(async (condition) => {
      const source = join(packed.consumerDir, `consumer-${condition}.ts`);

      await writeFile(
        source,
        `
import { correlationSpanProcessor, register } from "@interfere/next/instrumentation";
import type { SpanProcessor } from "@interfere/server-core/otel/sdk/trace/processing";
const processor: SpanProcessor = correlationSpanProcessor();
void processor;
const pending: Promise<void> = register({ tracing: "application" });
void pending;
`,
      );

      const result = await exec(
        process.execPath,
        [
          tsc(),
          "--ignoreConfig",
          "--noEmit",
          "--strict",
          "--skipLibCheck",
          "--target",
          "ES2022",
          "--module",
          "ESNext",
          "--moduleResolution",
          "Bundler",
          "--customConditions",
          condition,
          source,
        ],
        { cwd: packed.consumerDir },
      );

      conditionResults.set(condition, {
        stderr: result.stderr,
        stdout: result.stdout,
      });
    }),
  );

  return async () => packed.cleanup();
});

test.each(typeModules)(
  "published instrumentation types support %s consumers",
  (module) => {
    const result = typeResults.get(module);

    expect(result?.stdout).toBe("");
    expect(result?.stderr).toBe("");
  },
);

test.each(runtimeCases)(
  "published runtime routing preserves the $condition entry",
  ({ condition, expectedEntry }) => {
    expect(runtimeEntries.get(condition)).toBe(expectedEntry);
  },
);

test.each(typeConditions)(
  "published types resolve register under %s",
  (condition) => {
    const result = conditionResults.get(condition);

    expect(result?.stdout).toBe("");
    expect(result?.stderr).toBe("");
  },
);
