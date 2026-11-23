import { join } from "node:path";
import { readFile } from "node:fs/promises";
import { chromium } from "@playwright/test";
import type { Browser, BrowserContext, Page } from "@playwright/test";
import { TraceMap, originalPositionFor } from "@jridgewell/trace-mapping";
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
import { attribute, exceptions } from "@interfere/test-utils/vitest/otlp";

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

test("consent gates recording and device storage while necessary errors remain deliverable", async () => {
  expect.hasAssertions();

  await open("/?denied=1");

  await click("Capture");

  await delivered("acceptance-manual");

  expect(
    collector.requests.filter((request) =>
      request.url.startsWith("/v3/replay/"),
    ),
  ).toHaveLength(0);

  expect(
    (await context.cookies()).filter((cookie) =>
      cookie.name.includes("device"),
    ),
  ).toHaveLength(0);

  await click("Grant consent");

  await click("Count 0");

  await click("Close SDK");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("closed");

  await expect
    .poll(
      () =>
        collector.requests.filter((request) =>
          request.url.startsWith("/v3/replay/"),
        ).length,
    )
    .toBeGreaterThan(0);

  const replay = collector.requests
    .filter((request) => request.url.startsWith("/v3/replay/"))
    .map((request) => request.body)
    .join("\n");

  expect(replay).toContain("Customer application");

  for (const secret of [
    "masked-secret-5329",
    "blocked-secret-7451",
    "password-secret-9371",
    "email-secret-6821",
  ]) {
    expect(replay).not.toContain(secret);
  }

  await open("/?denied=1");

  await click("Grant consent");

  await click("Revoke consent");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("revoked");

  await page.getByLabel("Email", { exact: true }).fill("after-revocation-1985");

  await click("Close SDK");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("closed");

  expect(
    collector.requests.map((request) => request.body).join("\n"),
  ).not.toContain("after-revocation-1985");
});

test("a minified captured stack resolves to the customer source and carries release attribution", async () => {
  await open();

  await click("Capture");

  const captured = await delivered("acceptance-manual");

  expect(attribute(captured.resource, "release.slug")).toMatch(/^rel_/u);

  expect(attribute(captured.resource, "deployment.environment.name")).toBe(
    "preview",
  );

  const event = captured.events.find((item) => item.name === "exception");

  assert(event);

  const stack = String(attribute(event.attributes, "exception.stacktrace"));

  const frame =
    /(?<file>http:\/\/[^\s)]+\.js):(?<line>\d+):(?<column>\d+)/u.exec(stack);

  expect(frame).not.toBeNull();

  const groups = frame?.groups;

  assert(groups?.["file"]);

  const file = new URL(groups["file"]);

  const map = new TraceMap(
    await readFile(join(app.root, "dist", `${file.pathname}.map`), "utf8"),
  );

  const original = originalPositionFor(map, {
    line: Number(groups["line"]),
    column: Number(groups["column"]) - 1,
  });

  expect(original.source).toMatch(/main\.tsx$/u);

  const source = await readFile(join(app.root, "main.tsx"), "utf8");

  assert(original.line !== null);

  expect(source.split("\n")[original.line - 1]).toContain("acceptance-manual");
});

test("closing and restarting restores capture without duplicating observers", async () => {
  await open();

  await click("Close SDK");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("closed");

  await click("Capture");

  await click("Restart SDK");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("restarted");

  await click("Capture");

  await click("Close SDK");

  await delivered("acceptance-manual");
});
