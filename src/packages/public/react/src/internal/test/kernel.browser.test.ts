import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
/* oxlint-disable typescript/no-unsafe-assignment, unicorn/no-useless-promise-resolve-reject */
import { PLUGIN_MANIFEST } from "@interfere/types/sdk/plugins/manifest";

import {
  SpanStatusCode,
  context,
  propagation,
  trace,
} from "@opentelemetry/api";
import { StackContextManager } from "@opentelemetry/sdk-trace-web";
import type {
  LogRecordProcessor,
  ReadableLogRecord,
} from "@opentelemetry/sdk-logs";
import {
  BasicTracerProvider,
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";

import { DISABLE_ALL_PLUGINS } from "../../../test/fixtures/disable-all-plugins.js";
import { buildKernel, ingestHandlers } from "../../../test/fixtures/kernel.js";
import { isBotUserAgent } from "../../util/bot.js";
import { getLogLevel, setLogLevel } from "../../util/log.js";
import { createKernel, readReleaseSlugFromGlobals } from "../kernel.js";
import type { KernelOptions } from "../kernel.js";
import { buildOtelProvider } from "../otel/provider.js";
import { HttpResponse, http } from "msw";
import {
  afterEach,
  beforeAll,
  beforeEach,
  describe,
  expect,
  onTestFinished,
  test,
  vi,
} from "vitest";

import { addBaselineHandlers, worker } from "../../../test/msw.js";

beforeAll(() => {
  addBaselineHandlers(
    http.get("https://api.interfere.com/v3/config", () =>
      HttpResponse.json({ plugins: {} }),
    ),
    ...ingestHandlers("https://api.interfere.com"),
    ...ingestHandlers("https://in.example.com"),
  );
});

const WAIT_OPTS = { timeout: 5000 };

const ownedKernels = new Set<Awaited<ReturnType<typeof createKernel>>>();

const providers = new Set<BasicTracerProvider>();

let priorUrl: string;

let priorHistory: unknown;

let priorCookie: Awaited<ReturnType<CookieStore["get"]>>;

let priorLogLevel: ReturnType<typeof getLogLevel>;

function withInMemoryTracer<T>(fn: (exporter: InMemorySpanExporter) => T) {
  const exporter = new InMemorySpanExporter();

  const provider = new BasicTracerProvider({
    spanProcessors: [new SimpleSpanProcessor(exporter)],
  });

  providers.add(provider);

  trace.disable();

  context.disable();

  trace.setGlobalTracerProvider(provider);

  context.setGlobalContextManager(new StackContextManager().enable());

  try {
    return fn(exporter);
  } finally {
    context.disable();

    trace.disable();
  }
}

function serveConfig(response: Response) {
  const requests: Request[] = [];

  worker.use(
    http.get("https://api.interfere.com/v3/config", ({ request }) => {
      requests.push(request);

      return response;
    }),
  );

  return requests;
}

beforeEach(async () => {
  priorLogLevel = getLogLevel();

  priorUrl = globalThis.location.href;

  priorHistory = globalThis.history.state;

  priorCookie = await cookieStore.get("interfere_did");
});

afterEach(async () => {
  try {
    const results = await Promise.allSettled([
      ...Array.from(ownedKernels, async (kernel) => kernel.dispose()),
      ...Array.from(providers, async (provider) => provider.shutdown()),
    ]);

    const failures = results.filter((result) => result.status === "rejected");

    if (failures.length > 0) {
      throw new AggregateError(
        failures.map((result) => {
          const decodedValue: unknown = result.reason;

          return decodedValue;
        }),
        "Kernel cleanup failed",
      );
    }
  } finally {
    ownedKernels.clear();

    providers.clear();

    context.disable();

    trace.disable();

    propagation.disable();

    setLogLevel(priorLogLevel);

    globalThis.history.replaceState(priorHistory, "", priorUrl);

    await cookieStore.delete("interfere_did");

    if (priorCookie?.name && priorCookie.value !== undefined) {
      await cookieStore.set({
        ...priorCookie,
        name: priorCookie.name,
        value: priorCookie.value,
      });
    }

    vi.unstubAllGlobals();
  }
});

function recordInSpan(
  exporter: InMemorySpanExporter,
  errors: Error[],
  handled = true,
) {
  const { kernel, tracker } = buildKernel();

  ownedKernels.add(kernel);

  tracker.start();

  const span = trace.getTracer("test").startSpan("op");

  context.with(trace.setSpan(context.active(), span), () => {
    for (const error of errors) {
      kernel.recordException(error, {
        mechanism: {
          handled,
          type: handled
            ? "manual.api.capture"
            : "auto.browser.global_handlers.onerror",
        },
      });
    }
  });

  span.end();

  const [finished] = exporter.getFinishedSpans();

  return {
    events: finished?.events.filter((e) => e.name === "exception"),
    status: finished?.status,
  };
}

describe("Kernel", () => {
  test("exposes consent/identity/device/session accessors that delegate to collaborators", () => {
    const { kernel, tracker } = buildKernel({
      pluginOverrides: DISABLE_ALL_PLUGINS,
    });

    ownedKernels.add(kernel);

    tracker.start();

    expect(kernel.session.getId()).toMatch(/\S/u);
    expect(kernel.session.getWindowId()).toMatch(/\S/u);
    expect(kernel.device.getDeviceId()).toBeNull();

    kernel.startDeviceInfoCapture();

    expect(kernel.device.getDeviceId()).toMatch(/\S/u);
    expect(kernel.identity.get()).toBeNull();

    expect(kernel.consent.get()).toStrictEqual({
      analytics: true,
      replay: true,
    });
  });

  test("device capture cleanup purges stored identity only on consent revocation", () => {
    const { kernel, tracker } = buildKernel({
      pluginOverrides: DISABLE_ALL_PLUGINS,
    });

    ownedKernels.add(kernel);

    tracker.start();

    const stop = kernel.startDeviceInfoCapture();
    const deviceId = kernel.device.getDeviceId();

    expect(deviceId).toMatch(/\S/u);
    expect(document.cookie).toContain("interfere_did=");
    expect(localStorage.getItem("interfere:device_id")).toBe(deviceId);

    stop("disabled");

    expect(kernel.device.getDeviceId()).toBe(deviceId);
    expect(localStorage.getItem("interfere:device_id")).toBe(deviceId);

    kernel.startDeviceInfoCapture()("consent-revoked");

    expect(kernel.device.getDeviceId()).toBeNull();
    expect(localStorage.getItem("interfere:device_id")).toBeNull();
    expect(document.cookie).not.toContain("interfere_did=");
  });

  test("clears identity and rotates the session through identity.clear", async () => {
    const { kernel, tracker } = buildKernel();

    ownedKernels.add(kernel);

    tracker.start();

    const initialSessionId = kernel.session.getId();

    await kernel.identity.set({
      identifier: "kernel-clear",
      source: { name: "Test", type: "custom" },
    });

    kernel.identity.clear();

    expect(kernel.identity.get()).toBeNull();
    expect(kernel.session.getId()).toMatch(/\S/u);
    expect(kernel.session.getId()).not.toBe(initialSessionId);
  });

  test("dispose cleans up the tracker", async () => {
    const { kernel, tracker } = buildKernel();

    ownedKernels.add(kernel);

    tracker.start();

    expect(kernel.session.getId()).toMatch(/\S/u);

    await kernel.dispose();

    expect(kernel.session.getId()).toBeNull();
  });

  describe("recordException", () => {
    test("emits an exception event on the active span with mechanism + parsed exceptions", () => {
      expect.assertions(1);

      withInMemoryTracer((exporter) => {
        const exceptionEvent = recordInSpan(exporter, [
          new TypeError("undefined is not a function"),
        ]).events?.[0];

        expect(exceptionEvent?.attributes).toMatchObject({
          "exception.message": "undefined is not a function",
          "exception.type": "TypeError",
          "interfere.exception.handled": true,
          "interfere.exception.mechanism": "manual.api.capture",
        });
      });
    });

    test("forwards the raw _debugIds registry on exception event attributes", () => {
      expect.assertions(1);

      withInMemoryTracer((exporter) => {
        setDebugIdRegistry({
          "Error\n    at fn (https://app.com/chunks/app.js:1:1)":
            "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        });

        const exceptionEvent = recordInSpan(exporter, [new Error("boom")])
          .events?.[0];

        expect(
          JSON.parse(
            String(
              exceptionEvent?.attributes?.["interfere.exception.debug_ids"],
            ),
          ),
        ).toStrictEqual({
          "Error\n    at fn (https://app.com/chunks/app.js:1:1)":
            "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        });
      });
    });

    test.each([{}, undefined, null, false, 42, "not-a-registry"])(
      "does not emit debug ID attributes for empty or non-object registry %j",
      (registry) => {
        expect.assertions(1);

        withInMemoryTracer((exporter) => {
          setDebugIdRegistry(registry);

          const exceptionEvent = recordInSpan(exporter, [new Error("boom")])
            .events?.[0];

          expect(
            exceptionEvent?.attributes?.["interfere.exception.debug_ids"],
          ).toBeUndefined();
        });
      },
    );

    test("walks Error.cause into the chain attribute", () => {
      expect.assertions(2);

      withInMemoryTracer((exporter) => {
        const root = new Error("root");

        root.name = "RootError";

        const mid = new Error("mid", { cause: root });

        mid.name = "MidError";

        const tip = new Error("tip", { cause: mid });

        tip.name = "TipError";

        const exceptionEvent = recordInSpan(exporter, [tip]).events?.[0];

        const chainAttr =
          exceptionEvent?.attributes?.["interfere.exception.chain"];

        expect(chainAttr).toBeTypeOf("string");

        const chain: unknown = JSON.parse(String(chainAttr));

        expect(chain).toStrictEqual([
          expect.objectContaining({ type: "TipError" }),
          expect.objectContaining({ type: "MidError" }),
          expect.objectContaining({ type: "RootError" }),
        ]);
      });
    });

    test("drops browser-extension noise without emitting", () => {
      expect.assertions(1);

      withInMemoryTracer((exporter) => {
        const err = new Error("Extension blew up");

        err.stack = [
          "Error: Extension blew up",
          "    at handler (chrome-extension://abc/content.js:10:5)",
        ].join("\n");

        expect(recordInSpan(exporter, [err]).events).toHaveLength(0);
      });
    });

    test("marks the active span ERROR only for unhandled captures", () => {
      expect.assertions(2);

      withInMemoryTracer((exporter) => {
        expect(
          recordInSpan(exporter, [new Error("caught")], true).status,
        ).toMatchObject({ code: SpanStatusCode.UNSET });
      });

      withInMemoryTracer((exporter) => {
        expect(
          recordInSpan(exporter, [new Error("uncaught")], false).status,
        ).toMatchObject({ code: SpanStatusCode.ERROR, message: "uncaught" });
      });
    });

    test("dedupes repeated calls for the same Error instance", () => {
      expect.assertions(1);

      withInMemoryTracer((exporter) => {
        const err = new Error("boom");

        expect(recordInSpan(exporter, [err, err]).events).toHaveLength(1);
      });
    });

    test("synthesises a private-provider span when no active span exists", async () => {
      const privateExporter = new InMemorySpanExporter();
      const globalExporter = new InMemorySpanExporter();

      const globalProvider = new BasicTracerProvider({
        spanProcessors: [new SimpleSpanProcessor(globalExporter)],
      });

      const { kernel, tracker } = buildKernel();

      ownedKernels.add(kernel);

      trace.disable();

      context.disable();

      providers.add(globalProvider);

      trace.setGlobalTracerProvider(globalProvider);

      context.setGlobalContextManager(new StackContextManager().enable());

      try {
        tracker.start();

        kernel.attachOtel(
          buildOtelProvider({
            additionalSpanProcessors: [
              new SimpleSpanProcessor(privateExporter),
            ],

            getDeviceId: () => tracker.getDeviceId(),
            getSessionId: () => tracker.sessionId(),
            clientOptions: {
              baseUrl: "https://in.example.com",
              publicKeyHeader: { apiKey: "test-key" },
            },
            sdkStack: [],
          }),
          vi.fn(),
        );

        const boom = new Error("boom");

        boom.stack =
          "Error: boom\n    at Array.map (<anonymous>)\n    at submitOrder (https://shop.example/assets/checkout-a1b2.js:1:4821)";

        kernel.recordException(boom, {
          mechanism: { handled: false, type: "browser.onerror" },
        });

        const stackless = new TypeError("stackless");

        delete stackless.stack;

        kernel.recordException(stackless, {
          mechanism: { handled: true, type: "browser.console.error" },
        });

        kernel.recordException(
          { kind: "non-error", serialized: 42, type: "number", value: "42" },
          {
            mechanism: { handled: false, type: "browser.onunhandledrejection" },
          },
        );

        const finished = privateExporter.getFinishedSpans();

        expect(finished).toHaveLength(3);
        expect(globalExporter.getFinishedSpans()).toHaveLength(0);

        expect(finished.map((span) => span.name)).toStrictEqual([
          "submitOrder (checkout-a1b2.js:1)",
          "TypeError",
          "number",
        ]);

        expect(finished.map((span) => span.status.code)).toStrictEqual([
          SpanStatusCode.ERROR,
          SpanStatusCode.UNSET,
          SpanStatusCode.ERROR,
        ]);

        expect(finished[0]?.attributes).toMatchObject({
          "interfere.capture.kind": "exception",
        });

        const exceptionEvent = finished[0]?.events.find(
          (e) => e.name === "exception",
        );

        expect(exceptionEvent?.attributes).toMatchObject({
          "exception.message": "boom",
          "interfere.exception.handled": false,
          "interfere.exception.mechanism": "browser.onerror",
        });
      } finally {
        await kernel.dispose();

        await globalProvider.shutdown();

        context.disable();

        trace.disable();
      }
    });
  });
});

describe(createKernel, () => {
  test("skips the config fetch when plugins are baked at build time", async () => {
    const fullPlugins = Object.fromEntries(
      PLUGIN_MANIFEST.map((plugin) => [plugin.name, plugin.name !== "replay"]),
    );

    setInjectedConfiguration("__INTERFERE_PLUGINS__", fullPlugins);

    const configRequests = serveConfig(HttpResponse.json({ plugins: {} }));

    try {
      const kernel = await createKernel({
        opts: { tracing: false },
      });

      ownedKernels.add(kernel);

      expect(configRequests).toHaveLength(0);
      expect(kernel.getPluginConfigState("replay")).toBe("disabled");
      expect(kernel.getPluginConfigState("errors")).toBe("enabled");

      await kernel.dispose();
    } finally {
      vi.unstubAllGlobals();
    }
  });

  test.each([
    [
      "returns a failed response",
      () => new HttpResponse("unavailable", { status: 503 }),
    ],
    ["omits plugins", () => HttpResponse.json({})],
    [
      "cannot be parsed",
      () =>
        new HttpResponse("{", {
          headers: { "content-type": "application/json" },
        }),
    ],
  ])(
    "uses local plugin defaults when remote config %s",
    async (_case, response) => {
      setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "test-key");

      const configRequests = serveConfig(response());

      const kernel = await createKernel({
        opts: { tracing: false },
      });

      ownedKernels.add(kernel);

      expect(configRequests).toHaveLength(1);
      expect(kernel.getPluginConfigState("replay")).toBe("enabled");

      await kernel.dispose();
    },
  );

  test("emits the initial browser navigation log once pageEvents activates, not at boot", async () => {
    expect.hasAssertions();

    const logs: ReadableLogRecord[] = [];

    const logProcessor = {
      forceFlush: async () => Promise.resolve(),
      onEmit: (record) => {
        logs.push(record);
      },
      shutdown: async () => Promise.resolve(),
    } satisfies LogRecordProcessor;

    globalThis.history.pushState({}, "", "/login");

    const kernel = await createKernel({
      opts: { _internalAdditionalLogRecordProcessors: [logProcessor] },
    });

    ownedKernels.add(kernel);

    expect(
      logs,
      "pageview is analytics-gated; nothing may ship before consent settles",
    ).toHaveLength(0);

    kernel.syncProviderConfig({});

    await vi.waitFor(() => {
      const pageload = logs.find(
        (l) => l.attributes["browser.navigation.type"] === "pageload",
      );

      expect(pageload).toMatchObject({
        attributes: expect.objectContaining({
          "browser.navigation.type": "pageload",
          "interfere.view.id": kernel.views.viewId(),
          "interfere.view.trigger": "pageload",
          "session.id": expect.any(String),
          "url.full": globalThis.location.href,
        }),
        eventName: "browser.navigation",
        instrumentationScope: {
          name: "@interfere/browser-events",
        },
        severityNumber: 0,
      });
    }, WAIT_OPTS);

    await kernel.dispose();
  });

  test("runs public processors before wrapper processors and owns their lifecycle", async () => {
    const exporter = new InMemorySpanExporter();

    const processor = new SimpleSpanProcessor(new InMemorySpanExporter());
    const wrapper = new SimpleSpanProcessor(exporter);

    const onStart = vi.spyOn(processor, "onStart");

    const publicEnd = vi.spyOn(processor, "onEnd");
    const wrapperEnd = vi.spyOn(wrapper, "onEnd");

    const forceFlush = vi.spyOn(processor, "forceFlush");

    const shutdown = vi.spyOn(processor, "shutdown");

    const opts = {
      tracing: { spanProcessors: [processor] },
    } satisfies KernelOptions;

    const kernel = await createKernel({
      opts: {
        ...opts,
        _internalAdditionalSpanProcessors: [wrapper],
      },
    });

    ownedKernels.add(kernel);

    kernel
      .getTracer("public-processors")
      .startSpan("custom", {
        attributes: { "url.full": "https://example.com/secret" },
      })
      .end();

    await kernel.flush();

    expect(
      exporter.getFinishedSpans().find((span) => span.name === "custom")
        ?.attributes["url.full"],
    ).toBe("https://example.com/secret");

    expect(publicEnd.mock.invocationCallOrder[0]).toBeLessThan(
      wrapperEnd.mock.invocationCallOrder[0] ?? 0,
    );

    expect(onStart).toHaveBeenCalledWith(
      expect.objectContaining({ name: "custom" }),
      expect.anything(),
    );

    expect(publicEnd).toHaveBeenCalledWith(
      expect.objectContaining({ name: "custom" }),
    );

    expect(forceFlush).toHaveBeenCalledWith();

    await kernel.dispose();

    expect(shutdown).toHaveBeenCalledTimes(1);
  });

  test("tracing false leaves wrapper processors inactive", async () => {
    const processor = new SimpleSpanProcessor(new InMemorySpanExporter());

    const onStart = vi.spyOn(processor, "onStart");

    const onEnd = vi.spyOn(processor, "onEnd");

    const forceFlush = vi.spyOn(processor, "forceFlush");

    const shutdown = vi.spyOn(processor, "shutdown");

    const kernel = await createKernel({
      opts: { tracing: false, _internalAdditionalSpanProcessors: [processor] },
    });

    ownedKernels.add(kernel);

    kernel.getTracer("disabled").startSpan("ignored").end();

    await kernel.flush();

    await kernel.dispose();

    expect(onStart).not.toHaveBeenCalled();
    expect(onEnd).not.toHaveBeenCalled();
    expect(forceFlush).not.toHaveBeenCalled();
    expect(shutdown).not.toHaveBeenCalled();
  });

  test("never emits the initial navigation log when analytics consent is denied", async () => {
    const logs: ReadableLogRecord[] = [];

    const logProcessor = {
      forceFlush: async () => Promise.resolve(),
      onEmit: (record) => {
        const navigationType = Boolean(
          record.attributes["browser.navigation.type"],
        );

        if (navigationType) {
          logs.push(record);
        }
      },
      shutdown: async () => Promise.resolve(),
    } satisfies LogRecordProcessor;

    const kernel = await createKernel({
      opts: { _internalAdditionalLogRecordProcessors: [logProcessor] },
    });

    ownedKernels.add(kernel);

    kernel.consent.set(false);

    kernel.syncProviderConfig({});

    await kernel.dispose();

    expect(logs).toHaveLength(0);
  });
});

describe(readReleaseSlugFromGlobals, () => {
  afterEach(() => vi.unstubAllGlobals());

  test.each([undefined, null, false, 42, {}])(
    "ignores non-string host global %j",
    (value) => {
      setInjectedConfiguration("__INTERFERE_RELEASE_SLUG__", value);

      expect(readReleaseSlugFromGlobals()).toBeNull();
    },
  );

  test("validates the injected release slug", () => {
    setInjectedConfiguration(
      "__INTERFERE_RELEASE_SLUG__",
      "rel_0123456789abcdef",
    );

    expect(readReleaseSlugFromGlobals()).toBe("rel_0123456789abcdef");

    setInjectedConfiguration("__INTERFERE_RELEASE_SLUG__", "malformed");

    expect(() => readReleaseSlugFromGlobals()).toThrow(/Invalid/u);
  });
});

test("detects the actual automated browser", () => {
  expect(navigator.webdriver).toBe(true);

  expect(isBotUserAgent()).toBe(true);
});

describe("createKernel injected-global wiring", () => {
  const FORCE_ENABLE_GLOBAL = "__INTERFERE_FORCE_ENABLE__";
  const SDK_STACK_GLOBAL = "__INTERFERE_SDK_STACK__";

  const FORCE_ENABLE_WARNING =
    "FORCE_ENABLE is active. Events will be dropped by production ingest. This should only be set during local development.";

  afterEach(() => {
    Reflect.deleteProperty(globalThis, FORCE_ENABLE_GLOBAL);

    Reflect.deleteProperty(globalThis, SDK_STACK_GLOBAL);
  });

  test("warns exactly once per initialization when FORCE_ENABLE is truthy", async () => {
    setLogLevel("warn");

    const warn = vi.spyOn(globalThis.console, "warn");

    setInjectedConfiguration(FORCE_ENABLE_GLOBAL, true);

    const kernel = await createKernel({
      opts: { tracing: false },
    });

    ownedKernels.add(kernel);

    expect(
      warn.mock.calls.filter(
        (call) =>
          call[0] === "[Interfere:kernel]" && call[1] === FORCE_ENABLE_WARNING,
      ),
    ).toHaveLength(1);

    const second = await createKernel({
      opts: { tracing: false },
    });

    ownedKernels.add(second);

    expect(
      warn.mock.calls.filter(
        (call) =>
          call[0] === "[Interfere:kernel]" && call[1] === FORCE_ENABLE_WARNING,
      ),
    ).toHaveLength(2);

    await kernel.dispose();
  });

  test("does not warn about FORCE_ENABLE when the global is unset", async () => {
    setLogLevel("warn");

    const warn = vi.spyOn(globalThis.console, "warn");

    const kernel = await createKernel({
      opts: { tracing: false },
    });

    ownedKernels.add(kernel);

    expect(warn).not.toHaveBeenCalledWith(
      "[Interfere:kernel]",
      FORCE_ENABLE_WARNING,
    );

    await kernel.dispose();
  });

  test("stamps _wrapperVersions onto the SDK_STACK global by reference", async () => {
    const versions = ["next@1.2.3"];

    const kernel = await createKernel({
      opts: {
        _wrapperVersions: versions,
        tracing: false,
      },
    });

    ownedKernels.add(kernel);

    expect(globalThis).toHaveProperty(SDK_STACK_GLOBAL, versions);

    expect(
      Object.getOwnPropertyDescriptor(globalThis, SDK_STACK_GLOBAL)?.value,
    ).toBe(versions);

    await kernel.dispose();
  });

  test("rejects a non-writable SDK_STACK global rather than silently dropping wrapper versions", async () => {
    Object.defineProperty(globalThis, SDK_STACK_GLOBAL, {
      configurable: true,
      value: undefined,
      writable: false,
    });

    await expect(
      createKernel({
        opts: {
          _wrapperVersions: ["next@1.2.3"],
          tracing: false,
        },
      }),
    ).rejects.toThrow(TypeError);
  });

  test("leaves the SDK_STACK global unset when _wrapperVersions is absent", async () => {
    const kernel = await createKernel({
      opts: { tracing: false },
    });

    ownedKernels.add(kernel);

    expect(globalThis).not.toHaveProperty(SDK_STACK_GLOBAL);

    await kernel.dispose();
  });
});

function setDebugIdRegistry(
  value: Record<string, string> | undefined | null | boolean | number | string,
) {
  const original = Object.getOwnPropertyDescriptor(globalThis, "_debugIds");

  Object.defineProperty(globalThis, "_debugIds", {
    configurable: true,
    writable: true,
    value,
  });

  onTestFinished(() => {
    if (original) {
      Object.defineProperty(globalThis, "_debugIds", original);
    } else {
      Reflect.deleteProperty(globalThis, "_debugIds");
    }
  });
}
