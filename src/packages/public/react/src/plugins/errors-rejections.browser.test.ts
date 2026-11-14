import { createError } from "../../test/fixtures/errors.js";
import type { InMemorySpanExporter } from "@opentelemetry/sdk-trace-base";

import { setupErrorsPlugin } from "../../test/fixtures/errors.test-utils.js";

import { beforeEach, expect, test } from "vitest";

let exporter: InMemorySpanExporter;

beforeEach(() => {
  ({ exporter } = setupErrorsPlugin());
});

test("records Error rejections without marking them synthetic", () => {
  const error = createError("RejectionError", "promise failed");

  const event = new PromiseRejectionEvent("unhandledrejection", {
    promise: Promise.resolve(),
    reason: error,
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
      type: "RejectionError",
      value: "promise failed",
      stack: error.stack,
      mechanism: {
        handled: false,
        type: "auto.browser.global_handlers.onunhandledrejection",
      },
    },
  ]);
});

test("does not export digest-bearing promise rejections", () => {
  const event = new PromiseRejectionEvent("unhandledrejection", {
    promise: Promise.resolve(),
    cancelable: true,
    reason: Object.assign(createError("Error", "server action"), {
      digest: "digest-abc",
    }),
  });

  event.preventDefault();

  globalThis.dispatchEvent(event);

  expect(exporter.getFinishedSpans()).toStrictEqual([]);
});
