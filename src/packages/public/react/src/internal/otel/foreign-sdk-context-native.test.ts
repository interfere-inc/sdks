import {
  SPAN_ID,
  TRACEPARENT,
  TRACE_ID,
  collectWarnings,
  driver,
  foreignSdkPages,
  freshnessStamp,
} from "../../../test/foreign-sdk.driver.js";
import type { SeenRequest } from "../../../test/foreign-sdk.driver.js";
import { beforeEach, expect, test } from "vitest";

const seen: SeenRequest[] = [];

const createPage = foreignSdkPages(seen);

beforeEach(() => {
  seen.length = 0;
});

test("interfere registers no global propagator before or after close() (B10a)", async () => {
  const runtime = driver((await createPage()).page);

  await expect(runtime.install()).resolves.toBe(1);

  const open = await runtime.state();

  expect(open.fields).toStrictEqual([]);

  await runtime.close();

  const closed = await runtime.state();

  expect(closed.fields).toStrictEqual([]);
});

test("a foreign global propagator registered first is kept (B10b)", async () => {
  const runtime = driver((await createPage()).page);

  await expect(runtime.foreign("propagator")).resolves.toBe(true);

  await expect(runtime.install()).resolves.toBe(1);

  const state = await runtime.state();

  expect(state.fields).toStrictEqual(["x-foreign-trace"]);
});

test("auto leaves the OpenTelemetry globals to a foreign propagator, writes traceparent where absent, merges its session into baggage and does not warn (B10c)", async () => {
  const { page } = await createPage();

  const warnings = collectWarnings(page);
  const diagnostics: string[] = [];

  page.on("console", (message) => {
    if (
      message.type() === "info" &&
      message.text().includes("OpenTelemetry owns tracing on this page")
    ) {
      diagnostics.push(message.text());
    }
  });

  const runtime = driver(page);

  await expect(runtime.foreign("propagator")).resolves.toBe(true);

  await expect(runtime.install()).resolves.toBe(1);

  await runtime.request("/api/foreign-propagator");

  expect(seen[0]?.headers["x-foreign-trace"]).toBeNull();

  expect(seen[0]?.headers["traceparent"]).toMatch(TRACEPARENT);

  expect(seen[0]?.headers["baggage"]).toMatch(/^interfere\.session\.id=/u);

  expect(
    warnings.filter((text) =>
      /another propagator owns trace headers/iu.test(text),
    ),
  ).toHaveLength(0);

  expect(
    warnings.filter((text) => text.includes("owns trace headers")),
  ).toStrictEqual([]);

  expect(diagnostics).toStrictEqual([]);

  await runtime.close();

  await expect(runtime.install()).resolves.toBe(2);

  await runtime.request("/api/foreign-propagator-reinit");

  expect(
    warnings.filter((text) => text.includes("owns trace headers")),
  ).toStrictEqual([]);

  expect(diagnostics).toStrictEqual([]);
});

test("init() is idempotent and init() after close() creates a fresh kernel (B11)", async () => {
  const runtime = driver((await createPage()).page);

  await expect(runtime.install()).resolves.toBe(1);

  await expect(runtime.install()).resolves.toBe(1);

  await runtime.close();

  await expect(runtime.install()).resolves.toBe(2);
});

async function installWithSsrContext(flags: "00" | "01") {
  const runtime = driver((await createPage()).page);

  await expect(
    runtime.install({
      meta: {
        traceparent: `00-${TRACE_ID}-${SPAN_ID}-${flags}`,
        tracestate: `${freshnessStamp()},vendor=abc`,
      },
    }),
  ).resolves.toBe(1);

  await runtime.request("/api/rooted");

  await expect.poll(async () => (await runtime.pageViews()).length).toBe(1);

  const spans = await runtime.spans();

  return {
    traceparent: seen[0]?.headers["traceparent"],
    links: spans
      .flatMap((span) => span.links)
      .filter((link) => link.traceId === TRACE_ID),
    pageViews: await runtime.pageViews(),
  };
}

test("a fresh stamped sampled document context is kept on the page view and the first request starts a browser-rooted trace (B13a)", async () => {
  const result = await installWithSsrContext("01");

  expect(result.traceparent).toMatch(TRACEPARENT);

  expect(result.traceparent).not.toContain(TRACE_ID);

  expect(result.links).toStrictEqual([]);

  expect(result.pageViews).toStrictEqual([
    { trigger: "pageload", traceId: TRACE_ID, traceFlags: 1 },
  ]);
});

test("a fresh stamped unsampled document context is kept on the page view with flags 00 and the first request starts a browser-rooted trace (B13b)", async () => {
  const result = await installWithSsrContext("00");

  expect(result.traceparent).toMatch(TRACEPARENT);

  expect(result.traceparent).not.toContain(TRACE_ID);

  expect(result.links).toStrictEqual([]);

  expect(result.pageViews).toStrictEqual([
    { trigger: "pageload", traceId: TRACE_ID, traceFlags: 0 },
  ]);
});

const stampedAgo = (seconds: number) =>
  `interfere=${Math.floor(Date.now() / 1000) - seconds}`;

test.each([
  {
    case: "fresh sampled",
    meta: () => ({
      traceparent: `00-${TRACE_ID}-${SPAN_ID}-01`,
      tracestate: stampedAgo(0),
    }),
  },
  {
    case: "fresh unsampled",
    meta: () => ({
      traceparent: `00-${TRACE_ID}-${SPAN_ID}-00`,
      tracestate: stampedAgo(0),
    }),
  },
  {
    case: "stale",
    meta: () => ({
      traceparent: `00-${TRACE_ID}-${SPAN_ID}-01`,
      tracestate: stampedAgo(600),
    }),
  },
  {
    case: "unstamped",
    meta: () => ({ traceparent: `00-${TRACE_ID}-${SPAN_ID}-01` }),
  },
])(
  "document load is browser-rooted under $case SSR meta and keeps its fetch child",
  async ({ meta }) => {
    const runtime = driver((await createPage()).page);

    await expect(runtime.install({ meta: meta() })).resolves.toBe(1);

    await expect
      .poll(async () =>
        (await runtime.spans()).some((span) => span.name === "documentLoad"),
      )
      .toBe(true);

    const spans = await runtime.spans();

    const load = spans.find((span) => span.name === "documentLoad");
    const fetchSpan = spans.find((span) => span.name === "documentFetch");

    expect(load?.parentSpanId).toBeNull();
    expect(load?.traceId).not.toBe(TRACE_ID);
    expect(fetchSpan?.parentSpanId).toBe(load?.spanId);
    expect(spans.every((span) => span.traceId !== TRACE_ID)).toBe(true);

    expect((await runtime.state()).fields).toStrictEqual([]);
  },
);

test("an unstamped document traceparent written by a foreign server propagator is not continued (B14)", async () => {
  const runtime = driver((await createPage()).page);

  await expect(
    runtime.install({
      meta: { traceparent: `00-${TRACE_ID}-${SPAN_ID}-01` },
    }),
  ).resolves.toBe(1);

  await runtime.request("/api/unstamped");

  expect(seen[0]?.headers["traceparent"]).toMatch(TRACEPARENT);

  expect(seen[0]?.headers["traceparent"]).not.toContain(TRACE_ID);
});
