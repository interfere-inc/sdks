import { createError } from "../../test/fixtures/errors.js";
import type { InMemorySpanExporter } from "@opentelemetry/sdk-trace";

import { setupErrorsPlugin } from "../../test/fixtures/errors.test-utils.js";
import type { Kernel } from "../internal/kernel.js";
import { beforeEach, expect, test, vi } from "vitest";

let kernel: Kernel;

let exporter: InMemorySpanExporter;

beforeEach(() => {
  ({ kernel, exporter } = setupErrorsPlugin());
});

test("records uncaught errors with the global-handler mechanism", () => {
  const error = createError("TypeError", "undefined is not a function");

  globalThis.onerror?.("msg", "source", 1, 1, error);

  const spans = exporter.getFinishedSpans();

  expect(spans).toHaveLength(1);
  expect(spans[0]?.name).toBe("handleClick (app.js:42)");
  expect(spans[0]?.events).toHaveLength(1);

  expect(spans[0]?.events[0]).toMatchObject({
    name: "exception",
    attributes: {
      "exception.type": "TypeError",
      "exception.message": error.message,
      "exception.stacktrace": error.stack,
      "interfere.exception.kind": "error",
      "interfere.exception.handled": false,
      "interfere.exception.mechanism": "auto.browser.global_handlers.onerror",
    },
  });
});

test.each([
  {
    distinct: false,
    expectedMessages: [
      "repeated",
      "repeated",
      "repeated",
      "repeated",
      "repeated",
    ],
  },
  {
    distinct: true,
    expectedMessages: [
      "repeated",
      "repeated",
      "repeated",
      "repeated",
      "repeated",
      "distinct",
    ],
  },
])(
  "caps repeated errors while distinct=$distinct remains reportable",
  ({ distinct, expectedMessages }) => {
    for (let i = 0; i < 50; i += 1) {
      globalThis.onerror?.(
        "repeated",
        "https://app.invalid/chunk.js",
        9,
        3,
        createError("TypeError", "repeated"),
      );
    }

    expect(exporter.getFinishedSpans()).toHaveLength(5);

    if (distinct) {
      globalThis.onerror?.(
        "distinct",
        "https://app.invalid/app.js",
        1,
        1,
        createError("TypeError", "distinct"),
      );
    }

    expect(
      exporter
        .getFinishedSpans()
        .map((span) => span.events[0]?.attributes?.["exception.message"]),
    ).toStrictEqual(expectedMessages);
  },
);

test("records onerror without an Error argument as a synthetic Error", () => {
  globalThis.onerror?.("string error", "source", 1, 1);

  const spans = exporter.getFinishedSpans();

  expect(spans).toHaveLength(1);

  expect(spans[0]?.events[0]?.attributes).toMatchObject({
    "exception.type": "Error",
    "exception.message": "string error",
    "interfere.exception.kind": "error",
    "interfere.exception.mechanism": "auto.browser.global_handlers.onerror",
  });
});

test("forwards each call while Kernel deduplicates the same Error instance", () => {
  const record = vi.spyOn(kernel, "recordException");
  const error = createError();

  globalThis.onerror?.("msg", "source", 1, 1, error);

  globalThis.onerror?.("msg", "source", 1, 1, error);

  expect(record).toHaveBeenCalledTimes(2);

  expect(record.mock.calls.map(([value]) => value)).toStrictEqual([
    error,
    error,
  ]);

  expect(exporter.getFinishedSpans()).toHaveLength(1);
  expect(exporter.getFinishedSpans()[0]?.events).toHaveLength(1);
});
