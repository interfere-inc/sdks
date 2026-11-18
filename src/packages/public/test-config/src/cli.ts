import { spawn } from "node:child_process";
import { once } from "node:events";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, resolve } from "node:path";
import { z } from "zod";

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

const args = [
  ...z.array(z.string()).parse(JSON.parse(process.env["VITEST_ARGS"] ?? "[]")),
  ...(packageVariable === undefined ? [] : packageArgs(packageVariable)),
];

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
    ...args.filter(
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
