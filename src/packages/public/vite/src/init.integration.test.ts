import { gunzipSync } from "node:zlib";
import { z } from "zod";
import { join } from "node:path";
import { readFile } from "node:fs/promises";
import { chromium } from "@playwright/test";
import type {
  Browser,
  BrowserContext,
  CDPSession,
  Page,
} from "@playwright/test";
import { TraceMap, originalPositionFor } from "@jridgewell/trace-mapping";
import {
  afterAll,
  afterEach,
  assert,
  beforeAll,
  beforeEach,
  expect,
  test,
} from "vitest";
import { application } from "@interfere/test-utils/vitest/packed-application";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import {
  attribute,
  exceptions,
  exportedSpans,
} from "@interfere/test-utils/vitest/otlp";

let app: Awaited<ReturnType<typeof application>>;
let collector: Awaited<ReturnType<typeof receiver>>;
let website: Awaited<ReturnType<typeof receiver>>;
let browser: Browser;
let context: BrowserContext;
let page: Page;
let pageEvents: string[];
let pendingRequests: Set<string>;

beforeAll(async () => {
  collector = await receiver();

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
    VITE_INTERFERE_PUBLIC_KEY: "interfere_public_us_acceptance",
    ACCEPTANCE_INGEST_URL: collector.url,
    INTERFERE_ENVIRONMENT: "preview",
    INTERFERE_SOURCE_ID: "0123456789abcdef0123456789abcdef01234567",
  });

  website.serve(join(app.root, "dist"));

  browser = await chromium.launch({ headless: true });
}, 120_000);

beforeEach(async () => {
  collector.reset();

  context = await browser.newContext();

  await context.addInitScript(() => {
    Object.assign(globalThis, { __INTERFERE_FORCE_ENABLE__: true });
  });

  await context.route("https://**/*", async (route) =>
    route.abort("blockedbyclient"),
  );

  page = await context.newPage();

  pageEvents = [];

  pendingRequests = new Set();

  page.on("request", (request) => pendingRequests.add(request.url()));

  page.on("requestfinished", (request) =>
    pendingRequests.delete(request.url()),
  );

  page.on("pageerror", (error) =>
    pageEvents.push(`pageerror: ${error.message}`),
  );

  page.on("console", (message) => {
    if (message.type() === "error" || message.type() === "warning") {
      pageEvents.push(`console.${message.type()}: ${message.text()}`);
    }
  });

  page.on("requestfailed", (request) => {
    pendingRequests.delete(request.url());

    pageEvents.push(
      `requestfailed: ${request.url()} ${request.failure()?.errorText ?? ""}`,
    );
  });
});

afterEach(async () => context.close());

afterAll(async () => {
  await browser.close();

  await app.close();

  await website.close();

  await collector.close();
});

async function rendered() {
  try {
    await page
      .getByRole("button", { name: "Capture", exact: true })
      .waitFor({ timeout: 30_000 });
  } catch (error) {
    throw new Error(
      `The acceptance application did not render: ${JSON.stringify({
        events: pageEvents.slice(-20),
        pending: [...pendingRequests],
        collector: collector.requests.map(
          (request) => `${request.method} ${request.url} ${request.status}`,
        ),
      })}`,
      { cause: error },
    );
  }
}

async function open(query = "") {
  await page.goto(website.url + query);

  await rendered();

  await expect
    .poll(async () => page.getByLabel("Session", { exact: true }).textContent())
    .toMatch(/\S/u);
}

async function click(name: string) {
  await page.getByRole("button", { name, exact: true }).click();
}

async function delivered(message: string) {
  await expect
    .poll(() => exceptions(collector.requests, message).length, {
      timeout: 15_000,
    })
    .toBe(1);

  const [captured] = exceptions(collector.requests, message);

  assert(captured);

  return captured;
}

async function replayCapture(
  cdp: CDPSession,
  registrationId: string,
  concurrentEvents: 1 | 2,
) {
  expect(registrationId).not.toBe("");

  await expect
    .poll(
      async () => {
        await Promise.all(
          Array.from({ length: concurrentEvents }, async () =>
            cdp.send("ServiceWorker.dispatchSyncEvent", {
              registrationId,
              origin: website.url,
              tag: "workbox-background-sync:acceptance",
              lastChance: false,
            }),
          ),
        );

        return exceptions(collector.requests, "acceptance-manual").length;
      },
      { timeout: 15_000 },
    )
    .toBe(1);
}

test("production installation captures errors once after repeated initialization and preserves the host result", async () => {
  await open();

  await click("Initialize again");

  await click("Capture");

  await click("Span result");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("original-result");

  await click("Span failure");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("same-error");

  await click("Close SDK");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("closed");

  await delivered("acceptance-manual");

  await delivered("acceptance-span-error");

  expect(
    exportedSpans(collector.requests).filter(
      (item) => item.name === "acceptance-result",
    ),
  ).toHaveLength(1);

  await click("Count 0");

  await expect(
    page.getByRole("button", { name: "Count 1" }).count(),
  ).resolves.toBe(1);

  expect(
    collector.requests
      .filter((request) => request.url.startsWith("/drain/"))
      .every((request) => request.headers.get("cookie") === null),
  ).toBe(true);
});

test.each([
  {
    button: "Unhandled error",
    message: "acceptance-unhandled",
    expectedFallbacks: 0,
  },
  { button: "Reject", message: "acceptance-rejection", expectedFallbacks: 0 },
  {
    button: "Render error",
    message: "acceptance-render",
    expectedFallbacks: 1,
  },
])(
  "$button delivers an actual browser exception",
  async ({ button, message, expectedFallbacks }) => {
    await open();

    await click(button);

    const error = await delivered(message);

    expect(attribute(error.attributes, "session.id")).toBe(
      await page.getByLabel("Session", { exact: true }).textContent(),
    );

    await expect(page.getByText("Customer fallback").count()).resolves.toBe(
      expectedFallbacks,
    );

    await click("Count 0");

    await expect(
      page.getByRole("button", { name: "Count 1" }).count(),
    ).resolves.toBe(1);
  },
);

test("account switches identify the new user and logout rotates attribution", async () => {
  await open();

  const first = await page.getByLabel("Session", { exact: true }).textContent();

  await click("Identify A");

  await expect
    .poll(
      () =>
        collector.requests.filter(
          (request) => request.url === "/v3/session/identify",
        ).length,
    )
    .toBe(1);

  await click("Identify B");

  await click("Identify B");

  await expect
    .poll(
      () =>
        collector.requests.filter(
          (request) => request.url === "/v3/session/identify",
        ).length,
    )
    .toBe(2);

  await click("Logout");

  const second = await page
    .getByLabel("Session", { exact: true })
    .textContent();

  expect(second).not.toBe(first);

  await click("Identify B");

  await expect
    .poll(
      () =>
        collector.requests.filter(
          (request) => request.url === "/v3/session/identify",
        ).length,
    )
    .toBe(3);

  const identities = collector.requests
    .filter((request) => request.url === "/v3/session/identify")
    .map((request) =>
      z
        .object({ sessionId: z.string(), identifier: z.string() })
        .parse(JSON.parse(request.body)),
    );

  expect(identities).toMatchObject([
    { sessionId: first, identifier: "consumer-a" },
    { sessionId: first, identifier: "consumer-b" },
    { sessionId: second, identifier: "consumer-b" },
  ]);

  await click("Capture");

  expect(
    attribute((await delivered("acceptance-manual")).attributes, "session.id"),
  ).toBe(second);

  await page.reload();

  await rendered();

  await expect(
    page.getByLabel("Session", { exact: true }).textContent(),
  ).resolves.toBe(second);
});

test("consent gates recording and device storage while necessary errors remain deliverable", async () => {
  expect.hasAssertions();

  await open("/?denied=1");

  await click("Capture");

  await delivered("acceptance-manual");

  expect(
    collector.requests.filter((request) =>
      request.url.startsWith("/v3/replay/"),
    ),
  ).toHaveLength(0);

  expect(
    (await context.cookies()).filter((cookie) =>
      cookie.name.includes("device"),
    ),
  ).toHaveLength(0);

  await click("Grant consent");

  await click("Count 0");

  await click("Close SDK");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("closed");

  await expect
    .poll(
      () =>
        collector.requests.filter((request) =>
          request.url.startsWith("/v3/replay/"),
        ).length,
    )
    .toBeGreaterThan(0);

  const replay = collector.requests
    .filter((request) => request.url.startsWith("/v3/replay/"))
    .map((request) => request.body)
    .join("\n");

  expect(replay).toContain("Customer application");

  for (const secret of [
    "masked-secret-5329",
    "blocked-secret-7451",
    "password-secret-9371",
    "email-secret-6821",
  ]) {
    expect(replay).not.toContain(secret);
  }

  await open("/?denied=1");

  await click("Grant consent");

  await click("Revoke consent");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("revoked");

  await page.getByLabel("Email", { exact: true }).fill("after-revocation-1985");

  await click("Close SDK");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("closed");

  expect(
    collector.requests.map((request) => request.body).join("\n"),
  ).not.toContain("after-revocation-1985");
});

test("a minified captured stack resolves to the customer source and carries release attribution", async () => {
  await open();

  await click("Capture");

  const captured = await delivered("acceptance-manual");

  expect(attribute(captured.resource, "release.slug")).toMatch(/^rel_/u);

  expect(attribute(captured.resource, "deployment.environment.name")).toBe(
    "preview",
  );

  const event = captured.events.find((item) => item.name === "exception");

  assert(event);

  const stack = String(attribute(event.attributes, "exception.stacktrace"));

  const frame =
    /(?<file>http:\/\/[^\s)]+\.js):(?<line>\d+):(?<column>\d+)/u.exec(stack);

  expect(frame).not.toBeNull();

  const groups = frame?.groups;

  assert(groups?.["file"]);

  const file = new URL(groups["file"]);

  const map = new TraceMap(
    await readFile(join(app.root, "dist", `${file.pathname}.map`), "utf8"),
  );

  const original = originalPositionFor(map, {
    line: Number(groups["line"]),
    column: Number(groups["column"]) - 1,
  });

  expect(original.source).toMatch(/main\.tsx$/u);

  const source = await readFile(join(app.root, "main.tsx"), "utf8");

  assert(original.line !== null);

  expect(source.split("\n")[original.line - 1]).toContain("acceptance-manual");
});

test("closing and restarting restores capture without duplicating observers", async () => {
  await open();

  await click("Close SDK");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("closed");

  await click("Capture");

  await click("Restart SDK");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("restarted");

  await click("Capture");

  await click("Close SDK");

  await delivered("acceptance-manual");
});

test.each([429, 503])(
  "the installed service worker durably replays an HTTP %i export once across reload and overlapping sync events",
  async (status) => {
    expect.hasAssertions();

    const cdp = await context.newCDPSession(page);

    let registrationId = "";

    cdp.on("ServiceWorker.workerRegistrationUpdated", ({ registrations }) => {
      registrationId =
        registrations.find(
          (registration) => registration.scopeURL === `${website.url}/`,
        )?.registrationId ?? registrationId;
    });

    await cdp.send("ServiceWorker.enable");

    await open("/?queue=1");

    collector.fail("/drain/v1/traces", status, 100);

    const queued = page.waitForResponse(
      (response) =>
        response.url().includes("/drain/v1/traces") &&
        response.status() === 202 &&
        requestBodyText(response.request()).includes("acceptance-manual"),
    );

    await click("Capture");

    await queued;

    expect(exceptions(collector.requests, "acceptance-manual")).toHaveLength(0);

    await page.reload();

    await rendered();

    collector.fail("/drain/v1/traces", status, 0);

    await replayCapture(cdp, registrationId, 2);

    await click("Count 0");

    await expect(
      page.getByRole("button", { name: "Count 1" }).count(),
    ).resolves.toBe(1);
  },
);

test("offline capture is delivered after a native background sync", async () => {
  const cdp = await context.newCDPSession(page);

  let registrationId = "";

  cdp.on("ServiceWorker.workerRegistrationUpdated", ({ registrations }) => {
    registrationId =
      registrations.find(
        (registration) => registration.scopeURL === `${website.url}/`,
      )?.registrationId ?? registrationId;
  });

  await cdp.send("ServiceWorker.enable");

  await open("/?queue=1");

  await context.setOffline(true);

  const queued = page.waitForResponse(
    (response) =>
      response.url().includes("/drain/v1/traces") &&
      response.status() === 202 &&
      requestBodyText(response.request()).includes("acceptance-manual"),
  );

  await click("Capture");

  await queued;

  expect(exceptions(collector.requests, "acceptance-manual")).toHaveLength(0);

  await context.setOffline(false);

  await replayCapture(cdp, registrationId, 1);
});

test("invalid public keys halt export without a retry storm or breaking the host", async () => {
  await open();

  collector.fail(
    "/drain/",
    401,
    100,
    JSON.stringify({ code: "AUTH_PK_INVALID" }),
  );

  await click("Span result");

  await click("Span result");

  await click("Span result");

  await expect
    .poll(
      () =>
        collector.requests.filter((request) => request.status === 401).length,
    )
    .toBeGreaterThanOrEqual(3);

  const rejected = collector.requests.filter(
    (request) => request.status === 401,
  ).length;

  await click("Capture");

  await click("Close SDK");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("closed");

  expect(
    collector.requests.filter((request) => request.status === 401),
  ).toHaveLength(rejected);

  await click("Count 0");

  await expect(
    page.getByRole("button", { name: "Count 1" }).count(),
  ).resolves.toBe(1);
});

test("unload flushes a captured error without an explicit SDK close", async () => {
  expect.hasAssertions();

  await context.unrouteAll();

  await open();

  await click("Capture");

  await page.goto("about:blank");

  await delivered("acceptance-manual");
});

function requestBodyText(request: { postDataBuffer: () => Buffer | null }) {
  const body = request.postDataBuffer();

  if (!body) {
    return "";
  }

  return (
    body[0] === 0x1f && body[1] === 0x8b ? gunzipSync(body) : body
  ).toString();
}

function replayRequests() {
  return collector.requests.filter((request) =>
    request.url.startsWith("/v3/replay/"),
  );
}

function replayText() {
  return replayRequests()
    .map((request) => request.body)
    .join("\n");
}

test("navigating away right after a small page renders delivers its buffered replay chunk", async () => {
  expect.hasAssertions();

  await context.unrouteAll();

  const teardown = Promise.withResolvers<undefined>();

  collector.hold("/v3/replay/", teardown.promise);

  await open();

  await click("Navigate");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("navigated");

  expect(replayRequests()).toHaveLength(0);

  await page.goto("about:blank");

  teardown.resolve(undefined);

  await expect
    .poll(() => replayRequests().length, { timeout: 4000 })
    .toBeGreaterThan(0);

  expect(replayText()).toContain("Customer application");

  expect(replayText()).toContain("navigated");
});

test("navigating away from a bulky page after its first flush delivers the snapshot and the recorded tail", async () => {
  expect.hasAssertions();

  await context.unrouteAll();

  await page.clock.install();

  await open("/?bulk=1");

  await expect
    .poll(
      async () => {
        await page.clock.fastForward(500);

        return replayText().includes("Customer application");
      },
      { timeout: 4000 },
    )
    .toBe(true);

  const flushed = replayRequests().length;

  const teardown = Promise.withResolvers<undefined>();

  collector.hold("/v3/replay/", teardown.promise);

  await click("Navigate");

  await expect
    .poll(async () => page.getByLabel("Result").textContent())
    .toBe("navigated");

  await page.goto("about:blank");

  teardown.resolve(undefined);

  await expect
    .poll(() => replayRequests().length, { timeout: 4000 })
    .toBeGreaterThan(flushed);

  expect(replayRequests().at(-1)?.body).toContain("navigated");
});
