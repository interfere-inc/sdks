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
import {
  identities,
  replay,
  replayWindows,
} from "../test/journey-observations";
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
const cleanups: (() => Promise<void>)[] = [];

beforeAll(async () => {
  collector = await receiver({ respond: acceptanceRelease });

  cleanups.push(async () => collector.close());

  website = await receiver();

  cleanups.push(async () => website.close());

  app = await application(`${import.meta.dirname}/..`, [
    "@interfere/vite",
    "@interfere/react",
    "react",
    "react-dom",
    "vite",
  ]);

  cleanups.push(async () => app.close());

  await app.copyFixture("acceptance");

  await app.copyFixture("journeys");

  await app.build({
    INTERFERE_API_KEY: "interfere_ak_acceptance",
    INTERFERE_API_URL: collector.url,
    ACCEPTANCE_INGEST_URL: collector.url,
    INTERFERE_ENVIRONMENT: "preview",
    INTERFERE_SOURCE_ID: "0123456789abcdef0123456789abcdef01234567",
  });

  website.serve(join(app.root, "dist"));

  browser = await chromium.launch({ headless: true });

  cleanups.push(async () => browser.close());
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
});

afterEach(async () => context.close());

afterAll(async () => {
  for (const cleanup of cleanups.toReversed()) {
    // oxlint-disable-next-line eslint/no-await-in-loop -- Close only created resources in reverse lifecycle order.
    await cleanup();
  }
});

async function open(query = "", target = page) {
  expect((await target.goto(website.url + query))?.status()).toBe(200);

  await target.getByRole("button", { name: "Capture", exact: true }).waitFor();

  await expect
    .poll(async () =>
      target.getByLabel("Session", { exact: true }).textContent(),
    )
    .toMatch(/\S/u);
}

async function action(name: string, result: string, target = page) {
  await target.getByRole("button", { name, exact: true }).click();

  await expect
    .poll(async () =>
      target.getByLabel("Result", { exact: true }).textContent(),
    )
    .toBe(result);
}

async function captured(marker: string, target = page) {
  await target.getByLabel("Marker", { exact: true }).fill(marker);

  await action("Capture", `captured:${marker}`, target);

  await expect
    .poll(() => exceptions(collector.requests, marker).length, {
      timeout: 15_000,
    })
    .toBe(1);

  const [event] = exceptions(collector.requests, marker);

  assert(event);

  return event;
}

test("identity queued during initialization reaches the receiver with the active session", async () => {
  await open("/?early=1");

  await expect.poll(() => identities(collector.requests).length).toBe(1);

  expect(identities(collector.requests)[0]).toMatchObject({
    identifier: "early-user",
    sessionId: await page.getByLabel("Session", { exact: true }).textContent(),
  });

  await captured("early-identity-capture");
});

test("identity deduplicates the same source while preserving a different source for the same user", async () => {
  await open();

  await action("Identify A", "identified-a");

  await action("Identify A", "identified-a");

  await action("Identify other source", "identified-other");

  await expect.poll(() => identities(collector.requests).length).toBe(2);

  await action("Close SDK", "closed");

  expect(
    identities(collector.requests).map((identity) => [
      identity.identifier,
      identity.source.name,
    ]),
  ).toStrictEqual([
    ["user-a", "Journeys"],
    ["user-a", "Other source"],
  ]);
});

test("logout separates errors before and after logout and reload retains the anonymous session", async () => {
  await open();

  await action("Identify A", "identified-a");

  const before = await captured("before-logout");

  await action("Logout", "anonymous");

  const after = await captured("after-logout");

  const session = attribute(after.attributes, "session.id");

  expect(session).toStrictEqual(expect.any(String));
  expect(session).not.toBe(attribute(before.attributes, "session.id"));

  await page.reload();

  await page.getByRole("button", { name: "Capture", exact: true }).waitFor();

  expect(
    attribute((await captured("after-reload")).attributes, "session.id"),
  ).toBe(session);
});

test("two tabs preserve one session and distinguish their window attribution", async () => {
  await open();

  const first = await captured("first-tab");

  const other = await context.newPage();

  await open("", other);

  const second = await captured("second-tab", other);

  expect(attribute(second.attributes, "session.id")).toBe(
    attribute(first.attributes, "session.id"),
  );

  await action("Close SDK", "closed", other);

  await action("Close SDK", "closed");

  await expect.poll(() => replayWindows(collector.requests).size).toBe(2);

  expect(
    new Set(
      collector.requests
        .filter((request) => request.url.startsWith("/v3/replay/"))
        .map((request) => request.url),
    ),
  ).toStrictEqual(
    new Set([
      `/v3/replay/upload/${String(attribute(first.attributes, "session.id"))}`,
    ]),
  );
});

test("spa navigation retains the session and does not duplicate subsequent captured errors", async () => {
  await open();

  const before = await captured("before-navigation");

  await action("Navigate", "navigated");

  expect(new URL(page.url()).searchParams.get("navigation")).toBe("1");

  const after = await captured("after-navigation");

  await action("Close SDK", "closed");

  expect(attribute(after.attributes, "session.id")).toBe(
    attribute(before.attributes, "session.id"),
  );

  expect(exceptions(collector.requests, "after-navigation")).toHaveLength(1);
});

test("concurrent initialization keeps a single capture pipeline and preserves session attribution", async () => {
  await open();

  const before = await captured("before-concurrent-init");

  await action("Initialize concurrently", "initialized");

  const after = await captured("after-concurrent-init");

  await action("Close SDK", "closed");

  expect(attribute(after.attributes, "session.id")).toBe(
    attribute(before.attributes, "session.id"),
  );

  expect(exceptions(collector.requests, "after-concurrent-init")).toHaveLength(
    1,
  );
});

test("remounting a StrictMode provider preserves capture and current identity without another identify", async () => {
  await open();

  await action("Identify A", "identified-a");

  await expect.poll(() => identities(collector.requests).length).toBe(1);

  const before = await captured("before-remount");

  await page
    .getByRole("button", { name: "Remount provider", exact: true })
    .click();

  await expect
    .poll(async () => page.getByLabel("Result", { exact: true }).textContent())
    .toBe("ready");

  const after = await captured("after-remount");

  await action("Close SDK", "closed");

  expect(attribute(after.attributes, "session.id")).toBe(
    attribute(before.attributes, "session.id"),
  );

  expect(exceptions(collector.requests, "after-remount")).toHaveLength(1);
  expect(identities(collector.requests)).toHaveLength(1);
});

test("consent can be granted again after revocation while necessary errors remain deliverable", async () => {
  await open("/?denied=1");

  await captured("denied-error");

  expect(replay(collector.requests)).toBe("");

  await action("Grant consent", "granted");

  await captured("first-grant-error");

  await action("Revoke consent", "revoked");

  await captured("revoked-error");

  await action("Grant consent", "granted");

  await captured("second-grant-error");

  await action("Close SDK", "closed");

  await expect
    .poll(() => replay(collector.requests))
    .toContain("second-grant-error");
});

test("default replay preserves ordinary inputs and protects passwords and explicit privacy classes", async () => {
  await open();

  await captured("privacy-ready");

  await action("Close SDK", "closed");

  await expect
    .poll(() => replay(collector.requests))
    .toContain("visible-text-7351");

  expect(replay(collector.requests)).toContain(
    "visible-email-2841@example.test",
  );

  expect(replay(collector.requests)).toContain("555-010-6789");
  expect(replay(collector.requests)).not.toContain("secret-password-9251");
  expect(replay(collector.requests)).not.toContain("secret-mask-8462");
  expect(replay(collector.requests)).not.toContain("secret-block-3971");
});

test("nested async spans preserve the customer result and export a connected parent-child trace", async () => {
  await open();

  await action("Nested spans", "child-result");

  await action("Close SDK", "closed");

  await expect
    .poll(
      () =>
        exportedSpans(collector.requests).filter((span) =>
          span.name.startsWith("journey-"),
        ).length,
    )
    .toBe(2);

  const parent = exportedSpans(collector.requests).find(
    (span) => span.name === "journey-parent",
  );

  const child = exportedSpans(collector.requests).find(
    (span) => span.name === "journey-child",
  );

  assert(parent);

  assert(child);

  expect(child.traceId).toBe(parent.traceId);
  expect(child.parentSpanId).toBe(parent.spanId);

  assert(child.status);

  expect(child.status.code).not.toBe(2);
});
