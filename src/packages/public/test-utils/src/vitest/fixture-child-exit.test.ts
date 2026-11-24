import { spawn } from "node:child_process";
import type { ChildProcess } from "node:child_process";
import { once } from "node:events";
import { mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterAll, beforeAll, expect, test } from "vitest";
import { z } from "zod";
import { childArguments } from "./packed-application";

let directory: string;
let parent: ChildProcess;
let child: number;

function running() {
  try {
    process.kill(child, 0);

    return true;
  } catch {
    return false;
  }
}

beforeAll(async () => {
  directory = await mkdtemp(join(tmpdir(), "fixture-child-exit-"));

  await writeFile(
    join(directory, "server.cjs"),
    'require("node:http").createServer().listen(0, "127.0.0.1", () => process.send("listening"));',
  );

  await writeFile(
    join(directory, "parent.cjs"),
    'const child = require("node:child_process").spawn(process.execPath, JSON.parse(process.argv[2]), { stdio: ["ignore", "ignore", "ignore", "ipc"] }); child.on("message", () => process.stdout.write(String(child.pid)));',
  );

  const started = spawn(
    process.execPath,
    [
      join(directory, "parent.cjs"),
      JSON.stringify(childArguments(join(directory, "server.cjs"))),
    ],
    { stdio: ["ignore", "pipe", "inherit"] },
  );

  parent = started;

  const events: unknown[] = await once(started.stdout, "data");

  child = z.coerce.number().int().parse(String(events[0]));
});

afterAll(async () => {
  if (running()) {
    process.kill(child, "SIGKILL");
  }

  parent.kill("SIGKILL");

  await rm(directory, { recursive: true, force: true });
});

test("a started child exits when the process that started it is killed", async () => {
  expect(running()).toBe(true);

  parent.kill("SIGKILL");

  await expect.poll(running).toBe(false);
});
