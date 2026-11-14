import { context, propagation } from "@opentelemetry/api";
import { StackContextManager } from "@opentelemetry/sdk-trace-web";
import { expect, test, vi } from "vitest";

import { PROPAGATOR_OWNED_ELSEWHERE_WARNING } from "../otel/ownership.js";
import { ATTR_REQUEST_TRACE_ID } from "../otel/request-observer/request-observer-events.js";
import {
  FOREIGN_TRACEPARENT,
  ForeignPropagator,
  PAGE_URL,
  exporters,
  ownershipHarness,
  sendXhr,
  sessionBaggage,
  settle,
  tagged,
  traceIdsLeft,
  vendorDiagnostics,
} from "./kernel-ownership.test-utils.js";

const OWNED_FIELDS = ["traceparent", "tracestate", "baggage"];

const harness = ownershipHarness();

test("auto with no registered propagator owns the propagator and context manager, writes its headers and releases both on dispose", async () => {
  expect.hasAssertions();

  const kernel = await harness.boot();

  expect(propagation.fields()).toStrictEqual(OWNED_FIELDS);

  await fetch(PAGE_URL);

  expect(harness.seen().map((entry) => tagged(entry))).toStrictEqual([
    { baggage: sessionBaggage(kernel), foreign: null, traceparent: "own" },
  ]);

  await harness.dispose();

  expect(propagation.fields()).toStrictEqual([]);
  expect(context.setGlobalContextManager(new StackContextManager())).toBe(true);
});

test("auto leaves the OpenTelemetry globals to a propagator another SDK registered, writes traceparent where absent, merges baggage and observes requests as events", async () => {
  expect.hasAssertions();

  const warn = vi.spyOn(console, "warn");
  const info = vi.spyOn(console, "info");

  expect(propagation.setGlobalPropagator(new ForeignPropagator())).toBe(true);

  const { observations, options, requestSpans } = exporters();

  const kernel = await harness.boot(options);

  expect(propagation.fields()).toStrictEqual(["x-foreign-trace"]);

  await fetch(PAGE_URL);

  await sendXhr(PAGE_URL);

  await settle();

  expect(harness.seen().map((entry) => tagged(entry))).toStrictEqual([
    { baggage: sessionBaggage(kernel), foreign: null, traceparent: "own" },
    { baggage: sessionBaggage(kernel), foreign: null, traceparent: "own" },
  ]);

  expect(vendorDiagnostics(info)).toStrictEqual([]);

  expect(vendorDiagnostics(warn)).toStrictEqual([]);

  expect(
    observations().map((record) => [
      record.attributes["interfere.request.kind"],
      record.attributes[ATTR_REQUEST_TRACE_ID],
    ]),
  ).toStrictEqual([
    ["fetch", traceIdsLeft(harness.seen())[0]],
    ["xhr", traceIdsLeft(harness.seen())[1]],
  ]);

  expect(requestSpans()).toStrictEqual([]);

  await harness.dispose();

  expect(propagation.fields()).toStrictEqual(["x-foreign-trace"]);
});

test("explicit managed under a propagator another SDK registered warns once and still writes its own headers where absent", async () => {
  expect.hasAssertions();

  const warn = vi.spyOn(console, "warn");

  expect(propagation.setGlobalPropagator(new ForeignPropagator())).toBe(true);

  const { options, requestSpans } = exporters();

  const kernel = await harness.boot({ ...options, tracing: "managed" });

  await fetch(PAGE_URL);

  await sendXhr(PAGE_URL, { traceparent: FOREIGN_TRACEPARENT });

  await settle();

  expect(harness.seen().map((entry) => tagged(entry))).toStrictEqual([
    { baggage: sessionBaggage(kernel), foreign: null, traceparent: "own" },
    {
      baggage: sessionBaggage(kernel),
      foreign: null,
      traceparent: FOREIGN_TRACEPARENT,
    },
  ]);

  expect(vendorDiagnostics(warn)).toStrictEqual([
    [PROPAGATOR_OWNED_ELSEWHERE_WARNING],
  ]);

  expect(
    requestSpans().map((span) => span.attributes["http.response.status_code"]),
  ).toStrictEqual([204, 204]);

  expect(propagation.fields()).toStrictEqual(["x-foreign-trace"]);
});
