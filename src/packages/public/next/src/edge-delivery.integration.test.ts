import { execFile, spawn } from "node:child_process";
import { once } from "node:events";
import { createServer } from "node:http";
import { fileURLToPath } from "node:url";
import { promisify } from "node:util";
import { beforeAll, expect, test } from "vitest";

const execute = promisify(execFile);

interface EdgeDeliveryResult {
  output: string;
  errors: string;
  requests: string[];
  code: number | null;
  signal: string | null;
}

let result: EdgeDeliveryResult;

beforeAll(async () => {
  const requests: string[] = [];

  const server = createServer((request, response) => {
    let body = "";

    request.setEncoding("utf8");

    request.on("data", (chunk: string) => {
      body += chunk;
    });

    request.on("end", () => {
      requests.push(body);

      response.writeHead(204);

      response.end();
    });
  });

  server.listen(0, "127.0.0.1");

  await once(server, "listening");

  try {
    const address = server.address();

    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Node server.address returns an AddressInfo, a pipe name, or null; this listener uses TCP.
    if (address === null || typeof address === "string") {
      throw new Error("Expected a local TCP collector");
    }

    const environment = {
      INTERFERE_PUBLIC_KEY: "interfere_pk_test",
      INTERFERE_API_URL: `http://127.0.0.1:${address.port}`,
      INTERFERE_ENVIRONMENT: "preview",
      NEXT_PUBLIC_INTERFERE_FORCE_ENABLE: "1",
      INTERFERE_SOURCE_ID: "a".repeat(40),
    };

    const bundle = await execute(
      "bun",
      [
        "build",
        fileURLToPath(
          new URL("../test/edge-delivery.entry.ts", import.meta.url),
        ),
        "--target=browser",
        "--format=iife",
        "--conditions=edge-light",
        "--conditions=browser",
        "--conditions=monorepo",
        "--define",
        `process.env=${JSON.stringify(environment)}`,
      ],
      { maxBuffer: 10 * 1024 * 1024 },
    );

    const child = spawn(
      process.execPath,
      [fileURLToPath(new URL("../test/edge-delivery-vm.ts", import.meta.url))],
      {
        stdio: ["pipe", "pipe", "pipe"],
        timeout: 5000,
      },
    );

    let output = "";
    let errors = "";

    child.stdout.setEncoding("utf8");

    child.stderr.setEncoding("utf8");

    child.stdout.on("data", (chunk: string) => {
      output += chunk;
    });

    child.stderr.on("data", (chunk: string) => {
      errors += chunk;
    });

    child.stdin.end(bundle.stdout);

    await once(child, "close");

    result = {
      output,
      errors,
      requests,
      code: child.exitCode,
      signal: child.signalCode,
    };
  } finally {
    server.closeAllConnections();

    await new Promise<void>((resolve, reject) => {
      server.close((error) => {
        if (error) {
          reject(error);
        } else {
          resolve();
        }
      });
    });
  }
});

test("exports an exception and exits naturally in an isolated Edge VM without AbortSignal.any", () => {
  expect(JSON.parse(result.output)).toStrictEqual([
    "undefined",
    "undefined",
    "undefined",
    "undefined",
  ]);

  expect(result.requests, result.errors).toHaveLength(1);
  expect(result.requests[0]).toContain("isolated-edge-export");
  expect(result.errors).toBe("");
  expect(result.signal).toBeNull();
  expect(result.code).toBe(0);
});
