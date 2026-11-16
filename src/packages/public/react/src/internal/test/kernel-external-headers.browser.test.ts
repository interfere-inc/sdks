import { ROOT_CONTEXT, propagation, trace } from "@opentelemetry/api";
import { expect, test } from "vitest";
import {
  FOREIGN_TRACEPARENT,
  FOREIGN_TRACE_ID,
  PAGE_URL,
  exporters,
  ownershipHarness,
  sendXhr,
  sessionBaggage,
  settle,
  tagged,
} from "./kernel-ownership.test-utils.js";

const harness = ownershipHarness();

test("external propagation keeps caller context and sends only baggage", async () => {
  expect.hasAssertions();

  const { observations, options, requestSpans } = exporters();

  const kernel = await harness.boot({
    ...options,
    tracing: { propagation: "external" },
  });

  expect(propagation.fields()).toStrictEqual([]);

  const carrier: Record<string, string> = {};

  propagation.inject(
    trace.setSpanContext(ROOT_CONTEXT, {
      traceId: FOREIGN_TRACE_ID,
      spanId: "1234567890abcdef",
      traceFlags: 1,
    }),
    carrier,
  );

  expect(carrier).toStrictEqual({});

  await fetch(PAGE_URL);

  await sendXhr(PAGE_URL, { traceparent: FOREIGN_TRACEPARENT });

  await settle();

  expect(harness.seen().map((entry) => tagged(entry))).toStrictEqual([
    { baggage: sessionBaggage(kernel), foreign: null, traceparent: null },
    {
      baggage: sessionBaggage(kernel),
      foreign: null,
      traceparent: FOREIGN_TRACEPARENT,
    },
  ]);

  const attributes = [
    ...observations().map((record) => record.attributes),
    ...requestSpans().map((span) => span.attributes),
  ];

  expect(
    attributes.some(
      (attrs) => attrs["interfere.request.trace_id"] === FOREIGN_TRACE_ID,
    ),
  ).toBe(true);
});

test("external propagation with baggage disabled writes no SDK headers", async () => {
  expect.hasAssertions();

  await harness.boot({
    tracing: { propagation: "external", propagateTraceBaggage: false },
  });

  await fetch(PAGE_URL);

  await sendXhr(PAGE_URL);

  expect(harness.seen().map((entry) => tagged(entry))).toStrictEqual([
    { baggage: null, foreign: null, traceparent: null },
    { baggage: null, foreign: null, traceparent: null },
  ]);
});
