import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { once } from "node:events";
import { createServer } from "node:net";
import { setTimeout } from "node:timers/promises";

async function freePort() {
  const server = createServer();

  server.listen(0, "127.0.0.1");

  await once(server, "listening");

  const address = server.address();

  // oxlint-disable-next-line anti-slop/no-runtime-typeof
  assert.ok(address !== null && typeof address !== "string");

  server.close();

  await once(server, "close");

  return address.port;
}

async function ready(url: string, signal: AbortSignal) {
  while (!signal.aborted) {
    try {
      // oxlint-disable-next-line eslint/no-await-in-loop -- Readiness is polled sequentially until the server answers.
      const response = await fetch(`${url}/api/journey?mode=healthy`);

      // oxlint-disable-next-line eslint/no-await-in-loop -- Drain each probe before the next one.
      await response.text();

      if (response.ok) {
        return;
      }
    } catch {
      // oxlint-disable-next-line eslint/no-await-in-loop -- Back off between refused connections.
      await setTimeout(100);
    }
  }

  throw new Error("next did not become ready");
}

const command = process.env["NEXT_LAUNCH_COMMAND"] === "dev" ? "dev" : "start";

const port = await freePort();

const child = spawn(
  process.execPath,
  [
    "node_modules/next/dist/bin/next",
    command,
    ...(command === "dev" ? ["--turbopack"] : []),
    "--hostname",
    "127.0.0.1",
    "--port",
    String(port),
  ],
  {
    env: {
      ...process.env,
      NODE_ENV: command === "dev" ? "development" : "production",
    },
    stdio: "inherit",
  },
);

process.once("SIGTERM", () => {
  child.once("exit", () => process.exit(0));

  child.kill("SIGTERM");
});

const url = `http://127.0.0.1:${port}`;

await ready(url, AbortSignal.timeout(25_000));

process.send?.(url);
