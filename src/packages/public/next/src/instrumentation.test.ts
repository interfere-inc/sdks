import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
/* oxlint-disable eslint/no-console */
import { setupServer } from "msw/node";
import { HttpResponse, http, passthrough } from "msw";
import type { HttpHandler } from "msw";
import { releaseSourceIdEnvKeys } from "@interfere/types/integrations";
import { SERVICE_NAME_GLOBAL } from "@interfere/types/sdk/service-name";
import { deriveReleaseSlug } from "@interfere/types/releases/slug";
import { RELEASE_SLUG_GLOBAL } from "@interfere/types/sdk/injected-globals";

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
import type { ServerInstrumentationOptions } from "./instrumentation.js";
import { fetchAndCacheRemoteConfig } from "./internal/server/remote-config.js";
import { once } from "node:events";
import {
  afterEach,
  assert,
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

function warnings(fragment: string) {
  return vi
    .mocked(console.warn)
    .mock.calls.filter(([message]) => String(message).includes(fragment));
}

describe("managed resource", () => {
  const quiet = {
    captureUncaughtException: false,
    captureUnhandledRejection: false,
    consoleBridge: false,
  };

  async function registeredResource(
    options: ServerInstrumentationOptions = {},
  ) {
    await listenCollector(http.post("*", () => HttpResponse.json({})));

    const exporter = new InMemorySpanExporter();

    const mod = await load();

    await mod.register({
      ...quiet,
      ...options,
      _internalAdditionalSpanProcessors: [
        new SimpleSpanProcessor({ exporter }),
      ],
    });

    trace.getTracer("resource").startSpan("probe").end();

    await mod.flush();

    const [span] = exporter.getFinishedSpans();

    assert(span);

    return span.resource.attributes;
  }

  test("describes spans with the SDK identity and a release slug", async () => {
    const attributes = await registeredResource();

    expect(attributes).toMatchObject({
      "interfere.sdk.name": "@interfere/next",
      "release.slug": deriveReleaseSlug(commitSha),
      "service.name": `unknown_service:${process.argv0}`,
    });

    expect(attributes).not.toHaveProperty("service.namespace");
  });

  test.each([
    {
      scenario: "OTEL_SERVICE_NAME over the baked app name",
      environment: "environment-service",
      serviceName: undefined,
      expected: "environment-service",
    },
    {
      scenario: "the serviceName option over OTEL_SERVICE_NAME",
      environment: "environment-service",
      serviceName: "configured-service",
      expected: "configured-service",
    },
    {
      scenario: "the baked app name without runtime configuration",
      environment: undefined,
      serviceName: undefined,
      expected: "platform",
    },
  ])(
    "names the service from $scenario",
    async ({ environment, serviceName, expected }) => {
      setInjectedConfiguration(SERVICE_NAME_GLOBAL, "platform");

      vi.stubEnv("OTEL_SERVICE_NAME", environment);

      const attributes = await registeredResource(
        serviceName === undefined ? {} : { serviceName },
      );

      expect(attributes["service.name"]).toBe(expected);
    },
  );

  test("honors serviceName and serviceNamespace", async () => {
    const attributes = await registeredResource({
      serviceName: "@my-org/api",
      serviceNamespace: "customer",
    });

    expect(attributes).toMatchObject({
      "service.name": "@my-org/api",
      "service.namespace": "customer",
    });
  });

  test("warns and omits release.slug when no commit SHA resolves", async () => {
    vi.stubEnv("INTERFERE_SOURCE_ID", undefined);

    const attributes = await registeredResource();

    expect(attributes).not.toHaveProperty("release.slug");
    expect(warnings("release.slug")).toHaveLength(1);
  });

  test("keeps the managed ID generator private", async () => {
    await expect(load()).resolves.not.toHaveProperty(
      "PrerenderSafeIdGenerator",
    );
  });
});
