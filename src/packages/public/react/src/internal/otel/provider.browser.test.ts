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
import type { TracerProvider } from "@opentelemetry/sdk-trace";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace";

import {
  CompositePropagator,
  W3CBaggagePropagator,
  W3CTraceContextPropagator,
} from "@opentelemetry/core";
import { buildOtelProvider } from "./provider.js";
import type { OtelProviderHandle } from "./provider.js";
import { SDK_NAME, SDK_VERSION } from "../version.js";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

import { worker } from "../../../test/msw.js";

function captureSpans() {
  const exporter = new InMemorySpanExporter();
  const processor = new SimpleSpanProcessor({ exporter });

  return { processor, spans: exporter.getFinishedSpans() };
}

const SDK_STACK = ["@interfere/next@10.0.0", "@interfere/react@10.0.0"];

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
  const foreignProviders: TracerProvider[] = [];

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
});
