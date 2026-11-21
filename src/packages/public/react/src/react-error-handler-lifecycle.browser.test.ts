import type { InMemorySpanExporter } from "@opentelemetry/sdk-trace";

import { createError } from "../test/fixtures/errors.js";
import { createTracedKernel } from "../test/fixtures/react-error-handler.js";
import type { Kernel } from "./internal/kernel.js";
import { reactErrorHandler } from "./react-error-handler.js";
import { beforeEach, expect, test, vi } from "vitest";

let kernel: Kernel;

let exporter: InMemorySpanExporter;

beforeEach(() => {
  ({ exporter, kernel } = createTracedKernel());
});

test("records before invoking each user callback with the original arguments", async () => {
  expect.assertions(8);

  const captured: unknown[] = [];
  const info = { componentStack: null };

  const calls = vi.fn<ReturnType<typeof reactErrorHandler>["onCaughtError"]>();

  function onError(
    ...[error, receivedInfo]: Parameters<
      ReturnType<typeof reactErrorHandler>["onCaughtError"]
    >
  ) {
    calls(error, receivedInfo);

    captured.push(
      exporter
        .getFinishedSpans()
        .map((span) => span.events[0]?.attributes?.["exception.message"]),
    );

    expect(receivedInfo).toBe(info);
    expect(error).toBeInstanceOf(Error);
  }

  const handlers = reactErrorHandler({
    kernel,
    onCaughtError: onError,
    onUncaughtError: onError,
    onRecoverableError: onError,
  });

  const errors = [
    createError("Error", "caught"),
    createError("Error", "uncaught"),
    createError("Error", "recovered"),
  ];

  handlers.onCaughtError(errors[0], info);

  handlers.onUncaughtError(errors[1], info);

  handlers.onRecoverableError(errors[2], info);

  expect(calls.mock.calls.map(([error]) => error)).toStrictEqual(errors);

  expect(captured).toStrictEqual([
    ["caught"],
    ["caught", "uncaught"],
    ["caught", "uncaught", "recovered"],
  ]);

  await kernel.flush();
});

test("records non-Error throws and delivers them to callbacks unchanged", async () => {
  const onError =
    vi.fn<ReturnType<typeof reactErrorHandler>["onCaughtError"]>();

  const info = { componentStack: null };

  const handlers = reactErrorHandler({
    kernel,
    onCaughtError: onError,
    onUncaughtError: onError,
    onRecoverableError: onError,
  });

  handlers.onCaughtError("string thrown", info);

  handlers.onUncaughtError(42, info);

  handlers.onRecoverableError(null, info);

  await kernel.flush();

  expect(onError.mock.calls).toStrictEqual([
    ["string thrown", info],
    [42, info],
    [null, info],
  ]);

  const events = exporter
    .getFinishedSpans()
    .flatMap((span) => span.events)
    .map((event) => event.attributes ?? {});

  expect(events).toHaveLength(3);

  expect(events[0]).toMatchObject({
    "exception.message": "string thrown",
    "interfere.exception.handled": true,
    "interfere.exception.mechanism": "auto.function.react.caught_error",
  });

  expect(events[1]).toMatchObject({
    "exception.type": "number",
    "interfere.exception.handled": false,
    "interfere.exception.kind": "non-error",
    "interfere.exception.mechanism": "auto.function.react.uncaught_error",
  });

  expect(events[2]).toMatchObject({
    "exception.type": "null",
    "interfere.exception.kind": "non-error",
    "interfere.exception.mechanism": "auto.function.react.recoverable_error",
  });
});

test("records the same Error only once across caught and uncaught paths", async () => {
  const error = createError("Error", "once");
  const handlers = reactErrorHandler({ kernel });

  handlers.onCaughtError(error, {});

  handlers.onUncaughtError(error, {});

  await kernel.flush();

  const spans = exporter.getFinishedSpans();

  expect(spans).toHaveLength(1);
  expect(spans[0]?.events).toHaveLength(1);
  expect(spans[0]?.events[0]?.attributes?.["exception.message"]).toBe("once");

  expect(
    spans[0]?.events[0]?.attributes?.["interfere.exception.mechanism"],
  ).toBe("auto.function.react.caught_error");
});

test("resolves a late kernel and stops capturing after the getter clears", async () => {
  let active: Kernel | null = null;
  const handlers = reactErrorHandler({ kernel: () => active });

  handlers.onCaughtError(createError("Error", "before"), {});

  expect(exporter.getFinishedSpans()).toStrictEqual([]);

  active = kernel;

  handlers.onCaughtError(createError("Error", "after"), {});

  active = null;

  handlers.onCaughtError(createError("Error", "cleared"), {});

  await kernel.flush();

  const spans = exporter.getFinishedSpans();

  expect(spans).toHaveLength(1);
  expect(spans[0]?.events[0]?.attributes?.["exception.message"]).toBe("after");
});
