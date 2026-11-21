import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import { register } from "./register.js";
import { runtime } from "./runtime.js";
/* oxlint-disable typescript/require-await */
import {
  ROOT_CONTEXT,
  TraceFlags,
  context,
  propagation,
  trace,
} from "@opentelemetry/api";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import {
  CompositePropagator,
  TraceState,
  W3CBaggagePropagator,
  W3CTraceContextPropagator,
} from "@opentelemetry/core";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
  TracerProvider,
} from "@opentelemetry/sdk-trace";

import { traceMeta, withRequestSpan } from "./request-trace.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

let exporter: InMemorySpanExporter;

let provider: TracerProvider;

describe("request-trace", () => {
  beforeEach(() => {
    vi.unstubAllEnvs();

    vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test");

    exporter = new InMemorySpanExporter();

    provider = new TracerProvider({
      spanProcessors: [new SimpleSpanProcessor({ exporter })],
    });

    trace.disable();

    context.disable();

    propagation.disable();

    trace.setGlobalTracerProvider(provider);

    context.setGlobalContextManager(
      new AsyncLocalStorageContextManager().enable(),
    );

    propagation.setGlobalPropagator(
      new CompositePropagator({
        propagators: [
          new W3CTraceContextPropagator(),
          new W3CBaggagePropagator(),
        ],
      }),
    );
  });

  afterEach(async () => {
    context.disable();

    trace.disable();

    propagation.disable();

    await provider.shutdown();

    vi.restoreAllMocks();
  });

  test("returns no meta when no span is active", () => {
    expect(traceMeta()).toStrictEqual([]);
  });

  test("emits meta for an unsampled active span with its flags intact", () => {
    const active = trace.setSpanContext(ROOT_CONTEXT, {
      traceId: "0af7651916cd43dd8448eb211c80319c",
      spanId: "b7ad6b7169203331",
      traceFlags: TraceFlags.NONE,
      isRemote: true,
      traceState: new TraceState("vendor=abc123"),
    });

    expect(context.with(active, traceMeta)).toStrictEqual([
      {
        name: "traceparent",
        content: "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-00",
      },
      { name: "tracestate", content: "vendor=abc123" },
    ]);
  });

  test("opens a SERVER span and exposes its traceparent to traceMeta inside the request", async () => {
    let metaInside: ReturnType<typeof traceMeta> = [];

    await withRequestSpan(new Request("https://app.test/page"), async () => {
      metaInside = traceMeta();

      return "ok";
    });

    const [span] = exporter.getFinishedSpans();

    expect(span?.name).toBe("GET /page");

    const traceparent = metaInside.find((m) => m.name === "traceparent");

    expect(traceparent?.content).toContain(
      `00-${span?.spanContext().traceId}-`,
    );
  });

  test("continues an inbound traceparent", async () => {
    const traceId = "0af7651916cd43dd8448eb211c80319c";
    let metaInside: ReturnType<typeof traceMeta> = [];

    await withRequestSpan(
      new Request("https://app.test/page", {
        headers: { traceparent: `00-${traceId}-b7ad6b7169203331-01` },
      }),
      async () => {
        metaInside = traceMeta();

        return "ok";
      },
    );

    expect(exporter.getFinishedSpans()[0]?.spanContext().traceId).toBe(traceId);

    expect(metaInside.find((m) => m.name === "traceparent")?.content).toContain(
      `00-${traceId}-`,
    );
  });

  test("records the error and rethrows when next throws", async () => {
    await expect(
      withRequestSpan(new Request("https://app.test/page"), async () => {
        throw new Error("ssr boom");
      }),
    ).rejects.toThrow("ssr boom");

    const [span] = exporter.getFinishedSpans();

    expect(span?.events.some((e) => e.name === "exception")).toBe(true);
  });

  test("runs next without a span when the SDK is disabled", async () => {
    vi.stubEnv("INTERFERE_API_KEY", undefined);

    vi.stubEnv("NODE_ENV", "development");

    const result = await withRequestSpan(
      new Request("https://app.test/page"),
      async () => "ok",
    );

    expect(result).toBe("ok");
    expect(exporter.getFinishedSpans()).toHaveLength(0);
  });

  test("runs next without a span when the application owns tracing", async () => {
    const collector = await receiver();

    vi.stubEnv("INTERFERE_API_URL", collector.url);

    await register({ tracing: "application", consoleBridge: false });

    try {
      const result = await withRequestSpan(
        new Request("https://app.test/page"),
        async () => "ok",
      );

      expect(result).toBe("ok");
      expect(exporter.getFinishedSpans()).toHaveLength(0);
    } finally {
      await runtime.shutdown();

      await collector.close();
    }
  });
});

describe("trace meta ownership", () => {
  test("emits no trace meta when the application owns tracing", async () => {
    const collector = await receiver();

    vi.stubEnv("INTERFERE_API_URL", collector.url);

    vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test");

    context.setGlobalContextManager(
      new AsyncLocalStorageContextManager().enable(),
    );

    propagation.setGlobalPropagator(new W3CTraceContextPropagator());

    const span = new TracerProvider()
      .getTracer("ownership")
      .startSpan("render");

    const active = trace.setSpan(ROOT_CONTEXT, span);

    try {
      await register({ tracing: "application", consoleBridge: false });

      expect(context.with(active, () => traceMeta())).toStrictEqual([]);

      await runtime.shutdown();

      expect(
        context.with(active, () => traceMeta()).map(({ name }) => name),
      ).toContain("traceparent");
    } finally {
      await runtime.shutdown();

      await collector.close();

      span.end();

      context.disable();

      propagation.disable();

      vi.unstubAllEnvs();
    }
  });
});
