import { context, trace } from "@opentelemetry/api";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
  TracerProvider,
} from "@opentelemetry/sdk-trace";

import {
  installGlobalHandlers,
  uninstallGlobalHandlers,
} from "./global-handlers.js";
import type { GlobalHandlerOptions } from "./global-handlers.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

let exporter: InMemorySpanExporter;

let provider: TracerProvider;

function exceptionEvents() {
  return exporter
    .getFinishedSpans()
    .flatMap((s) =>
      s.events.flatMap((e) =>
        e.name === "exception" ? [e.attributes ?? {}] : [],
      ),
    );
}

function install(overrides: Partial<GlobalHandlerOptions> = {}) {
  const flush = vi.spyOn(provider, "forceFlush");
  const flushed = Promise.withResolvers<true>();

  installGlobalHandlers({
    captureUncaughtException: true,
    captureUnhandledRejection: true,
    flush: async () => {
      const result = provider.forceFlush();

      flushed.resolve(true);

      return result;
    },
    ...overrides,
  });

  return { flush, flushed: flushed.promise };
}

function emitUncaught(error: Error) {
  const listener = process.listeners("uncaughtException").at(-1);

  listener?.(error, "uncaughtException");
}

function emitUnhandled(reason: Error) {
  const listener = process.listeners("unhandledRejection").at(-1);

  listener?.(reason, Promise.resolve());
}

describe("global-handlers", () => {
  beforeEach(() => {
    vi.unstubAllEnvs();

    vi.stubEnv("NEXT_PUBLIC_INTERFERE_FORCE_ENABLE", "1");

    vi.stubEnv("INTERFERE_API_KEY", "interfere_pk_test_123");

    exporter = new InMemorySpanExporter();

    provider = new TracerProvider({
      spanProcessors: [new SimpleSpanProcessor({ exporter })],
    });

    trace.disable();

    context.disable();

    trace.setGlobalTracerProvider(provider);

    context.setGlobalContextManager(
      new AsyncLocalStorageContextManager().enable(),
    );
  });

  afterEach(async () => {
    uninstallGlobalHandlers();

    context.disable();

    trace.disable();

    await provider.shutdown();

    vi.restoreAllMocks();
  });

  test("captures uncaughtException with the node uncaught mechanism and flushes", async () => {
    expect.hasAssertions();

    const { flush, flushed } = install();

    emitUncaught(new Error("interfere-harness-server-uncaught-exception"));

    const events = exceptionEvents();

    expect(events).toHaveLength(1);

    expect(events[0]).toMatchObject({
      "exception.message": "interfere-harness-server-uncaught-exception",
      "exception.type": "Error",
      "interfere.exception.handled": false,
      "interfere.exception.kind": "error",
      "interfere.exception.mechanism":
        "auto.node.global_handlers.uncaught_exception",
    });

    await flushed;

    expect(flush).toHaveBeenCalledTimes(1);
  });

  test("captures unhandledRejection with the node unhandled mechanism and flushes", async () => {
    expect.hasAssertions();

    const { flush, flushed } = install();

    emitUnhandled(new Error("interfere-harness-server-unhandled-rejection"));

    const events = exceptionEvents();

    expect(events).toHaveLength(1);

    expect(events[0]).toMatchObject({
      "exception.message": "interfere-harness-server-unhandled-rejection",
      "interfere.exception.mechanism":
        "auto.node.global_handlers.unhandled_rejection",
    });

    await flushed;

    expect(flush).toHaveBeenCalledTimes(1);
  });

  test("installs each listener once", () => {
    const before = process.listeners("uncaughtException").length;

    install();

    install();

    expect(process.listeners("uncaughtException")).toHaveLength(before + 1);
  });

  test.each([null, undefined, false, 42, "rejected", { detail: "rejected" }])(
    "captures arbitrary unhandled rejection reasons: %j",
    async (reason) => {
      expect.hasAssertions();

      const { flush, flushed } = install();
      const listener = process.listeners("unhandledRejection").at(-1);

      expect(listener).toBeDefined();

      listener?.(reason, Promise.resolve());

      expect(exceptionEvents()).toHaveLength(1);

      expect(exceptionEvents()[0]).toMatchObject({
        "interfere.exception.mechanism":
          "auto.node.global_handlers.unhandled_rejection",
      });

      await flushed;

      expect(flush).toHaveBeenCalledTimes(1);
    },
  );

  test("removes exactly its listeners when uninstalled", () => {
    const before = process.listeners("uncaughtException");
    const beforeRejected = process.listeners("unhandledRejection");

    install();

    expect(process.listeners("uncaughtException")).toHaveLength(
      before.length + 1,
    );

    uninstallGlobalHandlers();

    expect(process.listeners("uncaughtException")).toStrictEqual(before);

    expect(process.listeners("unhandledRejection")).toStrictEqual(
      beforeRejected,
    );
  });
});
