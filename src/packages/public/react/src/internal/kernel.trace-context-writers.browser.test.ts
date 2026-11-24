import { MECHANISM_TYPE } from "@interfere/types/sdk/errors";
import { SpanKind, SpanStatusCode, propagation } from "@opentelemetry/api";
import type {
  Context,
  TextMapPropagator,
  TextMapSetter,
} from "@opentelemetry/api";
import { HttpResponse, http } from "msw";
import { expect, test } from "vitest";

import { worker } from "../../test/msw.js";
import {
  PAGE_URL,
  exporters,
  ownershipHarness,
  sendXhr,
  sessionBaggage,
  settle,
  tagged,
} from "./kernel.ownership.test-utils.js";

const BROKEN_URL = `${location.origin}/ownership-broken`;

const harness = ownershipHarness();

function installDatadog(match: string) {
  Object.assign(globalThis, {
    DD_RUM: {
      getInitConfiguration: () => ({
        allowedTracingUrls: [{ match, propagatorTypes: ["tracecontext"] }],
      }),
    },
  });
}

function installSentry(propagateTraceparent: boolean) {
  Object.assign(globalThis, {
    __SENTRY__: {
      version: "10.78.0",
      "10.78.0": {
        defaultCurrentScope: {
          getClient: () => ({ getOptions: () => ({ propagateTraceparent }) }),
        },
      },
    },
  });
}

const TRACEPARENT_PROPAGATOR = {
  extract: (active: Context) => active,
  fields: () => ["traceparent"],
  inject: (
    _context: Context,
    carrier: Record<string, string>,
    setter: TextMapSetter<Record<string, string>>,
  ) => {
    setter.set(carrier, "x-other-otel", "1");
  },
} satisfies TextMapPropagator<Record<string, string>>;

async function wait(ms: number) {
  await new Promise<void>((resolve) => {
    setTimeout(resolve, ms);
  });
}

test("a Profound-like page (Datadog RUM, external propagation, baggage, consent) records request spans, failed-request evidence and joins the bare rejection to the request's trace", async () => {
  expect.hasAssertions();

  installDatadog(location.origin);

  worker.use(http.get(BROKEN_URL, () => HttpResponse.error()));

  const { allSpans, observations, options, requestSpans } = exporters();

  const kernel = await harness.boot({
    ...options,
    tracing: { propagation: "external" },
  });

  kernel.consent.set(true);

  expect(propagation.fields()).toStrictEqual([]);

  await fetch(PAGE_URL);

  await sendXhr(PAGE_URL);

  let rejection = new Error("fetch resolved");

  try {
    await fetch(BROKEN_URL);
  } catch (error) {
    rejection = error instanceof Error ? error : rejection;
  }

  expect(rejection).toBeInstanceOf(TypeError);

  kernel.recordException(rejection, {
    mechanism: {
      handled: false,
      type: MECHANISM_TYPE.browser.onunhandledrejection,
    },
  });

  await wait(1100);

  await settle();

  expect(harness.seen().map((entry) => tagged(entry))).toStrictEqual([
    { baggage: sessionBaggage(kernel), foreign: null, traceparent: null },
    { baggage: sessionBaggage(kernel), foreign: null, traceparent: null },
  ]);

  expect(observations()).toStrictEqual([]);

  expect(
    requestSpans().map((span) => [
      span.kind,
      span.attributes["http.request.method"],
      span.attributes["http.response.status_code"],
      span.attributes["session.id"],
    ]),
  ).toStrictEqual([
    [SpanKind.CLIENT, "GET", 204, kernel.session.getId()],
    [SpanKind.CLIENT, "GET", 204, kernel.session.getId()],
  ]);

  const failed = allSpans().find(
    (span) => span.attributes["url.full"] === BROKEN_URL,
  );

  expect([
    failed?.kind,
    failed?.attributes["error.type"],
    failed?.status.code,
    failed?.events
      .filter((event) => event.name === "exception")
      .map((event) => event.attributes?.["exception.type"]),
  ]).toStrictEqual([
    SpanKind.CLIENT,
    "TypeError",
    SpanStatusCode.ERROR,
    ["NetworkError"],
  ]);

  const bare = allSpans().find(
    (span) =>
      span.attributes["interfere.capture.kind"] === "exception" &&
      span.events.some(
        (event) =>
          event.attributes?.["interfere.exception.mechanism"] ===
          MECHANISM_TYPE.browser.onunhandledrejection,
      ),
  );

  expect([
    bare?.spanContext().traceId,
    bare?.parentSpanContext?.spanId,
  ]).toStrictEqual([
    failed?.spanContext().traceId,
    failed?.spanContext().spanId,
  ]);
});

test("unset propagation defers traceparent to Datadog RUM for the URLs it traces, from its first request even when it initialises after Interfere", async () => {
  expect.hasAssertions();

  const kernel = await harness.boot();

  await fetch(PAGE_URL);

  installDatadog(location.origin);

  await fetch(PAGE_URL);

  await sendXhr(PAGE_URL);

  installDatadog("https://elsewhere.invalid");

  await fetch(PAGE_URL);

  expect(harness.seen().map((entry) => tagged(entry))).toStrictEqual([
    { baggage: sessionBaggage(kernel), foreign: null, traceparent: "own" },
    { baggage: sessionBaggage(kernel), foreign: null, traceparent: null },
    { baggage: sessionBaggage(kernel), foreign: null, traceparent: null },
    { baggage: sessionBaggage(kernel), foreign: null, traceparent: "own" },
  ]);
});

test.each([
  [true, null],
  [false, "own"],
] as const)(
  "unset propagation defers traceparent to Sentry only when propagateTraceparent is %s",
  async (propagateTraceparent, expected) => {
    expect.hasAssertions();

    installSentry(propagateTraceparent);

    const kernel = await harness.boot();

    await fetch(PAGE_URL);

    expect(harness.seen().map((entry) => tagged(entry))).toStrictEqual([
      {
        baggage: sessionBaggage(kernel),
        foreign: null,
        traceparent: expected,
      },
    ]);
  },
);

test("unset propagation defers traceparent to another OpenTelemetry propagator that writes it, and records the request span", async () => {
  expect.hasAssertions();

  expect(propagation.setGlobalPropagator(TRACEPARENT_PROPAGATOR)).toBe(true);

  const { options, requestSpans } = exporters();

  const kernel = await harness.boot(options);

  await fetch(PAGE_URL);

  await settle();

  expect(harness.seen().map((entry) => tagged(entry))).toStrictEqual([
    { baggage: sessionBaggage(kernel), foreign: null, traceparent: null },
  ]);

  expect(requestSpans()).toHaveLength(1);
});

test("explicit interfere propagation still writes traceparent where Datadog RUM traces", async () => {
  expect.hasAssertions();

  installDatadog(location.origin);

  const kernel = await harness.boot({
    tracing: { propagation: "interfere" },
  });

  await fetch(PAGE_URL);

  expect(harness.seen().map((entry) => tagged(entry))).toStrictEqual([
    { baggage: sessionBaggage(kernel), foreign: null, traceparent: "own" },
  ]);
});
