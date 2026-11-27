import { afterAll, beforeAll, expect, test } from "vitest";
import { application } from "@interfere/test-utils/vitest/packed-application";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import { exceptions } from "@interfere/test-utils/vitest/otlp";

let collector: Awaited<ReturnType<typeof receiver>>;
let sentry: Awaited<ReturnType<typeof receiver>>;
let outbound: Awaited<ReturnType<typeof receiver>>;
let app: Awaited<ReturnType<typeof application>>;
let host: Awaited<ReturnType<typeof app.start>>;
const cleanups: (() => Promise<void>)[] = [];

function interfereWarnings() {
  return host
    .output()
    .split("\n")
    .filter((line) => line.includes("[interfere]"));
}

function sentryErrorEvents(message: string) {
  return sentry.requests.filter(
    (request) =>
      request.url.startsWith("/api/1/envelope/") &&
      request.body.includes('"type":"event"') &&
      request.body.includes(message),
  );
}

beforeAll(async () => {
  collector = await receiver();

  cleanups.push(async () => collector.close());

  sentry = await receiver();

  cleanups.push(async () => sentry.close());

  outbound = await receiver();

  cleanups.push(async () => outbound.close());

  app = await application(`${import.meta.dirname}/..`, [
    "@interfere/next",
    "@sentry/nextjs",
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

  await app.copyFixture("coexistence-sentry");

  const env = {
    INTERFERE_API_KEY: "interfere_secret_us_acceptance",
    INTERFERE_API_URL: collector.url,
    INTERFERE_SOURCE_ID: "0123456789abcdef0123456789abcdef01234567",
    INTERFERE_ENVIRONMENT: "preview",
    OUTBOUND_URL: outbound.url,
    SENTRY_DSN: `${sentry.url.replace("://", "://public@")}/1`,
  };

  await app.buildNext(env, "turbopack");

  host = await app.start("serve.ts", env);

  cleanups.push(async () => host.close());
}, 240_000);

afterAll(async () => {
  for (const cleanup of cleanups.toReversed()) {
    // oxlint-disable-next-line eslint/no-await-in-loop -- Resources close in reverse order of creation.
    await cleanup();
  }
});

test("real @sentry/nextjs and Interfere both capture one route error without breaking requests", async () => {
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
    .poll(() => sentryErrorEvents("next-route-error").length, {
      timeout: 15_000,
    })
    .toBeGreaterThan(0);

  await expect
    .poll(() => exceptions(collector.requests, "next-route-error").length, {
      timeout: 15_000,
    })
    .toBeGreaterThan(0);

  const healthz = await fetch(`${host.url}/api/healthz`);

  expect(healthz.status).toBe(200);

  await healthz.text();

  expect(exceptions(collector.requests, "next-route-error")).toHaveLength(1);

  expect(sentryErrorEvents("next-route-error")).toHaveLength(1);

  expect(interfereWarnings()).toStrictEqual([
    expect.stringContaining(
      "[interfere] Adopted the OpenTelemetry tracer provider",
    ),
  ]);
});

test("an outbound fetch under real @sentry/nextjs carries at most one traceparent", async () => {
  expect.hasAssertions();

  for (const attempt of [1, 2]) {
    // oxlint-disable-next-line eslint/no-await-in-loop -- Each outbound call is observed before the next.
    const response = await fetch(`${host.url}/api/outbound`);

    expect(response.status).toBe(200);

    // oxlint-disable-next-line eslint/no-await-in-loop -- Consume each response before continuing.
    await expect(response.json()).resolves.toStrictEqual({ status: 200 });

    expect(
      outbound.requests.filter((request) => request.url === "/outbound"),
    ).toHaveLength(attempt);
  }

  for (const request of outbound.requests) {
    expect(request.headers.get("traceparent") ?? "").not.toContain(",");
  }

  expect(interfereWarnings()).toStrictEqual([
    expect.stringContaining(
      "[interfere] Adopted the OpenTelemetry tracer provider",
    ),
  ]);
});
