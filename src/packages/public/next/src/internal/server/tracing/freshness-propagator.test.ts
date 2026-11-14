/* oxlint-disable typescript/no-unsafe-assignment */
import { omitUndefined } from "@interfere/helpers/omit-undefined";

import {
  ROOT_CONTEXT,
  TraceFlags,
  defaultTextMapSetter,
  trace,
} from "@opentelemetry/api";
import {
  CompositePropagator,
  TraceState,
  W3CTraceContextPropagator,
  suppressTracing,
} from "@opentelemetry/core";

import { FreshnessStampingPropagator } from "./freshness-propagator.js";
import { describe, expect, test } from "vitest";

const TRACE_ID = "0af7651916cd43dd8448eb211c80319c";

const SPAN_ID = "b7ad6b7169203331";

const STAMP_RE = /(?:^|,)interfere=(?<seconds>\d+)(?:,|$)/u;

const clientTraceDataSetter = {
  set: (
    carrier: { key: string; value: string }[],
    key: string,
    value: string,
  ) => {
    carrier.push({ key, value });
  },
};

function contextWithSpan(traceState?: TraceState) {
  return trace.setSpanContext(
    ROOT_CONTEXT,
    omitUndefined({
      spanId: SPAN_ID,
      traceFlags: TraceFlags.SAMPLED,
      traceId: TRACE_ID,
      traceState,
    }),
  );
}

describe(FreshnessStampingPropagator, () => {
  test("stamps tracestate with unix seconds at inject time", () => {
    const propagator = new FreshnessStampingPropagator();
    const carrier: Record<string, string> = {};
    const before = Math.floor(Date.now() / 1000);

    propagator.inject(contextWithSpan(), carrier, defaultTextMapSetter);

    const after = Math.floor(Date.now() / 1000);

    expect(carrier["traceparent"]).toBeUndefined();

    const match = STAMP_RE.exec(carrier["tracestate"] ?? "");
    const stampedAtSeconds = Number(match?.groups?.["seconds"]);

    expect(stampedAtSeconds).toBeGreaterThanOrEqual(before);
    expect(stampedAtSeconds).toBeLessThanOrEqual(after);
  });

  test("preserves existing tracestate entries", () => {
    const propagator = new FreshnessStampingPropagator();
    const carrier: Record<string, string> = {};

    propagator.inject(
      contextWithSpan(new TraceState("vendor=abc123")),
      carrier,
      defaultTextMapSetter,
    );

    expect(carrier["tracestate"]).toMatch(STAMP_RE);
    expect(carrier["tracestate"]).toMatch(/(?:^|,)vendor=abc123(?:,|$)/u);
  });

  test("overwrites an inherited stamp with the current time", () => {
    const propagator = new FreshnessStampingPropagator();
    const carrier: Record<string, string> = {};

    propagator.inject(
      contextWithSpan(new TraceState("interfere=1000000000")),
      carrier,
      defaultTextMapSetter,
    );

    const match = STAMP_RE.exec(carrier["tracestate"] ?? "");

    expect(Number(match?.groups?.["seconds"])).toBeGreaterThan(1_000_000_000);
    expect(carrier["tracestate"]).not.toContain("interfere=1000000000");
  });

  test("injects nothing when no span context is present", () => {
    const propagator = new FreshnessStampingPropagator();
    const carrier: Record<string, string> = {};

    propagator.inject(ROOT_CONTEXT, carrier, defaultTextMapSetter);

    expect(carrier["tracestate"]).toBeUndefined();
  });

  test("injects nothing when tracing is suppressed", () => {
    const propagator = new FreshnessStampingPropagator();
    const carrier: Record<string, string> = {};

    propagator.inject(
      suppressTracing(contextWithSpan()),
      carrier,
      defaultTextMapSetter,
    );

    expect(carrier["tracestate"]).toBeUndefined();
  });

  test("composes after a stock W3C propagator without clobbering traceparent", () => {
    const composite = new CompositePropagator({
      propagators: [
        new W3CTraceContextPropagator(),
        new FreshnessStampingPropagator(),
      ],
    });

    const carrier: Record<string, string> = {};

    composite.inject(contextWithSpan(), carrier, defaultTextMapSetter);

    expect(carrier["traceparent"]).toBe(`00-${TRACE_ID}-${SPAN_ID}-01`);
    expect(carrier["tracestate"]).toMatch(STAMP_RE);
  });

  test("keeps the stamp when a stock W3C propagator runs after it with no span tracestate", () => {
    const composite = new CompositePropagator({
      propagators: [
        new FreshnessStampingPropagator(),
        new W3CTraceContextPropagator(),
      ],
    });

    const carrier: Record<string, string> = {};

    composite.inject(contextWithSpan(), carrier, defaultTextMapSetter);

    expect(carrier["traceparent"]).toBe(`00-${TRACE_ID}-${SPAN_ID}-01`);
    expect(carrier["tracestate"]).toMatch(STAMP_RE);
  });

  test("extract is a passthrough and fields declares tracestate", () => {
    const propagator = new FreshnessStampingPropagator();

    expect(propagator.extract(ROOT_CONTEXT)).toBe(ROOT_CONTEXT);
    expect(propagator.fields()).toStrictEqual(["tracestate"]);
  });

  test("skips a bare clientTraceMetadata carrier on static-generation renders", () => {
    const propagator = new FreshnessStampingPropagator({
      isStaticRender: () => true,
    });

    const carrier: { key: string; value: string }[] = [];

    propagator.inject(contextWithSpan(), carrier, clientTraceDataSetter);

    expect(carrier).toStrictEqual([]);
  });

  test("stamps a clientTraceMetadata carrier on static renders when a composed propagator already wrote traceparent", () => {
    const propagator = new FreshnessStampingPropagator({
      isStaticRender: () => true,
    });

    const carrier = [{ key: "traceparent", value: "00-abc-def-01" }];

    propagator.inject(contextWithSpan(), carrier, clientTraceDataSetter);

    expect(carrier.find((entry) => entry.key === "tracestate")?.value).toMatch(
      STAMP_RE,
    );
  });

  test("stamps non-array carriers during static renders (outgoing request headers)", () => {
    const propagator = new FreshnessStampingPropagator({
      isStaticRender: () => true,
    });

    const carrier: Record<string, string> = {};

    propagator.inject(contextWithSpan(), carrier, defaultTextMapSetter);

    expect(carrier["tracestate"]).toMatch(STAMP_RE);
  });

  test("ignores malformed carrier entries when deciding whether traceparent was injected", () => {
    const propagator = new FreshnessStampingPropagator({
      isStaticRender: () => true,
    });

    const carrier: unknown[] = [
      null,
      undefined,
      true,
      42,
      "traceparent",
      {},
      { key: 42 },
    ];

    const setter = {
      set(target: unknown[], key: string, value: string) {
        target.push({ key, value });
      },
    };

    const { length } = carrier;

    propagator.inject(contextWithSpan(), carrier, setter);

    expect(carrier).toHaveLength(length);

    carrier.push({ key: "traceparent", value: "00-abc-def-01" });

    propagator.inject(contextWithSpan(), carrier, setter);

    expect(carrier.at(-1)).toStrictEqual({
      key: "tracestate",
      value: expect.stringMatching(STAMP_RE),
    });
  });
});
