import { createError } from "../../test/fixtures/errors.js";
import type { InMemorySpanExporter } from "@opentelemetry/sdk-trace-base";

import { setupErrorsPlugin } from "../../test/fixtures/errors.test-utils.js";

import { beforeEach, expect, test } from "vitest";

let exporter: InMemorySpanExporter;

beforeEach(() => {
  ({ exporter } = setupErrorsPlugin());
});

function rejectUnhandled(reason: Error) {
  const event = new PromiseRejectionEvent("unhandledrejection", {
    promise: Promise.resolve(),
    reason,
    cancelable: true,
  });

  event.preventDefault();

  globalThis.dispatchEvent(event);
}

function exceptionAttributes() {
  const spans = exporter.getFinishedSpans();

  expect(spans).toHaveLength(1);

  return spans[0]?.events[0]?.attributes;
}

test.each([
  ["Chromium", "Failed to fetch"],
  ["WebKit", "Load failed"],
  ["Gecko", "NetworkError when attempting to fetch resource."],
])(
  "records an unhandled %s fetch rejection with its page and network state",
  (_, message) => {
    rejectUnhandled(createError("TypeError", message));

    expect(exceptionAttributes()).toMatchObject({
      "exception.type": "TypeError",
      "exception.message": message,
      "interfere.exception.handled": false,
      "error.type": "transport_failure",
      "interfere.page.leaving": false,
      "interfere.page.visibility": "visible",
    });
  },
);

test("records Zero's logged relay of a zero-cache fetch failure as a handled transport failure", () => {
  const protocolError = createError(
    "ProtocolError",
    "Fetch from API server threw error: fetch failed",
  );

  console.error(
    "InternalServerError:\n\nFetch from API server threw error: fetch failed",
    protocolError,
  );

  expect(exceptionAttributes()).toMatchObject({
    "exception.type": "ProtocolError",
    "exception.message": "Fetch from API server threw error: fetch failed",
    "interfere.exception.handled": true,
    "interfere.exception.mechanism": "auto.browser.console.error",
    "error.type": "transport_failure",
  });
});

test("records a wrapped error whose cause is a fetch transport failure", () => {
  rejectUnhandled(
    Object.assign(createError("SyncError", "sync stopped"), {
      cause: createError("TypeError", "Failed to fetch"),
    }),
  );

  expect(exceptionAttributes()).toMatchObject({
    "exception.type": "SyncError",
    "error.type": "transport_failure",
  });
});

test("adds no transport attributes to an application error", () => {
  rejectUnhandled(createError("TypeError", "Cannot read properties of null"));

  const attributes = exceptionAttributes();

  expect(attributes).not.toHaveProperty("error.type");
  expect(attributes).not.toHaveProperty("interfere.page.visibility");
});
