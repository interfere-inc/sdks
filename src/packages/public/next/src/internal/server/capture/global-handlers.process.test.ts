import { execFile, spawn } from "node:child_process";
import type { ChildProcessWithoutNullStreams } from "node:child_process";
import { mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { createInterface } from "node:readline";
import { fileURLToPath } from "node:url";
import { promisify } from "node:util";
import { afterAll, beforeAll, describe, expect, test } from "vitest";

const execute = promisify(execFile);

const modes = [
  "before-exit",
  "broken-flush",
  "host-listener",
  "rejection",
  "sigterm",
  "sigterm-host",
  "sigterm-stuck",
  "uncaught",
] as const;

type HandlerMode = (typeof modes)[number];

interface HandlerRun {
  readonly child: ChildProcessWithoutNullStreams;
  readonly result: Promise<{
    code: number | null;
    errors: string;
    exitsBeforeFlush: (number | null)[];
    lines: string[];
    signal: NodeJS.Signals | null;
  }>;
}

let fixtureDirectory: string;
const runs = new Map<HandlerMode, HandlerRun>();

function startHandlerProcess(fixturePath: string, mode: HandlerMode) {
  const child = spawn(process.execPath, [fixturePath, mode], {
    env: {
      ...process.env,
      INTERFERE_PUBLIC_KEY: "interfere_pk_test_123",
      NEXT_PUBLIC_INTERFERE_FORCE_ENABLE: "1",
    },
    stdio: "pipe",
  });

  const exited = new Promise<{
    code: number | null;
    signal: NodeJS.Signals | null;
  }>((resolve, reject) => {
    child.once("exit", (code, signal) => resolve({ code, signal }));

    child.once("error", reject);
  });

  const exitsBeforeFlush: (number | null)[] = [];
  const lines: string[] = [];
  const errors: string[] = [];

  child.stdin.on("error", () => {
    errors.push("stdin closed");
  });

  child.stderr.setEncoding("utf8");

  child.stderr.on("data", (data: string) => errors.push(data));

  const result = (async () => {
    for await (const line of createInterface({ input: child.stdout })) {
      lines.push(line);

      if (line === "flush:waiting") {
        exitsBeforeFlush.push(child.exitCode);

        child.stdin.end("release\n");
      }
    }

    const { code, signal } = await exited;

    return { code, errors: errors.join(""), exitsBeforeFlush, lines, signal };
  })();

  return { child, result } satisfies HandlerRun;
}

beforeAll(async () => {
  fixtureDirectory = await mkdtemp(join(tmpdir(), "next-global-handler-"));

  const fixturePath = join(fixtureDirectory, "handler.cjs");

  await execute("bun", [
    "build",
    fileURLToPath(
      new URL(
        "../../../../test/fixtures/global-handler-process.ts",
        import.meta.url,
      ),
    ),
    "--target=node",
    "--conditions=monorepo",
    "--format=cjs",
    `--outfile=${fixturePath}`,
  ]);

  for (const mode of modes) {
    runs.set(mode, startHandlerProcess(fixturePath, mode));
  }
});

afterAll(async () => {
  for (const run of runs.values()) {
    run.child.kill("SIGKILL");
  }

  await rm(fixtureDirectory, { recursive: true, force: true });
});

async function outcome(mode: HandlerMode) {
  const run = runs.get(mode);

  if (!run) {
    throw new Error(`Handler process ${mode} did not start`);
  }

  return run.result;
}

describe("global handlers in a real process", () => {
  test("exits with code 1 only after the supplied flush completes when no other listener exists", async () => {
    expect.hasAssertions();

    await expect(outcome("uncaught")).resolves.toStrictEqual({
      code: 1,
      errors: "",
      exitsBeforeFlush: [null],
      lines: ["flush:started", "flush:waiting", "flush:done"],
      signal: null,
    });
  });

  test("still exits with code 1 when flush throws synchronously", async () => {
    expect.hasAssertions();

    const { errors, ...rest } = await outcome("broken-flush");

    expect(errors).toMatch(/^\[interfere\] Telemetry flush failed/u);

    expect(rest).toStrictEqual({
      code: 1,
      exitsBeforeFlush: [],
      lines: ["flush:started"],
      signal: null,
    });
  });

  test("does not exit on a native unhandled rejection", async () => {
    expect.hasAssertions();

    await expect(outcome("rejection")).resolves.toStrictEqual({
      code: 0,
      errors: "",
      exitsBeforeFlush: [null],
      lines: ["flush:started", "flush:waiting", "flush:done", "alive:1"],
      signal: null,
    });
  });

  test("keeps the process serving when the host has its own uncaughtException listener", async () => {
    expect.hasAssertions();

    await expect(outcome("host-listener")).resolves.toStrictEqual({
      code: 0,
      errors: "",
      exitsBeforeFlush: [null],
      lines: [
        "host:logged",
        "flush:started",
        "flush:waiting",
        "flush:done",
        "alive:1",
      ],
      signal: null,
    });
  });

  test("flushes once when the event loop drains", async () => {
    expect.hasAssertions();

    await expect(outcome("before-exit")).resolves.toStrictEqual({
      code: 0,
      errors: "",
      exitsBeforeFlush: [null],
      lines: ["flush:started", "flush:waiting", "flush:done", "alive:0"],
      signal: null,
    });
  });

  test("flushes on SIGTERM and then terminates by that signal", async () => {
    expect.hasAssertions();

    await expect(outcome("sigterm")).resolves.toStrictEqual({
      code: null,
      errors: "",
      exitsBeforeFlush: [null],
      lines: ["flush:started", "flush:waiting", "flush:done"],
      signal: "SIGTERM",
    });
  });

  test("terminates by SIGTERM when the shutdown flush never settles", async () => {
    expect.hasAssertions();

    await expect(outcome("sigterm-stuck")).resolves.toStrictEqual({
      code: null,
      errors: "",
      exitsBeforeFlush: [],
      lines: ["flush:started"],
      signal: "SIGTERM",
    });
  });

  test("leaves SIGTERM shutdown to a host that handles it", async () => {
    expect.hasAssertions();

    const { code, lines, signal } = await outcome("sigterm-host");

    expect({ code, lines, signal }).toStrictEqual({
      code: 0,
      lines: ["host:shutdown", "flush:started", "flush:waiting", "host:exit"],
      signal: null,
    });
  });
});
