import { chromium } from "@playwright/test";
import type { Browser } from "@playwright/test";
import { afterAll, beforeAll, expect, onTestFinished, test } from "vitest";
import { application } from "@interfere/test-utils/vitest/packed-application";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import { exceptions, exportedSpans } from "@interfere/test-utils/vitest/otlp";

let collector: Awaited<ReturnType<typeof receiver>>;
let app: Awaited<ReturnType<typeof application>>;
let host: Awaited<ReturnType<typeof app.start>>;
let browser: Browser;
const cleanups: (() => Promise<void>)[] = [];

beforeAll(async () => {
  collector = await receiver({
    sdkConfig: { platform: "vercel", serverTracing: "drain" },
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
    "@opentelemetry/api",
    "@opentelemetry/sdk-trace",
  ]);

  cleanups.push(async () => app.close());

  await app.copyFixture("acceptance");

  await app.copyFixture("coexistence");

  const env = {
    INTERFERE_PUBLIC_KEY: "interfere_public_us_acceptance",
    INTERFERE_API_KEY: "interfere_secret_us_acceptance",
    INTERFERE_API_URL: collector.url,
    INTERFERE_SOURCE_ID: "0123456789abcdef0123456789abcdef01234567",
    INTERFERE_ENVIRONMENT: "preview",
    OTEL_TRACES_SAMPLER: "always_on",
  };

  await app.buildNext(env, "turbopack");

  host = await app.start("serve.ts", env);

  cleanups.push(async () => host.close());

  browser = await chromium.launch({ headless: true });

  cleanups.push(async () => browser.close());
}, 180_000);

afterAll(async () => {
  for (const cleanup of cleanups.toReversed()) {
    // oxlint-disable-next-line eslint/no-await-in-loop -- Resources close in reverse order of creation.
    await cleanup();
  }
});

test("a prior tracer provider keeps healthy routes available and onRequestError captures exceptions", async () => {
  expect.hasAssertions();

  for (const path of ["/", "/api/healthz"]) {
    // oxlint-disable-next-line eslint/no-await-in-loop -- Exercise each route before the intentional failure.
    const response = await fetch(host.url + path);

    expect(response.status).toBe(200);

    // oxlint-disable-next-line eslint/no-await-in-loop -- Consume each response before continuing.
    await response.text();
  }

  const failure = await fetch(`${host.url}/api/failure`);

  expect(failure.status).toBe(500);

  await failure.text();

  await expect
    .poll(() => exceptions(collector.requests, "next-route-error").length, {
      timeout: 15_000,
    })
    .toBe(1);

  expect((await fetch(`${host.url}/api/healthz`)).status).toBe(200);

  expect(host.output()).not.toContain(
    "[interfere] Another OpenTelemetry tracer provider",
  );
});

test("a missing replay chunk leaves other lazy plugins and application interactions running", async () => {
  const context = await browser.newContext();

  await context.addInitScript(() => {
    Object.assign(globalThis, { __INTERFERE_FORCE_ENABLE__: true });
  });

  onTestFinished(async () => context.close());

  const page = await context.newPage();

  const blocked = Promise.withResolvers<string>();
  const pluginFailure = Promise.withResolvers<string>();

  page.on("console", (message) => {
    if (
      message.text().includes("failed to load plugin") &&
      message.text().includes("replay")
    ) {
      pluginFailure.resolve(message.text());
    }
  });

  await page.route(/\/_next\/static\/.*\.js(?:\?.*)?$/u, async (route) => {
    const response = await route.fetch();

    const body = await response.text();

    if (body.includes("replay halted")) {
      blocked.resolve(route.request().url());

      // oxlint-disable-next-line landmines/no-test-doubles -- Return a real HTTP404 for the requested built chunk to exercise module loading failure.
      await route.fulfill({
        status: 404,
        contentType: "text/javascript",
        body: "",
      });
    } else {
      await route.fulfill({ response, body });
    }
  });

  expect((await page.goto(host.url))?.status()).toBe(200);

  await expect(blocked.promise).resolves.toContain("/_next/static/");

  await expect(pluginFailure.promise).resolves.toContain("replay");

  const before = exportedSpans(collector.requests).filter(
    (span) => span.name === "rage_click",
  ).length;

  await page
    .getByRole("button", { name: "Rage target" })
    .click({ clickCount: 3 });

  await page.getByRole("button", { name: "Successful action" }).click();

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("action-result");

  await page.getByRole("button", { name: "Capture client error" }).click();

  await expect
    .poll(() => exceptions(collector.requests, "next-client-error").length, {
      timeout: 15_000,
    })
    .toBe(1);

  // WARNING: rage_click waits in the 60 s trace batch; only the exception flush above exports it, so assert it after that flush.
  await expect
    .poll(
      () =>
        exportedSpans(collector.requests).filter(
          (span) => span.name === "rage_click",
        ).length,
      { timeout: 15_000 },
    )
    .toBe(before + 1);
});
