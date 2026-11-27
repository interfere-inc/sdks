import { performance } from "node:perf_hooks";
import { afterAll, assert, beforeAll, expect, test } from "vitest";
import { application } from "@interfere/test-utils/vitest/packed-application";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import { attribute, exportedSpans } from "@interfere/test-utils/vitest/otlp";

let collector: Awaited<ReturnType<typeof receiver>>;
let app: Awaited<ReturnType<typeof application>>;
let host: Awaited<ReturnType<typeof app.start>>;
const cleanups: (() => Promise<void>)[] = [];

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
  ]);

  cleanups.push(async () => app.close());

  await app.copyFixture("acceptance");

  await app.copyFixture("pages-router");

  const env = {
    INTERFERE_API_KEY: "interfere_secret_us_pages_router",
    INTERFERE_API_URL: collector.url,
    INTERFERE_SOURCE_ID: "0123456789abcdef0123456789abcdef01234567",
    INTERFERE_ENVIRONMENT: "preview",
    OTEL_TRACES_SAMPLER: "always_on",
  };

  await app.buildNext(env, "turbopack");

  host = await app.start("serve.ts", env);

  cleanups.push(async () => host.close());
}, 180_000);

afterAll(async () => {
  for (const cleanup of cleanups.toReversed()) {
    // oxlint-disable-next-line eslint/no-await-in-loop -- Resources close in reverse order of creation.
    await cleanup();
  }
});

function captureEvents(message: string) {
  return exportedSpans(collector.requests).flatMap((span) =>
    span.events
      .filter(
        (event) =>
          event.name === "exception" &&
          attribute(event.attributes, "exception.message") === message &&
          attribute(event.attributes, "interfere.exception.mechanism") !==
            undefined,
      )
      .map(
        (event) => (key: string) =>
          attribute(event.attributes, key) ?? attribute(span.attributes, key),
      ),
  );
}

async function delivered(message: string) {
  await expect
    .poll(() => captureEvents(message).length, { timeout: 4000 })
    .toBe(1);

  const [values] = captureEvents(message);

  assert(values);

  return values;
}

async function timed(path: string) {
  const started = performance.now();

  const response = await fetch(`${host.url}${path}`);

  await response.text();

  return { status: response.status, elapsed: performance.now() - started };
}

test.each([
  {
    kind: "getServerSideProps",
    path: "/server-props",
    routeType: "render",
    mechanism: "auto.function.nextjs.on_request_error",
  },
  {
    kind: "API route",
    path: "/api/pages-failure",
    routeType: "route",
    mechanism: "auto.function.nextjs.route_handler",
  },
])(
  "a $kind throw is captured without holding the response for the telemetry flush",
  async ({ kind, path, routeType, mechanism }) => {
    const warm = await timed(path);

    expect(warm.status).toBe(200);

    const marker = `pages-${kind.replaceAll(" ", "-").toLowerCase()}-error`;

    collector.stall("/drain/v1/traces", 2500);

    const failure = await timed(`${path}?marker=${marker}`);

    expect(failure.status).toBe(500);

    expect(failure.elapsed).toBeLessThan(1500);

    const error = await delivered(marker);

    expect(error("interfere.exception.mechanism")).toBe(mechanism);

    expect(error("interfere.nextjs.router_kind")).toBe("Pages Router");

    expect(error("interfere.nextjs.route_type")).toBe(routeType);
  },
);
