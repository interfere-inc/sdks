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
  afterAll,
  afterEach,
  beforeAll,
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

describe("register / flush", () => {
  test("installs nothing and sends nothing during next build", async () => {
    vi.stubEnv("NEXT_PHASE", "phase-production-build");

    const requests: string[] = [];

    await listenCollector(
      http.all("*", ({ request }) => {
        requests.push(request.url);

        return HttpResponse.json({});
      }),
    );

    const mod = await load();

    await mod.register();

    await mod.flush();

    expect({
      rejected: process.listeners("unhandledRejection"),
      remoteConfig: vi.mocked(fetchAndCacheRemoteConfig).mock.calls.length,
      requests,
      tracerRegistered: providerSpies.register.mock.calls.length,
      uncaught: process.listeners("uncaughtException"),
    }).toStrictEqual({
      rejected,
      remoteConfig: 0,
      requests: [],
      tracerRegistered: 0,
      uncaught,
    });
  });

  test("flushes tracer and logger providers after registration", async () => {
    const mod = await load();

    await mod.register();

    await mod.flush();

    expect(providerSpies.flushTraces).toHaveBeenCalledTimes(1);
    expect(providerSpies.flushLogs).toHaveBeenCalledTimes(1);
  });

  test("skips the console bridge when disabled", async () => {
    vi.spyOn(console, "info");

    const mod = await load();

    const exporter = new InMemoryLogRecordExporter();
    const originalInfo = console.info;

    await mod.register({
      consoleBridge: false,
      _internalAdditionalLogRecordProcessors: [
        new SimpleLogRecordProcessor({ exporter }),
      ],
    });

    expect(console.info).toBe(originalInfo);

    console.info("not bridged");

    await setImmediate();

    await mod.flush();

    expect(exporter.getFinishedLogRecords()).toStrictEqual([]);
  });

  test("forwards a disabled capture flag to the global handlers", async () => {
    const mod = await load();

    await mod.register({ captureUncaughtException: false });

    expect(process.listeners("uncaughtException")).toStrictEqual(uncaught);

    expect(process.listenerCount("unhandledRejection")).toBe(
      rejected.length + 1,
    );
  });
});

beforeAll(() => {
  server.listen({ onUnhandledFrame: "error" });
});

afterAll(() => {
  server.close();
});
