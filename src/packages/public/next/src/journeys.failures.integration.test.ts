import {
  afterAll,
  assert,
  beforeAll,
  describe,
  expect,
  onTestFinished,
  test,
} from "vitest";
import { application } from "@interfere/test-utils/vitest/packed-application";
import { attribute, exportedSpans } from "@interfere/test-utils/vitest/otlp";
import { journey } from "../test/journeys.fixture";

function captureEvents(message: string) {
  return exportedSpans(journey.collector.requests).flatMap((span) =>
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

function spanExceptionCounts(message: string) {
  return exportedSpans(journey.collector.requests)
    .map(
      (span) =>
        span.events.filter(
          (event) =>
            event.name === "exception" &&
            attribute(event.attributes, "exception.message") === message,
        ).length,
    )
    .filter((count) => count > 0);
}

async function failure(message: string) {
  await expect
    .poll(() => captureEvents(message).length, { timeout: 4000 })
    .toBe(1);

  const [values] = captureEvents(message);

  assert(values);

  return values;
}

function digestCaptures(digest: string) {
  return exportedSpans(journey.collector.requests).flatMap((span) =>
    span.events.filter(
      (event) =>
        event.name === "exception" &&
        attribute(event.attributes, "interfere.exception.mechanism") !==
          undefined &&
        (attribute(event.attributes, "interfere.error.digest") ??
          attribute(span.attributes, "interfere.error.digest")) === digest,
    ),
  );
}

async function settled(url: string, marker: string) {
  await (await fetch(`${url}/api/journey?marker=${marker}`)).text();

  await failure(marker);
}

async function visit(url: string) {
  const context = await journey.browser.newContext();

  onTestFinished(async () => context.close());

  await context.addInitScript(() => {
    Object.assign(globalThis, { __INTERFERE_FORCE_ENABLE__: true });
  });

  const page = await context.newPage();

  await page.goto(url);

  return page;
}

describe("app router render failures", () => {
  test("a server component render throw joins the client error boundary by digest and is captured once", async () => {
    const marker = "journey-rsc-render-error";

    const page = await visit(
      `${journey.host.url}/failures/rsc?marker=${marker}`,
    );

    await expect
      .poll(async () => page.getByLabel("Digest").textContent())
      .toMatch(/\S/u);

    const digest = await page.getByLabel("Digest").textContent();

    assert(digest);

    const error = await failure(marker);

    expect(error("interfere.error.digest")).toBe(digest);

    expect(error("interfere.exception.mechanism")).toBe(
      "auto.function.nextjs.on_request_error",
    );

    await settled(journey.host.url, "after-rsc-render-barrier");

    expect(captureEvents(marker)).toHaveLength(1);

    expect(digestCaptures(digest)).toHaveLength(1);

    expect(error("interfere.nextjs.route_type")).toBe("render");
  });

  test("a suspense child that throws after the shell was sent keeps the 200 response and is captured once", async () => {
    const marker = "journey-suspense-error";

    const response = await fetch(
      `${journey.host.url}/failures/suspense?marker=${marker}`,
    );

    expect(response.status).toBe(200);

    await expect(response.text()).resolves.toContain("Streamed shell");

    const error = await failure(marker);

    expect(error("interfere.error.digest")).toMatch(/\S/u);

    await settled(journey.host.url, "after-suspense-barrier");

    expect(captureEvents(marker)).toHaveLength(1);

    expect(error("interfere.nextjs.route_type")).toBe("render");
  });

  test("a generateMetadata throw is captured once as a render failure", async () => {
    const marker = "journey-metadata-error";

    await (
      await fetch(`${journey.host.url}/failures/metadata?marker=${marker}`)
    ).text();

    const error = await failure(marker);

    expect(error("interfere.exception.mechanism")).toBe(
      "auto.function.nextjs.on_request_error",
    );

    await settled(journey.host.url, "after-metadata-barrier");

    expect(captureEvents(marker)).toHaveLength(1);

    expect(error("interfere.nextjs.route_type")).toBe("render");
  });

  test("a root layout throw renders global-error and is captured once by the browser", async () => {
    const marker = "journey-global-error";

    const page = await visit(`${journey.host.url}/global-failure#${marker}`);

    await expect
      .poll(async () =>
        page.getByRole("heading", { name: "Global failure" }).count(),
      )
      .toBe(1);

    await expect(page.getByLabel("Global message").textContent()).resolves.toBe(
      marker,
    );

    await page.goto("about:blank");

    await failure(marker);

    await settled(journey.host.url, "after-global-error-barrier");

    expect(captureEvents(marker)).toHaveLength(1);
  });
});

test("a throw inside an after() callback keeps the response and is captured exactly once", async () => {
  const marker = "journey-after-callback-error";

  const response = await fetch(
    `${journey.host.url}/api/after?marker=${marker}`,
  );

  expect(response.status).toBe(200);

  await expect(response.json()).resolves.toStrictEqual({ scheduled: true });

  expect(captureEvents(marker)).toHaveLength(0);

  const error = await failure(marker);

  expect(error("interfere.exception.mechanism")).toBe(
    "auto.function.nextjs.after",
  );

  expect(error("interfere.exception.handled")).toBe(false);

  await settled(journey.host.url, "after-callback-barrier");

  expect(captureEvents(marker)).toHaveLength(1);
});

test("a proxy throw is captured exactly once with the proxy mechanism and route type", async () => {
  const marker = "journey-proxy-error";

  const response = await fetch(
    `${journey.host.url}/proxy-failure?marker=${marker}`,
  );

  expect(response.status).toBe(500);

  await response.text();

  const error = await failure(marker);

  expect(error("interfere.exception.mechanism")).toBe(
    "auto.function.nextjs.proxy",
  );

  await settled(journey.host.url, "after-proxy-barrier");

  expect(captureEvents(marker)).toHaveLength(1);

  expect(spanExceptionCounts(marker)).toStrictEqual([1]);

  expect(error("interfere.nextjs.route_type")).toBe("proxy");
});

function launchEnvironment(command: "start" | "dev") {
  return {
    INTERFERE_PUBLIC_KEY: "interfere_public_us_journeys",
    INTERFERE_API_KEY: journey.secret,
    INTERFERE_API_URL: journey.collector.url,
    INTERFERE_SOURCE_ID: "0123456789abcdef0123456789abcdef01234567",
    INTERFERE_ENVIRONMENT: "preview",
    OTEL_TRACES_SAMPLER: "always_on",
    NEXT_LAUNCH_COMMAND: command,
  };
}

async function launchedJourney(url: string, prefix: string) {
  const routeMarker = `${prefix}-route-error`;

  const route = await fetch(`${url}/api/journey?marker=${routeMarker}`);

  expect(route.status).toBe(500);

  await route.text();

  const routeError = await failure(routeMarker);

  expect(routeError("interfere.exception.mechanism")).toBe(
    "auto.function.nextjs.route_handler",
  );

  const renderMarker = `${prefix}-render-error`;

  await (await fetch(`${url}/failures/rsc?marker=${renderMarker}`)).text();

  const renderError = await failure(renderMarker);

  expect(renderError("interfere.error.digest")).toMatch(/\S/u);

  await settled(url, `${prefix}-barrier`);

  expect(captureEvents(routeMarker)).toHaveLength(1);

  expect(captureEvents(renderMarker)).toHaveLength(1);
}

describe("next start", () => {
  let started: Awaited<ReturnType<typeof journey.app.start>>;

  beforeAll(async () => {
    started = await journey.app.start(
      "launch-next.ts",
      launchEnvironment("start"),
    );
  }, 60_000);

  afterAll(async () => started.close());

  test("the production build served by next start captures route and render failures once", async () => {
    expect.hasAssertions();

    await launchedJourney(started.url, "next-start");
  });
});

describe("next dev", () => {
  let dev: Awaited<ReturnType<typeof application>>;
  let started: Awaited<ReturnType<typeof dev.start>> | undefined;

  beforeAll(async () => {
    dev = await application(`${import.meta.dirname}/..`, [
      "@interfere/next",
      "next",
      "react",
      "react-dom",
      "typescript",
      "@types/node",
      "@types/react",
      "@types/react-dom",
    ]);

    await dev.copyFixture("acceptance");

    await dev.copyFixture("journeys");

    started = await dev.start("launch-next.ts", launchEnvironment("dev"));

    await (await fetch(`${started.url}/failures/rsc?marker=dev-warmup`)).text();
  }, 120_000);

  afterAll(async () => {
    await started?.close();

    await dev.close();
  });

  test("next dev captures route and render failures once", async () => {
    expect.hasAssertions();

    assert(started);

    await launchedJourney(started.url, "next-dev");
  });
});
