import { nativeBrowserFixture } from "@interfere/test-config/native-browser";
import { HttpResponse, http } from "msw";
import { expect, test } from "vitest";

const open = nativeBrowserFixture(
  new URL("../../../../test/datadog.fixture.ts", import.meta.url),
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
              "x-wire-baggage": request.headers.get("baggage") ?? "",
            },
          },
        );
      }

      return new HttpResponse(null, { status: 204 });
    }),
  ],
);

const cases = (["datadog-first", "interfere-first"] as const).flatMap((order) =>
  [false, true].map((baggage) => ({
    order,
    baggage,
    baggagePattern: baggage ? /^interfere\.session\.id=.+$/u : /^$/u,
  })),
);

test.each(cases)(
  "actual Datadog $order baggage=$baggage preserves one correlated trace per request",
  async (options) => {
    expect.hasAssertions();

    const { page } = await open();

    await page.evaluate(
      async (input) =>
        globalThis.datadogFixture.install({ ...input, sampled: true }),
      options,
    );

    await page.waitForFunction(() => globalThis.datadogFixture.ready());

    const wire = await page.evaluate(async () =>
      Promise.all([
        globalThis.datadogFixture.request("fetch"),
        globalThis.datadogFixture.request("request"),
        globalThis.datadogFixture.request("xhr"),
      ]),
    );

    await page.waitForFunction(
      () => globalThis.datadogFixture.resourceEvents() === 3,
    );

    const records = await page.evaluate(async () =>
      globalThis.datadogFixture.snapshot(),
    );

    expect(new Set(wire.map((entry) => entry.traceparent)).size).toBe(3);

    for (const entry of wire) {
      const header = entry.traceparent;

      expect(entry.status).toBe(200);
      expect(header).toMatch(/^00-[a-f0-9]{32}-[a-f0-9]{16}-01$/u);

      const linked = records.filter(
        (record) =>
          record.attributes["interfere.request.trace_id"] ===
          header?.split("-")[1],
      );

      expect(linked.length).toBeGreaterThan(0);

      expect(entry.baggage).toMatch(options.baggagePattern);

      for (const record of linked) {
        expect(record.attributes["session.id"]).toStrictEqual(
          expect.any(String),
        );
      }
    }
  },
);

test.each([false, true])(
  "actual Datadog unsampled injectAll=%s never invents a trace association",
  async (injectAll) => {
    expect.hasAssertions();

    const { page } = await open();

    await page.evaluate(
      async (all) =>
        globalThis.datadogFixture.install({
          order: "datadog-first",
          baggage: false,
          sampled: false,
          injectAll: all,
        }),
      injectAll,
    );

    await page.waitForFunction(() => globalThis.datadogFixture.ready());

    const wire = await page.evaluate(async () =>
      Promise.all([
        globalThis.datadogFixture.request("fetch"),
        globalThis.datadogFixture.request("request"),
        globalThis.datadogFixture.request("xhr"),
      ]),
    );

    await page.waitForFunction(
      () => globalThis.datadogFixture.resourceEvents() === 3,
    );

    const records = await page.evaluate(async () =>
      globalThis.datadogFixture.snapshot(),
    );

    const expectedPattern = injectAll
      ? /^00-[a-f0-9]{32}-[a-f0-9]{16}-00$/u
      : /^$/u;

    for (const entry of wire) {
      const header = entry.traceparent;

      expect(entry.status).toBe(200);
      expect(header).toMatch(expectedPattern);
    }

    const expectedIds = wire.flatMap(({ traceparent }) =>
      traceparent ? [traceparent.split("-")[1]] : [],
    );

    const actualIds = records.flatMap((record) =>
      record.attributes["interfere.request.trace_id"] === undefined
        ? []
        : [record.attributes["interfere.request.trace_id"]],
    );

    expect(new Set(actualIds)).toStrictEqual(new Set(expectedIds));
  },
);

test.each(["revoke", "revokeRegrant", "dispose"] as const)(
  "delayed actual Datadog callback after %s cannot emit identity",
  async (operation) => {
    expect.hasAssertions();

    const { page } = await open();

    await page.evaluate(async () =>
      globalThis.datadogFixture.install({
        order: "datadog-first",
        baggage: false,
        sampled: true,
        holdBridge: true,
      }),
    );

    await page.waitForFunction(() => globalThis.datadogFixture.ready());

    const wire = await page.evaluate(async () =>
      globalThis.datadogFixture.request("fetch"),
    );

    expect(wire.traceparent).toMatch(/^00-[a-f0-9]{32}-[a-f0-9]{16}-01$/u);

    await page.waitForFunction(
      () => globalThis.datadogFixture.resourceEvents() === 1,
    );

    await page.evaluate(async (op) => {
      await globalThis.datadogFixture[op]();

      globalThis.datadogFixture.release();
    }, operation);

    const records = await page.evaluate(async () =>
      globalThis.datadogFixture.snapshot(),
    );

    expect(
      records.some(
        (record) =>
          record.attributes["interfere.request.trace_id"] ===
          wire.traceparent?.split("-")[1],
      ),
    ).toBe(false);
  },
);

test("delayed actual Datadog callback keeps request-time session after rotation", async () => {
  expect.hasAssertions();

  const { page } = await open();

  await page.evaluate(async () =>
    globalThis.datadogFixture.install({
      order: "datadog-first",
      baggage: false,
      sampled: true,
      holdBridge: true,
    }),
  );

  await page.waitForFunction(() => globalThis.datadogFixture.ready());

  const session = await page.evaluate(() =>
    globalThis.datadogFixture.session(),
  );

  const wire = await page.evaluate(async () =>
    globalThis.datadogFixture.request("fetch"),
  );

  await page.waitForFunction(
    () => globalThis.datadogFixture.resourceEvents() === 1,
  );

  const next = await page.evaluate(() => {
    globalThis.datadogFixture.rotate();

    globalThis.datadogFixture.release();

    return globalThis.datadogFixture.session();
  });

  expect(next).not.toBe(session);

  const records = await page.evaluate(async () =>
    globalThis.datadogFixture.snapshot(),
  );

  const linked = records.filter(
    (record) =>
      record.attributes["interfere.request.trace_id"] ===
      wire.traceparent?.split("-")[1],
  );

  expect(linked).toHaveLength(1);
  expect(linked[0]?.attributes["session.id"]).toBe(session);
});
