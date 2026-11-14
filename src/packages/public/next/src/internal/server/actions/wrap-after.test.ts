/* oxlint-disable typescript/prefer-promise-reject-errors, typescript/require-await */
import { context, trace } from "@opentelemetry/api";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import {
  BasicTracerProvider,
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import { notFound } from "next/navigation";

import { withInterfereAfter } from "./wrap-after.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

let exporter: InMemorySpanExporter;

let provider: BasicTracerProvider;

function exceptionEvents() {
  return exporter
    .getFinishedSpans()
    .flatMap((s) =>
      s.events.flatMap((e) =>
        e.name === "exception" ? [e.attributes ?? {}] : [],
      ),
    );
}

describe(withInterfereAfter, () => {
  beforeEach(() => {
    vi.unstubAllEnvs();

    vi.stubEnv("NEXT_PUBLIC_INTERFERE_FORCE_ENABLE", "1");

    vi.stubEnv("INTERFERE_PUBLIC_KEY", "interfere_pk_test_123");

    exporter = new InMemorySpanExporter();

    provider = new BasicTracerProvider({
      spanProcessors: [new SimpleSpanProcessor(exporter)],
    });

    trace.disable();

    context.disable();

    trace.setGlobalTracerProvider(provider);

    context.setGlobalContextManager(
      new AsyncLocalStorageContextManager().enable(),
    );
  });

  afterEach(async () => {
    context.disable();

    trace.disable();

    await provider.shutdown();
  });

  test("captures a synchronous callback throw once with the after mechanism, then rethrows it", async () => {
    const failure = new Error("after callback failed");

    const task = withInterfereAfter(() => {
      throw failure;
    });

    await expect(task()).rejects.toBe(failure);

    const events = exceptionEvents();

    expect(events).toHaveLength(1);

    expect(events[0]).toMatchObject({
      "exception.message": "after callback failed",
      "interfere.exception.handled": false,
      "interfere.exception.mechanism": "auto.function.nextjs.after",
    });
  });

  test("captures an async callback rejection", async () => {
    const task = withInterfereAfter(async () => {
      throw new Error("async after failed");
    });

    await expect(task()).rejects.toThrow("async after failed");

    expect(exceptionEvents()).toStrictEqual([
      expect.objectContaining({
        "exception.message": "async after failed",
        "interfere.exception.mechanism": "auto.function.nextjs.after",
      }),
    ]);
  });

  test("captures a rejected promise passed to after()", async () => {
    const task = withInterfereAfter(Promise.reject("after promise rejected"));

    await expect(task).rejects.toBe("after promise rejected");

    expect(exceptionEvents()).toStrictEqual([
      expect.objectContaining({
        "interfere.exception.mechanism": "auto.function.nextjs.after",
      }),
    ]);
  });

  test("resolves a successful callback's value without recording anything", async () => {
    const task = withInterfereAfter(() => 42);

    await expect(task()).resolves.toBe(42);

    await expect(withInterfereAfter(Promise.resolve("done"))).resolves.toBe(
      "done",
    );

    expect(exceptionEvents()).toHaveLength(0);
  });

  test("does not run the callback until after() invokes it", () => {
    const callback = vi.fn<() => void>();

    withInterfereAfter(callback);

    expect(callback).not.toHaveBeenCalled();
  });

  test("ignores Next.js control-flow throws", async () => {
    const task = withInterfereAfter(() => {
      notFound();
    });

    await expect(task()).rejects.toThrow("NEXT_HTTP_ERROR_FALLBACK");

    expect(exceptionEvents()).toHaveLength(0);
  });
});
