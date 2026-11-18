import {
  existsSync,
  mkdirSync,
  mkdtempSync,
  readFileSync,
  rmSync,
  symlinkSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { spawn, spawnSync } from "node:child_process";
import type { ChildProcess } from "node:child_process";
import { randomUUID } from "node:crypto";
import { once } from "node:events";
import { text } from "node:stream/consumers";
import { fileURLToPath } from "node:url";
import { afterAll, beforeAll, describe, expect, test, vi } from "vitest";

const directory = mkdtempSync(join(tmpdir(), "vitest-args-"));
const cli = fileURLToPath(new URL("cli.ts", import.meta.url));
const config = join(directory, "vitest.config.mts");

beforeAll(() => {
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

describe("disabled container cleanup", () => {
  let result: ReturnType<typeof spawnSync>;

  beforeAll(() => {
    result = spawnSync(process.execPath, [cli, "run"], {
      cwd: directory,
      env: {
        ...process.env,
        TESTCONTAINERS_RYUK_DISABLED: "true",
        VITEST_ARGS: "not-json",
      },
      encoding: "utf8",
      timeout: 30_000,
    });
  });

  test("rejects it before starting Vitest", () => {
    expect(result.status).toBe(1);
    expect(result.stderr).toContain("Unset TESTCONTAINERS_RYUK_DISABLED");
    expect(result.stdout).toBe("");
  });
});

test.each([
  { environment: undefined, script: [], appended: [] },
  {
    environment: '["--exclude=**/*.browser.test.*"]',
    packageArgs:
      '{"@fixture/local":["--shard=2/8"],"@fixture/other":["--shard=1/8"]}',
    script: [],
    appended: ["--exclude=**/*.browser.test.*", "--shard=2/8"],
  },
  {
    environment: '["--exclude=**/*.browser.test.*"]',
    packageArgs: '{"@fixture/other":["--shard=1/8"]}',
    script: [],
    appended: ["--exclude=**/*.browser.test.*"],
  },
  {
    environment: '["--exclude=**/*.browser.test.*","filename with spaces"]',
    script: [],
    appended: ["--exclude=**/*.browser.test.*", "filename with spaces"],
  },
  {
    environment: '["--exclude=**/*.browser.test.*","--maxWorkers=4"]',
    script: [],
    appended: ["--exclude=**/*.browser.test.*", "--maxWorkers=4"],
  },
  {
    environment: '["--exclude=**/*.browser.test.*","--maxWorkers=4"]',
    script: ["--maxWorkers=1"],
    appended: ["--exclude=**/*.browser.test.*"],
  },
])(
  "passes environment arguments to the package executable: $environment $packageArgs",
  async ({ environment, packageArgs, script, appended }) => {
    const local = mkdtempSync(join(directory, "instrumented-"));
    const installation = join(local, "node_modules/vitest");
    const output = join(local, "arguments.json");

    mkdirSync(installation, { recursive: true });

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

    const originalArgv = process.argv;
    const originalDirectory = process.cwd();

    writeFileSync(
      join(local, "package.json"),
      JSON.stringify({ name: "@fixture/local" }),
    );

    vi.stubEnv("VITEST_ARGS", environment);

    vi.stubEnv("VITEST_PACKAGE_ARGS", packageArgs);

    vi.resetModules();

    process.argv = [process.execPath, cli, "run", "local.test.ts", ...script];

    try {
      process.chdir(local);

      await import("./cli.ts");

      expect(JSON.parse(readFileSync(output, "utf8"))).toStrictEqual([
        "run",
        "local.test.ts",
        ...script,
        ...appended,
      ]);
    } finally {
      process.argv = originalArgv;

      process.chdir(originalDirectory);

      vi.unstubAllEnvs();

      vi.resetModules();
    }
  },
);

describe("runs the package's own Vitest executable", () => {
  let result: ReturnType<typeof spawnSync>;

  beforeAll(() => {
    const local = join(directory, "local");
    const installation = join(local, "node_modules/vitest");

    mkdirSync(installation, { recursive: true });

    writeFileSync(
      join(installation, "package.json"),
      JSON.stringify({
        name: "vitest",
        exports: { "./package.json": "./package.json" },
      }),
    );

    writeFileSync(
      join(installation, "vitest.mjs"),
      "process.stdout.write(JSON.stringify(process.argv.slice(2)));",
    );

    result = spawnSync(process.execPath, [cli, "run", "local.test.ts"], {
      cwd: local,
      env: { ...process.env, VITEST_ARGS: '["--exclude=**/*.browser.test.*"]' },
      encoding: "utf8",
      timeout: 30_000,
    });
  });

  test("preserves the executable output", () => {
    expect(result.status).toBe(0);

    expect(JSON.parse(String(result.stdout))).toStrictEqual([
      "run",
      "local.test.ts",
      "--exclude=**/*.browser.test.*",
    ]);
  });
});

describe("closes its output when a crashed Vitest leaves a descendant holding it", () => {
  const local = join(directory, "crashing");
  const descendant = join(local, "descendant.pid");

  let child: ChildProcess | undefined;
  let closed: unknown[];
  let stderr: string;

  beforeAll(async () => {
    const installation = join(local, "node_modules/vitest");

    mkdirSync(installation, { recursive: true });

    writeFileSync(
      join(installation, "package.json"),
      JSON.stringify({
        name: "vitest",
        exports: { "./package.json": "./package.json" },
      }),
    );

    writeFileSync(
      join(installation, "vitest.mjs"),
      `import { spawn } from "node:child_process";
import { writeFileSync } from "node:fs";
const child = spawn(process.execPath, ["-e", "require('node:net').createServer().listen(0)"], { stdio: "inherit" });
writeFileSync(${JSON.stringify(descendant)}, String(child.pid));
throw new Error("vitest crashed");`,
    );

    const cliProcess = spawn(process.execPath, [cli, "run"], {
      cwd: local,
      stdio: ["ignore", "pipe", "pipe"],
    });

    child = cliProcess;

    const exited = once(cliProcess, "close");

    [, stderr] = await Promise.all([
      text(cliProcess.stdout),
      text(cliProcess.stderr),
    ]);

    closed = await exited;
  });

  afterAll(() => {
    child?.kill("SIGKILL");

    if (existsSync(descendant)) {
      spawnSync("kill", ["-9", readFileSync(descendant, "utf8")]);
    }
  });

  test("ends with the crashed Vitest exit status", () => {
    expect(closed).toStrictEqual([1, null]);

    expect(stderr).toContain("vitest crashed");
  });
});

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

describe("CLI shard selection", () => {
  let results: Awaited<ReturnType<typeof run>>[];

  beforeAll(async () => {
    results = await Promise.all([
      run(json, "[]"),
      run(json, '["--shard=1/2"]'),
      run(json, '["--shard=2/2"]'),
    ]);
  });

  test("keeps two shards disjoint and exhaustive", () => {
    expect.hasAssertions();

    for (const result of results) {
      expect(result.stderr).toBe("");
      expect(result.status).toBe(0);
    }

    const [all, first, second] = results.map((result) => files(result.stdout));

    expect(first?.length).toBeGreaterThan(0);
    expect(second?.length).toBeGreaterThan(0);
    expect(first?.filter((file) => second?.includes(file))).toStrictEqual([]);

    expect([...(first ?? []), ...(second ?? [])].toSorted()).toStrictEqual(
      all?.toSorted(),
    );
  });
});

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
