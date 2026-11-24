/* oxlint-disable vitest/no-import-node-test, vitest/prefer-importing-vitest-globals */
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdtemp, readFile, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import test, { after as afterAll, before as beforeAll } from "node:test";

import { chromium } from "playwright";
import { HttpResponse, getResponse, http } from "msw";

const ORIGIN = "https://react-key-deopt.test";

const DEOPT = "unexpected name in keyed access";

type FixtureMode = "allocate" | "sdk";

let temporary: string;

const deopts = new Map<FixtureMode, number>();

let kernelStarted = false;

function bundle(mode: FixtureMode) {
  const outdir = join(temporary, mode);

  execFileSync(
    "bun",
    [
      "build",
      resolve(import.meta.dirname, "react-key-deopt.fixture.ts"),
      "--target=browser",
      "--format=esm",
      "--minify-syntax",
      `--outdir=${outdir}`,
      "--entry-naming=app.js",
      `--define=FIXTURE_MODE="${mode}"`,
      '--define=process.env.NODE_ENV="production"',
    ],
    { cwd: resolve(import.meta.dirname, ".."), timeout: 20_000 },
  );

  return join(outdir, "app.js");
}

async function countKeyedNameDeopts(mode: FixtureMode) {
  const script = await readFile(bundle(mode), "utf8");

  const log = join(temporary, `${mode}.v8.log`);

  const handlers = [
    http.get(`${ORIGIN}/`, () =>
      HttpResponse.html(
        `<!doctype html><html><head><script>window.__INTERFERE_PUBLIC_KEY__="pk_fixture";window.__INTERFERE_API_URL__="${ORIGIN}";window.__INTERFERE_FORCE_ENABLE__=true;</script></head><body><div id="root"></div><script type="module" src="/app.js"></script></body></html>`,
      ),
    ),
    http.get(
      `${ORIGIN}/app.js`,
      () =>
        new HttpResponse(script, {
          headers: { "content-type": "text/javascript" },
        }),
    ),
    http.all(`${ORIGIN}/*`, () => new HttpResponse(null, { status: 204 })),
  ];

  const browser = await chromium.launch({
    headless: true,
    args: [
      "--no-sandbox",
      `--js-flags=--log-deopt --logfile=${log} --no-logfile-per-isolate`,
    ],
  });

  try {
    const page = await browser.newPage();

    await page.route("**/*", async (route) => {
      const request = route.request();

      const response =
        new URL(request.url()).origin === ORIGIN
          ? await getResponse(
              handlers,
              new Request(request.url(), { method: request.method() }),
            )
          : undefined;

      if (!response) {
        await route.abort();

        return;
      }

      await route.fulfill({
        status: response.status,
        headers: Object.fromEntries(response.headers),
        body: Buffer.from(await response.arrayBuffer()),
      });
    });

    await page.goto(`${ORIGIN}/`);

    await page.waitForSelector("body[data-ready]");

    if (mode === "sdk") {
      kernelStarted =
        (await page.locator("body").getAttribute("data-kernel")) === "true";
    }

    await page.waitForSelector("body[data-streamed]");
  } finally {
    await browser.close();
  }

  return (await readFile(log, "utf8"))
    .split("\n")
    .filter((line) => line.startsWith("code-deopt") && line.includes(DEOPT))
    .length;
}

beforeAll(async () => {
  temporary = await mkdtemp(join(tmpdir(), "react-key-deopt-"));

  deopts.set("allocate", await countKeyedNameDeopts("allocate"));

  deopts.set("sdk", await countKeyedNameDeopts("sdk"));
});

afterAll(async () => {
  await rm(temporary, { recursive: true, force: true });
});

void test("the deopt oracle detects React's keyed-name deopt loop when allocation precedes the first render", (context) => {
  const count = deopts.get("allocate") ?? 0;

  context.diagnostic(`${count} "${DEOPT}" deopts in 6 s`);

  if (count < 10) {
    context.skip(`this V8 no longer loops on promoted React keys (${count})`);

    return;
  }

  assert.ok(count >= 10);
});

void test("initializing the SDK before the first render does not put React into the keyed-name deopt loop", (context) => {
  const count = deopts.get("sdk") ?? 0;

  context.diagnostic(`${count} "${DEOPT}" deopts in 6 s`);

  assert.equal(kernelStarted, true);

  assert.ok(count < 10, `${count} "${DEOPT}" deopts in 6 s`);
});
