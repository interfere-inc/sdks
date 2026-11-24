/* oxlint-disable typescript/no-unsafe-assignment */
import type { InMemorySpanExporter } from "@opentelemetry/sdk-trace";

import { setupErrorsPlugin } from "../../test/fixtures/errors.test-utils.js";
import type { Kernel } from "../internal/kernel.js";
import { beforeEach, expect, test, vi } from "vitest";

let kernel: Kernel;

let exporter: InMemorySpanExporter;

beforeEach(() => {
  ({ kernel, exporter } = setupErrorsPlugin());
});

test("records a string rejection with a synthetic Error chain", () => {
  const event = new PromiseRejectionEvent("unhandledrejection", {
    promise: Promise.resolve(),
    reason: "TrackerStorageType is not defined",
    cancelable: true,
  });

  event.preventDefault();

  globalThis.dispatchEvent(event);

  const spans = exporter.getFinishedSpans();

  expect(spans).toHaveLength(1);

  const chain: unknown = JSON.parse(
    String(spans[0]?.events[0]?.attributes?.["interfere.exception.chain"]),
  );

  expect(chain).toStrictEqual([
    {
      kind: "error",
      type: "Error",
      value: "TrackerStorageType is not defined",
      stack: expect.any(String),
      mechanism: {
        handled: false,
        synthetic: true,
        type: "auto.browser.global_handlers.onunhandledrejection",
      },
    },
  ]);
});

test("forwards synthetic object rejection metadata and exports its payload", () => {
  const record = vi.spyOn(kernel, "recordException");

  const event = new PromiseRejectionEvent("unhandledrejection", {
    promise: Promise.resolve(),
    reason: { error: "boom" },
    cancelable: true,
  });

  event.preventDefault();

  globalThis.dispatchEvent(event);

  expect(record).toHaveBeenCalledTimes(1);

  expect(record.mock.calls[0]?.[1]?.mechanism).toStrictEqual({
    handled: false,
    synthetic: true,
    type: "auto.browser.global_handlers.onunhandledrejection",
  });

  const spans = exporter.getFinishedSpans();

  expect(spans).toHaveLength(1);

  expect(spans[0]?.events[0]?.attributes).toStrictEqual({
    "exception.type": "PlainObject",
    "exception.message": "Non-Error captured with keys: error",
    "interfere.exception.kind": "non-error",
    "interfere.exception.serialized": '{"error":"boom"}',
    "interfere.exception.handled": false,
    "interfere.exception.mechanism":
      "auto.browser.global_handlers.onunhandledrejection",
    "interfere.view.id": expect.any(String),
  });
});
