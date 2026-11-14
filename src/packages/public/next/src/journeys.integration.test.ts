import type { Page } from "@playwright/test";
import { readFile, readdir } from "node:fs/promises";
import { join } from "node:path";
import {
  assert,
  beforeEach,
  describe,
  expect,
  onTestFinished,
  test,
} from "vitest";
import {
  attribute,
  exceptions,
  exportedSpans,
} from "@interfere/test-utils/vitest/otlp";
import { journey } from "../test/journeys.fixture";

const { open, click, delivered, secret } = journey;

describe("concurrent browser sessions", () => {
  let pages: [Page, Page];

  beforeEach(async () => {
    pages = await Promise.all([open(), open()]);
  });

  test("concurrent browser sessions retain their own server error attribution", async () => {
    const [one, two] = pages;

    const sessions = await Promise.all(
      pages.map(async (page) =>
        page.getByLabel("Session", { exact: true }).textContent(),
      ),
    );

    expect(sessions[0]).not.toBe(sessions[1]);

    await Promise.all([
      journey.failRoute(one, "parallel-one-error"),
      journey.failRoute(two, "parallel-two-error"),
    ]);

    await journey.flushServer();

    const spans = await Promise.all([
      journey.captured("parallel-one-error"),
      journey.captured("parallel-two-error"),
    ]);

    expect(
      spans.map((span) => attribute(span.attributes, "interfere.session.id")),
    ).toStrictEqual(sessions);

    expect(spans[0].traceId).not.toBe(spans[1].traceId);
  });
});

test("successful server actions retain return values and record argument structure without values", async () => {
  const page = await open();

  const value = "private-action-argument-7561";

  await page.getByLabel("Marker").fill(value);

  await click(page, "Submit");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe(`accepted-${value}`);

  await (
    await fetch(
      `${journey.host.url}/api/journey?marker=successful-action-flush`,
    )
  ).text();

  await delivered("successful-action-flush");

  await expect
    .poll(
      () =>
        exportedSpans(journey.collector.requests).filter(
          (span) => span.name === "serverAction journey-submit",
        ).length,
      { timeout: 15_000 },
    )
    .toBe(1);

  const span = exportedSpans(journey.collector.requests).find(
    (item) => item.name === "serverAction journey-submit",
  );

  assert(span);

  expect(attribute(span.attributes, "interfere.server_action.outcome")).toBe(
    "success",
  );

  expect(attribute(span.attributes, "interfere.server_action.args_shape")).toBe(
    "string",
  );

  expect(JSON.stringify(span)).not.toContain(value);
});

test("a failed action is captured once and a later action still succeeds", async () => {
  const page = await open();

  await page.getByLabel("Marker").fill("fail-journey-action");

  await click(page, "Submit");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("action-rejected");

  const error = await delivered("fail-journey-action");

  expect(attribute(error.attributes, "interfere.session.id")).toBe(
    await page.getByLabel("Session", { exact: true }).textContent(),
  );

  await page.getByLabel("Marker").fill("recovered");

  await click(page, "Submit");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("accepted-recovered");

  await click(page, "Close SDK");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("closed");

  expect(
    exceptions(journey.collector.requests, "fail-journey-action"),
  ).toHaveLength(1);
});

test("client navigation preserves the session and captures on the destination", async () => {
  const page = await open();

  const session = await page
    .getByLabel("Session", { exact: true })
    .textContent();

  await page.getByRole("link", { name: "Client navigation" }).click();

  await expect.poll(() => page.url()).toBe(`${journey.host.url}/destination`);

  await expect
    .poll(async () => page.getByLabel("Session", { exact: true }).textContent())
    .toBe(session);

  await click(page, "Capture destination");

  const error = await delivered("destination-error");

  expect(attribute(error.attributes, "session.id")).toBe(session);

  await page.getByRole("link", { name: "Return" }).click();

  await expect.poll(() => page.url()).toBe(`${journey.host.url}/journeys`);

  await expect(
    page.getByLabel("Session", { exact: true }).textContent(),
  ).resolves.toBe(session);
});

test("a full reload restores session attribution without duplicating error listeners", async () => {
  const page = await open();

  const session = await page
    .getByLabel("Session", { exact: true })
    .textContent();

  await page.reload();

  await expect
    .poll(async () => page.getByLabel("Session", { exact: true }).textContent())
    .toBe(session);

  await page.getByLabel("Marker").fill("after-reload-error");

  await click(page, "Capture");

  const error = await delivered("after-reload-error");

  expect(attribute(error.attributes, "session.id")).toBe(session);

  await click(page, "Close SDK");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("closed");

  expect(
    exceptions(journey.collector.requests, "after-reload-error"),
  ).toHaveLength(1);
});

test.each([
  {
    mode: "redirect",
    status: 307,
    control: "NEXT_REDIRECT",
    location: "/api/journey?mode=healthy",
  },
  {
    mode: "missing",
    status: 404,
    control: "NEXT_HTTP_ERROR_FALLBACK",
    location: null,
  },
])(
  "next $mode control flow preserves the response without an SDK exception",
  async ({ mode, status, control, location }) => {
    const response = await fetch(
      `${journey.host.url}/api/journey?mode=${mode}`,
      {
        redirect: "manual",
      },
    );

    expect(response.status).toBe(status);

    await response.text();

    expect(response.headers.get("location")).toBe(location);

    const marker = `after-${mode}-barrier`;

    await (
      await fetch(`${journey.host.url}/api/journey?marker=${marker}`)
    ).text();

    await delivered(marker);

    expect(
      exportedSpans(journey.collector.requests)
        .flatMap((span) => span.events)
        .filter((event) => event.name === "exception")
        .some((event) =>
          String(attribute(event.attributes, "exception.message")).startsWith(
            control,
          ),
        ),
    ).toBe(false);
  },
);

test("instrumented route responses preserve streamed bytes and application headers", async () => {
  const response = await fetch(`${journey.host.url}/api/journey?mode=stream`);

  expect(response.status).toBe(200);
  expect(response.headers.get("x-customer-response")).toBe("preserved");
  expect(response.headers.get("content-type")).toBe("text/plain");

  await expect(response.text()).resolves.toBe("first\nsecond\n");
});

test("server action redirects reach the destination with the existing session", async () => {
  const page = await open();

  const session = await page
    .getByLabel("Session", { exact: true })
    .textContent();

  await click(page, "Redirect action");

  await expect.poll(() => page.url()).toBe(`${journey.host.url}/destination`);

  await expect
    .poll(async () => page.getByLabel("Session", { exact: true }).textContent())
    .toBe(session);

  await expect(
    page.getByRole("heading", { name: "Destination" }).count(),
  ).resolves.toBe(1);
});

test("production failures hide server messages from responses while reporting them to telemetry", async () => {
  const marker = "server-private-detail-9317";

  const response = await fetch(
    `${journey.host.url}/api/journey?marker=${marker}`,
  );

  expect(response.status).toBe(500);

  await expect(response.text()).resolves.not.toContain(marker);

  await delivered(marker);

  const healthy = await fetch(`${journey.host.url}/api/journey?mode=healthy`);

  expect(healthy.status).toBe(200);

  await expect(healthy.json()).resolves.toStrictEqual({ healthy: true });
});

test("production browser assets and telemetry never contain the build API key", async () => {
  const files = await readdir(join(journey.app.root, ".next/static"), {
    recursive: true,
  });

  const assets = await Promise.all(
    files
      .filter((file) => /\.(?:js|map)$/u.test(file))
      .map(async (file) =>
        readFile(join(journey.app.root, ".next/static", file), "utf8"),
      ),
  );

  expect(assets.length).toBeGreaterThan(0);
  expect(assets.join("\n")).not.toContain(secret);

  const page = await open();

  await expect(page.content()).resolves.not.toContain(secret);

  await page.getByLabel("Marker").fill("release-attribution-error");

  await click(page, "Capture");

  const error = await delivered("release-attribution-error");

  expect(attribute(error.resource, "release.slug")).toMatch(/^rel_/u);

  expect(attribute(error.resource, "deployment.environment.name")).toBe(
    "preview",
  );

  expect(
    journey.collector.requests
      .filter((request) => request.url.startsWith("/drain/"))
      .map((request) => request.body)
      .join("\n"),
  ).not.toContain(secret);
});

test("a production build without SDK credentials still serves customer routes", async () => {
  const response = await fetch(
    `${journey.unconfiguredHost.url}/api/journey?mode=healthy`,
  );

  expect(response.status).toBe(200);

  await expect(response.json()).resolves.toStrictEqual({ healthy: true });

  const page = await journey.browser.newPage();

  onTestFinished(async () => page.close());

  expect(
    (await page.goto(`${journey.unconfiguredHost.url}/journeys`))?.status(),
  ).toBe(200);

  await page.getByLabel("Marker").fill("credentials-absent");

  await click(page, "Submit");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("accepted-credentials-absent");

  await click(page, "Capture");

  await click(page, "Close SDK");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("closed");

  expect(
    exceptions(journey.collector.requests, "credentials-absent"),
  ).toHaveLength(0);
});

test("malformed successful release preflight cannot break the production build or disable capture", async () => {
  expect(
    journey.collector.requests.filter(
      (request) => request.url === "/v3/releases/config",
    ).length,
  ).toBeGreaterThan(0);

  const page = await open();

  await page.getByLabel("Marker").fill("malformed-preflight-capture");

  await click(page, "Capture");

  const error = await delivered("malformed-preflight-capture");

  expect(attribute(error.resource, "release.slug")).toMatch(/^rel_/u);
});
