import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { sessionIdSchema } from "@interfere/types/data/session";
import type { SessionId } from "@interfere/types/data/session";
import { releaseSlugSchema } from "@interfere/types/releases/slug";

import {
  ROOT_CONTEXT,
  context,
  defaultTextMapGetter,
  defaultTextMapSetter,
  propagation,
  trace,
} from "@opentelemetry/api";
import type { Context } from "@opentelemetry/api";
import {
  InMemoryLogRecordExporter,
  SimpleLogRecordProcessor,
} from "@opentelemetry/sdk-logs";
import {
  AggregationTemporality,
  InMemoryMetricExporter,
  PeriodicExportingMetricReader,
} from "@opentelemetry/sdk-metrics";
import type { SpanProcessor } from "@opentelemetry/sdk-trace-base";
import {
  BasicTracerProvider,
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";

import {
  CompositePropagator,
  W3CBaggagePropagator,
  W3CTraceContextPropagator,
} from "@opentelemetry/core";
import { buildOtelProvider } from "./provider.js";
import { snapshotRequestIdentity } from "./request-observer/request-identity.js";
import { requestObservationLogger } from "./request-observer/request-observer-events.js";
import type { OtelProviderHandle } from "./provider.js";
import { SDK_NAME, SDK_VERSION } from "../version.js";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";
import { z } from "zod";

import { worker } from "../../../test/msw.js";
import {
  decodedBodyJson,
  decodedBodyText,
} from "../../../test/fixtures/request-body.js";

function buildReader() {
  const exporter = new InMemoryMetricExporter(
    AggregationTemporality.CUMULATIVE,
  );

  const shutdownSpy = vi.spyOn(exporter, "shutdown");

  const reader = new PeriodicExportingMetricReader({
    exportIntervalMillis: 60_000,
    exporter,
  });

  return { reader, shutdownSpy };
}

function captureSpans() {
  const exporter = new InMemorySpanExporter();
  const processor = new SimpleSpanProcessor(exporter);

  return { processor, spans: exporter.getFinishedSpans() };
}

function captureLogs() {
  const exporter = new InMemoryLogRecordExporter();
  const processor = new SimpleLogRecordProcessor({ exporter });

  return { processor, logs: exporter.getFinishedLogRecords() };
}

const SDK_STACK = ["@interfere/next@10.0.0", "@interfere/react@10.0.0"];

const COLLECTOR_TRACES = "https://collector.test/drain/v1/traces";

function buildProvider(
  options: Partial<Parameters<typeof buildOtelProvider>[0]> = {},
) {
  return buildOtelProvider({
    clientOptions: {
      baseUrl: "https://collector.test",
      publicKeyHeader: { apiKey: "test-key" },
    },
    getDeviceId: () => null,
    getSessionId: () => null,
    sdkStack: SDK_STACK,
    ...options,
  });
}

const traceExportSchema = z.object({
  resourceSpans: z.array(
    z.object({
      scopeSpans: z.array(
        z.object({ spans: z.array(z.object({ name: z.string() })) }),
      ),
    }),
  ),
});

const W3C = new CompositePropagator({
  propagators: [new W3CTraceContextPropagator(), new W3CBaggagePropagator()],
});

const ALL_HEADERS = new Set(["baggage", "traceparent", "tracestate"] as const);

function headerPropagator(handle: OtelProviderHandle) {
  return {
    extract: W3C.extract.bind(W3C),
    inject: (
      ctx: Context,
      carrier: Record<string, string>,
      _setter: typeof defaultTextMapSetter,
    ) => {
      const headers = new Headers();

      handle.traceHeaders(ctx, headers, ALL_HEADERS);

      for (const [name, value] of headers) {
        carrier[name] = value;
      }
    },
  };
}

describe(buildOtelProvider, () => {
  let handle: OtelProviderHandle | null = null;
  const foreignProviders: BasicTracerProvider[] = [];

  beforeEach(() => {
    worker.use(
      ...["traces", "logs", "metrics"].map((signal) =>
        http.post(
          `https://collector.test/drain/v1/${signal}`,
          () => new HttpResponse(null, { status: 204 }),
        ),
      ),
    );

    context.disable();

    trace.disable();

    propagation.disable();
  });

  afterEach(async () => {
    try {
      const results = await Promise.allSettled([
        handle?.shutdown(),
        ...foreignProviders
          .splice(0)
          .map(async (provider) => provider.shutdown()),
      ]);

      const failures = results.filter((result) => result.status === "rejected");

      if (failures.length > 0) {
        throw new AggregateError(
          failures.map((result) => {
            const decodedValue: unknown = result.reason;

            return decodedValue;
          }),
          "Provider cleanup failed",
        );
      }
    } finally {
      handle = null;

      context.disable();

      trace.disable();

      propagation.disable();

      vi.restoreAllMocks();

      vi.unstubAllGlobals();
    }
  });

  test("external propagation preserves extraction and baggage without writing trace context", () => {
    handle = buildProvider({
      propagationOwnership: "external",
      getSessionId: () =>
        sessionIdSchema.parse("01900000-0000-7000-8000-000000000001"),
    });

    const traceparent =
      "00-1234567890abcdef1234567890abcdef-1234567890abcdef-00";

    const parent = headerPropagator(handle).extract(
      ROOT_CONTEXT,
      { traceparent, baggage: "test.region=west" },
      defaultTextMapGetter,
    );

    const headers = new Headers();

    handle.traceHeaders(
      trace.setSpanContext(parent, {
        spanId: "1234567890abcdef",
        traceFlags: 1,
        traceId: "1234567890abcdef1234567890abcdef",
      }),
      headers,
      new Set(["baggage", "traceparent", "tracestate"]),
    );

    expect(Object.fromEntries(headers)).toStrictEqual({
      baggage:
        "test.region=west,interfere.session.id=01900000-0000-7000-8000-000000000001",
    });
  });

  test("propagates the current session without changing trace context or other baggage", async () => {
    let sessionId: SessionId | null = sessionIdSchema.parse(
      "01900000-0000-7000-8000-000000000001",
    );

    handle = buildProvider({ getSessionId: () => sessionId });

    const propagator = headerPropagator(handle);

    const traceparent =
      "00-1234567890abcdef1234567890abcdef-1234567890abcdef-01";

    const parent = propagator.extract(
      ROOT_CONTEXT,
      { traceparent, baggage: "test.region=west" },
      defaultTextMapGetter,
    );

    const inject = () => {
      const headers: Record<string, string> = {};

      propagator.inject(parent, headers, defaultTextMapSetter);

      return headers;
    };

    expect(inject()).toMatchObject({
      traceparent,
      baggage: `test.region=west,interfere.session.id=${sessionId}`,
    });

    sessionId = sessionIdSchema.parse("01900000-0000-7000-8000-000000000002");

    expect(inject()["baggage"]).toBe(
      `test.region=west,interfere.session.id=${sessionId}`,
    );

    sessionId = null;

    expect(inject()["baggage"]).toBe("test.region=west");

    expect(
      propagation.getBaggage(parent)?.getEntry("interfere.session.id"),
    ).toBeUndefined();

    sessionId = sessionIdSchema.parse("01900000-0000-7000-8000-000000000001");

    await handle.shutdown();

    expect(inject()["baggage"]).toBe("test.region=west");

    handle = null;
  });

  test("propagates the device with the session so server-side spans attribute to it", async () => {
    const sessionId = sessionIdSchema.parse(
      "01900000-0000-7000-8000-000000000001",
    );

    let deviceId: string | null = null;

    handle = buildProvider({
      getDeviceId: () => deviceId,
      getSessionId: () => sessionId,
    });

    const propagator = headerPropagator(handle);

    const inject = () => {
      const headers: Record<string, string> = {};

      propagator.inject(ROOT_CONTEXT, headers, defaultTextMapSetter);

      return headers["baggage"];
    };

    expect(inject()).toBe(`interfere.session.id=${sessionId}`);

    deviceId = "0c5f6a3e-2b1d-4c7e-9a8b-1f2e3d4c5b6a";

    expect(inject()).toBe(
      `interfere.session.id=${sessionId},interfere.device.id=${deviceId}`,
    );

    deviceId = null;

    expect(inject()).toBe(`interfere.session.id=${sessionId}`);

    deviceId = "0c5f6a3e-2b1d-4c7e-9a8b-1f2e3d4c5b6a";

    await handle.shutdown();

    expect(inject()).toBeUndefined();

    handle = null;
  });

  test.each([
    [undefined, undefined, "unknown_service"],
    ["storefront", "shop", "storefront"],
  ])(
    "constructs providers with service %s in namespace %s",
    (serviceName, serviceNamespace, expectedName) => {
      const captured = captureSpans();

      handle = buildProvider(
        omitUndefined({
          serviceName,
          serviceNamespace,
          additionalSpanProcessors: [captured.processor],
          getSessionId: () =>
            sessionIdSchema.parse("01900000-0000-7000-8000-000000000001"),
          releaseSlug: releaseSlugSchema.parse("rel_0123456789abcdef"),
        }),
      );

      expect(handle.tracerProvider).toBeDefined();
      expect(handle.meterProvider).toBeDefined();
      expect(handle.metricReader).toBeDefined();
      expect(handle.flush).toBeTypeOf("function");
      expect(handle.shutdown).toBeTypeOf("function");

      handle.tracerProvider
        .getTracer("resource-test")
        .startSpan("resource")
        .end();

      expect(captured.spans).toHaveLength(1);

      expect(captured.spans[0]?.resource.attributes["service.namespace"]).toBe(
        serviceNamespace,
      );

      expect(captured.spans[0]?.resource.attributes).toMatchObject({
        "service.name": expectedName,
        "telemetry.sdk.language": "webjs",
        "interfere.sdk.name": SDK_NAME,
        "interfere.sdk.version": SDK_VERSION,
        "interfere.sdk.stack": SDK_STACK.join(", "),
        "release.slug": "rel_0123456789abcdef",
      });
    },
  );

  test("exports every span of a page-load burst, including the parent that ends last", async () => {
    const exported: string[] = [];

    worker.use(
      http.post(COLLECTOR_TRACES, async ({ request }) => {
        const { resourceSpans } = traceExportSchema.parse(
          await decodedBodyJson(request),
        );

        exported.push(
          ...resourceSpans.flatMap((r) =>
            r.scopeSpans.flatMap((s) => s.spans.map((span) => span.name)),
          ),
        );

        return new HttpResponse(null, { status: 204 });
      }),
    );

    handle = buildProvider();

    const tracer = handle.tracerProvider.getTracer("document-load");
    const documentLoad = tracer.startSpan("documentLoad");

    for (let index = 0; index < 80; index += 1) {
      tracer.startSpan(`resource ${index}`).end();
    }

    documentLoad.end();

    await handle.flush();

    expect(exported).toHaveLength(81);
    expect(exported).toContain("documentLoad");
  });

  test("redacts every span before a full batch reaches the Interfere exporter", async () => {
    const exported: string[] = [];

    worker.use(
      http.post(COLLECTOR_TRACES, async ({ request }) => {
        exported.push(await decodedBodyText(request));

        return new HttpResponse(null, { status: 204 });
      }),
    );

    const processorLifecycle = new SimpleSpanProcessor(
      new InMemorySpanExporter(),
    );

    const processor: SpanProcessor = {
      onStart: vi.fn<SpanProcessor["onStart"]>(),
      onEnd(span) {
        span.attributes["url.full"] = "https://example.com/redacted";
      },
      forceFlush: async () => processorLifecycle.forceFlush(),
      shutdown: async () => processorLifecycle.shutdown(),
    };

    handle = buildProvider({ additionalSpanProcessors: [processor] });

    const tracer = handle.tracerProvider.getTracer("redaction");

    for (let index = 0; index < 20; index += 1) {
      tracer
        .startSpan(`request ${index}`, {
          attributes: { "url.full": "https://example.com/secret" },
        })
        .end();
    }

    await handle.flush();

    expect(exported.length).toBeGreaterThan(0);
    expect(exported.join("")).toContain("https://example.com/redacted");
    expect(exported.join("")).not.toContain("https://example.com/secret");
  });

  test("works without optional sessionId / releaseSlug", () => {
    handle = buildProvider();

    expect(handle.tracerProvider).toBeDefined();
  });

  test("flush resolves without throwing", async () => {
    handle = buildProvider();

    await expect(handle.flush()).resolves.toBeUndefined();
  });

  test("stamps `device.id` as a per-span attribute when the getter resolves", () => {
    const captured = captureSpans();

    handle = buildProvider({
      additionalSpanProcessors: [captured.processor],
      getDeviceId: () => "dev_01ABCXYZ",
    });

    const span = handle.tracerProvider.getTracer("test").startSpan("probe");

    span.end();

    expect(captured.spans).toHaveLength(1);
    expect(captured.spans[0]?.attributes["device.id"]).toBe("dev_01ABCXYZ");
  });

  test("stamps `device.id` as a per-log attribute when the getter resolves", () => {
    const captured = captureLogs();

    handle = buildProvider({
      additionalLogRecordProcessors: [captured.processor],
      getDeviceId: () => "dev_01ABCXYZ",
    });

    handle.loggerProvider.getLogger("test").emit({ body: "hello" });

    expect(captured.logs).toHaveLength(1);
    expect(captured.logs[0]?.attributes["device.id"]).toBe("dev_01ABCXYZ");
  });

  test.each(["fetch", "xhr"] as const)(
    "%s completion cannot restore a request identity after consent changes",
    (kind) => {
      const captured = captureLogs();
      let consentEpoch = 0;

      const initialSession = sessionIdSchema.parse(
        "01900000-0000-7000-8000-000000000001",
      );

      let session = initialSession;
      let device: string | null = "request-device";

      const identity = {
        getSessionId: () => session,
        getDeviceId: () => device,
      };

      handle = buildProvider({
        ...identity,
        additionalLogRecordProcessors: [captured.processor],
      });

      const requestContext = snapshotRequestIdentity(
        ROOT_CONTEXT,
        identity,
        () => consentEpoch === 0,
      );

      const observe = requestObservationLogger(
        handle.loggerProvider,
        (url) => url,
      );

      const observation = {
        kind,
        method: "GET",
        url: "https://app.test/pending",
        status: 200,
      };

      session = sessionIdSchema.parse("01900000-0000-7000-8000-000000000002");

      observe(observation, requestContext);

      expect(captured.logs[0]?.attributes["session.id"]).toBe(initialSession);
      expect(captured.logs[0]?.attributes["device.id"]).toBe("request-device");

      consentEpoch = 1;

      device = null;

      observe(observation, requestContext);

      expect(captured.logs[1]?.attributes["session.id"]).toBeUndefined();
      expect(captured.logs[1]?.attributes["device.id"]).toBeUndefined();

      consentEpoch = 2;

      session = sessionIdSchema.parse("01900000-0000-7000-8000-000000000003");

      device = "regranted-device";

      observe(observation, requestContext);

      expect(captured.logs[2]?.attributes["session.id"]).toBeUndefined();
      expect(captured.logs[2]?.attributes["device.id"]).toBeUndefined();
      expect(captured.logs).toHaveLength(3);
    },
  );

  test("re-reads device.id per span so mid-page rotation lands on subsequent spans", () => {
    let current: string | null = "dev_first";
    const captured = captureSpans();

    handle = buildProvider({
      additionalSpanProcessors: [captured.processor],
      getDeviceId: () => current,
    });

    const tracer = handle.tracerProvider.getTracer("test");

    tracer.startSpan("before").end();

    current = "dev_second";

    tracer.startSpan("after").end();

    expect(captured.spans).toHaveLength(2);
    expect(captured.spans[0]?.attributes["device.id"]).toBe("dev_first");
    expect(captured.spans[1]?.attributes["device.id"]).toBe("dev_second");
  });

  describe("device-info (browser-detector) consent gating", () => {
    test("keeps browser attrs off the shared resource so metrics can never leak them", () => {
      const captured = captureSpans();

      handle = buildProvider({
        additionalSpanProcessors: [captured.processor],
        isDeviceInfoEnabled: () => true,
      });

      handle.tracerProvider.getTracer("test").startSpan("probe").end();

      const resourceAttrs = captured.spans[0]?.resource.attributes ?? {};

      expect(resourceAttrs["user_agent.original"]).toBeUndefined();
      expect(resourceAttrs["browser.language"]).toBeUndefined();
    });

    test("stamps browser attrs per-span only while the getter allows", () => {
      let allowed = false;
      const captured = captureSpans();

      handle = buildProvider({
        additionalSpanProcessors: [captured.processor],
        isDeviceInfoEnabled: () => allowed,
      });

      const tracer = handle.tracerProvider.getTracer("test");

      tracer.startSpan("denied").end();

      allowed = true;

      tracer.startSpan("granted").end();

      allowed = false;

      tracer.startSpan("revoked").end();

      expect(captured.spans).toHaveLength(3);

      expect(
        captured.spans[0]?.attributes["user_agent.original"],
      ).toBeUndefined();

      expect(captured.spans[1]?.attributes["user_agent.original"]).toBe(
        navigator.userAgent,
      );

      expect(
        captured.spans[2]?.attributes["user_agent.original"],
      ).toBeUndefined();
    });

    test("stamps browser attrs per-log only while the getter allows", () => {
      let allowed = true;
      const captured = captureLogs();

      handle = buildProvider({
        additionalLogRecordProcessors: [captured.processor],
        isDeviceInfoEnabled: () => allowed,
      });

      const logger = handle.loggerProvider.getLogger("test");

      logger.emit({ body: "granted" });

      allowed = false;

      logger.emit({ body: "revoked" });

      expect(captured.logs).toHaveLength(2);

      expect(captured.logs[0]?.attributes["user_agent.original"]).toBe(
        navigator.userAgent,
      );

      expect(
        captured.logs[1]?.attributes["user_agent.original"],
      ).toBeUndefined();
    });

    test("denies by default when no getter is supplied", () => {
      const captured = captureSpans();

      handle = buildProvider({
        additionalSpanProcessors: [captured.processor],
      });

      handle.tracerProvider.getTracer("test").startSpan("probe").end();

      expect(
        captured.spans[0]?.attributes["user_agent.original"],
      ).toBeUndefined();
    });
  });

  test("omits `device.id` when the getter returns null (SSR boot / pre-hydrate)", () => {
    const captured = captureSpans();

    handle = buildProvider({
      additionalSpanProcessors: [captured.processor],
    });

    handle.tracerProvider.getTracer("test").startSpan("probe").end();

    expect(captured.spans).toHaveLength(1);
    expect(captured.spans[0]?.attributes["device.id"]).toBeUndefined();
  });

  test("prefers the stamped __INTERFERE_ENVIRONMENT__ global for `deployment.environment.name`", () => {
    setInjectedConfiguration("__INTERFERE_ENVIRONMENT__", "staging");

    try {
      const captured = captureSpans();

      handle = buildProvider({
        additionalSpanProcessors: [captured.processor],
      });

      handle.tracerProvider.getTracer("test").startSpan("probe").end();

      expect(captured.spans).toHaveLength(1);

      expect(
        captured.spans[0]?.resource.attributes["deployment.environment.name"],
      ).toBe("staging");
    } finally {
      vi.unstubAllGlobals();
    }
  });

  test("reports development from a dev-server page without a stamped environment global", () => {
    const captured = captureSpans();

    handle = buildProvider({
      additionalSpanProcessors: [captured.processor],
    });

    handle.tracerProvider.getTracer("test").startSpan("probe").end();

    expect(
      captured.spans[0]?.resource.attributes["deployment.environment.name"],
    ).toBe("development");
  });

  test("fans additional metric readers into the MeterProvider", async () => {
    const { reader, shutdownSpy } = buildReader();

    handle = buildProvider({
      additionalMetricReaders: [reader],
    });

    await handle.shutdown();

    handle = null;

    expect(shutdownSpy).toHaveBeenCalledWith();
  });

  describe("global-provider isolation", () => {
    beforeEach(() => {
      trace.disable();

      context.disable();
    });

    afterEach(() => {
      trace.disable();

      context.disable();
    });

    test("does not stomp a foreign global tracer provider installed before us", () => {
      const foreignExporter = new InMemorySpanExporter();

      const foreignProvider = new BasicTracerProvider({
        spanProcessors: [new SimpleSpanProcessor(foreignExporter)],
      });

      trace.setGlobalTracerProvider(foreignProvider);

      foreignProviders.push(foreignProvider);

      const ours = captureSpans();

      handle = buildProvider({
        additionalSpanProcessors: [ours.processor],
      });

      trace.getTracer("global-consumer").startSpan("via-global").end();

      handle.tracerProvider.getTracer("private").startSpan("via-private").end();

      expect(foreignExporter.getFinishedSpans()).toHaveLength(1);
      expect(foreignExporter.getFinishedSpans()[0]?.name).toBe("via-global");
      expect(ours.spans).toHaveLength(1);
      expect(ours.spans[0]?.name).toBe("via-private");
    });

    test("installs our tracer provider ONLY on the private handle, never as global", () => {
      const ours = captureSpans();

      handle = buildProvider({
        additionalSpanProcessors: [ours.processor],
      });

      const foreignExporter = new InMemorySpanExporter();

      const foreignProvider = new BasicTracerProvider({
        spanProcessors: [new SimpleSpanProcessor(foreignExporter)],
      });

      expect(trace.setGlobalTracerProvider(foreignProvider)).toBe(true);

      foreignProviders.push(foreignProvider);

      trace.getTracer("global-consumer").startSpan("via-global").end();

      handle.tracerProvider.getTracer("private").startSpan("via-private").end();

      expect(foreignExporter.getFinishedSpans()).toHaveLength(1);
      expect(ours.spans).toHaveLength(1);
    });
  });
});
