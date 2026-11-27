import { z } from "zod";
import { join } from "node:path";
import { chromium } from "@playwright/test";
import type { Browser, BrowserContext, Page } from "@playwright/test";
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
import {
  attribute,
  exceptions,
  exportedSpans,
} from "@interfere/test-utils/vitest/otlp";

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

test("production installation captures errors once after repeated initialization and preserves the host result", async () => {
  await open();

  await click("Initialize again");

  await click("Capture");

  await click("Span result");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("original-result");

  await click("Span failure");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("same-error");

  await click("Close SDK");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("closed");

  await delivered("acceptance-manual");

  await delivered("acceptance-span-error");

  expect(
    exportedSpans(collector.requests).filter(
      (item) => item.name === "acceptance-result",
    ),
  ).toHaveLength(1);

  await click("Count 0");

  await expect(
    page.getByRole("button", { name: "Count 1" }).count(),
  ).resolves.toBe(1);

  expect(
    collector.requests
      .filter((request) => request.url.startsWith("/drain/"))
      .every((request) => request.headers.get("cookie") === null),
  ).toBe(true);
});

test.each([
  {
    button: "Unhandled error",
    message: "acceptance-unhandled",
    expectedFallbacks: 0,
  },
  { button: "Reject", message: "acceptance-rejection", expectedFallbacks: 0 },
  {
    button: "Render error",
    message: "acceptance-render",
    expectedFallbacks: 1,
  },
])(
  "$button delivers an actual browser exception",
  async ({ button, message, expectedFallbacks }) => {
    await open();

    await click(button);

    const error = await delivered(message);

    expect(attribute(error.attributes, "session.id")).toBe(
      await page.getByLabel("Session", { exact: true }).textContent(),
    );

    await expect(page.getByText("Customer fallback").count()).resolves.toBe(
      expectedFallbacks,
    );

    await click("Count 0");

    await expect(
      page.getByRole("button", { name: "Count 1" }).count(),
    ).resolves.toBe(1);
  },
);

test("account switches identify the new user and logout rotates attribution", async () => {
  await open();

  const first = await page.getByLabel("Session", { exact: true }).textContent();

  await click("Identify A");

  await expect
    .poll(
      () =>
        collector.requests.filter(
          (request) => request.url === "/v3/session/identify",
        ).length,
    )
    .toBe(1);

  await click("Identify B");

  await click("Identify B");

  await expect
    .poll(
      () =>
        collector.requests.filter(
          (request) => request.url === "/v3/session/identify",
        ).length,
    )
    .toBe(2);

  await click("Logout");

  const second = await page
    .getByLabel("Session", { exact: true })
    .textContent();

  expect(second).not.toBe(first);

  await click("Identify B");

  await expect
    .poll(
      () =>
        collector.requests.filter(
          (request) => request.url === "/v3/session/identify",
        ).length,
    )
    .toBe(3);

  const identities = collector.requests
    .filter((request) => request.url === "/v3/session/identify")
    .map((request) =>
      z
        .object({ sessionId: z.string(), identifier: z.string() })
        .parse(JSON.parse(request.body)),
    );

  expect(identities).toMatchObject([
    { sessionId: first, identifier: "consumer-a" },
    { sessionId: first, identifier: "consumer-b" },
    { sessionId: second, identifier: "consumer-b" },
  ]);

  await click("Capture");

  expect(
    attribute((await delivered("acceptance-manual")).attributes, "session.id"),
  ).toBe(second);

  await page.reload();

  await rendered();

  await expect(
    page.getByLabel("Session", { exact: true }).textContent(),
  ).resolves.toBe(second);
});
