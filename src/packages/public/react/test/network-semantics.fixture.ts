import { context, propagation } from "@opentelemetry/api";
import { W3CTraceContextPropagator } from "@opentelemetry/core";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import type { ReadableSpan } from "@opentelemetry/sdk-trace-base";
import {
  StackContextManager,
  WebTracerProvider,
} from "@opentelemetry/sdk-trace-web";
import { registerBundledInstrumentations } from "../src/internal/otel/instrumentations/instrumentations.js";
import type { BundledInstrumentations } from "../src/internal/otel/instrumentations/instrumentations.js";
import { w3cTraceHeaders } from "./trace-headers.test-utils.js";
import { createCountSignal } from "./fixtures/upload-signal.js";

let handle: BundledInstrumentations | undefined;
let provider: WebTracerProvider | undefined;
let exporter = new InMemorySpanExporter();
const suspectCorsSpans: ReadableSpan[] = [];
const suspectCorsSignal = createCountSignal(suspectCorsSpans);

class ObservedSpanProcessor extends SimpleSpanProcessor {
  override onEnd(span: ReadableSpan) {
    super.onEnd(span);

    if (span.attributes["interfere.propagation.suspect_cors"] === true) {
      suspectCorsSpans.push(span);

      suspectCorsSignal.notify();
    }
  }
}

const fixture = {
  async suspectCors(count: number) {
    await suspectCorsSignal.reached(count);

    return suspectCorsSpans.length;
  },
  async install(
    options: {
      backendHosts?: string[];
      propagateTo?: string[];
      captureFailedRequests?: boolean;
    } = {},
  ) {
    handle?.dispose();

    await provider?.shutdown();

    exporter = new InMemorySpanExporter();

    suspectCorsSpans.length = 0;

    context.disable();

    propagation.disable();

    context.setGlobalContextManager(new StackContextManager().enable());

    propagation.setGlobalPropagator(new W3CTraceContextPropagator());

    provider = new WebTracerProvider({
      spanProcessors: [new ObservedSpanProcessor(exporter)],
    });

    handle = registerBundledInstrumentations({
      ...options,
      ignoreUrls: [],
      traceHeaders: w3cTraceHeaders(),
      tracerProvider: provider,
    });
  },
  dispose() {
    handle?.dispose();
  },
  spans() {
    return exporter.getFinishedSpans().map((span) => ({
      name: span.name,
      attributes: span.attributes,
      events: span.events,
    }));
  },
};

declare global {
  var networkSemanticsFixture: typeof fixture;
}

globalThis.networkSemanticsFixture = fixture;
