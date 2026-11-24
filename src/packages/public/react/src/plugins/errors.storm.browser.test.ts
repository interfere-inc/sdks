import type { InMemorySpanExporter } from "@opentelemetry/sdk-trace";

import { setupErrorsPlugin } from "../../test/fixtures/errors.test-utils.js";
import { beforeEach, expect, test } from "vitest";

const STORM_SIZE = 1000;

const PER_KEY_LIMIT = 5;

let exporter: InMemorySpanExporter;

beforeEach(() => {
  ({ exporter } = setupErrorsPlugin());
});

async function nextTask() {
  await new Promise<void>((resolve) => {
    setTimeout(resolve, 0);
  });
}

function messagesFor(mechanism: string) {
  return exporter
    .getFinishedSpans()
    .filter(
      (span) =>
        span.events[0]?.attributes?.["interfere.exception.mechanism"] ===
        mechanism,
    )
    .map((span) => span.events[0]?.attributes?.["exception.message"]);
}

test("caps a storm of distinct unhandled rejections sharing one message", async () => {
  for (let i = 0; i < STORM_SIZE; i += 1) {
    void Promise.reject(new TypeError("rejection storm"));
  }

  void Promise.reject(new TypeError("distinct rejection"));

  await nextTask();

  await nextTask();

  expect(
    messagesFor("auto.browser.global_handlers.onunhandledrejection"),
  ).toStrictEqual([
    ...Array.from({ length: PER_KEY_LIMIT }, () => "rejection storm"),
    "distinct rejection",
  ]);
});

test("caps a storm of distinct console errors sharing one message", () => {
  for (let i = 0; i < STORM_SIZE; i += 1) {
    console.error(new TypeError("console storm"));
  }

  console.error(new TypeError("distinct console"));

  expect(messagesFor("auto.browser.console.error")).toStrictEqual([
    ...Array.from({ length: PER_KEY_LIMIT }, () => "console storm"),
    "distinct console",
  ]);
});
