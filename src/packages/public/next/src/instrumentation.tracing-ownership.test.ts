import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
/* oxlint-disable eslint/no-console */
import { setupServer } from "msw/node";
import { HttpResponse, http, passthrough } from "msw";
import type { HttpHandler } from "msw";
import { releaseSourceIdEnvKeys } from "@interfere/types/integrations";
import { SERVICE_NAME_GLOBAL } from "@interfere/types/sdk/service-name";
import { RELEASE_SLUG_GLOBAL } from "@interfere/types/sdk/injected-globals";

import { context, logs, propagation, trace } from "@opentelemetry/api";
// oxlint-disable-next-line import/no-namespace -- This API consumes the complete module namespace.
import * as instrumentation from "@opentelemetry/instrumentation";
import {
  InMemoryLogRecordExporter,
  LoggerProvider,
  SimpleLogRecordProcessor,
} from "@interfere/server-core/otel/sdk/logs";
import { TracerProvider } from "@interfere/server-core/otel/sdk/trace/provider";
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

  test.each([
    {
      consoleBridge: undefined,
      expected: ["debug", "log", "info", "warn", "error"],
    },
    {
      consoleBridge: true,
      expected: ["debug", "log", "info", "warn", "error"],
    },
    { consoleBridge: false, expected: [] },
    { consoleBridge: {}, expected: ["debug", "log", "info", "warn", "error"] },
    {
      consoleBridge: { level: "debug" },
      expected: ["debug", "log", "info", "warn", "error"],
    },
    {
      consoleBridge: { level: "info" },
      expected: ["log", "info", "warn", "error"],
    },
    { consoleBridge: { level: "warn" }, expected: ["warn", "error"] },
    { consoleBridge: { level: "error" }, expected: ["error"] },
  ] as const)(
    "bridges console into the host logger with $consoleBridge",
    async ({ consoleBridge, expected }) => {
      expect.assertions(8);

      const records = new InMemoryLogRecordExporter();

      const hostLogger = new LoggerProvider({
        processors: [new SimpleLogRecordProcessor({ exporter: records })],
      });

      logs.setGlobalLoggerProvider(hostLogger);

      const mod = await load();

      const methods = ["debug", "log", "info", "warn", "error"] as const;
      const originals = { ...console };

      await mod.register(
        omitUndefined({
          captureUncaughtException: false,
          captureUnhandledRejection: false,
          consoleBridge,
          tracing: "application" as const,
        }),
      );

      for (const method of methods) {
        expect(console[method] !== originals[method]).toBe(
          expected.some((bridged) => bridged === method),
        );

        console[method](`threshold-${method}`);
      }

      await setImmediate();

      await mod.flush();

      expect(logs.getLoggerProvider()).toBe(hostLogger);
      expect(providerSpies.flushLogs).not.toHaveBeenCalled();

      expect(
        records.getFinishedLogRecords().map((record) => record.body),
      ).toStrictEqual(expected.map((method) => `threshold-${method}`));
    },
  );

  test("application mode bridges console into a private, unregistered logger provider when the host registered none", async () => {
    const bodies = await recordingCollector();

    const records = new InMemoryLogRecordExporter();

    const mod = await load();

    await mod.register({
      ...quiet,
      _internalAdditionalLogRecordProcessors: [
        new SimpleLogRecordProcessor({ exporter: records }),
      ],
      consoleBridge: true,
      tracing: "application",
    });

    console.info("bridged privately");

    await setImmediate();

    await mod.flush();

    expect(logs.getLoggerProvider()).not.toBeInstanceOf(LoggerProvider);
    expect(providerSpies.flushLogs).toHaveBeenCalledTimes(1);
    expect(bodies.join("\n")).toContain("bridged privately");

    expect(records.getFinishedLogRecords()).toStrictEqual(
      expect.arrayContaining([
        expect.objectContaining({ body: "bridged privately" }),
      ]),
    );
  });
});
