/* oxlint-disable typescript/prefer-promise-reject-errors, typescript/require-await, typescript/only-throw-error */
import { context, trace } from "@opentelemetry/api";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import {
  BasicTracerProvider,
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import { NextRequest } from "next/server";
import { NextFetchEvent } from "next/dist/server/web/spec-extension/fetch-event";

import { onRequestError } from "../capture/capture.js";
import { withInterfereProxy } from "./wrap-proxy.js";
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

function request(pathname: string) {
  return new NextRequest(
    new URL(pathname, "https://app.example.com"),
  ) satisfies NextRequest;
}

const event = new NextFetchEvent({
  request: request("/"),
  page: "/",
  context: undefined,
});

describe(withInterfereProxy, () => {
  beforeEach(() => {
    vi.unstubAllEnvs();

    vi.stubEnv("NEXT_PUBLIC_INTERFERE_FORCE_ENABLE", "1");

    vi.stubEnv("INTERFERE_API_KEY", "interfere_pk_test_123");

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

    vi.restoreAllMocks();
  });

  test("captures a synchronous proxy throw with the proxy mechanism + request context, then rethrows", () => {
    const wrapped = withInterfereProxy(() => {
      throw new Error("Simulated middleware error!");
    });

    // oxlint-disable-next-line typescript/promise-function-async
    expect(() => wrapped(request("/api/middleware-error"), event)).toThrow(
      "Simulated middleware error!",
    );

    const events = exceptionEvents();

    expect(events).toHaveLength(1);

    expect(events[0]).toMatchObject({
      "exception.message": "Simulated middleware error!",
      "http.request.method": "GET",
      "interfere.exception.handled": false,
      "interfere.exception.mechanism": "auto.function.nextjs.proxy",
      "interfere.nextjs.route_type": "proxy",
      "url.path": "/api/middleware-error",
    });
  });

  test("captures an async proxy rejection, then rethrows", async () => {
    const wrapped = withInterfereProxy(async () => {
      throw new Error("async proxy fail");
    });

    await expect(wrapped(request("/x"), event)).rejects.toThrow(
      "async proxy fail",
    );

    expect(exceptionEvents()[0]).toMatchObject({
      "interfere.exception.mechanism": "auto.function.nextjs.proxy",
    });
  });

  test("passes successful proxy results through untouched", () => {
    const response = new Response("ok");
    const wrapped = withInterfereProxy(() => response);

    expect(wrapped(request("/"), event)).toBe(response);
    expect(exceptionEvents()).toHaveLength(0);
  });

  test.each([null, undefined, false, 42, "failure", { detail: "failure" }])(
    "preserves arbitrary rejection identity: %j",
    async (reason) => {
      // oxlint-disable-next-line typescript/promise-function-async
      const wrapped = withInterfereProxy(() => Promise.reject(reason));

      await expect(wrapped(request("/rejected"), event)).rejects.toBe(reason);

      expect(exceptionEvents()).toHaveLength(1);

      expect(exceptionEvents()[0]).toMatchObject({
        "interfere.exception.mechanism": "auto.function.nextjs.proxy",
        "url.path": "/rejected",
      });
    },
  );

  test.each([
    ["an Error", () => new Error("proxy failure")],
    ["a string", () => "proxy failure"],
    ["an object", () => ({ detail: "proxy failure" })],
  ])(
    "records exactly one exception when Next's template also reports %s",
    async (_label, make) => {
      const thrown = make();

      const wrapped = withInterfereProxy(async () => {
        throw thrown;
      });

      const incoming = request("/dash?tab=1");

      try {
        await wrapped(incoming, event);
      } catch (error) {
        await onRequestError(
          error,
          {
            headers: Object.fromEntries(incoming.headers.entries()),
            method: incoming.method,
            path: "/dash?tab=1",
          },
          {
            revalidateReason: undefined,
            routePath: "/proxy",
            routeType: "proxy",
            routerKind: "Pages Router",
          },
        );
      }

      const events = exceptionEvents();

      expect(events).toHaveLength(1);

      expect(events[0]?.["interfere.exception.mechanism"]).toBe(
        "auto.function.nextjs.proxy",
      );
    },
  );

  test("skips a proxy AbortError", async () => {
    const wrapped = withInterfereProxy(async () => {
      throw new DOMException("aborted", "AbortError");
    });

    await expect(wrapped(request("/abort"), event)).rejects.toThrow("aborted");

    expect(exceptionEvents()).toHaveLength(0);
  });

  test("rethrows the customer's error when capture fails", () => {
    const failure = new Error("customer proxy failure");

    trace.disable();

    trace.setGlobalTracerProvider({
      getTracer() {
        throw new Error("injected tracer failure");
      },
    });

    const wrapped = withInterfereProxy(() => {
      throw failure;
    });

    expect(() => {
      void wrapped(request("/broken"), event);
    }).toThrow(failure);
  });
});
