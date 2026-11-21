import { context, trace } from "@opentelemetry/api";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
  TracerProvider,
} from "@opentelemetry/sdk-trace";

import { captureError, onRequestError } from "./capture.js";
import type { onRequestError as publicOnRequestError } from "../../../instrumentation.js";
import type { Instrumentation } from "next";
import {
  afterEach,
  beforeEach,
  describe,
  expect,
  expectTypeOf,
  test,
  vi,
} from "vitest";

let exporter: InMemorySpanExporter;

let provider: TracerProvider;

const controlFlow = (digest: string) =>
  Object.assign(new Error("control flow"), { digest });

function exceptionEvents() {
  return exporter
    .getFinishedSpans()
    .flatMap((s) =>
      s.events
        .filter((e) => e.name === "exception")
        .map((e) => e.attributes ?? {}),
    );
}

describe("onRequestError route context", () => {
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
    context.disable();

    trace.disable();

    await provider.shutdown();

    vi.restoreAllMocks();
  });

  test("captures route, method, and a 500 status for a thrown route handler", async () => {
    const error = Object.assign(new Error("Simulated API route error"), {
      digest: "digest-xyz",
    });

    await onRequestError(
      error,
      { method: "GET", path: "/api/test-error" },
      {
        routePath: "/api/test-error",
        routeType: "route",
        routerKind: "App Router",
      },
    );

    const events = exceptionEvents();

    expect(events).toHaveLength(1);

    expect(events[0]).toMatchObject({
      "http.request.method": "GET",
      "http.response.status_code": 500,
      "http.route": "/api/test-error",
      "interfere.error.digest": "digest-xyz",
      "interfere.exception.handled": false,
      "interfere.exception.mechanism": "auto.function.nextjs.route_handler",
      "url.path": "/api/test-error",
    });
  });

  test("uses the generic mechanism and no status for non-route surfaces (RSC render)", async () => {
    await onRequestError(
      new Error("render boom"),
      { method: "GET", path: "/dash" },
      {
        routePath: "/dash",
        routeType: "render",
        routerKind: "App Router",
      },
    );

    const [attrs] = exceptionEvents();

    expect(attrs?.["interfere.exception.mechanism"]).toBe(
      "auto.function.nextjs.on_request_error",
    );

    expect(attrs?.["http.request.method"]).toBe("GET");
    expect(attrs?.["http.route"]).toBe("/dash");
    expect(attrs?.["http.response.status_code"]).toBeUndefined();
  });

  test("tolerates a missing request argument, still tagging the route template", async () => {
    await onRequestError(new Error("no request"), undefined, {
      routePath: "/api/x",
      routeType: "route",
      routerKind: "App Router",
    });

    const [attrs] = exceptionEvents();

    expect(attrs?.["http.request.method"]).toBeUndefined();
    expect(attrs?.["url.path"]).toBeUndefined();
    expect(attrs?.["http.route"]).toBe("/api/x");
  });

  test("captures only allowlisted request headers, never auth/PII", async () => {
    await onRequestError(
      new Error("boom"),
      {
        headers: {
          authorization: "Bearer tok",
          cookie: "session=secret",
          referer: "https://app.example.com/",
          "user-agent": "Mozilla/5.0",
        },
        method: "POST",
        path: "/api/x",
      },
      {
        routePath: "/api/x",
        routeType: "route",
        routerKind: "App Router",
      },
    );

    const [attrs] = exceptionEvents();

    expect(attrs?.["http.request.header.user-agent"]).toBe("Mozilla/5.0");

    expect(attrs?.["http.request.header.referer"]).toBe(
      "https://app.example.com/",
    );

    expect(attrs?.["http.request.header.cookie"]).toBeUndefined();
    expect(attrs?.["http.request.header.authorization"]).toBeUndefined();
  });

  test("accepts Next's hook contract and captures a string throw with array headers", async () => {
    expectTypeOf<
      typeof publicOnRequestError
    >().toExtend<Instrumentation.onRequestError>();

    const hook: Instrumentation.onRequestError = onRequestError;

    await hook(
      "string failure",
      {
        headers: {
          "accept-language": ["en-US", "fr"],
          authorization: ["Bearer secret"],
          cookie: ["session=secret"],
        },
        method: "GET",
        path: "/api/headers",
      },
      {
        routePath: "/api/headers",
        routeType: "route",
        routerKind: "App Router",
        revalidateReason: undefined,
      },
    );

    const events = exceptionEvents();

    expect(events).toHaveLength(1);

    expect(events[0]).toMatchObject({
      "exception.type": "string",
      "exception.message": "string failure",
      "interfere.exception.kind": "non-error",
      "interfere.exception.handled": false,
      "http.request.header.accept-language": "en-US, fr",
      "http.response.status_code": 500,
    });

    expect(events[0]?.["exception.stacktrace"]).toBeUndefined();
    expect(events[0]?.["http.request.header.authorization"]).toBeUndefined();
    expect(events[0]?.["http.request.header.cookie"]).toBeUndefined();
  });

  test("deduplicates Error instances across request and manual capture", async () => {
    const error = new Error("same failure");

    const requestContext = { routeType: "route" } as const;

    await onRequestError(error, undefined, requestContext);

    await onRequestError(error, undefined, requestContext);

    captureError(error);

    expect(exceptionEvents()).toHaveLength(1);
  });

  test.each([null, undefined, false, 42, ["agent", 42], { value: "agent" }])(
    "ignores invalid header values supplied by JavaScript adapters: %j",
    async (value) => {
      const headers = {
        "user-agent": "original",
        referer: "https://app.example.com/",
      };

      Object.defineProperty(headers, "user-agent", { value });

      await onRequestError(
        new Error("invalid header"),
        { headers, method: "GET", path: "/api/headers" },
        {
          routePath: "/api/headers",
          routeType: "route",
          routerKind: "App Router",
        },
      );

      const [attrs] = exceptionEvents();

      expect(attrs?.["http.request.header.user-agent"]).toBeUndefined();

      expect(attrs?.["http.request.header.referer"]).toBe(
        "https://app.example.com/",
      );
    },
  );
});

describe("next control-flow sentinels are never captured as errors", () => {
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
    context.disable();

    trace.disable();

    await provider.shutdown();

    vi.restoreAllMocks();
  });

  test.each([
    ["redirect()", "NEXT_REDIRECT;replace;/login;307;"],
    ["notFound()", "NEXT_HTTP_ERROR_FALLBACK;404"],
    ["forbidden()", "NEXT_HTTP_ERROR_FALLBACK;403"],
    ["unauthorized()", "NEXT_HTTP_ERROR_FALLBACK;401"],
    ["legacy notFound()", "NEXT_NOT_FOUND"],
  ])("captureError skips %s and records nothing", (_label, digest) => {
    captureError(controlFlow(digest));

    expect(exceptionEvents()).toHaveLength(0);
  });

  test("still captures a real error that carries an unrelated digest", () => {
    captureError(controlFlow("digest-xyz"));

    expect(exceptionEvents()).toHaveLength(1);
  });

  test("names the fallback span after the throw site outside a request span", () => {
    const error = new Error("failure");

    error.stack =
      "Error: failure\n    at loadInvoice (webpack-internal:///(rsc)/./app/invoices/page.tsx:14:11)";

    captureError(error);

    const [span] = exporter.getFinishedSpans();

    expect(span?.name).toBe("loadInvoice (page.tsx:14)");

    expect(span?.attributes).toMatchObject({
      "interfere.capture.kind": "exception",
    });
  });

  test.each([null, undefined, 42, false, {}, ["NEXT_REDIRECT"]])(
    "captures errors with non-string digest %j",
    (digest) => {
      captureError(Object.assign(new Error("failure"), { digest }));

      expect(exceptionEvents()).toHaveLength(1);
    },
  );

  test.each([null, undefined, 42, false, "failure", { detail: "failure" }])(
    "captures non-Error values without treating them as control-flow errors %j",
    (value) => {
      captureError(value, { request: "ignored compatibility argument" });

      captureError(value, null);

      expect(exceptionEvents()).toHaveLength(2);

      expect(exceptionEvents()[0]?.["exception.message"]).toStrictEqual(
        expect.any(String),
      );
    },
  );

  test("onRequestError ignores a control-flow sentinel", async () => {
    await onRequestError(
      controlFlow("NEXT_REDIRECT;replace;/login;307;"),
      { method: "GET", path: "/api/x" },
      {
        routePath: "/api/x",
        routeType: "route",
        routerKind: "App Router",
      },
    );

    expect(exceptionEvents()).toHaveLength(0);
  });

  test("captureError returns normally when the tracer fails", () => {
    installBrokenTracerProvider();

    expect(() => {
      captureError(new Error("customer failure"));
    }).not.toThrow();
  });

  test("onRequestError resolves when the tracer fails", async () => {
    installBrokenTracerProvider();

    await expect(
      onRequestError(
        new Error("customer failure"),
        { method: "GET", path: "/api/x" },
        { routePath: "/api/x", routeType: "route", routerKind: "App Router" },
      ),
    ).resolves.toBeUndefined();
  });
});

function installBrokenTracerProvider() {
  trace.disable();

  trace.setGlobalTracerProvider({
    getTracer() {
      throw new Error("injected tracer failure");
    },
  });
}
