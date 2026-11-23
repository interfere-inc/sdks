import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
/* oxlint-disable eslint/no-console */
import { setupServer } from "msw/node";
import { HttpResponse, http, passthrough } from "msw";
import type { HttpHandler } from "msw";
import { releaseSourceIdEnvKeys } from "@interfere/types/integrations";
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

// oxlint-disable-next-line anti-slop/no-module-mocking, landmines/no-test-doubles -- The startup contract requires observing and holding the remote-config task independently of HTTP.
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

  // oxlint-disable-next-line landmines/no-test-doubles -- Reset the remote-config startup contract fixture between cases.
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

function warnings(fragment: string) {
  return vi
    .mocked(console.warn)
    .mock.calls.filter(([message]) => String(message).includes(fragment));
}

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

  test("managed mode keeps the tracer provider when only a foreign propagator exists and warns once about headers", async () => {
    propagation.setGlobalPropagator(new W3CTraceContextPropagator());

    const mod = await load();

    await mod.register(quiet);

    expect(providerSpies.register).toHaveBeenCalledTimes(1);
    expect(propagation.fields()).toStrictEqual(["traceparent", "tracestate"]);
    expect(providerSpies.registerInstrumentations).not.toHaveBeenCalled();
    expect(warnings("propagator already owns trace headers")).toHaveLength(1);
    expect(warnings("already registered")).toHaveLength(0);
  });

  test.each(["managed", "application"] as const)(
    "an always_off OTEL_TRACES_SAMPLER never silences %s-mode capture through the isolated provider",
    async (tracing) => {
      vi.stubEnv("OTEL_TRACES_SAMPLER", "always_off");

      const bodies = await recordingCollector();

      hostOwnsOpenTelemetry();

      const mod = await load();

      await mod.register({ ...quiet, tracing });

      await trace
        .getTracer("host")
        .startActiveSpan("host-render", async (span) => {
          await mod.onRequestError(
            new Error(`boom-under-always-off-${tracing}`),
            { method: "GET", path: "/" },
            REQUEST_CONTEXT,
          );

          span.end();
        });

      await mod.flush();

      expect(bodies.join("\n")).toContain(`boom-under-always-off-${tracing}`);
    },
  );

  test("errors recorded on the host span carry Interfere URL redaction", async () => {
    const host = hostOwnsOpenTelemetry();

    const mod = await load();

    await mod.register({ ...quiet, tracing: "application" });

    await trace
      .getTracer("host")
      .startActiveSpan("host-render", async (span) => {
        await mod.onRequestError(
          new Error("failed https://shop.test/pay?token=secret-token-1"),
          { method: "GET", path: "/checkout?token=secret-token-2" },
          REQUEST_CONTEXT,
        );

        span.end();
      });

    const hostRender = host.exporter
      .getFinishedSpans()
      .find((span) => span.name === "host-render");

    expect(hostRender?.events[0]?.attributes).toMatchObject({
      "exception.message": "failed https://shop.test/pay?token=REDACTED",
      "url.path": "/checkout?token=REDACTED",
    });

    expect(hostRender?.status.message).toBe(
      "failed https://shop.test/pay?token=REDACTED",
    );

    expect(JSON.stringify(hostRender?.events)).not.toContain("secret-token");
  });
});
