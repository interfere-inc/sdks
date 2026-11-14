import {
  BasicTracerProvider,
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import { Kernel } from "../src/internal/kernel.js";
import { SessionTracker } from "../src/tracking/api.js";
import { clearExportCarryover } from "../src/internal/otel/export-carryover.js";
import { createReplayCarryover } from "../src/plugins/replay/upload/carryover.js";
import { deliverChunk } from "../src/plugins/replay/upload/delivery.js";

const query = new URL(location.href).searchParams;
let allowed = query.get("allowed") !== "false";

const target = {
  baseUrl: query.get("destination") ?? location.origin,
  publicKeyHeader: { apiKey: query.get("key") ?? "replay-public-key" },
  headers: {},
  timeoutInSeconds: 30,
  maxRetries: 0,
};

const ctx = new Kernel(new SessionTracker({ target }));
const exporter = new InMemorySpanExporter();

const provider = new BasicTracerProvider({
  spanProcessors: [new SimpleSpanProcessor(exporter)],
});

ctx.getTracer = (name) => provider.getTracer(name);

const carryover = createReplayCarryover(target, () => allowed);
let bodyJson = "[]";
let lastResult = "";

const fixture = {
  result: () => lastResult,
  async send(
    options: { body?: string; sessionId?: string; segmentId?: number } = {},
  ) {
    lastResult = await deliverChunk({
      target,
      carryover,
      ctx,
      compressor: { gzip: null, dispose: () => undefined },
      sessionId: options.sessionId ?? "original-session",
      chunk: { windowId: "original-window", segmentId: options.segmentId ?? 7 },
      payload: {
        bodyJson: options.body ?? bodyJson,
        activityCount: 0,
        eventCount: 1,
        firstTs: 123,
        lastTs: 123,
        hasFullSnapshot: true,
      },
      unload: true,
    });

    return exporter
      .getFinishedSpans()
      .filter((span) => span.name === "replay.chunk")
      .map((span) => span.attributes);
  },
  prepare(rows = 5000) {
    const values = Array.from({ length: rows }, () => crypto.randomUUID());

    bodyJson = JSON.stringify({ type: 2, timestamp: 123, data: values });

    window.addEventListener("beforeunload", () => {
      void deliverChunk({
        target,
        carryover,
        ctx,
        compressor: { gzip: null, dispose: () => undefined },
        sessionId: "original-session",
        chunk: { windowId: "original-window", segmentId: 7 },
        payload: {
          bodyJson,
          activityCount: 0,
          eventCount: 1,
          firstTs: 123,
          lastTs: 123,
          hasFullSnapshot: true,
        },
        unload: true,
      });
    });

    return bodyJson;
  },
  deny() {
    allowed = false;

    clearExportCarryover("replay:");
  },
};

declare global {
  var replayCarryoverFixture: typeof fixture;
}

globalThis.replayCarryoverFixture = fixture;
