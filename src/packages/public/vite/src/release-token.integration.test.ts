import { join } from "node:path";
import { chromium } from "@playwright/test";
import type { Browser } from "@playwright/test";
import { HttpResponse } from "msw";
import { afterAll, beforeAll, expect, test } from "vitest";
import { application } from "@interfere/test-utils/vitest/packed-application";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import type { ReceivedRequest } from "@interfere/test-utils/vitest/telemetry-receiver";
import { exceptions } from "@interfere/test-utils/vitest/otlp";

const TOKEN = "irt_packed_acceptance";

let app: Awaited<ReturnType<typeof application>>;
let collector: Awaited<ReturnType<typeof receiver>>;
let website: Awaited<ReturnType<typeof receiver>>;
let browser: Browser;
let buildRequests: ReceivedRequest[];

beforeAll(async () => {
  collector = await receiver({
    respond: (request) =>
      request.method === "POST" &&
      new URL(request.url, "http://receiver").pathname === "/v3/releases"
        ? HttpResponse.json({
            build: { hash: "0123456789abcdef0123456789abcdef01234567" },
            destination: {
              environment: "preview",
              id: "release_acceptance",
              slug: "rel_0123456789abcdef",
            },
            org: { id: "org_acceptance", slug: "acceptance" },
            source: { id: "source_acceptance" },
            surface: { id: "surface_acceptance", slug: "web" },
            token: TOKEN,
          })
        : undefined,
  });

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

  buildRequests = [...collector.requests];

  website.serve(join(app.root, "dist"));

  browser = await chromium.launch({ headless: true });
}, 120_000);

afterAll(async () => {
  await browser.close();

  await app.close();

  await website.close();

  await collector.close();
});

test("a build with an API key and no public key exports browser telemetry under its baked release token", async () => {
  expect.hasAssertions();

  const context = await browser.newContext();

  await context.addInitScript(() => {
    Object.assign(globalThis, { __INTERFERE_FORCE_ENABLE__: true });
  });

  await context.route("https://**/*", async (route) =>
    route.abort("blockedbyclient"),
  );

  const page = await context.newPage();

  await page.goto(website.url);

  await page
    .getByRole("button", { name: "Capture", exact: true })
    .click({ timeout: 30_000 });

  await expect
    .poll(() => exceptions(collector.requests, "acceptance-manual").length, {
      timeout: 15_000,
    })
    .toBe(1);

  const exports = collector.requests.filter((request) =>
    new URL(request.url, "http://receiver").pathname.startsWith("/drain/v1/"),
  );

  expect(exports.length).toBeGreaterThan(0);

  for (const request of exports) {
    expect(request.headers.get("x-interfere-release-token")).toBe(TOKEN);
    expect(request.headers.get("x-interfere-public-key")).toBeNull();

    expect(new URL(request.url, "http://receiver").search).not.toContain(
      "public_key",
    );
  }

  await context.close();
});

test("the build bakes its SDK config with the release token it registered", () => {
  const configs = buildRequests.filter(
    (request) =>
      new URL(request.url, "http://receiver").pathname === "/v3/config",
  );

  expect(configs).toHaveLength(1);

  expect(configs[0]?.headers.get("x-interfere-release-token")).toBe(TOKEN);
  expect(configs[0]?.headers.get("x-interfere-public-key")).toBeNull();
});
