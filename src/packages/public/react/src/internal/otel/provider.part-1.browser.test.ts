import { sessionIdSchema } from "@interfere/types/data/session";

import { ROOT_CONTEXT, context, propagation, trace } from "@opentelemetry/api";
import {
  InMemoryLogRecordExporter,
  SimpleLogRecordProcessor,
} from "@opentelemetry/sdk-logs";
import type { SpanProcessor, TracerProvider } from "@opentelemetry/sdk-trace";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace";

import { buildOtelProvider } from "./provider.js";
import { snapshotRequestIdentity } from "./request-observer/request-identity.js";
import { requestObservationLogger } from "./request-observer/request-observer-events.js";
import type { OtelProviderHandle } from "./provider.js";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";
import { z } from "zod";

import { worker } from "../../../test/msw.js";
import {
  decodedBodyJson,
  decodedBodyText,
} from "../../../test/fixtures/request-body.js";

function captureSpans() {
  const exporter = new InMemorySpanExporter();
  const processor = new SimpleSpanProcessor({ exporter });

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
      releaseTokenHeader: { apiKey: "test-key" },
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

    const processorLifecycle = new SimpleSpanProcessor({
      exporter: new InMemorySpanExporter(),
    });

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
});
