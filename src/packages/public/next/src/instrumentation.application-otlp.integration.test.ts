import { afterAll, beforeAll, expect, test } from "vitest";
import { application } from "@interfere/test-utils/vitest/packed-application";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import { attribute, exceptions } from "@interfere/test-utils/vitest/otlp";
import { EXCEPTION_DELIVERY_ATTRIBUTE } from "@interfere/types/sdk/exception-delivery";

let collector: Awaited<ReturnType<typeof receiver>>;
let app: Awaited<ReturnType<typeof application>>;
let host: Awaited<ReturnType<typeof app.start>>;
const cleanups: (() => Promise<void>)[] = [];

const HOST_KEY = "interfere_secret_us_acceptance";

function hostTraceExports() {
  return collector.requests.filter(
    (request) =>
      request.url === "/drain/v1/traces" &&
      request.headers.get("authorization") === `Bearer ${HOST_KEY}`,
  );
}

function interfereTraceExports() {
  return collector.requests.filter(
    (request) =>
      request.url.startsWith("/drain/v1/traces?public_key=") &&
      request.headers.get("content-type")?.includes("json"),
  );
}

beforeAll(async () => {
  collector = await receiver();

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
    "@vercel/otel",
  ]);

  cleanups.push(async () => app.close());

  await app.copyFixture("acceptance");

  await app.copyFixture("application-otlp");

  const env = {
    INTERFERE_PUBLIC_KEY: "interfere_public_us_acceptance",
    INTERFERE_API_KEY: HOST_KEY,
    INTERFERE_API_URL: collector.url,
    INTERFERE_SOURCE_ID: "0123456789abcdef0123456789abcdef01234567",
    INTERFERE_ENVIRONMENT: "preview",
    OTEL_TRACES_SAMPLER: "always_on",
  };

  await app.buildNext(env, "turbopack");

  host = await app.start("serve.ts", {
    ...env,
    OTEL_BSP_SCHEDULE_DELAY: "250",
    OTEL_BSP_MAX_EXPORT_BATCH_SIZE: "1",
    OTEL_EXPORTER_OTLP_ENDPOINT: `${collector.url}/drain`,
    OTEL_EXPORTER_OTLP_PROTOCOL: "http/protobuf",
    OTEL_EXPORTER_OTLP_HEADERS: `Authorization=Bearer%20${HOST_KEY}`,
  });

  cleanups.push(async () => host.close());
}, 240_000);

afterAll(async () => {
  for (const cleanup of cleanups.toReversed()) {
    // oxlint-disable-next-line eslint/no-await-in-loop -- Resources close in reverse order of creation.
    await cleanup();
  }
});

test("a host provider exporting over standard OTLP keeps every OpenTelemetry global while Interfere captures in application mode (example C)", async () => {
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
    .poll(
      () =>
        hostTraceExports().filter(
          (request) =>
            request.body.includes("next-route-error") &&
            request.body.includes(EXCEPTION_DELIVERY_ATTRIBUTE),
        ).length,
      { timeout: 15_000 },
    )
    .toBeGreaterThanOrEqual(1);

  const hostExport = hostTraceExports().find(
    (request) =>
      request.body.includes("next-route-error") &&
      request.body.includes(EXCEPTION_DELIVERY_ATTRIBUTE),
  );

  expect(hostExport?.headers.get("content-type")).toBe(
    "application/x-protobuf",
  );

  expect(hostExport?.headers.has("x-interfere-public-key")).toBe(false);

  expect(hostExport?.body).toContain("storefront");

  expect(hostExport?.body).toContain(EXCEPTION_DELIVERY_ATTRIBUTE);

  await expect
    .poll(
      () => exceptions(interfereTraceExports(), "next-route-error").length,
      { timeout: 15_000 },
    )
    .toBe(1);

  const [delivered] = exceptions(interfereTraceExports(), "next-route-error");

  expect(attribute(delivered?.attributes ?? [], "interfere.capture.kind")).toBe(
    "exception",
  );

  expect(
    attribute(
      delivered?.events[0]?.attributes ?? [],
      EXCEPTION_DELIVERY_ATTRIBUTE,
    ),
  ).toBeUndefined();

  expect(delivered?.links).toHaveLength(1);

  expect(delivered?.links[0]?.traceId).not.toBe(delivered?.traceId);

  expect((await fetch(`${host.url}/api/healthz`)).status).toBe(200);

  expect(host.output()).not.toContain("Another OpenTelemetry tracer provider");

  expect(host.output()).not.toContain("overrides the");

  expect(host.output()).not.toContain("is deprecated");
});
