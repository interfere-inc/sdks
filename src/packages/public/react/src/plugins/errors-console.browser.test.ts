import { createError } from "../../test/fixtures/errors.js";
import type { InMemorySpanExporter } from "@opentelemetry/sdk-trace-base";

import { setupErrorsPlugin } from "../../test/fixtures/errors.test-utils.js";

import { beforeEach, expect, test } from "vitest";

let exporter: InMemorySpanExporter;

beforeEach(() => {
  ({ exporter } = setupErrorsPlugin());
});

test.each(["first", "react", "multiple"] as const)(
  "captures the first Error from %s console arguments",
  (mode) => {
    const first = createError("ConsoleError", "logged error");
    const second = createError("Second", "second");

    const argumentsByMode = {
      first: [first],
      react: [
        "The above error occurred in the <BuyPageComponent> component:",
        first,
        "in Router",
      ],
      multiple: ["context:", first, "more:", second],
    };

    console.error(...argumentsByMode[mode]);

    const spans = exporter.getFinishedSpans();

    expect(spans).toHaveLength(1);
    expect(spans[0]?.events).toHaveLength(1);

    expect(spans[0]?.events[0]?.attributes).toMatchObject({
      "exception.type": "ConsoleError",
      "exception.message": "logged error",
      "exception.stacktrace": first.stack,
      "interfere.exception.kind": "error",
      "interfere.exception.mechanism": "auto.browser.console.error",
      "interfere.exception.handled": true,
    });
  },
);

test("ignores console arguments without an Error", () => {
  console.error("just a string");

  expect(exporter.getFinishedSpans()).toStrictEqual([]);
});

test("ignores digest-bearing console errors", () => {
  console.error(
    Object.assign(createError("Error", "server action"), {
      digest: "digest-abc",
    }),
  );

  expect(exporter.getFinishedSpans()).toStrictEqual([]);
});

test("ignores logged AbortErrors from cancelled work", () => {
  const controller = new AbortController();

  controller.abort();

  console.error("request cancelled", controller.signal.reason);

  console.error(new DOMException("The user aborted a request.", "AbortError"));

  expect(exporter.getFinishedSpans()).toStrictEqual([]);
});
