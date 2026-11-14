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

beforeAll(async () => {
  packed = await createPackedPackage(
    new URL("../package.json", import.meta.url),
  );

  return async () => packed.cleanup();
});

test.each(["CommonJS", "ESNext", "NodeNext"])(
  "published instrumentation types support %s consumers",
  async (module) => {
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
        join(dirname(require.resolve("typescript/package.json")), "bin/tsc"),
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

    expect(result.stdout).toBe("");
    expect(result.stderr).toBe("");
  },
);

test.each([
  { condition: "node", expectedEntry: "instrumentation.mjs" },
  { condition: "edge-light", expectedEntry: "instrumentation.edge.mjs" },
  { condition: "workerd", expectedEntry: "instrumentation.edge.mjs" },
  { condition: "worker", expectedEntry: "instrumentation.edge.mjs" },
])(
  "published runtime routing preserves the $condition entry",
  async ({ condition, expectedEntry }) => {
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

    expect(basename(fileURLToPath(result.stdout.trim()))).toBe(expectedEntry);
  },
);

test.each(["edge-light", "workerd", "worker"])(
  "published types resolve register under %s",
  async (condition) => {
    const source = join(packed.consumerDir, `consumer-${condition}.ts`);

    await writeFile(
      source,
      `
import { correlationSpanProcessor, register } from "@interfere/next/instrumentation";
import type { SpanProcessor } from "@opentelemetry/sdk-trace-base";
const processor: SpanProcessor = correlationSpanProcessor();
void processor;
const pending: Promise<void> = register({ tracing: "application" });
void pending;
`,
    );

    const result = await exec(
      process.execPath,
      [
        join(dirname(require.resolve("typescript/package.json")), "bin/tsc"),
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

    expect(result.stdout).toBe("");
    expect(result.stderr).toBe("");
  },
);
