import { ExportResultCode } from "@opentelemetry/core";
import { InterfereClient } from "@interfere/javascript";
import { exportTracePayload } from "../tracing/trace-payload";
import { HttpResponse, http } from "msw";
import { network } from "../../../../test/edge-network.fixture";
import {
  BasicTracerProvider,
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import type { MockInstance } from "vitest";

import { FetchTraceExporter } from "./edge-exporter.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

describe(FetchTraceExporter, () => {
  let fetchMock: MockInstance<typeof fetch>;
  let provider: BasicTracerProvider;
  let bareProvider: BasicTracerProvider;
  let spanExporter: InMemorySpanExporter;

  const makeSpan = () => {
    const span = bareProvider.getTracer("edge").startSpan("edge-op");

    span.end();

    const finishedSpan = spanExporter.getFinishedSpans().at(-1);

    if (!finishedSpan) {
      throw new Error("Expected a finished span");
    }

    return finishedSpan;
  };

  beforeEach(() => {
    network.use(
      http.post(
        "https://collector.test/drain/v1/traces",
        () => new HttpResponse(null, { status: 200 }),
      ),
    );

    fetchMock = vi.spyOn(globalThis, "fetch");

    provider = new BasicTracerProvider({
      spanProcessors: [
        new SimpleSpanProcessor(
          new FetchTraceExporter({
            baseUrl: "https://collector.test",
            workspaceTelemetryKeyHeader: { apiKey: "test-key" },
            headers: { "x-interfere-producer-version": "test" },
          }),
        ),
      ],
    });

    spanExporter = new InMemorySpanExporter();

    bareProvider = new BasicTracerProvider({
      spanProcessors: [new SimpleSpanProcessor(spanExporter)],
    });
  });

  afterEach(async () => {
    await provider.shutdown();

    await bareProvider.shutdown();

    vi.unstubAllGlobals();

    vi.restoreAllMocks();
  });

  test("pOSTs serialized spans to the sink via keepalive fetch", async () => {
    expect.hasAssertions();

    const received = Promise.withResolvers<true>();

    network.use(
      http.post("https://collector.test/drain/v1/traces", () => {
        received.resolve(true);

        return new HttpResponse(null, { status: 200 });
      }),
    );

    const span = provider.getTracer("edge").startSpan("edge-op");

    span.addEvent("exception", { "exception.message": "boom" });

    span.end();

    await received.promise;

    expect(fetchMock).toHaveBeenCalledTimes(1);

    const [url, init] = fetchMock.mock.calls[0] ?? [];

    expect(url).toContain("/drain/v1/traces");
    expect(init?.method).toBe("POST");
    expect(init?.keepalive).toBe(true);

    const headers = new Headers(init?.headers);

    expect(headers.get("content-type")).toBe("application/json");
    expect(headers.get("x-interfere-producer-version")).toBe("test");
    expect(headers.get("x-api-key")).toBe("test-key");
    expect(headers.has("x-interfere-public-key")).toBe(false);

    await expect(new Response(init?.body).text()).resolves.toContain("edge-op");
  });

  test.each([429, 503])(
    "retries %s with the same span body",
    async (status) => {
      const bodies: string[] = [];

      network.use(
        http.post(
          "https://collector.test/drain/v1/traces",
          async ({ request }) => {
            bodies.push(await request.text());

            return bodies.length === 1
              ? new HttpResponse(null, {
                  status,
                  headers: { "Retry-After": "0" },
                })
              : new HttpResponse(null);
          },
        ),
      );

      const exporter = new FetchTraceExporter({
        baseUrl: "https://collector.test",
        workspaceTelemetryKeyHeader: { apiKey: "test-key" },
      });

      const result = new Promise<import("@opentelemetry/core").ExportResult>(
        (resolve) => {
          exporter.export([makeSpan()], resolve);
        },
      );

      await expect(result).resolves.toStrictEqual({
        code: ExportResultCode.SUCCESS,
      });

      expect(bodies).toHaveLength(2);
      expect(bodies[0]).toBe(bodies[1]);
    },
  );

  test("skips fetch when there are no spans", () => {
    const exporter = new FetchTraceExporter({
      baseUrl: "https://collector.test",
      workspaceTelemetryKeyHeader: { apiKey: "test-key" },
      headers: {},
    });

    exporter.export([], vi.fn());

    expect(fetchMock).not.toHaveBeenCalled();
  });

  test.each([
    ["a null", null],
    ["an empty", new Uint8Array(0)],
  ])(
    "skips fetch and reports success for %s serialized payload",
    (_label, payload) => {
      const client = new InterfereClient({
        baseUrl: "https://collector.test",
        workspaceTelemetryKeyHeader: { apiKey: "test-key" },
      });

      const resultCallback = vi.fn<() => void>();

      exportTracePayload(payload, client, resultCallback);

      expect(resultCallback).toHaveBeenCalledWith({
        code: ExportResultCode.SUCCESS,
      });

      expect(fetchMock).not.toHaveBeenCalled();
    },
  );

  test("reports a failed export when the request rejects", async () => {
    expect.hasAssertions();

    network.use(
      http.post("https://collector.test/drain/v1/traces", () =>
        HttpResponse.error(),
      ),
    );

    const exporter = new FetchTraceExporter({
      baseUrl: "https://collector.test",
      workspaceTelemetryKeyHeader: { apiKey: "test-key" },
      headers: {},
    });

    const result = new Promise<import("@opentelemetry/core").ExportResult>(
      (resolve) => {
        exporter.export([makeSpan()], resolve);
      },
    );

    await expect(result).resolves.toStrictEqual({
      code: ExportResultCode.FAILED,
      error: expect.any(Error) as Error,
    });

    expect(fetchMock).toHaveBeenCalledTimes(1);
  });

  test("resolves forceFlush without exporting anything", async () => {
    const exporter = new FetchTraceExporter({
      baseUrl: "https://collector.test",
      workspaceTelemetryKeyHeader: { apiKey: "test-key" },
      headers: {},
    });

    await expect(exporter.forceFlush()).resolves.toBeUndefined();

    expect(fetchMock).not.toHaveBeenCalled();
  });
});
