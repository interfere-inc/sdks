import {
  mkdirSync,
  mkdtempSync,
  readFileSync,
  rmSync,
  symlinkSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { spawn } from "node:child_process";
import { randomUUID } from "node:crypto";
import { text } from "node:stream/consumers";
import { fileURLToPath } from "node:url";
import { afterAll, beforeAll, describe, expect, test, vi } from "vitest";

const directory = mkdtempSync(join(tmpdir(), "vitest-args-"));
const cli = fileURLToPath(new URL("cli.ts", import.meta.url));
const config = join(directory, "vitest.config.mts");

beforeAll(async () => {
  const selection = join(directory, "empty-selection.txt");

  writeFileSync(selection, "");

  const ryuk = process.env["TESTCONTAINERS_RYUK_DISABLED"];

  delete process.env["TESTCONTAINERS_RYUK_DISABLED"];

  vi.stubEnv("VITEST_SELECTION", selection);

  vi.stubEnv("VITEST_ARGS", "[]");

  try {
    await import("./cli.ts");
  } finally {
    vi.unstubAllEnvs();

    vi.resetModules();

    if (ryuk === undefined) {
      delete process.env["TESTCONTAINERS_RYUK_DISABLED"];
    } else {
      process.env["TESTCONTAINERS_RYUK_DISABLED"] = ryuk;
    }
  }

  delete process.env["VITEST_SELECTION"];

  mkdirSync(join(directory, "node_modules"));

  symlinkSync(
    fileURLToPath(new URL(".", import.meta.resolve("vitest/package.json"))),
    join(directory, "node_modules/vitest"),
  );

  writeFileSync(
    config,
    `export default { test: { globals: true, include: ["*.test.ts"] } };`,
  );

  for (const name of ["first", "second", "third", "fourth"]) {
    writeFileSync(
      join(directory, `${name}.test.ts`),
      `test("${name}", () => expect(true).toBe(true));`,
    );
  }
});

afterAll(() => rmSync(directory, { recursive: true, force: true }));

async function run(args: string[], environmentArgs: string) {
  const output = join(directory, `${randomUUID()}.json`);

  const child = spawn(
    process.execPath,
    [cli, ...args, "--config", config, "--outputFile", output],
    {
      cwd: directory,
      env: { ...process.env, VITEST_ARGS: environmentArgs },
      stdio: ["ignore", "pipe", "pipe"],
    },
  );

  try {
    const closed = new Promise<number | null>((resolve) => {
      child.once("close", resolve);
    });

    const [stdout, stderr] = await Promise.all([
      text(child.stdout),
      text(child.stderr),
    ]);

    const status = await closed;

    return {
      status,
      stderr,
      stdout: status === 0 ? readFileSync(output, "utf8") : stdout,
    };
  } finally {
    child.kill("SIGKILL");
  }
}

const files = (output: string) =>
  (JSON.parse(output) as { testResults: { name: string }[] }).testResults.map(
    (result) => result.name,
  );

const json = ["run", "--reporter=json", "--maxWorkers=1"];

describe.each([
  { name: "native filename filters", args: '["first.test.ts"]', count: 1 },
  {
    name: "environment exclusions",
    args: '["--exclude=**/first.test.ts"]',
    count: 3,
  },
])("CLI $name", ({ name, args, count }) => {
  let result: Awaited<ReturnType<typeof run>>;

  beforeAll(async () => {
    result = await run(json, args);
  });

  test("selects the expected files", () => {
    expect(result.status).toBe(0);
    expect(files(result.stdout)).toHaveLength(count);

    expect(
      files(result.stdout).some((file) => file.endsWith("first.test.ts")),
    ).toBe(name === "native filename filters");
  });
});

describe.each([
  { args: '["--unknown-test-option"]', expectedError: /Unknown option/u },
  { args: "{}", expectedError: /./u },
  { args: "[1]", expectedError: /./u },
  { args: "not-json", expectedError: /./u },
])("invalid environment arguments $args", ({ args, expectedError }) => {
  let result: Awaited<ReturnType<typeof run>>;

  beforeAll(async () => {
    result = await run(["run"], args);
  });

  test("preserves the nonzero exit", () => {
    expect(result.status).not.toBe(0);

    expect(result.stderr).toMatch(expectedError);
  });
});
