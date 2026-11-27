import { reportChromiumFailures } from "./chromium-diagnostics";
import { chromium } from "@playwright/test";
import type { Browser, Page } from "@playwright/test";
import { afterAll, assert, beforeAll, expect, onTestFinished } from "vitest";
import { application } from "@interfere/test-utils/vitest/packed-application";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import { attribute, exportedSpans } from "@interfere/test-utils/vitest/otlp";

let collector: Awaited<ReturnType<typeof receiver>>;
let app: Awaited<ReturnType<typeof application>>;
let host: Awaited<ReturnType<typeof app.start>>;
let browser: Browser;
let unconfiguredHost: Awaited<ReturnType<typeof app.start>>;
const secret = "interfere_secret_us_journey-build-only";
const cleanups: (() => Promise<void>)[] = [];

beforeAll(async () => {
  collector = await receiver();

  collector.fail("/v3/releases/config", 200, 1, "{}");

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

  await app.copyFixture("journeys");

  const env = {
    INTERFERE_API_KEY: secret,
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

  await startUnconfigured();
}, 180_000);

async function startUnconfigured() {
  const local = await application(`${import.meta.dirname}/..`, [
    "@interfere/next",
    "next",
    "react",
    "react-dom",
    "typescript",
    "@types/node",
    "@types/react",
    "@types/react-dom",
  ]);

  cleanups.push(async () => local.close());

  await local.copyFixture("acceptance");

  await local.copyFixture("journeys");

  const env = { INTERFERE_API_URL: collector.url };

  await local.buildNext(env, "turbopack");

  unconfiguredHost = await local.start("serve.ts", env);

  cleanups.push(async () => unconfiguredHost.close());
}

afterAll(async () => {
  for (const cleanup of cleanups.toReversed()) {
    // oxlint-disable-next-line eslint/no-await-in-loop -- Release dependent application resources before their servers.
    await cleanup();
  }
});

async function open() {
  const context = await browser.newContext();

  onTestFinished(async () => context.close());

  await context.addInitScript(() => {
    Object.assign(globalThis, { __INTERFERE_FORCE_ENABLE__: true });
  });

  const page = await context.newPage();

  reportChromiumFailures(page);

  expect((await page.goto(`${host.url}/journeys`))?.status()).toBe(200);

  await expect
    .poll(async () => page.getByLabel("Session", { exact: true }).textContent())
    .toMatch(/\S/u);

  return page;
}

async function click(page: Page, name: string) {
  await page.getByRole("button", { name, exact: true }).click();
}

async function failRoute(page: Page, marker: string) {
  await page.getByLabel("Marker").fill(marker);

  await click(page, "Fail route");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("route-500");
}

async function delivered(message: string) {
  await expect
    .poll(() => captures(message).length, { timeout: 15_000 })
    .toBe(1);

  const [span] = captures(message);

  assert(span);

  return span;
}

async function flushServer() {
  await (
    await fetch(`${host.url}/api/journey?marker=server-flush-barrier`)
  ).text();
}

function captures(message: string) {
  return exportedSpans(collector.requests).filter((span) =>
    span.events.some(
      (event) =>
        event.name === "exception" &&
        attribute(event.attributes, "exception.message") === message &&
        attribute(event.attributes, "interfere.exception.mechanism") !==
          undefined,
    ),
  );
}

async function captured(message: string) {
  await expect
    .poll(() => captures(message).length, { timeout: 15_000 })
    .toBe(1);

  const [span] = captures(message);

  assert(span);

  return span;
}

export const journey = {
  open,
  click,
  delivered,
  failRoute,
  flushServer,
  captured,
  secret,
  get collector() {
    return collector;
  },
  get app() {
    return app;
  },
  get host() {
    return host;
  },
  get unconfiguredHost() {
    return unconfiguredHost;
  },
  get browser() {
    return browser;
  },
};
