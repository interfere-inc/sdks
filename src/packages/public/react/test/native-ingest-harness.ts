import { arrayBuffer } from "node:stream/consumers";
import { omitUndefined } from "@interfere/helpers/omit-undefined";
/* oxlint-disable typescript/require-await */
import { execFile } from "node:child_process";
import { mkdtemp, readFile, rm } from "node:fs/promises";
import { createServer } from "node:http";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { promisify } from "node:util";
import { HttpResponse, getResponse, http } from "msw";
import { chromium } from "playwright";
import type { Browser, Page } from "playwright";
import { createCountSignal } from "./fixtures/upload-signal.js";
import { afterAll, beforeAll, expect, onTestFinished } from "vitest";

const execute = promisify(execFile);

let browser: Browser;
let directory: string;
let worker: string;
let pageScript: string;
let carryoverScript: string;
let replayScript: string;
let unloadScript: string;

beforeAll(async () => {
  directory = await mkdtemp(join(tmpdir(), "ingest-queue-"));

  await execute("bun", [
    "build",
    fileURLToPath(new URL("sw-queue-worker.fixture.ts", import.meta.url)),
    fileURLToPath(new URL("sw-queue-page.fixture.ts", import.meta.url)),
    fileURLToPath(new URL("export-carryover.fixture.ts", import.meta.url)),
    fileURLToPath(new URL("replay-carryover.fixture.ts", import.meta.url)),
    fileURLToPath(new URL("unload-export.fixture.ts", import.meta.url)),
    "--target=browser",
    "--outdir",
    directory,
  ]);

  [worker, pageScript, carryoverScript, replayScript, unloadScript] =
    await Promise.all([
      readFile(join(directory, "sw-queue-worker.fixture.js"), "utf8"),
      readFile(join(directory, "sw-queue-page.fixture.js"), "utf8"),
      readFile(join(directory, "export-carryover.fixture.js"), "utf8"),
      readFile(join(directory, "replay-carryover.fixture.js"), "utf8"),
      readFile(join(directory, "unload-export.fixture.js"), "utf8"),
    ]);

  browser = await chromium.launch();
}, 15_000);

afterAll(async () => {
  await browser.close();

  await rm(directory, { recursive: true, force: true });
});

export async function createHarness() {
  const requested: string[] = [];
  const received: { path: string; body: Buffer; headers: Headers }[] = [];
  const requestedSignal = createCountSignal(requested);

  let respond: () => Response | Promise<Response> = async () =>
    new HttpResponse(null, { status: 503 });

  const arrived = Promise.withResolvers<undefined>();

  const handlers = [
    http.all("*", async ({ request }) => {
      const path = new URL(request.url).pathname;

      if (path === "/replay-carryover") {
        return HttpResponse.html(
          '<script type="module" src="/replay-carryover.js"></script>',
        );
      }

      if (path === "/replay-carryover.js") {
        return new HttpResponse(replayScript, {
          headers: { "content-type": "text/javascript" },
        });
      }

      if (path === "/unload-export") {
        return HttpResponse.html(
          '<!doctype html><script type="module" src="/unload-export.js"></script>',
        );
      }

      if (path === "/unload-export.js") {
        return new HttpResponse(unloadScript, {
          headers: { "content-type": "text/javascript" },
        });
      }

      if (path === "/carryover") {
        return HttpResponse.html(
          '<!doctype html><script type="module" src="/carryover.js"></script>',
        );
      }

      if (path === "/carryover.js") {
        return new HttpResponse(carryoverScript, {
          headers: { "content-type": "text/javascript" },
        });
      }

      if (path === "/") {
        return HttpResponse.html(
          '<!doctype html><title>Queue runtime</title><script type="module" src="/page.js"></script>',
        );
      }

      if (path === "/worker.js" || path === "/page.js") {
        return new HttpResponse(path === "/worker.js" ? worker : pageScript, {
          headers: { "content-type": "text/javascript" },
        });
      }

      received.push({
        path,
        body: Buffer.from(await request.arrayBuffer()),
        headers: request.headers,
      });

      requested.push(path);

      requestedSignal.notify();

      arrived.resolve(undefined);

      return respond();
    }),
  ];

  const server = createServer((request, response) => {
    async function serve() {
      const body = await arrayBuffer(request);

      const headers = new Headers();

      for (const [name, value] of Object.entries(request.headers)) {
        if (value !== undefined) {
          headers.set(name, Array.isArray(value) ? value.join(",") : value);
        }
      }

      const resolved = await getResponse(
        handlers,
        new Request(
          `http://${request.headers.host}${request.url}`,
          omitUndefined({
            method: request.method ?? "GET",
            headers,
            body: body.byteLength > 0 ? body : undefined,
          }),
        ),
      );

      if (!resolved) {
        throw new Error("Missing network response");
      }

      response.writeHead(resolved.status, Object.fromEntries(resolved.headers));

      response.end(Buffer.from(await resolved.arrayBuffer()));
    }

    async function handle() {
      try {
        await serve();
      } catch {
        response.destroy();
      }
    }

    void handle();
  });

  await new Promise<void>((resolve) => {
    server.listen(0, "127.0.0.1", resolve);
  });

  const address = server.address();

  // oxlint-disable-next-line anti-slop/no-runtime-typeof
  if (address === null || typeof address === "string") {
    throw new Error("Missing server port");
  }

  const origin = `http://127.0.0.1:${address.port}`;
  const queueName = `native-queue-${crypto.randomUUID()}`;

  const context = await browser.newContext();

  const page = await context.newPage();

  const cdp = await context.newCDPSession(page);

  let registrationId: string | undefined;

  cdp.on("ServiceWorker.workerRegistrationUpdated", ({ registrations }) => {
    registrationId =
      registrations.find((entry) => entry.scopeURL === `${origin}/`)
        ?.registrationId ?? registrationId;
  });

  await cdp.send("ServiceWorker.enable");

  await page.goto(`${origin}/?queue=${queueName}`);

  onTestFinished(async () => {
    await context.close();

    server.closeAllConnections();

    await new Promise<void>((resolve, reject) => {
      server.close((error) => (error ? reject(error) : resolve()));
    });
  });

  return {
    context,
    origin,
    page,
    requested,
    received,
    requestedReached: requestedSignal.reached,
    arrived: arrived.promise,
    setResponse(next: typeof respond) {
      respond = next;
    },
    async sync() {
      await expect.poll(() => registrationId).toBeDefined();

      if (!registrationId) {
        throw new Error("Missing native service worker registration");
      }

      await cdp.send("ServiceWorker.dispatchSyncEvent", {
        origin,
        registrationId,
        tag: `workbox-background-sync:${queueName}`,
        lastChance: true,
      });
    },
  };
}

export async function entries(page: Page) {
  return page.evaluate(async () => {
    const fixture = globalThis.queueFixture;

    return fixture.entries();
  });
}
