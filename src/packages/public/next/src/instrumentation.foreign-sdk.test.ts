import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
/* oxlint-disable typescript/no-deprecated */
import { mswServer } from "@interfere/test-config/msw-server";
import { SERVER_TRACING_GLOBAL } from "@interfere/types/sdk/server-tracing";
import {
  context,
  isSpanContextValid,
  logs,
  propagation,
  trace,
} from "@opentelemetry/api";
import type { ProxyTracerProvider } from "@opentelemetry/api";
import { AsyncLocalStorageContextManager } from "@interfere/server-core/otel/context-async-hooks";
import {
  CompositePropagator,
  W3CBaggagePropagator,
  W3CTraceContextPropagator,
} from "@interfere/server-core/otel/core/propagation";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@interfere/server-core/otel/sdk/trace/processing";
import { TracerProvider } from "@interfere/server-core/otel/sdk/trace/provider";
import { once } from "node:events";
import { HttpResponse, http } from "msw";
import {
  afterAll,
  afterEach,
  beforeAll,
  beforeEach,
  expect,
  test,
  vi,
} from "vitest";

await import("./instrumentation.js");

const requests: string[] = [];

const server = mswServer(
  http.post("*", async ({ request }) => {
    requests.push(await request.text());

    return HttpResponse.json({});
  }),
);

let collectorUrl: string;

beforeAll(async () => {
  server.listen(0, "127.0.0.1");

  await once(server, "listening");

  const address = server.address();

  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Node returns a string for Unix sockets; this test requires a TCP listener.
  if (address === null || typeof address === "string") {
    throw new Error("Expected a TCP address");
  }

  collectorUrl = `http://127.0.0.1:${address.port}`;
});

beforeEach(() => {
  vi.resetModules();

  requests.length = 0;

  vi.stubEnv("INTERFERE_API_KEY", "interfere_pk_test");

  vi.stubEnv("INTERFERE_API_URL", collectorUrl);

  vi.stubEnv("INTERFERE_SOURCE_ID", "0123456789abcdef0123456789abcdef01234567");

  vi.stubEnv("NEXT_PUBLIC_INTERFERE_FORCE_ENABLE", "1");

  setInjectedConfiguration(SERVER_TRACING_GLOBAL, "full");

  vi.spyOn(console, "warn");
});

afterEach(async () => {
  const provider = (
    trace.getTracerProvider() as ProxyTracerProvider
  ).getDelegate() as Partial<TracerProvider>;

  await provider.shutdown?.();

  trace.disable();

  context.disable();

  propagation.disable();

  logs.disable();

  vi.unstubAllEnvs();

  vi.unstubAllGlobals();

  vi.restoreAllMocks();
});

afterAll(async () => {
  server.closeAllConnections();

  await server[Symbol.asyncDispose]();
});

const options = {
  consoleBridge: false,
  captureUncaughtException: false,
  captureUnhandledRejection: false,
};

const REQUEST_CONTEXT = {
  routerKind: "App Router",
  routePath: "/",
  routeType: "render",
} as const;

function hostOwnsOpenTelemetry() {
  const exporter = new InMemorySpanExporter();

  const provider = new TracerProvider({
    spanProcessors: [new SimpleSpanProcessor({ exporter })],
  });

  const registered = {
    tracer: trace.setGlobalTracerProvider(provider),
    propagator: propagation.setGlobalPropagator(
      new CompositePropagator({
        propagators: [
          new W3CTraceContextPropagator(),
          new W3CBaggagePropagator(),
        ],
      }),
    ),
    context: context.setGlobalContextManager(
      new AsyncLocalStorageContextManager().enable(),
    ),
  };

  return { exporter, provider, registered, fields: propagation.fields() };
}

function registrationWarnings() {
  return vi
    .mocked(console.warn)
    .mock.calls.filter(([message]) =>
      String(message).includes("already registered"),
    );
}

async function load() {
  return import("./instrumentation.js");
}

test("a host that owns provider, propagator and context manager first keeps them; Interfere warns once and captures through its isolated provider (N1a)", async () => {
  const host = hostOwnsOpenTelemetry();

  expect(host.registered).toStrictEqual({
    tracer: true,
    propagator: true,
    context: true,
  });

  const { register, flush, onRequestError } = await load();

  await expect(
    register({ ...options, tracing: "managed" }),
  ).resolves.toBeUndefined();

  expect(registrationWarnings()).toHaveLength(1);

  expect(propagation.fields()).toStrictEqual(host.fields);

  expect((trace.getTracerProvider() as ProxyTracerProvider).getDelegate()).toBe(
    host.provider,
  );

  await trace
    .getTracer("host")
    .startActiveSpan("host-request", async (span) => {
      await onRequestError(
        new Error("boom-under-host"),
        { method: "GET", path: "/" },
        REQUEST_CONTEXT,
      );

      span.end();
    });

  await flush();

  expect(host.exporter.getFinishedSpans().map((span) => span.name)).toContain(
    "host-request",
  );

  expect(requests.join("\n")).toContain("boom-under-host");

  expect(requests.join("\n")).not.toContain("host-request");
});

test("a host tracer provider registered first gets no global logger provider from Interfere (N1b)", async () => {
  hostOwnsOpenTelemetry();

  const before = logs.getLoggerProvider();

  const { register } = await load();

  await register(options);

  expect(logs.getLoggerProvider()).toBe(before);
});

test("a later foreign setup cannot replace the globals Interfere registered first and Interfere keeps exporting (N1c)", async () => {
  const { register, flush } = await load();

  await register(options);

  const host = hostOwnsOpenTelemetry();

  expect(host.registered).toStrictEqual({
    tracer: false,
    propagator: false,
    context: false,
  });

  expect(registrationWarnings()).toHaveLength(0);

  const span = trace.getTracer("customer").startSpan("after-foreign-attempt");

  const carrier: Record<string, string> = {};

  propagation.inject(trace.setSpan(context.active(), span), carrier);

  span.end();

  expect(carrier["traceparent"]).toMatch(
    new RegExp(`^00-${span.spanContext().traceId}-[0-9a-f]{16}-01$`, "u"),
  );

  await flush();

  expect(requests.join("\n")).toContain("after-foreign-attempt");

  expect(host.exporter.getFinishedSpans()).toStrictEqual([]);
});

test("a guest SDK reading trace.getActiveSpan() inside an Interfere-managed span sees the exported span (N2)", async () => {
  const { register, flush } = await load();

  await register(options);

  const seen = trace
    .getTracer("app")
    .startActiveSpan("host-visible", (span) => {
      const active = trace.getActiveSpan()?.spanContext();

      span.end();

      return active;
    });

  expect(seen).toBeDefined();

  expect(seen && isSpanContextValid(seen)).toBe(true);

  await flush();

  expect(requests.join("\n")).toContain("host-visible");

  expect(requests.join("\n")).toContain(seen?.traceId);
});

test.each(["application", "auto", undefined] as const)(
  "%s mode writes no global, warns nothing and records errors on the host's active span (N4)",
  async (tracing) => {
    const host = hostOwnsOpenTelemetry();

    const loggerBefore = logs.getLoggerProvider();

    const { register, flush, onRequestError } = await load();

    await register(omitUndefined({ ...options, tracing }));

    expect(registrationWarnings()).toHaveLength(0);

    expect(logs.getLoggerProvider()).toBe(loggerBefore);

    expect(propagation.fields()).toStrictEqual(host.fields);

    await trace
      .getTracer("host")
      .startActiveSpan("host-render", async (span) => {
        await onRequestError(
          new Error("boom-in-host-span"),
          { method: "GET", path: "/" },
          REQUEST_CONTEXT,
        );

        span.end();
      });

    await flush();

    const hostRender = host.exporter
      .getFinishedSpans()
      .find((span) => span.name === "host-render");

    expect(
      hostRender?.events.some(
        (event) =>
          event.name === "exception" &&
          event.attributes?.["exception.message"] === "boom-in-host-span",
      ),
    ).toBe(true);
  },
);
