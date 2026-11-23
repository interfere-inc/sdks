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
import {
  InMemoryLogRecordExporter,
  LoggerProvider,
  SimpleLogRecordProcessor,
} from "@opentelemetry/sdk-logs";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
  TracerProvider,
} from "@opentelemetry/sdk-trace";
import { setImmediate } from "node:timers/promises";
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

let uncaught: ReturnType<typeof process.listeners>;

let rejected: ReturnType<typeof process.listeners>;

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

  uncaught = process.listeners("uncaughtException");

  rejected = process.listeners("unhandledRejection");

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

describe("register / flush", () => {
  test("no-ops and leaves flush inert when no workspace API key is set", async () => {
    vi.stubEnv("INTERFERE_API_KEY", "");

    const mod = await load();

    await mod.register();

    expect(process.listeners("uncaughtException")).toStrictEqual(uncaught);
    expect(process.listeners("unhandledRejection")).toStrictEqual(rejected);

    await expect(mod.flush()).resolves.toBeUndefined();

    expect(providerSpies.flushTraces).not.toHaveBeenCalled();
    expect(fetchAndCacheRemoteConfig).not.toHaveBeenCalled();
  });

  test("registers providers and installs handlers while fetching remote config server-side", async () => {
    await listenCollector(http.post("*", () => HttpResponse.json({})));

    vi.spyOn(console, "info");

    const mod = await load();

    const exporter = new InMemoryLogRecordExporter();

    await mod.register({
      _internalAdditionalLogRecordProcessors: [
        new SimpleLogRecordProcessor({ exporter }),
      ],
    });

    expect(providerSpies.register).toHaveBeenCalledTimes(1);
    expect(providerSpies.registerInstrumentations).not.toHaveBeenCalled();

    console.info("native console bridge");

    await setImmediate();

    await mod.flush();

    expect(exporter.getFinishedLogRecords()).toStrictEqual(
      expect.arrayContaining([
        expect.objectContaining({ body: "native console bridge" }),
      ]),
    );

    expect(process.listenerCount("uncaughtException")).toBe(
      uncaught.length + 1,
    );

    expect(process.listenerCount("unhandledRejection")).toBe(
      rejected.length + 1,
    );

    expect(fetchAndCacheRemoteConfig).toHaveBeenCalledTimes(1);

    const { uninstallGlobalHandlers } =
      await import("./internal/server/capture/global-handlers.js");

    uninstallGlobalHandlers();

    expect(process.listeners("uncaughtException")).toStrictEqual(uncaught);
    expect(process.listeners("unhandledRejection")).toStrictEqual(rejected);
  });

  test("is idempotent across repeated calls", async () => {
    const mod = await load();

    await mod.register();

    await mod.register();

    expect(process.listenerCount("uncaughtException")).toBe(
      uncaught.length + 1,
    );

    expect(process.listenerCount("unhandledRejection")).toBe(
      rejected.length + 1,
    );

    expect(fetchAndCacheRemoteConfig).toHaveBeenCalledTimes(1);
  });

  test.each(["managed", "existing-provider", "application"] as const)(
    "fetches remote config once without blocking concurrent or repeated registration in %s mode",
    async (mode) => {
      if (mode === "existing-provider") {
        hostOwnsOpenTelemetry();
      }

      const pending = Promise.withResolvers<undefined>();

      // oxlint-disable-next-line landmines/no-test-doubles -- A pending task proves registration never awaits remote configuration.
      vi.mocked(fetchAndCacheRemoteConfig).mockReturnValue(pending.promise);

      const mod = await load();

      const options = {
        tracing: mode === "application" ? "application" : "managed",
        consoleBridge: false,
        captureUncaughtException: false,
        captureUnhandledRejection: false,
      } as const;

      try {
        await Promise.all([mod.register(options), mod.register(options)]);

        await mod.register(options);

        expect(fetchAndCacheRemoteConfig).toHaveBeenCalledTimes(1);
      } finally {
        pending.resolve(undefined);
      }
    },
  );
});
