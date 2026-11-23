import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
/* oxlint-disable eslint/no-console */
import { setupServer } from "msw/node";
import { HttpResponse, http } from "msw";
import { releaseSourceIdEnvKeys } from "@interfere/types/integrations";
import { SERVICE_NAME_GLOBAL } from "@interfere/types/sdk/service-name";
import { RELEASE_SLUG_GLOBAL } from "@interfere/types/sdk/injected-globals";

import { context, logs, propagation, trace } from "@opentelemetry/api";
// oxlint-disable-next-line import/no-namespace -- This API consumes the complete module namespace.
import * as instrumentation from "@opentelemetry/instrumentation";
import { LoggerProvider } from "@opentelemetry/sdk-logs";
import { TracerProvider } from "@opentelemetry/sdk-trace";
import { fetchAndCacheRemoteConfig } from "./internal/server/remote-config.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

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

describe("register / flush", () => {
  test.each(["throw", "reject"] as const)(
    "contains a remote config %s without retrying registration",
    async (failureMode) => {
      const failure = new Error("remote config unavailable");

      if (failureMode === "throw") {
        // oxlint-disable-next-line landmines/no-test-doubles -- Exercise synchronous failure containment at the startup boundary.
        vi.mocked(fetchAndCacheRemoteConfig).mockImplementation(() => {
          throw failure;
        });
      } else {
        // oxlint-disable-next-line landmines/no-test-doubles -- Exercise asynchronous failure containment at the startup boundary.
        vi.mocked(fetchAndCacheRemoteConfig).mockRejectedValue(failure);
      }

      const mod = await load();

      await expect(
        mod.register({ consoleBridge: false }),
      ).resolves.toBeUndefined();

      await mod.register();

      expect(fetchAndCacheRemoteConfig).toHaveBeenCalledTimes(1);

      expect(console.warn).toHaveBeenCalledWith(
        expect.stringContaining("[interfere] remote config failed"),
        failure,
      );
    },
  );

  test("resolves and warns once when a registration step throws", async () => {
    const failure = new Error("injected registration failure");

    const mod = await load();

    await expect(
      mod.register(
        Object.defineProperty({}, "serviceName", {
          get() {
            throw failure;
          },
        }),
      ),
    ).resolves.toBeUndefined();

    await expect(mod.flush()).resolves.toBeUndefined();

    expect(console.warn).toHaveBeenCalledWith(
      expect.stringContaining("[interfere] register() failed"),
      failure,
    );
  });

  test("flush resolves when a span processor flush rejects", async () => {
    const mod = await load();

    await mod.register({
      consoleBridge: false,
      _internalAdditionalSpanProcessors: [
        {
          async forceFlush() {
            await Promise.reject(new Error("export failed"));
          },
          onEnd: () => undefined,
          onStart: () => undefined,
          async shutdown() {
            await Promise.resolve();
          },
        },
      ],
    });

    await expect(mod.flush()).resolves.toBeUndefined();
  });
});
