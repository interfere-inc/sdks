import { chromium } from "@playwright/test";
import type { Browser } from "@playwright/test";
import { HttpResponse } from "msw";
import { afterAll, beforeAll, expect, test } from "vitest";
import { application } from "@interfere/test-utils/vitest/packed-application";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import type { ReceivedRequest } from "@interfere/test-utils/vitest/telemetry-receiver";
import { exceptions } from "@interfere/test-utils/vitest/otlp";

const TOKEN = "irt_next_packed_acceptance";

let collector: Awaited<ReturnType<typeof receiver>>;
let app: Awaited<ReturnType<typeof application>>;
let host: Awaited<ReturnType<typeof app.start>>;
let browser: Browser;
let buildRequests: ReceivedRequest[];
const cleanups: (() => Promise<void>)[] = [];

function drainRequests() {
  return collector.requests.filter((request) =>
    new URL(request.url, "http://receiver").pathname.startsWith("/drain/v1/"),
  );
}

beforeAll(async () => {
  collector = await receiver({
    respond: (request) => {
      const { pathname } = new URL(request.url, "http://receiver");

      if (pathname === "/v3/releases/config") {
        return HttpResponse.json(
          { code: "AUTH_AK_SURFACE_REQUIRED" },
          { status: 403 },
        );
      }

      return request.method === "POST" && pathname === "/v3/releases"
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
        : undefined;
    },
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

  await app.copyFixture("server-auth");

  const env = {
    INTERFERE_API_URL: collector.url,
    INTERFERE_SOURCE_ID: "0123456789abcdef0123456789abcdef01234567",
    INTERFERE_ENVIRONMENT: "preview",
    OTEL_SERVICE_NAME: "keyless-next",
  };

  await app.buildNext(
    { ...env, INTERFERE_API_KEY: "interfere_secret_us_acceptance" },
    "turbopack",
  );

  buildRequests = [...collector.requests];

  host = await app.start("serve.ts", env);

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

test("a build with only a workspace secret exports browser telemetry under its baked release token", async () => {
  expect.hasAssertions();

  const context = await browser.newContext();

  await context.addInitScript(() => {
    Object.assign(globalThis, { __INTERFERE_FORCE_ENABLE__: true });
  });

  const page = await context.newPage();

  expect((await page.goto(host.url))?.status()).toBe(200);

  await page
    .getByRole("button", { name: "Capture client error", exact: true })
    .click({ timeout: 30_000 });

  await expect
    .poll(() => exceptions(drainRequests(), "next-client-error").length, {
      timeout: 15_000,
    })
    .toBe(1);

  expect(
    collector.requests.filter(
      (request) =>
        request.method === "POST" &&
        new URL(request.url, "http://receiver").pathname === "/v3/releases",
    ),
  ).toHaveLength(1);

  const exports = drainRequests();

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

test("without INTERFERE_API_KEY the Node and Edge server runtimes send telemetry with the baked release token", async () => {
  expect.hasAssertions();

  for (const path of ["/api/failure", "/api/edge-failure"]) {
    // oxlint-disable-next-line eslint/no-await-in-loop -- Each route fails on its own runtime before export is checked.
    const response = await fetch(host.url + path);

    expect(response.status).toBe(500);

    // oxlint-disable-next-line eslint/no-await-in-loop -- Consume each response before continuing.
    await response.text();
  }

  await expect
    .poll(
      () =>
        [
          exceptions(drainRequests(), "next-route-error").length,
          exceptions(drainRequests(), "next-edge-route-error").length,
        ].every((count) => count > 0),
      { timeout: 15_000 },
    )
    .toBe(true);

  const server = drainRequests().filter(
    (request) =>
      exceptions([request], "next-route-error").length > 0 ||
      exceptions([request], "next-edge-route-error").length > 0,
  );

  expect(
    server.map((request) => [
      request.headers.get("x-interfere-release-token"),
      request.headers.get("authorization"),
      request.headers.get("x-api-key"),
      request.headers.get("x-interfere-public-key"),
    ]),
  ).toStrictEqual(server.map(() => [TOKEN, null, null, null]));
});

test("the key-only build registers its service in the release body", () => {
  const registrations = buildRequests.filter(
    (request) =>
      request.method === "POST" &&
      new URL(request.url, "http://receiver").pathname === "/v3/releases",
  );

  expect(registrations).toHaveLength(1);
  expect(registrations[0]?.headers.get("x-interfere-public-key")).toBeNull();

  expect(JSON.parse(registrations[0]?.body ?? "{}")).toMatchObject({
    service: { name: "keyless-next" },
  });
});
