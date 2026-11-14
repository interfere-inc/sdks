/* oxlint-disable typescript/no-unsafe-assignment */
import type { InMemorySpanExporter } from "@opentelemetry/sdk-trace-base";

import { createError } from "../test/fixtures/errors.js";
import { createTracedKernel } from "../test/fixtures/react-error-handler.js";
import type { Kernel } from "./internal/kernel.js";
import { reactErrorHandler } from "./react-error-handler.js";
import { beforeEach, expect, test } from "vitest";

let kernel: Kernel;

let exporter: InMemorySpanExporter;

beforeEach(() => {
  ({ exporter, kernel } = createTracedKernel());
});

test("returns the three React root callbacks", () => {
  const handlers = reactErrorHandler({ kernel });

  expect(handlers).toStrictEqual({
    onCaughtError: expect.any(Function),
    onRecoverableError: expect.any(Function),
    onUncaughtError: expect.any(Function),
  });
});

test.each([
  {
    callback: "onCaughtError",
    mechanism: { handled: true, type: "auto.function.react.caught_error" },
  },
  {
    callback: "onUncaughtError",
    mechanism: { handled: false, type: "auto.function.react.uncaught_error" },
  },
  {
    callback: "onRecoverableError",
    mechanism: {
      handled: true,
      synthetic: true,
      type: "auto.function.react.recoverable_error",
    },
  },
] as const)(
  "records $callback through the private provider",
  async ({ callback, mechanism }) => {
    const error = createError("TypeError", callback);
    const handlers = reactErrorHandler({ kernel });

    handlers[callback](error, { componentStack: null });

    await kernel.flush();

    const spans = exporter.getFinishedSpans();

    expect(spans).toHaveLength(1);
    expect(spans[0]?.name).toBe("handleClick (app.js:42)");
    expect(spans[0]?.events).toHaveLength(1);
    expect(spans[0]?.events[0]?.name).toBe("exception");

    const attributes = spans[0]?.events[0]?.attributes;

    expect(attributes).toMatchObject({
      "exception.type": error.name,
      "exception.message": error.message,
      "exception.stacktrace": error.stack,
      "interfere.exception.kind": "error",
      "interfere.exception.handled": mechanism.handled,
      "interfere.exception.mechanism": mechanism.type,
    });

    const chain: unknown = JSON.parse(
      String(attributes?.["interfere.exception.chain"]),
    );

    expect(chain).toStrictEqual([
      {
        kind: "error",
        type: error.name,
        value: error.message,
        stack: error.stack,
        mechanism,
      },
    ]);
  },
);

test("does not record without a supplied kernel", async () => {
  reactErrorHandler().onCaughtError(createError("Error", "no kernel"), {});

  await kernel.flush();

  expect(exporter.getFinishedSpans()).toStrictEqual([]);
});
