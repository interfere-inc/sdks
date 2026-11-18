import {
  ProxyTracerProvider,
  context,
  propagation,
  trace,
} from "@opentelemetry/api";
import { logs } from "@opentelemetry/api-logs";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import { W3CTraceContextPropagator } from "@opentelemetry/core";
import { NodeTracerProvider } from "@opentelemetry/sdk-trace-node";
import { HttpResponse, http, passthrough } from "msw";
import { once } from "node:events";
import { createServer } from "node:http";
import { setupServer } from "msw/node";
import {
  afterAll,
  afterEach,
  assert,
  beforeAll,
  beforeEach,
  expect,
  onTestFinished,
  test,
  vi,
} from "vitest";
import type { MockInstance } from "vitest";

await import("./instrumentation.js");

const server = setupServer(
  http.post("https://collector.test/*", () => HttpResponse.json({})),
);

const foreignProviders: NodeTracerProvider[] = [];

const originalConsole = { ...console };

let warn: MockInstance<Console["warn"]>;

function registerForeignProvider() {
  const provider = new NodeTracerProvider();

  foreignProviders.push(provider);

  context.setGlobalContextManager(
    new AsyncLocalStorageContextManager().enable(),
  );

  if (!trace.setGlobalTracerProvider(provider)) {
    const proxy = trace.getTracerProvider();

    // oxlint-disable-next-line typescript/no-deprecated -- The API exposes the registered provider only through its proxy delegate.
    if (proxy instanceof ProxyTracerProvider) {
      proxy.setDelegate(provider);
    }
  }

  propagation.setGlobalPropagator(new W3CTraceContextPropagator());

  return provider;
}

function activeDelegate() {
  const provider = trace.getTracerProvider();

  // oxlint-disable-next-line typescript/no-deprecated -- The API exposes the registered provider only through its proxy delegate.
  return provider instanceof ProxyTracerProvider
    ? provider.getDelegate()
    : provider;
}

async function load() {
  vi.resetModules();

  return import("./instrumentation.js");
}

beforeAll(() => {
  server.listen({ onUnhandledRequest: "error" });
});

beforeEach(() => {
  vi.stubEnv("INTERFERE_API_KEY", "interfere_pk_test");

  vi.stubEnv("INTERFERE_API_URL", "https://collector.test");

  vi.stubEnv("INTERFERE_SOURCE_ID", "abc0123456789def0123456789abcdef01234567");

  vi.stubEnv("NODE_ENV", "test");

  warn = vi.spyOn(console, "warn");
});

afterEach(async () => {
  Object.assign(console, originalConsole);

  const { uninstallGlobalHandlers } =
    await import("./internal/server/capture/global-handlers.js");

  uninstallGlobalHandlers();

  await Promise.all(
    foreignProviders.splice(0).map(async (provider) => provider.shutdown()),
  );

  trace.disable();

  context.disable();

  propagation.disable();

  logs.disable();

  vi.unstubAllEnvs();

  vi.restoreAllMocks();
});

afterAll(() => {
  server.close();
});

test("managed register() after a foreign tracer provider leaves the provider, context manager and propagator untouched and still flushes", async () => {
  expect.hasAssertions();

  const foreign = registerForeignProvider();
  const contextManager = context.active();
  const fields = propagation.fields();

  const mod = await load();

  await mod.register({ tracing: "managed" });

  expect(activeDelegate()).toBe(foreign);

  expect(propagation.fields()).toStrictEqual(fields);

  expect(context.active()).toBe(contextManager);

  expect(warn).toHaveBeenCalledTimes(1);

  await expect(mod.flush()).resolves.toBeUndefined();
});

test("managed register() after a foreign tracer provider names tracing application mode as the fix", async () => {
  expect.hasAssertions();

  registerForeignProvider();

  const mod = await load();

  await mod.register({ tracing: "managed" });

  expect(warn.mock.calls).toStrictEqual([
    [expect.stringContaining('tracing: "application"')],
  ]);
});

test("a foreign provider that replaces the proxy delegate after managed register() does not break flush", async () => {
  expect.hasAssertions();

  const mod = await load();

  await mod.register({ tracing: "managed" });

  const interfereDelegate = activeDelegate();

  const foreign = registerForeignProvider();

  expect(activeDelegate()).toBe(foreign);

  expect(activeDelegate()).not.toBe(interfereDelegate);

  await expect(mod.flush()).resolves.toBeUndefined();
});

test('register({ tracing: "application" }) writes no OpenTelemetry global and emits no warning', async () => {
  expect.hasAssertions();

  const foreign = registerForeignProvider();
  const loggerProvider = logs.getLoggerProvider();
  const fields = propagation.fields();

  const mod = await load();

  const options = { consoleBridge: true, tracing: "application" } as const;

  await mod.register({ ...options });

  expect(activeDelegate()).toBe(foreign);

  expect(propagation.fields()).toStrictEqual(fields);

  expect(logs.getLoggerProvider()).toBe(loggerProvider);

  expect(warn).not.toHaveBeenCalled();

  await expect(mod.flush()).resolves.toBeUndefined();
});

test("register(), capture and flush resolve beside a foreign provider whose tracer throws", async () => {
  expect.hasAssertions();

  trace.setGlobalTracerProvider({
    getTracer() {
      throw new Error("injected tracer failure");
    },
  });

  const collector = createServer((_, response) => {
    response.writeHead(204).end();
  }).listen(0, "127.0.0.1");

  onTestFinished(() => {
    collector.close();
  });

  await once(collector, "listening");

  const address = collector.address();

  assert(address instanceof Object);

  const url = `http://127.0.0.1:${address.port}`;

  server.use(http.all(`${url}/*`, () => passthrough()));

  vi.stubEnv("INTERFERE_API_URL", url);

  const mod = await load();

  await expect(
    mod.register({
      captureUncaughtException: false,
      captureUnhandledRejection: false,
      consoleBridge: false,
    }),
  ).resolves.toBeUndefined();

  await expect(
    mod.onRequestError(new Error("customer failure"), undefined, {
      routerKind: "App Router",
      routePath: "/broken",
      routeType: "route",
      renderSource: undefined,
      revalidateReason: undefined,
      renderType: undefined,
    }),
  ).resolves.toBeUndefined();

  await expect(mod.flush()).resolves.toBeUndefined();
});
