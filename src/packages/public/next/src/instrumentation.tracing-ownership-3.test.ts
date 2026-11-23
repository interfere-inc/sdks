import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
/* oxlint-disable eslint/no-console */
import { setupServer } from "msw/node";
import { HttpResponse, http, passthrough } from "msw";
import type { HttpHandler } from "msw";
import { releaseSourceIdEnvKeys } from "@interfere/types/integrations";
import { EXCEPTION_DELIVERY_ATTRIBUTE } from "@interfere/types/sdk/exception-delivery";
import { SERVICE_NAME_GLOBAL } from "@interfere/types/sdk/service-name";
import { RELEASE_SLUG_GLOBAL } from "@interfere/types/sdk/injected-globals";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import { W3CTraceContextPropagator } from "@opentelemetry/core";

import { context, logs, propagation, trace } from "@opentelemetry/api";
// oxlint-disable-next-line import/no-namespace -- This API consumes the complete module namespace.
import * as instrumentation from "@opentelemetry/instrumentation";
import { LoggerProvider } from "@opentelemetry/sdk-logs";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
  TracerProvider,
} from "@opentelemetry/sdk-trace";
import { mswServer } from "@interfere/test-config/msw-server";
import { fetchAndCacheRemoteConfig } from "./internal/server/remote-config.js";
import { once } from "node:events";
import {
  afterEach,
  beforeEach,
  describe,
  expect,
  onTestFinished,
  test,
  vi,
} from "vitest";

// oxlint-disable-next-line anti-slop/no-module-mocking, interfere-testing/no-test-doubles -- The startup contract requires observing and holding the remote-config task independently of HTTP.
vi.mock(
  import("./internal/server/remote-config.js"),
  async (importOriginal) => ({
    ...(await importOriginal()),
    fetchAndCacheRemoteConfig: vi.fn<typeof fetchAndCacheRemoteConfig>(),
  }),
);

vi.mock(import("@opentelemetry/instrumentation"), { spy: true });

await import("./instrumentation.js");

await import("./internal/server/capture/global-handlers.js");

const providerSpies = {
  register: vi.spyOn(trace, "setGlobalTracerProvider"),
  flushTraces: vi.spyOn(TracerProvider.prototype, "forceFlush"),
  flushLogs: vi.spyOn(LoggerProvider.prototype, "forceFlush"),
  registerInstrumentations: vi.spyOn(
    instrumentation,
    "registerInstrumentations",
  ),
};

const server = setupServer();

const originalConsole = {
  debug: console.debug,
  log: console.log,
  info: console.info,
  warn: console.warn,
  error: console.error,
};

const commitSha = "abc0123456789def0123456789abcdef01234567";

async function listenCollector(handler: HttpHandler) {
  const collector = mswServer(handler);

  collector.listen(0, "127.0.0.1");

  await once(collector, "listening");

  onTestFinished(async () => {
    collector.closeAllConnections();

    await new Promise<void>((resolve, reject) => {
      collector.close((error) => {
        if (error) {
          reject(error);
        } else {
          resolve();
        }
      });
    });
  });

  const address = collector.address();

  if (!(address instanceof Object)) {
    throw new TypeError("Missing collector port");
  }

  const url = `http://127.0.0.1:${address.port}`;

  vi.stubEnv("INTERFERE_API_URL", url);

  server.use(http.post(`${url}/*`, () => passthrough()));
}

async function load() {
  vi.resetModules();

  return import("./instrumentation.js");
}

beforeEach(() => {
  server.use(
    http.post("https://collector.test/drain/v1/logs", () =>
      HttpResponse.json({}, { status: 200 }),
    ),
  );

  server.use(
    http.post("https://collector.test/drain/v1/traces", () =>
      HttpResponse.json({}, { status: 200 }),
    ),
  );

  vi.clearAllMocks();

  // oxlint-disable-next-line interfere-testing/no-test-doubles -- Reset the remote-config startup contract fixture between cases.
  vi.mocked(fetchAndCacheRemoteConfig).mockReset().mockResolvedValue(undefined);

  providerSpies.register = vi.spyOn(trace, "setGlobalTracerProvider");

  providerSpies.flushTraces = vi.spyOn(TracerProvider.prototype, "forceFlush");

  providerSpies.flushLogs = vi.spyOn(LoggerProvider.prototype, "forceFlush");

  providerSpies.registerInstrumentations = vi.spyOn(
    instrumentation,
    "registerInstrumentations",
  );

  for (const key of releaseSourceIdEnvKeys) {
    vi.stubEnv(key, undefined);
  }

  vi.stubEnv("INTERFERE_SOURCE_ID", commitSha);

  setInjectedConfiguration(RELEASE_SLUG_GLOBAL, undefined);

  vi.stubEnv("INTERFERE_API_KEY", "interfere_pk_test");

  vi.stubEnv("INTERFERE_API_URL", "https://collector.test");

  vi.stubEnv("NODE_ENV", "test");

  vi.stubEnv("OTEL_SERVICE_NAME", undefined);

  setInjectedConfiguration(SERVICE_NAME_GLOBAL, undefined);

  vi.spyOn(console, "warn");
});

afterEach(async () => {
  Object.assign(console, originalConsole);

  const { uninstallGlobalHandlers } =
    await import("./internal/server/capture/global-handlers.js");

  uninstallGlobalHandlers();

  await Promise.all(
    providerSpies.register.mock.calls.map(async ([provider]) => {
      if (!(provider instanceof TracerProvider)) {
        throw new TypeError("Expected a native tracer provider registration");
      }

      return provider.shutdown();
    }),
  );

  const loggerProvider = logs.getLoggerProvider();

  if (loggerProvider instanceof LoggerProvider) {
    await loggerProvider.shutdown();
  }

  trace.disable();

  context.disable();

  propagation.disable();

  logs.disable();

  vi.unstubAllEnvs();

  vi.unstubAllGlobals();

  vi.restoreAllMocks();

  server.resetHandlers();
});

function hostOwnsOpenTelemetry() {
  const exporter = new InMemorySpanExporter();

  const provider = new TracerProvider({
    spanProcessors: [new SimpleSpanProcessor({ exporter })],
  });

  trace.setGlobalTracerProvider(provider);

  context.setGlobalContextManager(
    new AsyncLocalStorageContextManager().enable(),
  );

  propagation.setGlobalPropagator(new W3CTraceContextPropagator());

  return { exporter, provider };
}

async function recordingCollector() {
  const bodies: string[] = [];

  await listenCollector(
    http.post("*", async ({ request }) => {
      bodies.push(await request.text());

      return HttpResponse.json({});
    }),
  );

  return bodies;
}

describe("tracing ownership", () => {
  const quiet = {
    captureUncaughtException: false,
    captureUnhandledRejection: false,
    consoleBridge: false,
  };

  const REQUEST_CONTEXT = {
    routerKind: "App Router",
    routePath: "/",
    routeType: "render",
  } as const;

  async function raiseInsideHostSpan(
    mod: Awaited<ReturnType<typeof load>>,
    message: string,
  ) {
    return trace
      .getTracer("host")
      .startActiveSpan("host-render", async (span) => {
        await mod.onRequestError(
          new Error(message),
          { method: "GET", path: "/" },
          REQUEST_CONTEXT,
        );

        const spanContext = span.spanContext();

        span.end();

        return spanContext;
      });
  }

  test("application mode delivers an error raised inside a host span to Interfere when the host exports nowhere", async () => {
    const host = hostOwnsOpenTelemetry();

    const bodies = await recordingCollector();

    const mod = await load();

    await mod.register({ ...quiet, tracing: "application" });

    const hostContext = await raiseInsideHostSpan(
      mod,
      "boom-without-host-exporter",
    );

    await mod.flush();

    const exported = bodies.join("\n");

    expect(exported).toContain("boom-without-host-exporter");
    expect(exported).toContain(hostContext.traceId);
    expect(exported).toContain(hostContext.spanId);

    const hostRender = host.exporter
      .getFinishedSpans()
      .find((span) => span.name === "host-render");

    expect(hostRender?.events[0]?.attributes).toMatchObject({
      "exception.message": "boom-without-host-exporter",
      [EXCEPTION_DELIVERY_ATTRIBUTE]: "interfere",
    });
  });

  test('errors: "host" leaves an error raised inside a host span to the host exporter', async () => {
    const host = hostOwnsOpenTelemetry();

    const bodies = await recordingCollector();

    const mod = await load();

    await mod.register({ ...quiet, errors: "host", tracing: "application" });

    await raiseInsideHostSpan(mod, "boom-for-the-host-exporter");

    await mod.flush();

    expect(bodies.join("\n")).not.toContain("boom-for-the-host-exporter");

    const hostRender = host.exporter
      .getFinishedSpans()
      .find((span) => span.name === "host-render");

    expect(hostRender?.events[0]?.attributes).toMatchObject({
      "exception.message": "boom-for-the-host-exporter",
    });

    expect(hostRender?.events[0]?.attributes).not.toHaveProperty(
      EXCEPTION_DELIVERY_ATTRIBUTE,
    );
  });
});
