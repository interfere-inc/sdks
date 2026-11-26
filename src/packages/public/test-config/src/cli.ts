#!/usr/bin/env node
import { execFileSync, spawn } from "node:child_process";
import { once } from "node:events";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, relative, resolve } from "node:path";
import { z } from "zod";

import { selection } from "./selection.ts";

if (process.env["TESTCONTAINERS_RYUK_DISABLED"] === "true") {
  throw new Error(
    "Testcontainers cleanup is disabled. Unset TESTCONTAINERS_RYUK_DISABLED before running tests so Ryuk can remove containers and host-port forwarders after the test process exits.",
  );
}

const packageArgs = (variable: string) => {
  const { name } = z
    .object({ name: z.string() })
    .parse(JSON.parse(readFileSync(resolve("package.json"), "utf8")));

  return (
    z.record(z.string(), z.array(z.string())).parse(JSON.parse(variable))[
      name
    ] ?? []
  );
};

const packageVariable = process.env["VITEST_PACKAGE_ARGS"];

const suiteArgs = [
  ...z.array(z.string()).parse(JSON.parse(process.env["VITEST_ARGS"] ?? "[]")),
  ...(packageVariable === undefined ? [] : packageArgs(packageVariable)),
];

const selectionFile = process.env["VITEST_SELECTION"];

const git = (...gitArgs: string[]) =>
  execFileSync("git", gitArgs, {
    encoding: "utf8",
    maxBuffer: 64 * 1024 * 1024,
  });

const selected = (file: string) => {
  const repository = git("rev-parse", "--show-toplevel").trim();

  return selection(
    suiteArgs,
    readFileSync(file, "utf8")
      .split("\n")
      .filter((entry) => entry.length > 0),
    repository,
    relative(repository, process.cwd()),
    (directory) =>
      git("-C", repository, "ls-files", "-z", "--", directory)
        .split("\0")
        .filter((path) => path.length > 0),
  );
};

const args = selectionFile === undefined ? suiteArgs : selected(selectionFile);

const run = async (vitestArgs: string[]) => {
  const argv = process.argv.slice(2);

  const packageWorkers = argv.some(
    (argument) =>
      argument === "--maxWorkers" || argument.startsWith("--maxWorkers="),
  );

  const manifest = createRequire(resolve("package.json")).resolve(
    "vitest/package.json",
  );

  // WARNING: Vitest must lead its own process group and the group must die with it; a workerd runtime or fixture server left by a crashed run keeps the task's stdout and stderr open, and Turbo waits on them until the CI job timeout.
  const vitest = spawn(
    process.execPath,
    [
      ...process.execArgv,
      resolve(dirname(manifest), "vitest.mjs"),
      ...argv,
      ...vitestArgs.filter(
        (argument) => !(packageWorkers && argument.startsWith("--maxWorkers")),
      ),
    ],
    { detached: true, stdio: "inherit" },
  );

  await once(vitest, "spawn");

  const group = z.number().parse(vitest.pid);

  const signalGroup = (signal: NodeJS.Signals) => {
    try {
      process.kill(-group, signal);
    } catch (error) {
      if (z.object({ code: z.literal("ESRCH") }).safeParse(error).success) {
        return;
      }

      throw error;
    }
  };

  const forwarded = ["SIGHUP", "SIGINT", "SIGTERM"] as const;

  for (const signal of forwarded) {
    process.on(signal, signalGroup);
  }

  const [code, signal] = z
    .tuple([z.number().nullable(), z.string().nullable()])
    .parse(await once(vitest, "exit"));

  signalGroup("SIGKILL");

  for (const forwardedSignal of forwarded) {
    process.off(forwardedSignal, signalGroup);
  }

  process.exitCode = code ?? 1;

  if (signal !== null) {
    process.kill(process.pid, signal);
  }
};

if (args === undefined) {
  process.stdout.write(
    "The selected test files include none for this package and suite.\n",
  );
} else {
  await run(args);
}
