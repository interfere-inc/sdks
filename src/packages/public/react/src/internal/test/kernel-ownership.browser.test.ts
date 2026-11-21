import { context, propagation } from "@opentelemetry/api";
import { StackContextManager } from "@opentelemetry/sdk-trace";
import { expect, test, vi } from "vitest";

import { ATTR_REQUEST_TRACE_ID } from "../otel/request-observer/request-observer-events.js";
import {
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

const harness = ownershipHarness();

test("interfere registers no OpenTelemetry global, still writes its own headers and leaves the globals free after dispose", async () => {
  expect.hasAssertions();

  const kernel = await harness.boot();

  expect(propagation.fields()).toStrictEqual([]);

  await fetch(PAGE_URL);

  expect(harness.seen().map((entry) => tagged(entry))).toStrictEqual([
    { baggage: sessionBaggage(kernel), foreign: null, traceparent: "own" },
  ]);

  await harness.dispose();

  expect(propagation.fields()).toStrictEqual([]);
  expect(context.setGlobalContextManager(new StackContextManager())).toBe(true);
});

test("a propagator another SDK registered stays the global one; Interfere writes traceparent where absent, merges baggage and records request spans", async () => {
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

  expect(observations()).toStrictEqual([]);

  expect(
    requestSpans().map((span) => [
      span.spanContext().traceId,
      span.attributes[ATTR_REQUEST_TRACE_ID],
    ]),
  ).toStrictEqual(
    traceIdsLeft(harness.seen()).map((traceId) => [traceId, traceId]),
  );

  await harness.dispose();

  expect(propagation.fields()).toStrictEqual(["x-foreign-trace"]);
});
