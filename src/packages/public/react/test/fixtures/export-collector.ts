import { SpanKind, SpanStatusCode } from "@opentelemetry/api";
import type { ExportResult } from "@opentelemetry/core";
import { resourceFromAttributes } from "@opentelemetry/resources";
import {
  InMemoryLogRecordExporter,
  LoggerProvider,
  SimpleLogRecordProcessor,
} from "@opentelemetry/sdk-logs";
import type { ResourceMetrics } from "@opentelemetry/sdk-metrics";
import type { ReadableSpan } from "@opentelemetry/sdk-trace-base";

import { createTraceExporter } from "../../src/internal/otel/exporter.js";
import { HttpResponse, http } from "msw";
import { z } from "zod";

import { createCountSignal } from "./upload-signal.js";
import { decodedBodyText } from "./request-body.js";

export const INGEST = "https://in.example.com";

export const CLIENT_OPTIONS = {
  baseUrl: INGEST,
  publicKeyHeader: { apiKey: "test-key" },
};

export interface Sent {
  bytes: number;
  keepalive: boolean;
  signal: string;
  text: string;
}

export const sent: Sent[] = [];

export const sentSignal = createCountSignal(sent);

export function resetSent() {
  sent.length = 0;
}

export function collector(
  respond: (index: number) => Promise<Response> | Response,
) {
  return http.post(
    `${INGEST}/drain/v1/:signal`,
    async ({ params, request }) => {
      const text = await decodedBodyText(request);

      const index = sent.length;

      sent.push({
        bytes: (await request.clone().arrayBuffer()).byteLength,
        keepalive: request.keepalive,
        signal: String(params["signal"]),
        text,
      });

      sentSignal.notify();

      return respond(index);
    },
  );
}

export function gate() {
  const opened = Promise.withResolvers<null>();

  return {
    open: () => {
      opened.resolve(null);
    },
    wait: async () => opened.promise,
  };
}

export function resultCallback() {
  const called = Promise.withResolvers<ExportResult>();

  const results: ExportResult[] = [];

  return {
    callback: (result: ExportResult) => {
      results.push(result);

      called.resolve(result);
    },
    called: called.promise,
    results,
  };
}

export function makeSpan(attributes: Record<string, string> = {}) {
  return {
    attributes,
    droppedAttributesCount: 0,
    droppedEventsCount: 0,
    droppedLinksCount: 0,
    duration: [0, 0],
    endTime: [0, 0],
    ended: true,
    events: [],
    instrumentationScope: { name: "test" },
    kind: SpanKind.INTERNAL,
    links: [],
    name: "test-span",
    resource: resourceFromAttributes({}),
    spanContext: () => ({
      traceId: "a".repeat(32),
      spanId: "b".repeat(16),
      traceFlags: 1,
    }),
    startTime: [0, 0],
    status: { code: SpanStatusCode.UNSET },
  } satisfies ReadableSpan;
}

function incompressible(size: number) {
  return btoa(
    String.fromCodePoint(...crypto.getRandomValues(new Uint8Array(size))),
  ).slice(0, size);
}

/** Spans carrying unique random payloads, so their size on the wire stays close to `size` even when the export is gzipped. */
export function bulkySpans(count: number, size = 2000) {
  return Array.from({ length: count }, () =>
    makeSpan({ payload: incompressible(size) }),
  );
}

export function makeLogs(count: number, size = 2000) {
  const memory = new InMemoryLogRecordExporter();

  const provider = new LoggerProvider({
    processors: [new SimpleLogRecordProcessor({ exporter: memory })],
  });

  const logger = provider.getLogger("test");

  const body = "y".repeat(size);

  for (let index = 0; index < count; index += 1) {
    logger.emit({ body });
  }

  return memory.getFinishedLogRecords();
}

export function makeResourceMetrics() {
  return {
    resource: resourceFromAttributes({ "service.name": "test" }),
    scopeMetrics: [],
  } satisfies ResourceMetrics;
}

export function exportSpans(spans: ReadableSpan[]) {
  const result = resultCallback();

  createTraceExporter({ clientOptions: CLIENT_OPTIONS }).export(
    spans,
    result.callback,
  );

  return result;
}

const traceBodySchema = z.object({
  resourceSpans: z.array(
    z.object({
      scopeSpans: z.array(z.object({ spans: z.array(z.unknown()) })),
    }),
  ),
});

const logBodySchema = z.object({
  resourceLogs: z.array(
    z.object({
      scopeLogs: z.array(z.object({ logRecords: z.array(z.unknown()) })),
    }),
  ),
});

export function exportedSpanCount(entries: Sent[]) {
  return entries
    .map((entry) => traceBodySchema.parse(JSON.parse(entry.text)))
    .flatMap((body) => body.resourceSpans)
    .flatMap((resourceSpans) => resourceSpans.scopeSpans)
    .reduce((total, scope) => total + scope.spans.length, 0);
}

export function exportedLogCount(entries: Sent[]) {
  return entries
    .map((entry) => logBodySchema.parse(JSON.parse(entry.text)))
    .flatMap((body) => body.resourceLogs)
    .flatMap((resourceLogs) => resourceLogs.scopeLogs)
    .reduce((total, scope) => total + scope.logRecords.length, 0);
}

export function hidePage() {
  globalThis.dispatchEvent(
    new PageTransitionEvent("pagehide", { persisted: true }),
  );
}

export function showPage() {
  globalThis.dispatchEvent(
    new PageTransitionEvent("pageshow", { persisted: true }),
  );
}

export const ok = () => new HttpResponse(null);
