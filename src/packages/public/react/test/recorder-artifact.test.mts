/* oxlint-disable vitest/no-import-node-test, vitest/prefer-importing-vitest-globals */
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdtemp, readFile, readdir, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import test, { after as afterAll, before as beforeAll } from "node:test";
import { URL } from "node:url";
import { EventType, IncrementalSource } from "@rrweb/types";

import { chromium } from "playwright";
import type { Browser } from "playwright";
import { HttpResponse, getResponse, http } from "msw";
import { z } from "zod";

let browser: Browser;
let temporary: string;
let published: string;

beforeAll(async () => {
  const packageRoot = resolve(import.meta.dirname, "..");

  temporary = await mkdtemp(join(tmpdir(), "recorder-artifact-"));

  execFileSync(
    "bun",
    ["pm", "pack", "--ignore-scripts", "--quiet", "--destination", temporary],
    { cwd: packageRoot, timeout: 10_000 },
  );

  const tarball = (await readdir(temporary)).find((name) =>
    name.endsWith(".tgz"),
  );

  assert.ok(tarball);

  execFileSync("tar", ["-xzf", join(temporary, tarball), "-C", temporary], {
    timeout: 10_000,
  });

  published = join(temporary, "package");

  browser = await chromium.launch({ headless: true });
});

afterAll(async () => {
  try {
    await browser.close();
  } finally {
    await rm(temporary, { recursive: true, force: true });
  }
});

void test("the published SDK lazily ships the guarded recorder", async (context) => {
  const page = await browser.newPage();

  context.after(async () => page.close());

  const requests: string[] = [];
  const errors: string[] = [];

  page.on("pageerror", (error) => errors.push(error.message));

  const handlers = [
    http.get("https://sdk-artifact.test/*", async ({ request }) => {
      const url = new URL(request.url);

      requests.push(url.pathname);

      return new HttpResponse(
        url.pathname === "/"
          ? "<!doctype html><html><body></body></html>"
          : await readFile(join(published, url.pathname)),
        {
          headers: {
            "content-type":
              url.pathname === "/" ? "text/html" : "text/javascript",
          },
        },
      );
    }),
  ];

  await page.route("**/*", async (route) => {
    const request = route.request();

    assert.equal(new URL(request.url()).origin, "https://sdk-artifact.test");

    const response = await getResponse(
      handlers,
      new Request(request.url(), {
        method: request.method(),
        headers: await request.allHeaders(),
      }),
    );

    assert.ok(response);

    await route.fulfill({
      status: response.status,
      headers: Object.fromEntries(response.headers),
      body: Buffer.from(await response.arrayBuffer()),
    });
  });

  await page.goto("https://sdk-artifact.test/");

  const recorderModule = await page.evaluateHandle(async () => {
    const url = new globalThis.URL(
      "/dist/plugins/replay/recorder/recorder.mjs",
      globalThis.location.href,
    );

    // oxlint-disable-next-line typescript/no-unsafe-assignment -- Validate the built module against the source contract in the browser.
    const module: typeof import("../src/plugins/replay/recorder/recorder.js") =
      await import(url.href);

    return module;
  });

  assert.equal(
    requests.some((path) => path.includes("rrweb")),
    false,
  );

  const faults = await page.evaluate(
    async ({ module, snapshot, mutation }) => {
      const failures: string[] = [];
      const attached = Promise.withResolvers<undefined>();
      const continued = Promise.withResolvers<undefined>();
      const frames: HTMLIFrameElement[] = [];

      const observer = new globalThis.MutationObserver((records) => {
        for (const record of records) {
          for (const node of record.addedNodes) {
            if (node instanceof globalThis.HTMLIFrameElement) {
              frames.push(node);
            }
          }
        }
      });

      observer.observe(globalThis.document.body, { childList: true });

      Object.defineProperty(globalThis, "Zone", {
        configurable: true,
        value: {},
      });

      const recorder = await module.startReplayRecorder({
        onEvent: (event) => {
          if (
            event.type === snapshot &&
            event.data.source === mutation &&
            event.data.isAttachIframe
          ) {
            attached.resolve(undefined);
          }

          if (JSON.stringify(event).includes("recording continues")) {
            continued.resolve(undefined);
          }
        },
        onFault: (error) => failures.push(String(error)),
        onMutationStorm: (detail) => failures.push(detail),
        recordOptions: {
          blockClass: "blocked",
          ignoreClass: "ignored",
          maskTextClass: "masked",
        },
      });

      observer.disconnect();

      Reflect.deleteProperty(globalThis, "Zone");

      if (
        frames.length === 0 ||
        frames.length > 2 ||
        frames.some((frame) => frame.isConnected)
      ) {
        failures.push(`Unexpected temporary recorder frames: ${frames.length}`);
      }

      // oxlint-disable-next-line react/iframe-missing-sandbox -- Reproduce the same-origin measurement iframe.
      const iframe = globalThis.document.createElement("iframe");

      globalThis.document.body.append(iframe);

      Object.defineProperty(iframe.contentWindow, "Element", {
        configurable: true,
        value: undefined,
        writable: true,
      });

      await attached.promise;

      const marker = globalThis.document.createElement("p");

      marker.textContent = "recording continues";

      globalThis.document.body.append(marker);

      await continued.promise;

      recorder?.stop();

      return failures;
    },
    {
      module: recorderModule,
      snapshot: EventType.IncrementalSnapshot as const,
      mutation: IncrementalSource.Mutation as const,
    },
  );

  assert.equal(
    requests.filter((path) => /\/rrweb-.*\.mjs$/u.test(path)).length,
    1,
  );

  assert.deepEqual(errors, []);

  assert.deepEqual(faults, []);
});

void test("the published recorder snapshots at DOMContentLoaded before load", async (context) => {
  const root = resolve(import.meta.dirname, "..");

  const page = await browser.newPage();

  const parser = Promise.withResolvers<undefined>();
  const image = Promise.withResolvers<undefined>();

  context.after(async () => {
    parser.resolve(undefined);

    image.resolve(undefined);

    await page.close();
  });

  const handlers = [
    http.get("https://recorder-lifecycle.test/", () =>
      HttpResponse.html(
        '<!doctype html><html><head><script src="/parser.js"></script></head><body><img src="/pending.png"></body></html>',
      ),
    ),
    http.get("https://recorder-lifecycle.test/parser.js", async () => {
      await parser.promise;

      return new HttpResponse("", {
        headers: { "content-type": "text/javascript" },
      });
    }),
    http.get("https://recorder-lifecycle.test/pending.png", async () => {
      await image.promise;

      return new HttpResponse(null, { status: 404 });
    }),
    http.get(
      "https://recorder-lifecycle.test/dist/*",
      async ({ request }) =>
        new HttpResponse(
          await readFile(join(root, new URL(request.url).pathname)),
          { headers: { "content-type": "text/javascript" } },
        ),
    ),
  ];

  await page.route("**/*", async (route) => {
    const request = route.request();

    const response = await getResponse(
      handlers,
      new Request(request.url(), {
        method: request.method(),
        headers: await request.allHeaders(),
      }),
    );

    assert.ok(response);

    await route.fulfill({
      status: response.status,
      headers: Object.fromEntries(response.headers),
      body: Buffer.from(await response.arrayBuffer()),
    });
  });

  await page.goto("https://recorder-lifecycle.test/", {
    waitUntil: "commit",
  });

  const observation = await page.evaluateHandle(async () => {
    // oxlint-disable-next-line typescript/no-unsafe-assignment
    const module: typeof import("../src/plugins/replay/recorder/recorder.js") =
      await import(
        new URL("/dist/plugins/replay/recorder/recorder.mjs", location.href)
          .href
      );

    const events: { type: number }[] = [];
    const faults: unknown[] = [];

    const recorder = await module.startReplayRecorder({
      onEvent: (event) => events.push(event),
      onFault: (error) => {
        faults.push(error);
      },
      onMutationStorm: (detail) => {
        faults.push(detail);
      },
      recordOptions: {
        blockClass: "interfere-block",
        ignoreClass: "interfere-ignore",
        maskTextClass: "interfere-mask",
      },
    });

    return { events, faults, recorder, readyState: document.readyState };
  });

  assert.equal(
    await observation.evaluate((value) => value.readyState),
    "loading",
  );

  assert.equal(
    await observation.evaluate((value) =>
      value.events.some((event) => event.type === 2),
    ),
    false,
  );

  parser.resolve(undefined);

  await page.waitForLoadState("domcontentloaded");

  assert.equal(
    await observation.evaluate((value) =>
      value.events.some((event) => event.type === 2),
    ),
    true,
  );

  assert.equal(
    await observation.evaluate((value) =>
      value.events.some((event) => event.type === 1),
    ),
    false,
  );

  assert.equal(await page.evaluate(() => document.readyState), "interactive");

  assert.deepEqual(await observation.evaluate((value) => value.faults), []);

  await observation.evaluate((value) => value.recorder?.stop());

  image.resolve(undefined);

  await page.waitForLoadState("load");
});

void test("the published SDK neither depends on nor imports zone.js", async () => {
  const zone = /["'`](?:@opentelemetry\/context-zone|zone\.js)["'`/]/u;

  const files = (
    await readdir(join(published, "dist"), { recursive: true })
  ).filter((name) => name.endsWith(".mjs"));

  assert.ok(files.length > 0);

  const sources = await Promise.all(
    ["package.json", ...files.map((name) => join("dist", name))].map(
      async (name) => ({
        name,
        source: await readFile(join(published, name), "utf8"),
      }),
    ),
  );

  for (const { name, source } of sources) {
    assert.doesNotMatch(source, zone, name);
  }
});

void test("the published SDK bundles rrweb instead of depending on it", async () => {
  const rrwebPackage = /^(?:rrweb(?:-snapshot)?|rrdom|@rrweb\/.+)$/u;

  const rrwebImport =
    /(?:\bfrom|\bimport)\s*\(?\s*["'`](?:rrweb(?:-snapshot)?|rrdom|@rrweb\/[^"'`]+)["'`]/u;

  const dependencyMap = z.record(z.string(), z.string()).optional();

  const manifest = z
    .object({
      dependencies: dependencyMap,
      optionalDependencies: dependencyMap,
      peerDependencies: dependencyMap,
    })
    .parse(JSON.parse(await readFile(join(published, "package.json"), "utf8")));

  for (const [field, dependencies] of Object.entries(manifest)) {
    for (const name of Object.keys(dependencies ?? {})) {
      assert.doesNotMatch(name, rrwebPackage, `${field}: ${name}`);
    }
  }

  const files = (
    await readdir(join(published, "dist"), { recursive: true })
  ).filter((name) => name.endsWith(".mjs") || name.endsWith(".d.mts"));

  assert.ok(files.some((name) => name.endsWith(".d.mts")));

  const sources = await Promise.all(
    files.map(async (name) => ({
      name,
      source: await readFile(join(published, "dist", name), "utf8"),
    })),
  );

  for (const { name, source } of sources) {
    assert.doesNotMatch(source, rrwebImport, name);
  }
});
