import type { InMemorySpanExporter } from "@opentelemetry/sdk-trace-base";

import { createErrorsKernel } from "../../test/fixtures/errors.test-utils.js";
import { createError } from "../../test/fixtures/errors.js";
import { hasEventListenerPair } from "../internal/event-capabilities/event-capabilities.js";
import type { Kernel } from "../internal/kernel.js";
import errorsPlugin from "./errors.js";
import {
  afterEach,
  beforeEach,
  expect,
  onTestFinished,
  test,
  vi,
} from "vitest";

let kernel: Kernel;

let exporter: InMemorySpanExporter;

let cleanup: ReturnType<typeof errorsPlugin.setup>;

let originalLimit: number;

beforeEach(() => {
  originalLimit = Error.stackTraceLimit;

  ({ kernel, exporter } = createErrorsKernel());

  cleanup = undefined;
});

afterEach(() => {
  try {
    cleanup?.();
  } finally {
    Error.stackTraceLimit = originalLimit;

    vi.unstubAllGlobals();
  }
});

test("chains the previous onerror arguments and return value, then restores it", () => {
  const calls = vi.fn<OnErrorEventHandlerNonNull>();
  const original = globalThis.onerror;

  function previous(
    this: Window,
    ...args: Parameters<OnErrorEventHandlerNonNull>
  ) {
    calls.apply(this, args);

    return true;
  }

  onTestFinished(() => {
    // oxlint-disable-next-line unicorn/prefer-add-event-listener
    globalThis.onerror = original;
  });

  // oxlint-disable-next-line unicorn/prefer-add-event-listener
  globalThis.onerror = previous;

  cleanup = errorsPlugin.setup(kernel);

  const installed = globalThis.onerror;
  const error = createError("Error", "chained");

  expect(installed).not.toBe(previous);
  expect(globalThis.onerror("msg", "source", 3, 4, error)).toBe(true);

  expect(calls).toHaveBeenCalledExactlyOnceWith("msg", "source", 3, 4, error);

  expect(calls.mock.contexts).toStrictEqual([globalThis]);
  expect(exporter.getFinishedSpans()).toHaveLength(1);

  cleanup?.();

  cleanup = undefined;

  expect(globalThis.onerror).toBe(previous);

  globalThis.onerror("after", "source", 3, 4, createError());

  expect(calls).toHaveBeenCalledTimes(2);
  expect(exporter.getFinishedSpans()).toHaveLength(1);
});

test("restores the exact low and high pre-setup stack limits", () => {
  expect.assertions(4);

  for (const limit of [10, 75]) {
    Error.stackTraceLimit = limit;

    cleanup = errorsPlugin.setup(kernel);

    expect(Error.stackTraceLimit).toBe(Math.max(limit, 50));

    cleanup?.();

    cleanup = undefined;

    expect(Error.stackTraceLimit).toBe(limit);
  }
});

test.each(["addEventListener", "removeEventListener"] as const)(
  "requires callable %s before installing browser event listeners",
  (method) => {
    const methods = {
      addEventListener: globalThis.addEventListener,
      removeEventListener: globalThis.removeEventListener,
      [method]: undefined,
    };

    expect(
      hasEventListenerPair(
        methods.addEventListener,
        methods.removeEventListener,
      ),
    ).toBe(false);
  },
);

test("raises the stack limit to at least fifty during setup", () => {
  Error.stackTraceLimit = 10;

  cleanup = errorsPlugin.setup(kernel);

  expect(Error.stackTraceLimit).toBe(50);
});

test("does not capture digest-bearing global errors", () => {
  cleanup = errorsPlugin.setup(kernel);

  globalThis.onerror?.(
    "msg",
    "source",
    1,
    1,
    Object.assign(createError("Error", "server action"), {
      digest: "digest-abc",
    }),
  );

  expect(exporter.getFinishedSpans()).toStrictEqual([]);
});

test("keeps a later onerror handler installed and stops capturing beneath it", () => {
  const original = globalThis.onerror;

  onTestFinished(() => {
    // oxlint-disable-next-line unicorn/prefer-add-event-listener
    globalThis.onerror = original;
  });

  cleanup = errorsPlugin.setup(kernel);

  const ours = globalThis.onerror;
  const seen: string[] = [];

  const foreign: OnErrorEventHandlerNonNull = function foreign(
    this: Window,
    ...args
  ) {
    seen.push(args[0] instanceof Event ? args[0].type : args[0]);

    return ours?.apply(this, args) === true;
  };

  // oxlint-disable-next-line unicorn/prefer-add-event-listener
  globalThis.onerror = foreign;

  globalThis.onerror("during", "source", 1, 2, createError("Error", "during"));

  expect(exporter.getFinishedSpans()).toHaveLength(1);

  cleanup?.();

  cleanup = undefined;

  expect(globalThis.onerror).toBe(foreign);

  globalThis.onerror("after", "source", 1, 2, createError("Error", "after"));

  expect(seen).toStrictEqual(["during", "after"]);
  expect(exporter.getFinishedSpans()).toHaveLength(1);
});
