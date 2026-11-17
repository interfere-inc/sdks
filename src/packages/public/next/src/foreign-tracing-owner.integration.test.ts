import { describe, expect, test } from "vitest";
import { attribute, exceptions } from "@interfere/test-utils/vitest/otlp";
import {
  FOREIGN_TRACEPARENT,
  flushByLeaving,
  foreignTracingOwnerHarness,
  interfereBaggageKeys,
  traceparentsOf,
} from "../test/foreign-tracing-owner.test-utils.js";
import type { Echo } from "../test/foreign-tracing-owner.test-utils.js";

const harness = foreignTracingOwnerHarness();

const POLL = { timeout: 3000 } as const;

const INTERFERE_TRACEPARENT =
  /^00-(?<trace>[\da-f]{32})-(?<span>[\da-f]{16})-01$/u;

const INTERFERE_BAGGAGE = Array.from({ length: 4 }, () => [
  "interfere.session.id",
  "interfere.device.id",
]).flat();

async function expectObservedUnder(
  label: string,
  session: string | null,
  traceIds: (string | undefined)[],
) {
  await expect.poll(() => harness.observationsFor(label).length, POLL).toBe(4);

  const observations = harness.observationsFor(label);

  expect(
    new Set(
      observations.map((record) =>
        JSON.stringify([
          attribute(record.attributes, "session.id"),
          attribute(record.attributes, "interfere.request.trace_id"),
        ]),
      ),
    ),
  ).toStrictEqual(
    new Set(traceIds.map((traceId) => JSON.stringify([session, traceId]))),
  );
}

async function expectContinued(parentSpanIds: (string | undefined)[]) {
  const continued = new Set(parentSpanIds);

  await expect
    .poll(
      () =>
        harness.hostSpans().filter((span) => continued.has(span.parentSpanId))
          .length,
      POLL,
    )
    .toBeGreaterThanOrEqual(4);
}

function serverTraceIds(echoes: Echo[]) {
  return echoes.map((entry) => entry.server?.traceId);
}

describe("foreign writer installed after Interfere owns W3C, Interfere in application mode", () => {
  test("exactly one foreign context on the wire, the backend continues it, Interfere observes every request under its trace id", async () => {
    expect.hasAssertions();

    const { consoleLines, page, session } = await harness.openJourney({
      order: "interfere-first",
    });

    const { echoes, label } = await harness.journey(page, "owned-foreign");

    await flushByLeaving(page);

    const parents = traceparentsOf(echoes).map((parent) =>
      FOREIGN_TRACEPARENT.exec(parent ?? ""),
    );

    expect(parents.map((match) => match?.groups?.["sampled"])).toStrictEqual([
      "1",
      "1",
      "1",
      "1",
    ]);

    expect(echoes.map((entry) => entry.headers["tracestate"])).toStrictEqual([
      "vendor=s:1",
      "vendor=s:1",
      "vendor=s:1",
      "vendor=s:1",
    ]);

    expect(serverTraceIds(echoes)).toStrictEqual(
      parents.map((match) => `0000000000000000${match?.groups?.["trace"]}`),
    );

    expect(new Set(traceparentsOf(echoes)).size).toBe(4);

    expect(interfereBaggageKeys(echoes)).toStrictEqual(INTERFERE_BAGGAGE);

    await expectObservedUnder(label, session, serverTraceIds(echoes));

    await expectContinued(parents.map((match) => match?.groups?.["span"]));

    expect(
      consoleLines.filter((line) => line.includes("[interfere]")),
    ).toStrictEqual([]);
  });
});

describe("foreign writer installed first on vendor headers only, Interfere in application mode", () => {
  test("interfere writes the only W3C context next to the vendor headers, the backend continues it and every observation carries its trace id", async () => {
    expect.hasAssertions();

    const { consoleLines, page, session } = await harness.openJourney({
      foreign: { styles: ["vendor"] },
      order: "foreign-first",
    });

    const { echoes, label } = await harness.journey(page, "vendor-only");

    await flushByLeaving(page);

    const parents = traceparentsOf(echoes).map((parent) =>
      INTERFERE_TRACEPARENT.exec(parent ?? ""),
    );

    expect(
      echoes.map((entry, index) => [
        parents[index] !== null,
        entry.headers["tracestate"] ?? null,
        /^[\da-f]{16}$/u.test(entry.headers["x-vendor-trace-id"] ?? ""),
      ]),
    ).toStrictEqual(Array.from({ length: 4 }, () => [true, null, true]));

    expect(serverTraceIds(echoes)).toStrictEqual(
      parents.map((match) => match?.groups?.["trace"]),
    );

    expect(new Set(traceparentsOf(echoes)).size).toBe(4);

    expect(interfereBaggageKeys(echoes)).toStrictEqual(INTERFERE_BAGGAGE);

    await expectObservedUnder(label, session, serverTraceIds(echoes));

    await expectContinued(parents.map((match) => match?.groups?.["span"]));

    expect(
      consoleLines.filter((line) => line.includes("[interfere]")),
    ).toStrictEqual([]);
  });
});

test("errors and page views from the same journey carry the Interfere session the request observations carry", async () => {
  expect.hasAssertions();

  const { page, session } = await harness.openJourney();

  const { label } = await harness.journey(page, "correlated");

  await page.getByRole("button", { name: "Capture journey error" }).click();

  await expect
    .poll(
      () =>
        exceptions(harness.interfereRequests(), "example-journey-error").length,
      POLL,
    )
    .toBe(1);

  await flushByLeaving(page);

  const [exception] = exceptions(
    harness.interfereRequests(),
    "example-journey-error",
  );

  expect(attribute(exception?.attributes ?? [], "session.id")).toBe(session);

  await expect.poll(() => harness.observationsFor(label).length, POLL).toBe(4);

  expect(harness.pageLoads(session)).toStrictEqual(["/journey"]);

  expect(
    harness
      .hostSpans()
      .filter((span) =>
        span.events.some((event) => event.name === "exception"),
      ),
  ).toStrictEqual([]);
});

test("an exception captured just before the page leaves is delivered by the leave flush", async () => {
  expect.hasAssertions();

  const { page, session } = await harness.openJourney();

  await harness.journey(page, "leaving");

  await page.getByRole("button", { name: "Capture journey error" }).click();

  await flushByLeaving(page);

  await expect
    .poll(
      () =>
        exceptions(harness.interfereRequests(), "example-journey-error").filter(
          (exception) =>
            attribute(exception.attributes, "session.id") === session,
        ).length,
      POLL,
    )
    .toBe(1);
});

test("a foreign flags-00 context is continued unsampled by the backend, never exported as sampled, and Interfere still records every request", async () => {
  expect.hasAssertions();

  const { page } = await harness.openJourney({
    foreign: { injectUnsampled: true, sampled: false },
  });

  const { echoes, label } = await harness.journey(page, "unsampled");

  await flushByLeaving(page);

  const incoming = traceparentsOf(echoes).map(
    (parent) => FOREIGN_TRACEPARENT.exec(parent ?? "")?.groups,
  );

  expect(incoming.map((groups) => groups?.["sampled"])).toStrictEqual([
    "0",
    "0",
    "0",
    "0",
  ]);

  expect(
    echoes.map((entry) => [entry.server?.traceId, entry.server?.traceFlags]),
  ).toStrictEqual(
    incoming.map((groups) => [`0000000000000000${groups?.["trace"]}`, 0]),
  );

  const marker = await harness.sampledMarker();

  await expect
    .poll(
      () => harness.hostSpans().some((span) => span.traceId === marker),
      POLL,
    )
    .toBe(true);

  const unsampled = new Set(serverTraceIds(echoes));

  expect(
    harness.hostSpans().filter((span) => unsampled.has(span.traceId)),
  ).toStrictEqual([]);

  expect(interfereBaggageKeys(echoes)).toStrictEqual(INTERFERE_BAGGAGE);

  await expect.poll(() => harness.observationsFor(label).length, POLL).toBe(4);
});

const SILENT = [
  { foreign: { sampled: false }, name: "foreign unsampled, nothing injected" },
  { foreign: { sessionDelayMs: 600_000 }, name: "foreign session untracked" },
  { foreign: false, name: "foreign SDK absent" },
] as const;

describe.each(SILENT)("$name", ({ foreign }) => {
  test("application mode writes its own trace context, merges its session into baggage, the backend continues it and every request is recorded under it", async () => {
    expect.hasAssertions();

    const { page, session } = await harness.openJourney({ foreign });

    const { echoes, label } = await harness.journey(page, "independent");

    await flushByLeaving(page);

    const parents = traceparentsOf(echoes).map((parent) =>
      INTERFERE_TRACEPARENT.exec(parent ?? ""),
    );

    expect(
      echoes.map((entry, index) => [
        parents[index] !== null,
        entry.headers["tracestate"] ?? null,
        entry.server?.traceId,
      ]),
    ).toStrictEqual(
      parents.map((match) => [true, null, match?.groups?.["trace"]]),
    );

    expect(new Set(traceparentsOf(echoes)).size).toBe(4);

    expect(interfereBaggageKeys(echoes)).toStrictEqual(INTERFERE_BAGGAGE);

    await expectObservedUnder(label, session, serverTraceIds(echoes));
  });
});

test("an uninitialised Interfere leaves the foreign writer's headers unchanged and records nothing", async () => {
  expect.hasAssertions();

  const { page } = await harness.openJourney({ interfere: false });

  const { echoes, label } = await harness.journey(page, "interfere-off");

  await expect(page.evaluate(() => globalThis.fetch.name)).resolves.toBe(
    "wrappedFetch",
  );

  await flushByLeaving(page);

  expect(
    echoes.map((entry) => [
      FOREIGN_TRACEPARENT.test(entry.headers["traceparent"] ?? ""),
      entry.headers["tracestate"],
      /^[\da-f]{16}$/u.test(entry.headers["x-vendor-trace-id"] ?? ""),
    ]),
  ).toStrictEqual(Array.from({ length: 4 }, () => [true, "vendor=s:1", true]));

  expect(harness.observationsFor(label)).toStrictEqual([]);
});

test("denied analytics keeps necessary request observations, drops analytics-gated navigation and device identity, and never alters the foreign wire", async () => {
  expect.hasAssertions();

  const { page } = await harness.openJourney();

  await page.getByRole("button", { name: "Deny analytics" }).click();

  await page.evaluate(() => {
    globalThis.history.pushState(null, "", "/journey?consent=denied-a");

    globalThis.history.pushState(null, "", "/journey?consent=denied-b");
  });

  const denied = await harness.journey(page, "consent-denied");

  await page.getByRole("button", { name: "Grant analytics" }).click();

  await page.evaluate(() => {
    globalThis.history.pushState(null, "", "/journey?consent=granted");
  });

  const granted = await harness.journey(page, "consent-granted");

  const returnTo = new URL("/journey", page.url()).href;

  await flushByLeaving(page);

  expect(
    [...denied.echoes, ...granted.echoes].map((entry) => [
      FOREIGN_TRACEPARENT.test(entry.headers["traceparent"] ?? ""),
      /session\.id=foreign-[\da-f]{8}/u.test(entry.headers["baggage"] ?? ""),
      interfereBaggageKeys([entry]),
    ]),
  ).toStrictEqual([
    ...Array.from({ length: 4 }, () => [true, true, ["interfere.session.id"]]),
    ...Array.from({ length: 4 }, () => [
      true,
      true,
      ["interfere.session.id", "interfere.device.id"],
    ]),
  ]);

  await expect
    .poll(
      () =>
        ["denied-a", "denied-b", "granted"].map((name) =>
          harness.navigations(`consent=${name}`),
        ),
      POLL,
    )
    .toStrictEqual([[], ["push"], ["push"]]);

  await page.goto(returnTo);

  await expect
    .poll(
      () =>
        [denied, granted].map(
          (entry) => harness.observationsFor(entry.label).length,
        ),
      POLL,
    )
    .toStrictEqual([4, 4]);

  expect(harness.deviceIdsFor(denied.label)).toStrictEqual(new Set([null]));

  expect([...harness.deviceIdsFor(granted.label)]).toStrictEqual([
    expect.stringMatching(/^[\da-f-]{36}$/u),
  ]);
});
