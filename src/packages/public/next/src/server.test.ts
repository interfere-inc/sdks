import { context, trace } from "@opentelemetry/api";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
  TracerProvider,
} from "@opentelemetry/sdk-trace";

import { captureError, onRequestError } from "./server.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

let exporter: InMemorySpanExporter;

let provider: TracerProvider;

function exceptionEvents() {
  return exporter
    .getFinishedSpans()
    .flatMap((s) =>
      s.events
        .filter((e) => e.name === "exception")
        .map((e) => ({ attributes: e.attributes ?? {}, name: e.name })),
    );
}

describe("server", () => {
  beforeEach(() => {
    vi.unstubAllEnvs();

    vi.unstubAllGlobals();

    vi.stubEnv("NEXT_PUBLIC_INTERFERE_FORCE_ENABLE", "1");

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
    context.disable();

    trace.disable();

    await provider.shutdown();

    vi.restoreAllMocks();
  });

  test("no-ops when the env-gate is closed", () => {
    vi.stubEnv("NEXT_PUBLIC_INTERFERE_FORCE_ENABLE", "");

    vi.stubEnv("NODE_ENV", "development");

    captureError(new Error("test"));

    expect(exceptionEvents()).toHaveLength(0);
  });

  test("emits an exception event on a fresh span when no active span exists", () => {
    vi.stubEnv("INTERFERE_API_KEY", "interfere_pk_test_123");

    captureError(new Error("failed"));

    const events = exceptionEvents();

    expect(events).toHaveLength(1);

    expect(events[0]?.attributes).toMatchObject({
      "exception.message": "failed",
      "exception.type": "Error",
      "interfere.exception.handled": true,
      "interfere.exception.kind": "error",
      "interfere.exception.mechanism": "auto.function.nextjs.capture_error",
    });
  });

  test("emits the exception on the active span when one is in scope", () => {
    vi.stubEnv("INTERFERE_API_KEY", "interfere_pk_test_123");

    const span = trace.getTracer("test").startSpan("request");

    context.with(trace.setSpan(context.active(), span), () => {
      captureError(new Error("failed-in-span"));
    });

    span.end();

    const events = exceptionEvents();

    expect(events).toHaveLength(1);

    expect(events[0]?.attributes).toMatchObject({
      "exception.message": "failed-in-span",
      "exception.type": "Error",
    });

    expect(exporter.getFinishedSpans()).toHaveLength(1);
  });

  test("records on a fresh span when the active span has already ended", () => {
    vi.stubEnv("INTERFERE_API_KEY", "interfere_pk_test_123");

    const ended = trace.getTracer("test").startSpan("request");

    ended.end();

    context.with(trace.setSpan(context.active(), ended), () => {
      captureError(new Error("after-response"));
    });

    const events = exceptionEvents();

    expect(events).toHaveLength(1);

    expect(events[0]?.attributes).toMatchObject({
      "exception.message": "after-response",
      "interfere.exception.kind": "error",
    });
  });

  test("onRequestError dedupes by error instance and stamps request context", async () => {
    vi.stubEnv("INTERFERE_API_KEY", "interfere_pk_test_123");

    const error = new Error("request failure") as Error & { digest?: string };

    error.digest = "digest_123";

    await onRequestError(
      error,
      { method: "POST", path: "/api/interfere" },
      {
        routePath: "/api/interfere",
        routeType: "route",
        routerKind: "App Router",
      },
    );

    await onRequestError(
      error,
      { method: "POST", path: "/api/interfere" },
      {
        routePath: "/api/interfere",
        routeType: "route",
        routerKind: "App Router",
      },
    );

    const events = exceptionEvents();

    expect(events).toHaveLength(1);

    expect(events[0]?.attributes).toMatchObject({
      "interfere.error.digest": "digest_123",
      "interfere.exception.handled": false,
      "interfere.exception.mechanism": "auto.function.nextjs.route_handler",
    });
  });
});
