import { ROOT_CONTEXT, context, trace } from "@opentelemetry/api";
import { beforeEach, expect, onTestFinished, test, vi } from "vitest";

import { createError } from "../../../test/fixtures/errors.js";
import { setupErrorsPlugin } from "../../../test/fixtures/errors.test-utils.js";
import { installViewTracker } from "./navigation.js";

let harness: ReturnType<typeof setupErrorsPlugin>;

beforeEach(() => {
  harness = setupErrorsPlugin();

  const originalUrl = globalThis.location.href;
  const originalState: unknown = globalThis.history.state;

  const tracker = installViewTracker({
    documentContext: null,
    isEnabled: () => false,
    loggerProvider: harness.handle.loggerProvider,
    views: harness.kernel.views,
  });

  onTestFinished(() => {
    tracker.dispose();

    globalThis.history.replaceState(originalState, "", originalUrl);

    vi.useRealTimers();
  });
});

const VIEW_ID = "interfere.view.id";

function exceptions() {
  return harness.exporter
    .getFinishedSpans()
    .filter((span) => span.events.some((event) => event.name === "exception"));
}

test("global and console errors separated by navigation are roots stamped with their own views", () => {
  const before = harness.kernel.views.viewId();

  globalThis.onerror?.(
    "first",
    "https://vendor.test/tag.js",
    1,
    20,
    createError(),
  );

  globalThis.history.pushState(null, "", "/capture-context-next");

  const after = harness.kernel.views.viewId();

  console.error(createError("Error", "after navigation"));

  const [first, second] = exceptions();

  expect(after).not.toBe(before);
  expect(first?.parentSpanContext).toBeUndefined();
  expect(second?.parentSpanContext).toBeUndefined();
  expect(first?.attributes[VIEW_ID]).toBe(before);
  expect(second?.attributes[VIEW_ID]).toBe(after);
  expect(first?.events[0]?.attributes?.[VIEW_ID]).toBe(before);
  expect(first?.spanContext().traceId).not.toBe(second?.spanContext().traceId);
});

test("exceptions on one view share the view id, not a trace", () => {
  vi.useFakeTimers({ toFake: ["Date"] });

  console.error(createError());

  vi.setSystemTime(Date.now() + 2001);

  console.error(createError());

  const [first, second] = exceptions();

  expect(first?.attributes[VIEW_ID]).toBe(second?.attributes[VIEW_ID]);
  expect(first?.spanContext().traceId).not.toBe(second?.spanContext().traceId);
});

test("a background rejection does not inherit a recently captured interaction error", () => {
  const click = harness.handle.tracerProvider
    .getTracer("test")
    .startSpan("click");

  context.with(trace.setSpan(context.active(), click), () => {
    console.error(createError());
  });

  click.end();

  const event = new PromiseRejectionEvent("unhandledrejection", {
    promise: Promise.resolve(),
    reason: createError("Error", "background failure"),
    cancelable: true,
  });

  event.preventDefault();

  globalThis.dispatchEvent(event);

  expect(exceptions().at(-1)?.parentSpanContext).toBeUndefined();
});

test("explicit context survives another interaction and navigation", () => {
  const tracer = harness.handle.tracerProvider.getTracer("test");
  const first = tracer.startSpan("first", {}, ROOT_CONTEXT);

  const delayed = context.bind(trace.setSpan(context.active(), first), () => {
    console.error(createError("Error", "first operation"));
  });

  const second = tracer.startSpan("second", {}, ROOT_CONTEXT);

  context.with(trace.setSpan(context.active(), second), () => {
    console.error(createError("Error", "second operation"));

    globalThis.history.pushState(null, "", "/capture-context-other");
  });

  second.end();

  delayed();

  first.end();

  expect(
    exceptions().map(
      (span) => span.events[0]?.attributes?.["exception.message"],
    ),
  ).toStrictEqual(["second operation", "first operation"]);

  expect(exceptions().at(-1)?.spanContext().traceId).toBe(
    first.spanContext().traceId,
  );
});

test("one Error reported through console and onerror is recorded once across navigation", () => {
  const error = createError();

  console.error(error);

  globalThis.history.pushState(null, "", "/capture-context-duplicate");

  globalThis.onerror?.(
    error.message,
    "https://vendor.test/tag.js",
    1,
    20,
    error,
  );

  expect(exceptions()).toHaveLength(1);
});
