import { gunzipSync } from "node:zlib";
import { join } from "node:path";
import { chromium } from "@playwright/test";
import type {
  Browser,
  BrowserContext,
  CDPSession,
  Page,
} from "@playwright/test";
import {
  afterAll,
  afterEach,
  assert,
  beforeAll,
  beforeEach,
  expect,
  test,
} from "vitest";
import { application } from "@interfere/test-utils/vitest/packed-application";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import { acceptanceRelease } from "./acceptance-release.test-utils";
import { exceptions } from "@interfere/test-utils/vitest/otlp";

let app: Awaited<ReturnType<typeof application>>;
let collector: Awaited<ReturnType<typeof receiver>>;
let website: Awaited<ReturnType<typeof receiver>>;
let browser: Browser;
let context: BrowserContext;
let page: Page;
let pageEvents: string[];
let pendingRequests: Set<string>;

beforeAll(async () => {
  collector = await receiver({ respond: acceptanceRelease });

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
    INTERFERE_API_KEY: "interfere_ak_acceptance",
    INTERFERE_API_URL: collector.url,
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

async function delivered(message: string) {
  await expect
    .poll(() => exceptions(collector.requests, message).length, {
      timeout: 15_000,
    })
    .toBe(1);

  const [captured] = exceptions(collector.requests, message);

  assert(captured);

  return captured;
}

async function replayCapture(
  cdp: CDPSession,
  registrationId: string,
  concurrentEvents: 1 | 2,
) {
  expect(registrationId).not.toBe("");

  await expect
    .poll(
      async () => {
        await Promise.all(
          Array.from({ length: concurrentEvents }, async () =>
            cdp.send("ServiceWorker.dispatchSyncEvent", {
              registrationId,
              origin: website.url,
              tag: "workbox-background-sync:acceptance",
              lastChance: false,
            }),
          ),
        );

        return exceptions(collector.requests, "acceptance-manual").length;
      },
      { timeout: 15_000 },
    )
    .toBe(1);
}

function requestBodyText(request: { postDataBuffer: () => Buffer | null }) {
  const body = request.postDataBuffer();

  if (!body) {
    return "";
  }

  return (
    body[0] === 0x1f && body[1] === 0x8b ? gunzipSync(body) : body
  ).toString();
}

test.each([429, 503])(
  "the installed service worker durably replays an HTTP %i export once across reload and overlapping sync events",
  async (status) => {
    expect.hasAssertions();

    const cdp = await context.newCDPSession(page);

    let registrationId = "";

    cdp.on("ServiceWorker.workerRegistrationUpdated", ({ registrations }) => {
      registrationId =
        registrations.find(
          (registration) => registration.scopeURL === `${website.url}/`,
        )?.registrationId ?? registrationId;
    });

    await cdp.send("ServiceWorker.enable");

    await open("/?queue=1");

    collector.fail("/drain/v1/traces", status, 100);

    const queued = page.waitForResponse(
      (response) =>
        response.url().includes("/drain/v1/traces") &&
        response.status() === 202 &&
        requestBodyText(response.request()).includes("acceptance-manual"),
    );

    await click("Capture");

    await queued;

    expect(exceptions(collector.requests, "acceptance-manual")).toHaveLength(0);

    await page.reload();

    await rendered();

    collector.fail("/drain/v1/traces", status, 0);

    await replayCapture(cdp, registrationId, 2);

    await click("Count 0");

    await expect(
      page.getByRole("button", { name: "Count 1" }).count(),
    ).resolves.toBe(1);
  },
);

test("offline capture is delivered after a native background sync", async () => {
  const cdp = await context.newCDPSession(page);

  let registrationId = "";

  cdp.on("ServiceWorker.workerRegistrationUpdated", ({ registrations }) => {
    registrationId =
      registrations.find(
        (registration) => registration.scopeURL === `${website.url}/`,
      )?.registrationId ?? registrationId;
  });

  await cdp.send("ServiceWorker.enable");

  await open("/?queue=1");

  await context.setOffline(true);

  const queued = page.waitForResponse(
    (response) =>
      response.url().includes("/drain/v1/traces") &&
      response.status() === 202 &&
      requestBodyText(response.request()).includes("acceptance-manual"),
  );

  await click("Capture");

  await queued;

  expect(exceptions(collector.requests, "acceptance-manual")).toHaveLength(0);

  await context.setOffline(false);

  await replayCapture(cdp, registrationId, 1);
});

test("an unknown release halts export without a retry storm or breaking the host", async () => {
  await open();

  collector.fail(
    "/drain/",
    422,
    100,
    JSON.stringify({ code: "RELEASE_NOT_FOUND" }),
  );

  await click("Span result");

  await expect
    .poll(
      () =>
        collector.requests.filter((request) => request.status === 422).length,
      { timeout: 15_000 },
    )
    .toBeGreaterThanOrEqual(1);

  const rejected = collector.requests.filter(
    (request) => request.status === 422,
  ).length;

  await click("Span result");

  await click("Capture");

  await click("Close SDK");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("closed");

  expect(
    collector.requests.filter((request) => request.status === 422),
  ).toHaveLength(rejected);

  await click("Count 0");

  await expect(
    page.getByRole("button", { name: "Count 1" }).count(),
  ).resolves.toBe(1);
});

test("unload flushes a captured error without an explicit SDK close", async () => {
  expect.hasAssertions();

  await context.unrouteAll();

  await open();

  await click("Capture");

  await page.goto("about:blank");

  await delivered("acceptance-manual");
});
