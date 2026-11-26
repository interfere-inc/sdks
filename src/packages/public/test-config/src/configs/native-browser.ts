/* oxlint-disable eslint/no-console */
import { execFile } from "node:child_process";
import { mkdtemp, readFile, rm } from "node:fs/promises";
import { createServer } from "node:http";
import { arrayBuffer } from "node:stream/consumers";
import { getResponse } from "msw";
import type { RequestHandler } from "msw";
import { tmpdir } from "node:os";
import { basename, join } from "node:path";
import { fileURLToPath } from "node:url";
import { promisify } from "node:util";
import { chromium } from "playwright";
import type { Browser } from "playwright";
import { afterAll, beforeAll, onTestFinished } from "vitest";

const execute = promisify(execFile);

export function nativeBrowserFixture(
  entrypoint: URL,
  handlers: RequestHandler[] = [],
  launchOptions: Parameters<typeof chromium.launch>[0] = {},
) {
  let browser: Browser | undefined;
  let directory: string;
  let script: string;

  beforeAll(async () => {
    directory = await mkdtemp(join(tmpdir(), "native-browser-"));

    await execute("bun", [
      "build",
      fileURLToPath(entrypoint),
      "--target=browser",
      "--outdir",
      directory,
    ]);

    script = await readFile(
      join(directory, basename(entrypoint.pathname).replace(/\.ts$/u, ".js")),
      "utf8",
    );

    browser = await chromium.launch({
      ...launchOptions,
      channel: "chromium",
      ignoreDefaultArgs: ["--disable-back-forward-cache"],
    });
  }, 15_000);

  afterAll(async () => {
    await browser?.close();

    await rm(directory, { recursive: true, force: true });
  });

  return async () => {
    if (!browser) {
      throw new Error("Native browser has not started");
    }

    const errors: Error[] = [];

    const context = await browser.newContext();

    const page = await context.newPage();

    page.on("pageerror", (error) => console.error(error));

    page.on("requestfailed", (request) =>
      console.error(
        `requestfailed: ${request.url()} ${request.failure()?.errorText ?? ""}`,
      ),
    );

    const server = createServer((request, response) => {
      async function serve() {
        if (request.url === "/fixture.js") {
          response.setHeader("Content-Type", "text/javascript");

          response.end(script);

          return;
        }

        if (request.url === "/fixture" || request.url === "/other") {
          response.setHeader("Content-Type", "text/html");

          response.end(
            request.url === "/fixture"
              ? '<!doctype html><title>Native fixture</title><script type="module" src="/fixture.js"></script>'
              : "<!doctype html><title>Other document</title>",
          );

          return;
        }

        if (request.url === "/favicon.ico") {
          response.writeHead(204);

          response.end();

          return;
        }

        const headers = new Headers();

        for (let index = 0; index < request.rawHeaders.length; index += 2) {
          const name = request.rawHeaders[index];
          const value = request.rawHeaders[index + 1];

          if (name && value) {
            headers.append(name, value);
          }
        }

        const method = request.method ?? "GET";

        const resolved = await getResponse(
          handlers,
          new Request(`http://${request.headers.host}${request.url}`, {
            method,
            headers,
            body:
              method === "GET" || method === "HEAD"
                ? null
                : await arrayBuffer(request),
          }),
        );

        if (!resolved) {
          throw new Error(
            `Unhandled fixture request: ${method} ${request.url}`,
          );
        }

        response.writeHead(
          resolved.status,
          Object.fromEntries(resolved.headers),
        );

        response.end(Buffer.from(await resolved.arrayBuffer()));
      }

      async function handle() {
        try {
          await serve();
        } catch (error) {
          errors.push(
            error instanceof Error ? error : new Error(String(error)),
          );

          response.destroy();
        }
      }

      void handle();
    });

    onTestFinished(async () => {
      await context.close();

      server.closeAllConnections();

      await new Promise<void>((resolve, reject) => {
        server.close((error) => (error ? reject(error) : resolve()));
      });

      if (errors.length > 0) {
        throw new AggregateError(
          errors,
          "Native browser fixture network failed",
        );
      }
    });

    await new Promise<void>((resolve) => {
      server.listen(0, "127.0.0.1", resolve);
    });

    const address = server.address();

    // oxlint-disable-next-line anti-slop/no-runtime-typeof
    if (address === null || typeof address === "string") {
      throw new Error("Missing native fixture server address");
    }

    const origin = `http://127.0.0.1:${address.port}`;

    await page.goto(`${origin}/fixture`);

    return {
      page,
      async hide() {
        await page.goto(`${origin}/other`);
      },
      async show() {
        await page.goBack({ waitUntil: "commit" });
      },
    };
  };
}
