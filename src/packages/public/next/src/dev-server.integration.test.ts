import { chromium } from "@playwright/test";
import type { Browser } from "@playwright/test";
import { HttpResponse } from "msw";
import { afterAll, beforeAll, expect, test } from "vitest";
import { application } from "@interfere/test-utils/vitest/packed-application";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import { exceptions } from "@interfere/test-utils/vitest/otlp";
import { createReleaseRequest } from "@interfere/types/releases/definition";
import { LOCAL_DEV_RELEASE_SLUG } from "@interfere/types/releases/slug";

const TOKEN = "irt_next_dev_packed_acceptance";

let collector: Awaited<ReturnType<typeof receiver>>;
let app: Awaited<ReturnType<typeof application>>;
let host: Awaited<ReturnType<typeof app.start>>;
let browser: Browser;
const cleanups: (() => Promise<void>)[] = [];

function drainRequests() {
  return collector.requests.filter((request) =>
    new URL(request.url, "http://receiver").pathname.startsWith("/drain/v1/"),
  );
}

beforeAll(async () => {
  collector = await receiver({
    respond: (request) =>
      request.method === "POST" &&
      new URL(request.url, "http://receiver").pathname === "/v3/releases"
        ? HttpResponse.json({
            build: { hash: LOCAL_DEV_RELEASE_SLUG },
            destination: {
              environment: null,
              id: "release_local_dev",
              slug: LOCAL_DEV_RELEASE_SLUG,
            },
            org: { id: "org_local_dev", slug: "local-dev" },
            source: { id: "source_local_dev" },
            surface: null,
            token: TOKEN,
          })
        : undefined,
  });

  cleanups.push(async () => collector.close());

  app = await application(`${import.meta.dirname}/..`, [
    "@interfere/next",
    "next",
    "react",
    "react-dom",
    "typescript",
    "@types/node",
    "@types/react",
    "@types/react-dom",
  ]);

  cleanups.push(async () => app.close());

  await app.copyFixture("acceptance");

  host = await app.start("serve-dev.ts", {
    INTERFERE_API_KEY: "interfere_secret_us_acceptance",
    INTERFERE_API_URL: collector.url,
    INTERFERE_SOURCE_ID: "0123456789abcdef0123456789abcdef01234567",
    OTEL_SERVICE_NAME: "dev-next",
  });

  cleanups.push(async () => host.close());

  browser = await chromium.launch({ headless: true });

  cleanups.push(async () => browser.close());
}, 240_000);

afterAll(async () => {
  for (const cleanup of cleanups.toReversed()) {
    // oxlint-disable-next-line eslint/no-await-in-loop -- Resources close in reverse order of creation.
    await cleanup();
  }
});

test("the dev server registers the local-development release without a destination", () => {
  const registrations = collector.requests.filter(
    (request) =>
      request.method === "POST" &&
      new URL(request.url, "http://receiver").pathname === "/v3/releases",
  );

  expect(registrations).toHaveLength(1);

  expect(registrations[0]?.headers.get("x-interfere-public-key")).toBeNull();

  expect(
    createReleaseRequest.parse(JSON.parse(registrations[0]?.body ?? "null")),
  ).toMatchObject({
    destination: null,
    service: { name: "dev-next" },
    slug: LOCAL_DEV_RELEASE_SLUG,
  });
});

test("next dev with an API key and no public key exports browser telemetry under its minted release token", async () => {
  expect.hasAssertions();

  const context = await browser.newContext();

  await context.addInitScript(() => {
    Object.assign(globalThis, { __INTERFERE_FORCE_ENABLE__: true });
  });

  const page = await context.newPage();

  expect((await page.goto(host.url))?.status()).toBe(200);

  await page
    .getByRole("button", { name: "Capture client error", exact: true })
    .click({ timeout: 90_000 });

  await expect
    .poll(() => exceptions(drainRequests(), "next-client-error").length, {
      timeout: 30_000,
    })
    .toBe(1);

  const exports = drainRequests();

  expect(exports.length).toBeGreaterThan(0);

  for (const request of exports.filter(
    (candidate) => exceptions([candidate], "next-client-error").length > 0,
  )) {
    expect(request.headers.get("x-interfere-release-token")).toBe(TOKEN);
    expect(request.headers.get("x-interfere-public-key")).toBeNull();
  }

  await context.close();
});
