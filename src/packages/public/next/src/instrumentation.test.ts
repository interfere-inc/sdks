import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
/* oxlint-disable eslint/no-console */
import { setupServer } from "msw/node";
import { HttpResponse, http, passthrough } from "msw";
import type { HttpHandler } from "msw";
import { releaseSourceIdEnvKeys } from "@interfere/types/integrations";
import { EXCEPTION_DELIVERY_ATTRIBUTE } from "@interfere/types/sdk/exception-delivery";
import { SERVICE_NAME_GLOBAL } from "@interfere/types/sdk/service-name";
import { deriveReleaseSlug } from "@interfere/types/releases/slug";
import { RELEASE_SLUG_GLOBAL } from "@interfere/types/sdk/injected-globals";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import { W3CTraceContextPropagator } from "@opentelemetry/core";

import {
  DiagLogLevel,
  context,
  diag,
  propagation,
  trace,
} from "@opentelemetry/api";
// oxlint-disable-next-line import/no-namespace -- This API consumes the complete module namespace.
import * as instrumentation from "@opentelemetry/instrumentation";
import { logs } from "@opentelemetry/api-logs";
import {
  InMemoryLogRecordExporter,
  LoggerProvider,
  SimpleLogRecordProcessor,
} from "@opentelemetry/sdk-logs";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import { NodeTracerProvider } from "@opentelemetry/sdk-trace-node";
import { setImmediate } from "node:timers/promises";
import { gunzipSync } from "node:zlib";
import { mswServer } from "@interfere/test-config/msw-server";
import type { ServerInstrumentationOptions } from "./instrumentation.js";
import { fetchAndCacheRemoteConfig } from "./internal/server/remote-config.js";
import type { IncomingHttpHeaders } from "node:http";
import { once } from "node:events";
import {
  afterAll,
  afterEach,
  assert,
  beforeAll,
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

await import("./instrumentation.js");

await import("./internal/server/capture/global-handlers.js");

const providerSpies = {
  register: vi.spyOn(NodeTracerProvider.prototype, "register"),
  flushTraces: vi.spyOn(NodeTracerProvider.prototype, "forceFlush"),
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

  // oxlint-disable-next-line interfere-testing/no-test-doubles -- Reset the remote-config startup contract fixture between cases.
  vi.mocked(fetchAndCacheRemoteConfig).mockReset().mockResolvedValue(undefined);

  providerSpies.register = vi.spyOn(NodeTracerProvider.prototype, "register");

  providerSpies.flushTraces = vi.spyOn(
    NodeTracerProvider.prototype,
    "forceFlush",
  );

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
    providerSpies.register.mock.contexts.map(async (provider) => {
      if (!(provider instanceof NodeTracerProvider)) {
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
      _internalAdditionalSpanProcessors: [new SimpleSpanProcessor(exporter)],
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

  test("exports an actual gzip-compressed span payload", async () => {
    const received: {
      headers: IncomingHttpHeaders;
      body: Buffer;
      route: string;
    }[] = [];

    await listenCollector(
      http.post("*", async ({ request }) => {
        const url = new URL(request.url);

        received.push({
          headers: Object.fromEntries(request.headers),
          body: Buffer.from(await request.arrayBuffer()),
          route: `${request.method} ${url.pathname}${url.search}`,
        });

        return HttpResponse.json({});
      }),
    );

    const mod = await load();

    await mod.register({ ...quiet, compression: "gzip" });

    trace.getTracer("gzip-contract").startSpan("compressed-span").end();

    await mod.flush();

    expect(received).toHaveLength(1);
    expect(received[0]?.headers["content-encoding"]).toBe("gzip");

    expect(received[0]?.route).toBe("POST /drain/v1/traces");

    expect(received[0]?.headers.authorization).toBe("Bearer interfere_pk_test");

    assert(received[0]);

    expect(JSON.parse(gunzipSync(received[0].body).toString())).toMatchObject({
      resourceSpans: [
        { scopeSpans: [{ spans: [{ name: "compressed-span" }] }] },
      ],
    });
  });
});

describe("URL redaction", () => {
  const SECRET = "server-secret";

  async function recordRequest(options: ServerInstrumentationOptions) {
    await listenCollector(http.post("*", () => HttpResponse.json({})));

    const exporter = new InMemorySpanExporter();

    const mod = await load();

    await mod.register({
      captureUncaughtException: false,
      captureUnhandledRejection: false,
      consoleBridge: false,
      ...options,
      _internalAdditionalSpanProcessors: [new SimpleSpanProcessor(exporter)],
    });

    const tracer = trace.getTracer("next-server");

    await tracer.startActiveSpan(
      `GET /orders?token=${SECRET}`,
      {
        attributes: {
          "http.referrer": `https://shop.test/cart?coupon=${SECRET}`,
          "http.target": `/orders?token=${SECRET}`,
          "url.fragment": SECRET,
          "url.full": `https://shop.test/orders?token=${SECRET}#${SECRET}`,
          "url.query": `token=${SECRET}`,
        },
      },
      async (request) => {
        tracer.startSpan("render").end();

        await mod.onRequestError(
          new Error(
            `render failed for https://shop.test/orders?token=${SECRET}#${SECRET}`,
          ),
          {
            headers: { referer: `https://shop.test/cart?coupon=${SECRET}` },
            method: "GET",
            path: `/orders?token=${SECRET}`,
          },
          {
            routePath: "/orders",
            routeType: "render",
            routerKind: "App Router",
          },
        );

        request.end();
      },
    );

    await mod.flush();

    return exporter.getFinishedSpans();
  }

  test("redacts URLs for every exporter sharing the provider without changing trace relationships", async () => {
    const spans = await recordRequest({ privacy: { urls: true } });

    const request = spans.find((span) => span.name.startsWith("GET"));
    const render = spans.find((span) => span.name === "render");

    expect(
      JSON.stringify(
        spans.map(({ attributes, events, name, status }) => ({
          attributes,
          events,
          name,
          status,
        })),
      ),
    ).not.toContain(SECRET);

    expect(request?.name).toBe("GET /orders?token=REDACTED");

    expect(request?.attributes).toStrictEqual({
      "http.referrer": "https://shop.test/cart?coupon=REDACTED",
      "http.target": "/orders?token=REDACTED",
      "url.full": "https://shop.test/orders?token=REDACTED",
      "url.query": "token=REDACTED",
    });

    expect(request?.status.message).toBe(
      "render failed for https://shop.test/orders?token=REDACTED",
    );

    expect(request?.events[0]?.attributes).toMatchObject({
      "exception.message":
        "render failed for https://shop.test/orders?token=REDACTED",
      "http.request.header.referer": "https://shop.test/cart?coupon=REDACTED",
      "http.route": "/orders",
      "url.path": "/orders?token=REDACTED",
    });

    expect(
      String(request?.events[0]?.attributes?.["exception.stacktrace"]),
    ).toMatch(
      /^Error: render failed for https:\/\/shop\.test\/orders\?token=REDACTED\n/u,
    );

    expect(render?.parentSpanContext?.spanId).toBe(
      request?.spanContext().spanId,
    );
  });

  test("redacts sensitive parameters by default and keeps the rest", async () => {
    const spans = await recordRequest({});

    const request = spans.find((span) => span.name.startsWith("GET"));

    expect(request?.name).toBe("GET /orders?token=REDACTED");

    expect(request?.attributes["url.full"]).toBe(
      `https://shop.test/orders?token=REDACTED#${SECRET}`,
    );

    expect(request?.attributes["http.referrer"]).toBe(
      `https://shop.test/cart?coupon=${SECRET}`,
    );
  });
});

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

      // oxlint-disable-next-line interfere-testing/no-test-doubles -- A pending task proves registration never awaits remote configuration.
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

  test.each(["throw", "reject"] as const)(
    "contains a remote config %s without retrying registration",
    async (failureMode) => {
      const failure = new Error("remote config unavailable");

      if (failureMode === "throw") {
        // oxlint-disable-next-line interfere-testing/no-test-doubles -- Exercise synchronous failure containment at the startup boundary.
        vi.mocked(fetchAndCacheRemoteConfig).mockImplementation(() => {
          throw failure;
        });
      } else {
        // oxlint-disable-next-line interfere-testing/no-test-doubles -- Exercise asynchronous failure containment at the startup boundary.
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

  test("does not exit after an uncaught exception outside production", async () => {
    const exit = vi.spyOn(process, "exit");
    let exports = 0;

    await listenCollector(
      http.post("*/drain/v1/traces", () => {
        exports += 1;

        return HttpResponse.json({});
      }),
    );

    const mod = await load();

    await mod.register();

    const handler = process
      .listeners("uncaughtException")
      .find((listener) => !uncaught.includes(listener));

    expect(handler).toBeDefined();

    if (!handler) {
      throw new Error("Expected an uncaught exception handler");
    }

    handler(new Error("test exception"), "uncaughtException");

    await setImmediate();

    expect(providerSpies.flushTraces).toHaveBeenCalledTimes(1);

    const [traceFlush] = providerSpies.flushTraces.mock.results;
    const [logFlush] = providerSpies.flushLogs.mock.results;

    assert(traceFlush?.type === "return");

    assert(logFlush?.type === "return");

    await traceFlush.value;

    await logFlush.value;

    expect(exports).toBe(1);
    expect(exit).not.toHaveBeenCalled();
  });

  test("keeps a production server alive when the host has its own uncaughtException listener", async () => {
    vi.stubEnv("NODE_ENV", "production");

    // oxlint-disable-next-line interfere-testing/no-test-doubles -- A real exit would terminate the test worker; the assertion is that it is never requested.
    const exit = vi
      .spyOn(process, "exit")
      .mockImplementation((() => undefined) as typeof process.exit);

    let exports = 0;

    await listenCollector(
      http.post("*/drain/v1/traces", () => {
        exports += 1;

        return HttpResponse.json({});
      }),
    );

    const mod = await load();

    await mod.register();

    const handler = process
      .listeners("uncaughtException")
      .find((listener) => !uncaught.includes(listener));

    assert(handler, "Expected an uncaught exception handler");

    const host = vi.fn<(error: Error) => void>();

    process.on("uncaughtException", host);

    onTestFinished(() => {
      process.off("uncaughtException", host);
    });

    handler(new Error("production host-owned exception"), "uncaughtException");

    await expect.poll(() => exports).toBe(1);

    await setImmediate();

    await setImmediate();

    expect(exit).not.toHaveBeenCalled();
  });

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
  server.listen({ onUnhandledRequest: "error" });
});

afterAll(() => {
  server.close();
});

test("direct Node traces and logs survive Retry-After 30 beyond the old deadline", async () => {
  const attempts = { traces: 0, logs: 0 };
  const scheduled = Promise.withResolvers<undefined>();
  let retries = 0;
  const ignore = () => undefined;

  diag.setLogger(
    {
      error: ignore,
      warn: ignore,
      debug: ignore,
      info(message) {
        if (message.startsWith("Export retry time")) {
          scheduled.reject(new Error(message));
        }
      },
      verbose(message) {
        if (message === "Scheduling export retry in 30000ms") {
          retries += 1;

          if (retries === 2) {
            scheduled.resolve(undefined);
          }
        }
      },
    },
    DiagLogLevel.VERBOSE,
  );

  vi.useFakeTimers({
    shouldAdvanceTime: true,
    toFake: ["setTimeout", "clearTimeout", "Date"],
  });

  onTestFinished(() => {
    vi.useRealTimers();

    diag.disable();
  });

  await listenCollector(
    http.post("*/drain/v1/:signal", ({ params }) => {
      const signal = params["signal"] === "traces" ? "traces" : "logs";

      attempts[signal] += 1;

      return attempts[signal] === 1
        ? new HttpResponse(null, {
            status: 503,
            headers: { "Retry-After": "30" },
          })
        : HttpResponse.json({});
    }),
  );

  const mod = await load();

  await mod.register({
    consoleBridge: false,
    captureUncaughtException: false,
    captureUnhandledRejection: false,
  });

  trace.getTracer("retry").startSpan("survives-overload").end();

  logs.getLogger("retry").emit({ body: "survives-overload" });

  const flushed = mod.flush();

  await scheduled.promise;

  await vi.advanceTimersByTimeAsync(30_000);

  await flushed;

  expect(attempts).toStrictEqual({ traces: 2, logs: 2 });
});

function warnings(fragment: string) {
  return vi
    .mocked(console.warn)
    .mock.calls.filter(([message]) => String(message).includes(fragment));
}

function hostOwnsOpenTelemetry() {
  const exporter = new InMemorySpanExporter();

  const provider = new NodeTracerProvider({
    spanProcessors: [new SimpleSpanProcessor(exporter)],
  });

  provider.register({
    contextManager: new AsyncLocalStorageContextManager().enable(),
    propagator: new W3CTraceContextPropagator(),
  });

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
