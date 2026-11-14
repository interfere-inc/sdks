import { propagation } from "@opentelemetry/api";
import type { Context, TextMapPropagator } from "@opentelemetry/api";
import { HttpResponse, http } from "msw";
import { expect, test, vi } from "vitest";

import { DISABLE_ALL_PLUGINS } from "../../../test/fixtures/disable-all-plugins.js";
import { worker } from "../../../test/msw.js";
import {
  API,
  OWN_TRACEPARENT,
  PAGE_URL,
  exporters,
  ownershipHarness,
  sendXhr,
  sessionBaggage,
  settle,
  tagged,
  vendorDiagnostics,
} from "./kernel-ownership.test-utils.js";

const harness = ownershipHarness();

const BACKEND = "https://backend.test";

const EXAMPLE_API = "https://api.acme.test";

interface Wire {
  baggage: string | null;
  traceparent: string | null;
  tracestate: string | null;
  url: string;
}

function record(origin: string) {
  const seen: Wire[] = [];

  worker.use(
    http.all(`${origin}/*`, ({ request }) => {
      seen.push({
        baggage: request.headers.get("baggage"),
        traceparent: request.headers.get("traceparent"),
        tracestate: request.headers.get("tracestate"),
        url: request.url,
      });

      return new HttpResponse(null, { status: 204 });
    }),
  );

  return seen;
}

class InertAnnouncingPropagator implements TextMapPropagator {
  // oxlint-disable-next-line eslint/no-empty-function -- Writes nothing by design.
  inject() {}

  extract(active: Context) {
    return active;
  }

  fields() {
    return ["traceparent"];
  }
}

test("a page (inert global propagator, consent false, Datadog RUM present, tracing.headers false) writes no header to the page origin or a configured backend host while every request is still observed", async () => {
  expect.hasAssertions();

  Object.assign(globalThis, {
    DD_RUM: { getInitConfiguration: () => ({}) },
  });

  expect(propagation.setGlobalPropagator(new InertAnnouncingPropagator())).toBe(
    true,
  );

  worker.use(
    http.get(`${API}/v3/config`, () =>
      HttpResponse.json({
        backendHosts: ["backend.test"],
        plugins: DISABLE_ALL_PLUGINS,
      }),
    ),
  );

  const backend = record(BACKEND);

  const info = vi.spyOn(console, "info");
  const warn = vi.spyOn(console, "warn");

  const { observations, options, requestSpans } = exporters();

  await harness.boot({
    ...options,
    consent: false,
    tracing: { captureFailedRequests: false, headers: false },
  });

  expect(propagation.fields()).toStrictEqual(["traceparent"]);

  await fetch(PAGE_URL);

  await sendXhr(PAGE_URL);

  await fetch(`${BACKEND}/orders`);

  await sendXhr(`${BACKEND}/orders`);

  await settle();

  expect(harness.seen()).toStrictEqual([
    { baggage: null, foreign: null, traceparent: null },
    { baggage: null, foreign: null, traceparent: null },
  ]);

  expect(
    backend.map((entry) => [
      entry.traceparent,
      entry.tracestate,
      entry.baggage,
    ]),
  ).toStrictEqual([
    [null, null, null],
    [null, null, null],
  ]);

  expect(
    observations().map((recordEntry) => [
      recordEntry.attributes["interfere.request.kind"],
      recordEntry.attributes["url.full"],
    ]),
  ).toStrictEqual([
    ["fetch", PAGE_URL],
    ["xhr", PAGE_URL],
    ["fetch", `${BACKEND}/orders`],
    ["xhr", `${BACKEND}/orders`],
  ]);

  expect(requestSpans()).toStrictEqual([]);

  expect(vendorDiagnostics(warn)).toStrictEqual([]);

  expect(vendorDiagnostics(info)).toStrictEqual([]);
});

test("a propagateTo entry with its own header list receives exactly those headers while the page origin keeps every header", async () => {
  expect.hasAssertions();

  const api = record(EXAMPLE_API);

  const { options, requestSpans } = exporters();

  const kernel = await harness.boot({
    ...options,
    tracing: {
      propagateTo: [
        { headers: ["traceparent", "tracestate"], origin: EXAMPLE_API },
      ],
    },
  });

  await fetch(PAGE_URL);

  await fetch(`${EXAMPLE_API}/v1/orders`);

  await sendXhr(`${EXAMPLE_API}/v1/orders`);

  await settle();

  expect(harness.seen().map((entry) => tagged(entry))).toStrictEqual([
    { baggage: sessionBaggage(kernel), foreign: null, traceparent: "own" },
  ]);

  expect(
    api.map((entry) => [
      OWN_TRACEPARENT.test(entry.traceparent ?? ""),
      entry.tracestate,
      entry.baggage,
    ]),
  ).toStrictEqual([
    [true, null, null],
    [true, null, null],
  ]);

  expect(
    requestSpans().map((span) => span.attributes["http.response.status_code"]),
  ).toStrictEqual([204]);
});

test("a tracing.headers list restricts the page origin to those headers and request spans still record", async () => {
  expect.hasAssertions();

  const { options, requestSpans } = exporters();

  const kernel = await harness.boot({
    ...options,
    tracing: { headers: ["baggage"] },
  });

  await fetch(PAGE_URL);

  await sendXhr(PAGE_URL);

  await settle();

  expect(harness.seen()).toStrictEqual([
    { baggage: sessionBaggage(kernel), foreign: null, traceparent: null },
    { baggage: sessionBaggage(kernel), foreign: null, traceparent: null },
  ]);

  expect(
    requestSpans().map((span) => [
      span.attributes["http.response.status_code"],
      span.attributes["interfere.request.trace_id"],
    ]),
  ).toStrictEqual([
    [204, undefined],
    [204, undefined],
  ]);
});
