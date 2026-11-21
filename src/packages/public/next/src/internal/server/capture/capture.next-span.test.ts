import { context, trace } from "@opentelemetry/api";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
  TracerProvider,
} from "@opentelemetry/sdk-trace";

import { captureError, onRequestError } from "./capture.js";
import { nextSpanExceptionProcessor } from "./next-span-exceptions.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

let exporter: InMemorySpanExporter;

let provider: TracerProvider;

const ROUTE = {
  routePath: "/api/x",
  routeType: "route",
  routerKind: "App Router",
} as const;

function exceptionMechanisms(message: string) {
  return exporter
    .getFinishedSpans()
    .flatMap((span) =>
      span.events.flatMap((event) =>
        event.name === "exception" &&
        event.attributes?.["exception.message"] === message
          ? [event.attributes["interfere.exception.mechanism"] ?? "none"]
          : [],
      ),
    );
}

function nextSpanThrows(error: Error, tracer = "next.js") {
  const span = trace.getTracer(tracer).startSpan("executing api route");

  span.recordException(error);

  span.end();
}

describe("Next's own span exception events", () => {
  beforeEach(() => {
    vi.unstubAllEnvs();

    vi.stubEnv("NEXT_PUBLIC_INTERFERE_FORCE_ENABLE", "1");

    vi.stubEnv("INTERFERE_API_KEY", "interfere_pk_test_123");

    exporter = new InMemorySpanExporter();

    provider = new TracerProvider({
      spanProcessors: [
        nextSpanExceptionProcessor,
        new SimpleSpanProcessor({ exporter }),
      ],
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

  test("withdraws Next's event when Interfere captures the same throw afterwards", async () => {
    const error = new Error("route first");

    nextSpanThrows(error);

    await onRequestError(error, { method: "GET", path: "/api/x" }, ROUTE);

    expect(exceptionMechanisms("route first")).toStrictEqual([
      "auto.function.nextjs.route_handler",
    ]);
  });

  test("skips Next's event when Interfere captured the throw first", () => {
    const error = new Error("capture first");

    captureError(error);

    nextSpanThrows(error);

    expect(exceptionMechanisms("capture first")).toStrictEqual([
      "auto.function.nextjs.capture_error",
    ]);
  });

  test("withdraws Next's event when a separately bundled SDK copy captures the throw", async () => {
    const error = new Error("separate bundle after");

    nextSpanThrows(error);

    vi.resetModules();

    const separate = await import("./capture.js");

    separate.captureError(error);

    expect(exceptionMechanisms("separate bundle after")).toStrictEqual([
      "auto.function.nextjs.capture_error",
    ]);
  });

  test("skips Next's event when a separately bundled SDK copy captured the throw first", async () => {
    const error = new Error("separate bundle first");

    vi.resetModules();

    const separate = await import("./capture.js");

    separate.captureError(error);

    nextSpanThrows(error);

    await onRequestError(error, undefined, ROUTE);

    expect(exceptionMechanisms("separate bundle first")).toStrictEqual([
      "auto.function.nextjs.capture_error",
    ]);
  });

  test("keeps Next's event for a throw Interfere never captures", () => {
    nextSpanThrows(new Error("caught by the app"));

    expect(exceptionMechanisms("caught by the app")).toStrictEqual(["none"]);
  });

  test("keeps exception events from other tracers", async () => {
    const error = new Error("foreign tracer");

    nextSpanThrows(error, "@opentelemetry/instrumentation-http");

    await onRequestError(error, undefined, ROUTE);

    expect(exceptionMechanisms("foreign tracer")).toStrictEqual([
      "none",
      "auto.function.nextjs.route_handler",
    ]);
  });

  test("drops Next's event for framework control flow", () => {
    nextSpanThrows(
      Object.assign(new Error("NEXT_REDIRECT"), {
        digest: "NEXT_REDIRECT;replace;/login;307;",
      }),
    );

    expect(exceptionMechanisms("NEXT_REDIRECT")).toStrictEqual([]);
  });
});
