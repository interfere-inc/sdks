import { execFile, spawn } from "node:child_process";
import { once } from "node:events";
import { cp, mkdir, mkdtemp, rm, symlink } from "node:fs/promises";
import { mswServer } from "@interfere/test-config/msw-server";
import { HttpResponse, http } from "msw";
import { createRequire } from "node:module";
import path from "node:path";
import { promisify } from "node:util";

const shutdown = new AbortController();

const stop = () => shutdown.abort();

const api = mswServer(
  http.all("*", () => new HttpResponse(null, { status: 500 })),
);

process.once("SIGTERM", stop);

process.once("SIGINT", stop);

let project: string | undefined;

try {
  api.listen({ port: 0, host: "127.0.0.1", signal: shutdown.signal });

  await once(api, "listening", { signal: shutdown.signal });

  const address = api.address();

  // oxlint-disable-next-line anti-slop/no-runtime-typeof
  if (address === null || typeof address === "string") {
    throw new Error("API did not bind");
  }

  project = await mkdtemp(path.resolve("../.native-next-"));

  const require = createRequire(import.meta.url);
  const next = require.resolve("next/dist/bin/next");

  const env = {
    HOME: process.env["HOME"],
    INTERFERE_API_KEY: "interfere_secret_us_e2e_placeholder",
    INTERFERE_API_URL: `http://127.0.0.1:${address.port}`,
    INTERFERE_ENVIRONMENT: "preview",
    INTERFERE_SOURCE_ID: "abc123def4567890abc123def4567890abc123de",
    NEXT_TELEMETRY_DISABLED: "1",
    NODE_ENV: "production",
    PATH: process.env["PATH"],
    TMPDIR: process.env["TMPDIR"],
  } as const;

  shutdown.signal.throwIfAborted();

  await cp("test/next-fixture", project, { recursive: true });

  await mkdir(path.join(project, "node_modules", "@interfere"), {
    recursive: true,
  });

  await symlink(
    process.cwd(),
    path.join(project, "node_modules", "@interfere", "next"),
    "dir",
  );

  const modules = path.join(project, "node_modules");

  await Promise.all(
    ["next", "react", "react-dom"].map(async (dependency) =>
      symlink(
        path.dirname(require.resolve(`${dependency}/package.json`)),
        path.join(modules, dependency),
        "dir",
      ),
    ),
  );

  const building = promisify(execFile)(
    "node",
    [next, "build", project, "--turbopack"],
    {
      cwd: project,
      env,
      maxBuffer: 10 * 1024 * 1024,
      timeout: 150_000,
      signal: shutdown.signal,
    },
  );

  const buildClosed = new Promise<void>((resolve) => {
    building.child.once("close", () => resolve());
  });

  try {
    const { stdout, stderr } = await building;

    process.stdout.write(stdout + stderr);
  } finally {
    await buildClosed;
  }

  shutdown.signal.throwIfAborted();

  const app = spawn(
    "node",
    [
      next,
      "start",
      project,
      "--hostname",
      "127.0.0.1",
      "--port",
      process.env["SDK_BUILD_TEST_PORT"] ?? "3037",
    ],
    { env, stdio: "inherit", signal: shutdown.signal },
  );

  const appClosed = new Promise<void>((resolve) => {
    app.once("close", () => resolve());
  });

  try {
    const exited: unknown[] = await once(app, "exit");

    const [code] = exited;

    if (!shutdown.signal.aborted && code !== 0) {
      throw new Error(`Next exited with ${String(code)}`);
    }
  } finally {
    await appClosed;
  }
} catch (error) {
  if (
    !(
      shutdown.signal.aborted &&
      error instanceof Error &&
      error.name === "AbortError"
    )
  ) {
    throw error;
  }
} finally {
  process.off("SIGTERM", stop);

  process.off("SIGINT", stop);

  api.closeAllConnections();

  api.close();

  if (project) {
    await rm(project, { force: true, recursive: true });
  }
}
