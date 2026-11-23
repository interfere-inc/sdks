import { join } from "node:path";
import { chromium } from "@playwright/test";
import type { Browser, BrowserContext, Page } from "@playwright/test";
import {
  afterAll,
  afterEach,
  beforeAll,
  beforeEach,
  expect,
  test,
} from "vitest";
import { application } from "@interfere/test-utils/vitest/packed-application";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";

let app: Awaited<ReturnType<typeof application>>;
let collector: Awaited<ReturnType<typeof receiver>>;
let website: Awaited<ReturnType<typeof receiver>>;
let browser: Browser;
let context: BrowserContext;
let page: Page;
let pageEvents: string[];
let pendingRequests: Set<string>;

beforeAll(async () => {
  collector = await receiver();

  website = await receiver();

  app = await application(`${import.meta.dirname}/..`, [
    "@interfere/vite",
    "@interfere/react",
    "react",
    "react-dom",
    "vite",
  ]);

  await app.copyFixture("acceptance");

  await app.build({
    VITE_INTERFERE_PUBLIC_KEY: "interfere_public_us_acceptance",
    ACCEPTANCE_INGEST_URL: collector.url,
    INTERFERE_ENVIRONMENT: "preview",
    INTERFERE_SOURCE_ID: "0123456789abcdef0123456789abcdef01234567",
  });

  website.serve(join(app.root, "dist"));

  browser = await chromium.launch({ headless: true });
}, 120_000);

beforeEach(async () => {
  collector.reset();

  context = await browser.newContext();

  await context.addInitScript(() => {
    Object.assign(globalThis, { __INTERFERE_FORCE_ENABLE__: true });
  });

  await context.route("https://**/*", async (route) =>
    route.abort("blockedbyclient"),
  );

  page = await context.newPage();

  pageEvents = [];

  pendingRequests = new Set();

  page.on("request", (request) => pendingRequests.add(request.url()));

  page.on("requestfinished", (request) =>
    pendingRequests.delete(request.url()),
  );

  page.on("pageerror", (error) =>
    pageEvents.push(`pageerror: ${error.message}`),
  );

  page.on("console", (message) => {
    if (message.type() === "error" || message.type() === "warning") {
      pageEvents.push(`console.${message.type()}: ${message.text()}`);
    }
  });

  page.on("requestfailed", (request) => {
    pendingRequests.delete(request.url());

    pageEvents.push(
      `requestfailed: ${request.url()} ${request.failure()?.errorText ?? ""}`,
    );
  });
});

afterEach(async () => context.close());

afterAll(async () => {
  await browser.close();

  await app.close();

  await website.close();

  await collector.close();
});

async function rendered() {
  try {
    await page
      .getByRole("button", { name: "Capture", exact: true })
      .waitFor({ timeout: 30_000 });
  } catch (error) {
    throw new Error(
      `The acceptance application did not render: ${JSON.stringify({
        events: pageEvents.slice(-20),
        pending: [...pendingRequests],
        collector: collector.requests.map(
          (request) => `${request.method} ${request.url} ${request.status}`,
        ),
      })}`,
      { cause: error },
    );
  }
}

async function open(query = "") {
  await page.goto(website.url + query);

  await rendered();

  await expect
    .poll(async () => page.getByLabel("Session", { exact: true }).textContent())
    .toMatch(/\S/u);
}

async function click(name: string) {
  await page.getByRole("button", { name, exact: true }).click();
}

function replayRequests() {
  return collector.requests.filter((request) =>
    request.url.startsWith("/v3/replay/"),
  );
}

function replayText() {
  return replayRequests()
    .map((request) => request.body)
    .join("\n");
}

test("navigating away right after a small page renders delivers its buffered replay chunk", async () => {
  expect.hasAssertions();

  await context.unrouteAll();

  const teardown = Promise.withResolvers<undefined>();

  collector.hold("/v3/replay/", teardown.promise);

  await open();

  await click("Navigate");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("navigated");

  expect(replayRequests()).toHaveLength(0);

  await page.goto("about:blank");

  teardown.resolve(undefined);

  await expect
    .poll(() => replayRequests().length, { timeout: 4000 })
    .toBeGreaterThan(0);

  expect(replayText()).toContain("Customer application");

  expect(replayText()).toContain("navigated");
});

test("navigating away from a bulky page after its first flush delivers the snapshot and the recorded tail", async () => {
  expect.hasAssertions();

  await context.unrouteAll();

  await page.clock.install();

  await open("/?bulk=1");

  await expect
    .poll(
      async () => {
        await page.clock.fastForward(500);

        return replayText().includes("Customer application");
      },
      { timeout: 4000 },
    )
    .toBe(true);

  const flushed = replayRequests().length;

  const teardown = Promise.withResolvers<undefined>();

  collector.hold("/v3/replay/", teardown.promise);

  await click("Navigate");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("navigated");

  await page.goto("about:blank");

  teardown.resolve(undefined);

  await expect
    .poll(() => replayRequests().length, { timeout: 4000 })
    .toBeGreaterThan(flushed);

  expect(replayRequests().at(-1)?.body).toContain("navigated");
});
