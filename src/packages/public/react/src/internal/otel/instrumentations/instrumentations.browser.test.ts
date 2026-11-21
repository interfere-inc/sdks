/* oxlint-disable typescript/require-await */
import { LoggerProvider } from "@opentelemetry/sdk-logs";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
  TracerProvider,
} from "@opentelemetry/sdk-trace";

import { registerBundledInstrumentations } from "./instrumentations.js";
import { applyFetchSpanAttrs } from "./request-outcome.js";
import type { HttpStatusCodeRange } from "./instrumentations.js";
import { afterEach, describe, expect, test } from "vitest";

const INGEST_RE = /\/v1\/ingest/u;

describe(registerBundledInstrumentations, () => {
  const disposers: (() => void)[] = [];
  const providers: TracerProvider[] = [];
  const loggerProviders: LoggerProvider[] = [];

  afterEach(async () => {
    const results = await Promise.allSettled([
      ...disposers.splice(0).map(async (dispose) => dispose()),
      ...providers.splice(0).map(async (p) => p.shutdown()),
      ...loggerProviders.splice(0).map(async (p) => p.shutdown()),
    ]);

    const failures = results.filter((result) => result.status === "rejected");

    if (failures.length > 0) {
      throw new AggregateError(
        failures.map((result) => {
          const decodedValue: unknown = result.reason;

          return decodedValue;
        }),
        "Instrumentation cleanup failed",
      );
    }
  });

  function makeProvider() {
    const p = new TracerProvider();

    providers.push(p);

    return p;
  }

  function makeLoggerProvider() {
    const p = new LoggerProvider();

    loggerProviders.push(p);

    return p;
  }

  test("returns a handle when registered against a private provider", () => {
    const tracerProvider = makeProvider();

    const handle = registerBundledInstrumentations({
      ignoreUrls: [INGEST_RE],
      tracerProvider,
    });

    disposers.push(handle.dispose);

    expect(handle.dispose).toBeTypeOf("function");
    expect(handle.setPageEventsEnabled).toBeTypeOf("function");
  });

  test("accepts a string + regex mix in ignoreUrls", () => {
    const tracerProvider = makeProvider();

    const handle = registerBundledInstrumentations({
      ignoreUrls: [INGEST_RE, "https://other.test"],
      tracerProvider,
    });

    disposers.push(handle.dispose);

    expect(handle.dispose).toBeTypeOf("function");
  });

  test("returns a handle when a logger provider is supplied", () => {
    const tracerProvider = makeProvider();
    const loggerProvider = makeLoggerProvider();

    const handle = registerBundledInstrumentations({
      ignoreUrls: [INGEST_RE],
      loggerProvider,
      tracerProvider,
    });

    disposers.push(handle.dispose);

    expect(handle.dispose).toBeTypeOf("function");
  });

  test("survives repeated pageEvents toggling and dispose while disabled", () => {
    const tracerProvider = makeProvider();

    const handle = registerBundledInstrumentations({
      ignoreUrls: [INGEST_RE],
      tracerProvider,
    });

    disposers.push(handle.dispose);

    handle.setPageEventsEnabled(true);

    handle.setPageEventsEnabled(true);

    handle.setPageEventsEnabled(false);

    handle.setPageEventsEnabled(false);

    handle.setPageEventsEnabled(true);

    expect(() => {
      handle.dispose();
    }).not.toThrow();
  });
});

describe(applyFetchSpanAttrs, () => {
  const failureRanges: readonly HttpStatusCodeRange[] = [[500, 599]];

  async function captureFetchAttrs(
    request: Parameters<typeof applyFetchSpanAttrs>[1],
    responseUrl = "https://app.test/api/items",
  ) {
    const exporter = new InMemorySpanExporter();

    const provider = new TracerProvider({
      spanProcessors: [new SimpleSpanProcessor({ exporter })],
    });

    const span = provider
      .getTracer("fetch-attributes-test")
      .startSpan("request");

    const response = new Response(null, {
      headers: { "content-type": "text/plain" },
      status: 503,
    });

    onTestFinished(async () => {
      await provider.shutdown();
    });

    Object.defineProperty(response, "url", { value: responseUrl });

    void applyFetchSpanAttrs(
      span,
      request,
      response,
      failureRanges,
      { dispose: () => undefined, leftSince: () => false },
      () => true,
      (pathname) => (pathname === "/api/items" ? "/api/[resource]" : undefined),
    );

    span.end();

    await provider.forceFlush();

    const [finishedSpan] = exporter.getFinishedSpans();

    if (!finishedSpan) {
      throw new Error("Expected an exported fetch span");
    }

    return {
      attrs: finishedSpan.attributes,
      events: finishedSpan.events,
      name: finishedSpan.name,
    };
  }

  test("captures failed fetches made with a string URL and RequestInit method", async () => {
    const { attrs, events, name } = await captureFetchAttrs({ method: "POST" });

    expect(attrs["url.path"]).toBe("/api/items");
    expect(attrs["http.route"]).toBe("/api/[resource]");
    expect(name).toBe("POST /api/[resource]");

    expect(events[0]?.attributes).toMatchObject({
      "exception.message": "HTTP 503 POST https://app.test/api/items",
      "interfere.exception.handled": false,
    });
  });

  test("captures failed fetches made with URL inputs", async () => {
    const { name, events } = await captureFetchAttrs(
      new URL("https://app.test/api/items"),
    );

    expect(name).toBe("GET /api/[resource]");

    expect(events[0]?.attributes?.["exception.message"]).toBe(
      "HTTP 503 GET https://app.test/api/items",
    );
  });

  test("captures failed fetches made with Request inputs", async () => {
    const { name, events } = await captureFetchAttrs(
      new Request("https://app.test/api/items", { method: "PATCH" }),
    );

    expect(name).toBe("PATCH /api/[resource]");

    expect(events[0]?.attributes?.["exception.message"]).toBe(
      "HTTP 503 PATCH https://app.test/api/items",
    );
  });
});
