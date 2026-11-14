import { context, propagation } from "@opentelemetry/api";
import { StackContextManager } from "@opentelemetry/sdk-trace-web";
import { expect, test, vi } from "vitest";

import { ATTR_REQUEST_TRACE_ID } from "../otel/request-observer/request-observer-events.js";
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
  traceIdsLeft,
  vendorDiagnostics,
} from "./kernel-ownership.test-utils.js";

const harness = ownershipHarness();

test("a request that already carries traceparent keeps it, gets Interfere baggage appended and records the trace id it left with", async () => {
  expect.hasAssertions();

  const { options, requestSpans } = exporters();

  const kernel = await harness.boot(options);

  await fetch(PAGE_URL, {
    headers: { baggage: "vendor=1", traceparent: FOREIGN_TRACEPARENT },
  });

  await sendXhr(PAGE_URL, {
    baggage: "vendor=1",
    traceparent: FOREIGN_TRACEPARENT,
  });

  await fetch(PAGE_URL);

  await settle();

  expect(harness.seen().map((entry) => tagged(entry))).toStrictEqual([
    {
      baggage: `vendor=1, ${sessionBaggage(kernel)}`,
      foreign: null,
      traceparent: FOREIGN_TRACEPARENT,
    },
    {
      baggage: `vendor=1, ${sessionBaggage(kernel)}`,
      foreign: null,
      traceparent: FOREIGN_TRACEPARENT,
    },
    { baggage: sessionBaggage(kernel), foreign: null, traceparent: "own" },
  ]);

  const spans = requestSpans();

  expect(
    spans.map((span) => span.attributes[ATTR_REQUEST_TRACE_ID]),
  ).toStrictEqual([
    FOREIGN_TRACE_ID,
    FOREIGN_TRACE_ID,
    spans[2]?.spanContext().traceId,
  ]);

  expect(spans.map((span) => span.spanContext().traceId)).not.toContain(
    FOREIGN_TRACE_ID,
  );
});

test.each([
  ["an explicit tracing.mode", { tracing: { mode: "application" } } as const],
  ["the tracing shorthand", { tracing: "application" } as const],
])(
  "application mode from %s writes no global, writes traceparent where absent, keeps a foreign one, appends baggage and records observations joined to the trace each request left with",
  async (_source, opts) => {
    expect.hasAssertions();

    const warn = vi.spyOn(console, "warn");

    const { observations, options, requestSpans } = exporters();

    const kernel = await harness.boot({ ...opts, ...options });

    expect(propagation.fields()).toStrictEqual([]);

    expect(context.setGlobalContextManager(new StackContextManager())).toBe(
      true,
    );

    context.disable();

    await fetch(PAGE_URL);

    await sendXhr(PAGE_URL);

    await fetch(PAGE_URL, { headers: { traceparent: FOREIGN_TRACEPARENT } });

    await settle();

    expect(harness.seen().map((entry) => tagged(entry))).toStrictEqual([
      { baggage: sessionBaggage(kernel), foreign: null, traceparent: "own" },
      { baggage: sessionBaggage(kernel), foreign: null, traceparent: "own" },
      {
        baggage: sessionBaggage(kernel),
        foreign: null,
        traceparent: FOREIGN_TRACEPARENT,
      },
    ]);

    expect(vendorDiagnostics(warn)).toStrictEqual([]);

    const left = traceIdsLeft(harness.seen());

    const common = {
      "url.full": PAGE_URL,
      "url.path": "/ownership-test",
      "http.request.method": "GET",
      "http.response.status_code": 204,
      "interfere.view.id": kernel.views.viewId(),
      "session.id": kernel.session.getId(),
    };

    expect(observations().map((record) => record.attributes)).toStrictEqual([
      {
        ...common,
        "interfere.request.kind": "fetch",
        [ATTR_REQUEST_TRACE_ID]: left[0],
      },
      {
        ...common,
        "interfere.request.kind": "xhr",
        [ATTR_REQUEST_TRACE_ID]: left[1],
      },
      {
        ...common,
        "interfere.request.kind": "fetch",
        [ATTR_REQUEST_TRACE_ID]: FOREIGN_TRACE_ID,
      },
    ]);

    expect(
      observations().map((record) => record.spanContext?.traceId),
    ).toStrictEqual(left);

    expect(new Set(left).size).toBe(3);

    expect(requestSpans()).toStrictEqual([]);
  },
);
