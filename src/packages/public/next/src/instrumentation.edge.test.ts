import assert from "node:assert/strict";
import { HttpResponse, http } from "msw";
import { delivered, network, requests } from "../test/edge-network.fixture";
import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
/* oxlint-disable typescript/no-unsafe-assignment */
import { SERVER_TRACING_GLOBAL } from "@interfere/types/sdk/server-tracing";
import { EXCEPTION_DELIVERY_ATTRIBUTE } from "@interfere/types/sdk/exception-delivery";
import type { ProxyTracerProvider } from "@opentelemetry/api";
import { context, trace } from "@opentelemetry/api";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import {
  BasicTracerProvider,
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import type { MockInstance } from "vitest";
import { afterEach, beforeAll, beforeEach, expect, test, vi } from "vitest";

let registration: MockInstance<typeof trace.setGlobalTracerProvider>;

beforeAll(async () => {
  await import("./instrumentation.edge.js");
});

beforeEach(() => {
  vi.resetModules();

  trace.disable();

  registration = vi.spyOn(trace, "setGlobalTracerProvider");

  vi.stubEnv("INTERFERE_PUBLIC_KEY", "interfere_pk_test");

  vi.stubEnv("INTERFERE_API_URL", "https://edge.invalid");

  vi.stubEnv("INTERFERE_ENVIRONMENT", "preview");

  vi.stubEnv("NEXT_PUBLIC_INTERFERE_FORCE_ENABLE", "");

  setInjectedConfiguration(SERVER_TRACING_GLOBAL, undefined);

  vi.spyOn(console, "warn");

  vi.spyOn(console, "info");

  network.use(
    http.post(
      "https://edge.invalid/drain/v1/traces",
      () => new HttpResponse(null, { status: 204 }),
    ),
  );
});

afterEach(async () => {
  try {
    const providers = registration.mock.calls.flatMap(([provider]) =>
      provider instanceof BasicTracerProvider ? [provider] : [],
    );

    await Promise.all(providers.map(async (provider) => provider.shutdown()));

    for (const request of requests) {
      assert.equal(request.method, "POST");

      assert.equal(request.url, "https://edge.invalid/drain/v1/traces");

      assert.equal(
        request.headers.get("x-interfere-public-key"),
        "interfere_pk_test",
      );
    }
  } finally {
    trace.disable();

    vi.unstubAllGlobals();

    vi.unstubAllEnvs();

    vi.restoreAllMocks();
  }
});

test("leaves the global tracer non-recording without a public key", async () => {
  vi.stubEnv("INTERFERE_PUBLIC_KEY", "");

  const { register } = await import("./instrumentation.edge.js");

  await expect(register()).resolves.toBeUndefined();

  const span = trace.getTracer("edge-test").startSpan("no-key");

  expect(span.isRecording()).toBe(false);

  span.end();

  expect(registration).not.toHaveBeenCalled();
  expect(requests).toStrictEqual([]);
  expect(console.warn).not.toHaveBeenCalled();
});

test("keeps one registered provider and the original service across repeated calls", async () => {
  const { register } = await import("./instrumentation.edge.js");

  const { flushServerTelemetry } =
    await import("./internal/server/exporters/server-flush.js");

  await register({ serviceName: "first-service" });

  const tracer = trace.getTracer("edge-test");

  await register({ serviceName: "ignored-service" });

  expect(trace.getTracer("edge-test")).toBe(tracer);
  expect(registration).toHaveBeenCalledTimes(1);

  const span = tracer.startSpan("after-second-register");

  expect(span.isRecording()).toBe(true);

  span.end();

  await flushServerTelemetry();

  await delivered();

  expect(requests).toHaveLength(1);

  await expect(requests[0]?.json()).resolves.toMatchObject({
    resourceSpans: [
      {
        resource: {
          attributes: expect.arrayContaining([
            { key: "service.name", value: { stringValue: "first-service" } },
          ]),
        },
      },
    ],
  });
});

test("exports Edge spans directly even when drain mode is configured", async () => {
  setInjectedConfiguration(SERVER_TRACING_GLOBAL, "drain");

  setInjectedConfiguration("__INTERFERE_FORCE_ENABLE__", true);

  const { register } = await import("./instrumentation.edge.js");

  const { flushServerTelemetry } =
    await import("./internal/server/exporters/server-flush.js");

  await register();

  const span = trace.getTracer("edge-test").startSpan("drain-no-export");

  expect(span.isRecording()).toBe(true);

  span.addEvent("exception", { "exception.message": "retained locally" });

  span.end();

  await flushServerTelemetry();

  await delivered();

  expect(registration.mock.results[0]?.value).toBe(true);
  expect(requests).toHaveLength(1);
});

test("resolves and warns once when a registration step throws", async () => {
  const failure = new Error("injected registration failure");

  const { register } = await import("./instrumentation.edge.js");

  await expect(
    register(
      Object.defineProperty({}, "serviceName", {
        get() {
          throw failure;
        },
      }),
    ),
  ).resolves.toBeUndefined();

  expect(registration).not.toHaveBeenCalled();

  expect(console.warn).toHaveBeenCalledWith(
    expect.stringContaining("[interfere] register() failed"),
    failure,
  );
});

test("application mode registers no global provider and captures errors through its isolated provider", async () => {
  vi.stubEnv("OTEL_TRACES_SAMPLER", "always_off");

  const { onRequestError, register } =
    await import("./instrumentation.edge.js");

  await expect(register({ tracing: "application" })).resolves.toBeUndefined();

  expect(registration).not.toHaveBeenCalled();

  const span = trace.getTracer("edge-test").startSpan("unowned");

  expect(span.isRecording()).toBe(false);

  span.end();

  await onRequestError(
    new Error("edge-application-error"),
    { method: "GET", path: "/" },
    { routerKind: "App Router", routePath: "/", routeType: "render" },
  );

  await delivered();

  expect(requests).toHaveLength(1);

  await expect(requests[0]?.text()).resolves.toContain(
    "edge-application-error",
  );

  expect(
    vi
      .mocked(console.warn)
      .mock.calls.filter(([message]) =>
        String(message).includes("already registered"),
      ),
  ).toStrictEqual([]);
});

async function raiseInsideApplicationSpan(message: string) {
  const exporter = new InMemorySpanExporter();

  const application = new BasicTracerProvider({
    spanProcessors: [new SimpleSpanProcessor(exporter)],
  });

  trace.setGlobalTracerProvider(application);

  context.setGlobalContextManager(
    new AsyncLocalStorageContextManager().enable(),
  );

  const { onRequestError } = await import("./instrumentation.edge.js");

  const spanContext = await trace
    .getTracer("edge-application")
    .startActiveSpan("host-render", async (span) => {
      await onRequestError(
        new Error(message),
        { method: "GET", path: "/" },
        { routerKind: "App Router", routePath: "/", routeType: "render" },
      );

      const current = span.spanContext();

      span.end();

      return current;
    });

  context.disable();

  return { exporter, spanContext };
}

test("application mode delivers an error raised inside a recording application span and marks the application copy", async () => {
  const { register } = await import("./instrumentation.edge.js");

  await expect(register({ tracing: "application" })).resolves.toBeUndefined();

  const { exporter, spanContext } = await raiseInsideApplicationSpan(
    "edge-application-span-error",
  );

  await delivered();

  expect(requests).toHaveLength(1);

  const body = await requests[0]?.text();

  expect(body).toContain("edge-application-span-error");
  expect(body).toContain(spanContext.traceId);
  expect(body).toContain(spanContext.spanId);

  expect(exporter.getFinishedSpans()[0]?.events[0]?.attributes).toMatchObject({
    "exception.message": "edge-application-span-error",
    [EXCEPTION_DELIVERY_ATTRIBUTE]: "interfere",
  });
});

test('errors: "host" leaves an error raised inside a recording application span to the application exporter', async () => {
  const { register } = await import("./instrumentation.edge.js");

  await expect(
    register({ errors: "host", tracing: "application" }),
  ).resolves.toBeUndefined();

  const { exporter } = await raiseInsideApplicationSpan(
    "edge-application-host-only",
  );

  expect(requests).toStrictEqual([]);

  expect(exporter.getFinishedSpans()[0]?.events[0]?.attributes).toMatchObject({
    "exception.message": "edge-application-host-only",
  });

  expect(
    exporter.getFinishedSpans()[0]?.events[0]?.attributes,
  ).not.toHaveProperty(EXCEPTION_DELIVERY_ATTRIBUTE);
});

test("a foreign edge provider registered first stays global and Interfere warns once (E1)", async () => {
  const foreign = new BasicTracerProvider();

  expect(trace.setGlobalTracerProvider(foreign)).toBe(true);

  const { register } = await import("./instrumentation.edge.js");

  await expect(register({ tracing: "managed" })).resolves.toBeUndefined();

  // oxlint-disable-next-line typescript/no-deprecated -- OTel 1.x exposes the registered provider through its proxy delegate.
  const provider = trace.getTracerProvider() as ProxyTracerProvider;

  const delegate = provider.getDelegate();

  expect(delegate).toBe(foreign);

  expect(
    vi
      .mocked(console.warn)
      .mock.calls.filter(([message]) =>
        String(message).includes("already registered"),
      ),
  ).toHaveLength(1);
});
