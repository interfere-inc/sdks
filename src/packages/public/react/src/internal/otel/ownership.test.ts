import { context, propagation } from "@opentelemetry/api";
import { W3CTraceContextPropagator } from "@opentelemetry/core";
import { StackContextManager } from "@opentelemetry/sdk-trace-web";
import { afterEach, expect, test } from "vitest";
import { claimTracingOwnership } from "./ownership.js";

afterEach(() => {
  propagation.disable();

  context.disable();
});

test("auto claims only unowned OpenTelemetry globals and releases them", () => {
  const owner = claimTracingOwnership({
    mode: "auto",
    contextManager: new StackContextManager(),
    propagator: new W3CTraceContextPropagator(),
  });

  expect([
    owner.mode,
    owner.ownsPropagator,
    owner.ownsContextManager,
    owner.writesTraceContext,
  ]).toStrictEqual(["managed", true, true, true]);

  owner.release();

  owner.release();

  expect(propagation.setGlobalPropagator(new W3CTraceContextPropagator())).toBe(
    true,
  );

  expect(context.setGlobalContextManager(new StackContextManager())).toBe(true);
});

test("auto leaves an existing OpenTelemetry propagator and context owner unchanged", () => {
  const external = new W3CTraceContextPropagator();

  expect(propagation.setGlobalPropagator(external)).toBe(true);

  const owner = claimTracingOwnership({
    mode: "auto",
    contextManager: new StackContextManager(),
    propagator: new W3CTraceContextPropagator(),
  });

  expect([
    owner.mode,
    owner.ownsPropagator,
    owner.ownsContextManager,
  ]).toStrictEqual(["application", false, false]);

  owner.release();

  expect(propagation.fields()).toStrictEqual(external.fields());
});

test.each([
  { mode: "auto" as const, resolved: "application", context: false },
  { mode: "application" as const, resolved: "application", context: false },
  { mode: "managed" as const, resolved: "managed", context: true },
])(
  "external propagation in $mode never claims the propagator slot",
  ({ mode, resolved, context: ownsContext }) => {
    const owner = claimTracingOwnership({
      mode,
      propagation: "external",
      contextManager: new StackContextManager(),
      propagator: new W3CTraceContextPropagator(),
    });

    expect([
      owner.mode,
      owner.ownsPropagator,
      owner.ownsContextManager,
      owner.writesTraceContext,
    ]).toStrictEqual([resolved, false, ownsContext, false]);

    expect(
      propagation.setGlobalPropagator(new W3CTraceContextPropagator()),
    ).toBe(true);

    owner.release();

    expect(propagation.fields()).toStrictEqual(["traceparent", "tracestate"]);
  },
);
