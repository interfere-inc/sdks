/* oxlint-disable typescript/require-await */
import { mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { runInNewContext } from "node:vm";
import { build } from "vite";
import { beforeAll, expect, test } from "vitest";

const PUBLIC_KEY = "interfere_pub_us_test_123";

const GLOBAL_NAME = "interfereServer";

interface ServerEntry {
  captureError: (error: Error) => void;
  isEnabledOnServer: () => boolean;
  readInterfereServerEnv: () => {
    apiKey: string | null;
    nodeEnvironment: string;
    publicKey: string | null;
  };
  traceMeta: () => unknown[];
  withRequestSpan: <T>(request: Request, next: () => Promise<T>) => Promise<T>;
}

let server: ServerEntry;

async function bundleWithProcessReads() {
  const directory = await mkdtemp(path.join(tmpdir(), "interfere-workerd-"));

  const entry = path.join(directory, "entry.ts");

  await writeFile(
    entry,
    ["env", "capture", "request/trace"]
      .map(
        (name) =>
          `export * from ${JSON.stringify(path.join(import.meta.dirname, `${name}.ts`))};`,
      )
      .join("\n"),
  );

  try {
    const output = await build({
      configFile: false,
      define: {
        "import.meta.env.VITE_INTERFERE_PUBLIC_KEY": JSON.stringify(PUBLIC_KEY),
      },
      logLevel: "silent",
      plugins: [
        {
          name: "workers-platform-module",
          resolveId: (id) => (id === "cloudflare:workers" ? `\0${id}` : null),
          load: (id) =>
            id === "\0cloudflare:workers"
              ? "export function waitUntil(promise) { promise.catch(() => {}); }"
              : null,
        },
      ],
      root: path.join(import.meta.dirname, "../../.."),
      ssr: {
        noExternal: true,
        resolve: { conditions: ["workerd", "worker"] },
      },
      build: {
        minify: false,
        rolldownOptions: { output: { format: "iife", name: GLOBAL_NAME } },
        ssr: entry,
        write: false,
      },
    });

    const results = Array.isArray(output) ? output : [output];

    return results
      .flatMap((result) => ("output" in result ? result.output : []))
      .flatMap((file) =>
        file.type === "chunk" && file.isEntry ? [file.code] : [],
      )
      .join("\n");
  } finally {
    await rm(directory, { force: true, recursive: true });
  }
}

beforeAll(async () => {
  const code = await bundleWithProcessReads();

  server = runInNewContext(`${code}\n${GLOBAL_NAME}`, {
    AbortController,
    Headers,
    ReadableStream,
    Request,
    Response,
    TextDecoder,
    TextEncoder,
    URL,
    clearTimeout,
    console,
    crypto,
    fetch,
    performance,
    queueMicrotask,
    setTimeout,
  }) as ServerEntry;
}, 60_000);

test("the bundled server entry runs on a runtime with no process global", async () => {
  expect(runInNewContext("typeof process", {})).toBe("undefined");

  expect(server.readInterfereServerEnv().publicKey).toBe(PUBLIC_KEY);
  expect(server.readInterfereServerEnv().apiKey).toBeNull();
  expect(server.readInterfereServerEnv().nodeEnvironment).toBe("production");
  expect(server.isEnabledOnServer()).toBe(true);
  expect(server.traceMeta()).toHaveLength(0);

  expect(() => {
    server.captureError(new Error("no process"));
  }).not.toThrow();

  await expect(
    server.withRequestSpan(
      new Request("https://app.test/page"),
      async () => "ok",
    ),
  ).resolves.toBe("ok");
});
