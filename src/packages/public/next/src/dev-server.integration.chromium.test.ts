import { reportChromiumFailures } from "../test/chromium-diagnostics";
import { chromium } from "@playwright/test";
import type { Browser } from "@playwright/test";
import { HttpResponse } from "msw";
import { afterAll, beforeAll, expect, test, vi } from "vitest";
import { application } from "@interfere/test-utils/vitest/packed-application";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import { exceptions } from "@interfere/test-utils/vitest/otlp";
import { createReleaseRequest } from "@interfere/types/releases/definition";
import { LOCAL_DEV_RELEASE_SLUG } from "@interfere/types/releases/slug";

const TOKEN = "irt_next_dev_packed_acceptance";
const SETUP_MS = 240_000;
const COMPILE_MS = 120_000;
const EXPORT_MS = 30_000;

let collector: Awaited<ReturnType<typeof receiver>>;
let app: Awaited<ReturnType<typeof application>>;
let host: Awaited<ReturnType<typeof app.start>>;
let browser: Browser;
let browserExports: ReturnType<typeof drainRequests>;
const cleanups: (() => Promise<void>)[] = [];

function drainRequests() {
  return collector.requests.filter((request) =>
    new URL(request.url, "http://receiver").pathname.startsWith("/drain/v1/"),
  );
}

beforeAll(
  async () => {
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

    const context = await browser.newContext();

    cleanups.push(async () => context.close());

    await context.addInitScript(() => {
      Object.assign(globalThis, { __INTERFERE_FORCE_ENABLE__: true });
    });

    const page = await context.newPage();

    reportChromiumFailures(page);

    const response = await page.goto(host.url, { timeout: COMPILE_MS });

    if (response?.status() !== 200) {
      throw new Error(`next dev answered ${response?.status()}`);
    }

    await page
      .getByLabel("Session", { exact: true })
      .filter({ hasText: /\S/u })
      .waitFor({ timeout: COMPILE_MS });

    await page
      .getByRole("button", { name: "Capture client error", exact: true })
      .click();

    browserExports = await vi.waitFor(
      () => {
        const requests = drainRequests();

        if (exceptions(requests, "next-client-error").length === 0) {
          throw new Error(
            "The browser has not exported the captured exception",
          );
        }

        return requests;
      },
      { timeout: EXPORT_MS },
    );
  },
  SETUP_MS + COMPILE_MS + EXPORT_MS,
);

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

test("next dev with an API key and no public key exports browser telemetry under its minted release token", () => {
  expect.hasAssertions();

  expect(exceptions(browserExports, "next-client-error")).toHaveLength(1);

  for (const request of browserExports.filter(
    (candidate) => exceptions([candidate], "next-client-error").length > 0,
  )) {
    expect(request.headers.get("x-interfere-release-token")).toBe(TOKEN);
    expect(request.headers.get("x-interfere-public-key")).toBeNull();
  }
});
