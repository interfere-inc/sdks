import { nativeBrowserFixture } from "@interfere/test-config/native-browser";
import { HttpResponse, http } from "msw";
import { expect, test } from "vitest";
import type { Options } from "../../../../test/sentry-posthog.fixture.js";

const ORDERS: Options["order"][] = ["vendor-first", "interfere-first"];

const open = nativeBrowserFixture(
  new URL("../../../../test/sentry-posthog.fixture.ts", import.meta.url),
  [
    http.all("*", ({ request }) => {
      const url = new URL(request.url);

      if (url.pathname === "/v3/config") {
        return HttpResponse.json({
          plugins: {
            device: false,
            errors: false,
            logs: false,
            pageEvents: false,
            rageClick: false,
            replay: false,
          },
        });
      }

      if (url.pathname === "/echo") {
        return HttpResponse.json(
          {},
          {
            headers: {
              "x-wire-traceparent": request.headers.get("traceparent") ?? "",
              "x-wire-posthog-session":
                request.headers.get("x-posthog-session-id") ?? "",
            },
          },
        );
      }

      return HttpResponse.json({});
    }),
  ],
);

test.each(ORDERS)(
  "actual Sentry generic correlation works with %s initialization",
  async (order) => {
    expect.hasAssertions();

    const { page } = await open();

    await page.evaluate(
      async (input) =>
        globalThis.sentryPosthogFixture.install({
          vendor: "sentry",
          order: input,
        }),
      order,
    );

    const wire = await page.evaluate(async () =>
      Promise.all([
        globalThis.sentryPosthogFixture.request("fetch"),
        globalThis.sentryPosthogFixture.request("request"),
        globalThis.sentryPosthogFixture.request("xhr"),
      ]),
    );

    await page.waitForFunction(
      () => globalThis.sentryPosthogFixture.breadcrumbs() >= 2,
    );

    const records = await page.evaluate(async () =>
      globalThis.sentryPosthogFixture.snapshot(),
    );

    for (const entry of wire) {
      expect(entry.status).toBe(200);
      expect(entry.traceparent).toMatch(/^00-[a-f0-9]{32}-[a-f0-9]{16}-01$/u);

      const linked = records.filter(
        (record) =>
          record.attributes["interfere.request.trace_id"] ===
          entry.traceparent?.split("-")[1],
      );

      expect(linked.length).toBeGreaterThan(0);

      for (const record of linked) {
        expect(record.attributes["session.id"]).toStrictEqual(
          expect.any(String),
        );
      }
    }
  },
);

test.each(ORDERS)(
  "actual PostHog fetch session headers and XHR coexist with W3C ownership with %s initialization",
  async (order) => {
    expect.hasAssertions();

    const { page } = await open();

    await page.evaluate(
      async (input) =>
        globalThis.sentryPosthogFixture.install({
          vendor: "posthog",
          order: input,
        }),
      order,
    );

    const session = await page.evaluate(() =>
      globalThis.sentryPosthogFixture.posthogSession(),
    );

    const wire = await page.evaluate(async () =>
      Promise.all([
        globalThis.sentryPosthogFixture.request("fetch"),
        globalThis.sentryPosthogFixture.request("request"),
        globalThis.sentryPosthogFixture.request("xhr"),
      ]),
    );

    const records = await page.evaluate(async () =>
      globalThis.sentryPosthogFixture.snapshot(),
    );

    for (const entry of wire) {
      expect(entry.status).toBe(200);
      expect(entry.traceparent).toMatch(/^00-[a-f0-9]{32}-[a-f0-9]{16}-01$/u);
      expect(entry.session).toBe(session);

      expect(
        records.some(
          (record) =>
            record.attributes["interfere.request.trace_id"] ===
            entry.traceparent?.split("-")[1],
        ),
      ).toBe(true);
    }
  },
);

test("actual PostHog alone sends session headers for fetch and XHR", async () => {
  expect.hasAssertions();

  const { page } = await open();

  await page.evaluate(() =>
    globalThis.sentryPosthogFixture.installVendorOnly("posthog"),
  );

  const session = await page.evaluate(() =>
    globalThis.sentryPosthogFixture.posthogSession(),
  );

  const wire = await page.evaluate(async () =>
    Promise.all([
      globalThis.sentryPosthogFixture.request("fetch"),
      globalThis.sentryPosthogFixture.request("request"),
      globalThis.sentryPosthogFixture.request("xhr"),
    ]),
  );

  for (const entry of wire) {
    expect(entry.status).toBe(200);
    expect(entry.traceparent).toBe("");
    expect(entry.session).toBe(session);
  }
});
