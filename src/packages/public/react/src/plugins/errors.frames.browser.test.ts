import { createError, createErrorChain } from "../../test/fixtures/errors.js";
import type { InMemorySpanExporter } from "@opentelemetry/sdk-trace";

import { setupErrorsPlugin } from "../../test/fixtures/errors.test-utils.js";

import { beforeEach, expect, test } from "vitest";

let exporter: InMemorySpanExporter;

beforeEach(() => {
  ({ exporter } = setupErrorsPlugin());
});

test("exports source coordinates as fallback frames", () => {
  const error = createError(
    "ReferenceError",
    "TrackerStorageType is not defined",
  );

  globalThis.onerror?.(
    "msg",
    "https://customer.com/inline-tag.js",
    42,
    17,
    error,
  );

  const spans = exporter.getFinishedSpans();

  expect(spans).toHaveLength(1);

  const frames: unknown = JSON.parse(
    String(
      spans[0]?.events[0]?.attributes?.["interfere.exception.fallback_frames"],
    ),
  );

  expect(frames).toStrictEqual([
    {
      column: 17,
      file: "https://customer.com/inline-tag.js",
      id: "fallback-0",
      line: 42,
    },
  ]);
});

test("omits fallback frames without a source string", () => {
  globalThis.onerror?.(
    "msg",
    undefined,
    1,
    1,
    createError("TypeError", "boom"),
  );

  const spans = exporter.getFinishedSpans();

  expect(spans).toHaveLength(1);

  expect(spans[0]?.events[0]?.attributes).not.toHaveProperty(
    "interfere.exception.fallback_frames",
  );
});

test("serializes all three cause-chain errors with the root mechanism", () => {
  globalThis.onerror?.("msg", "source", 1, 1, createErrorChain(3));

  const spans = exporter.getFinishedSpans();

  expect(spans).toHaveLength(1);

  const chain: unknown = JSON.parse(
    String(spans[0]?.events[0]?.attributes?.["interfere.exception.chain"]),
  );

  expect(chain).toStrictEqual([
    {
      kind: "error",
      type: "Error0",
      value: "error at depth 0",
      stack: createError("Error0", "error at depth 0").stack,
      mechanism: {
        handled: false,
        type: "auto.browser.global_handlers.onerror",
      },
    },
    ...[1, 2].map((depth) => ({
      kind: "error",
      type: `Error${depth}`,
      value: `error at depth ${depth}`,
      stack: createError(`Error${depth}`, `error at depth ${depth}`).stack,
    })),
  ]);
});
