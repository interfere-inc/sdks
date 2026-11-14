import { browserStorage } from "@interfere/test-config/browser-storage";
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
import { afterEach, beforeEach, vi } from "vitest";

import { w3cTraceHeaders } from "../../../../test/trace-headers.test-utils.js";
import type { ActivityMonitor } from "../../activity/activity.js";
import { registerBundledInstrumentations } from "./instrumentations.js";
import type {
  BundledInstrumentations,
  InstrumentationsInput,
} from "./instrumentations.js";

export const ALLOWED = "https://allowed.cors.test";

export const FOREIGN =
  "00-12345678901234567890123456789012-1234567890123456-01";

/** Registers the propagation instrumentations against a fresh private provider per test, resolves the first suspect-CORS span, resets the OpenTelemetry globals and clears the origin memo between tests. */
export function propagationHarness() {
  let exporter = new InMemorySpanExporter();
  let provider: WebTracerProvider | undefined;
  let instrumentations: BundledInstrumentations | undefined;
  let activity: ActivityMonitor | undefined;
  let failedRequests = Promise.withResolvers<ReadableSpan[]>();

  beforeEach(() => {
    browserStorage.session.remove("interfere:propagation_proven");

    browserStorage.session.remove("interfere:propagation_denied");

    exporter = new InMemorySpanExporter();

    failedRequests = Promise.withResolvers<ReadableSpan[]>();

    const failures: ReadableSpan[] = [];

    provider = new WebTracerProvider({
      spanProcessors: [
        new SimpleSpanProcessor(exporter),
        {
          onStart: () => undefined,
          onEnd: (span) => {
            if (
              span.attributes["interfere.propagation.suspect_cors"] === true
            ) {
              failures.push(span);

              if (failures.length === 1) {
                failedRequests.resolve(failures);
              }
            }
          },
          // oxlint-disable-next-line typescript/promise-function-async -- This observation processor has no asynchronous work to drain.
          forceFlush: () => Promise.resolve(),
          // oxlint-disable-next-line typescript/promise-function-async -- This observation processor owns no asynchronous resources.
          shutdown: () => Promise.resolve(),
        },
      ],
    });

    propagation.setGlobalPropagator(new W3CTraceContextPropagator());

    context.setGlobalContextManager(new StackContextManager().enable());

    vi.spyOn(console, "warn");
  });

  afterEach(async () => {
    activity?.stop();

    activity = undefined;

    vi.useRealTimers();

    instrumentations?.dispose();

    instrumentations = undefined;

    await provider?.shutdown();

    context.disable();

    propagation.disable();
  });

  return {
    exporter: () => exporter,
    failedRequests: async () => failedRequests.promise,
    install(options: Partial<InstrumentationsInput> = {}) {
      if (!provider) {
        throw new Error("install() runs inside a test");
      }

      instrumentations = registerBundledInstrumentations({
        ignoreUrls: [],
        tracerProvider: provider,
        propagateTo: [ALLOWED],
        traceHeaders: w3cTraceHeaders(),
        ...options,
      });

      return instrumentations;
    },
    instrumentations: () => instrumentations,
    watch(monitor: ActivityMonitor) {
      activity = monitor;
    },
  };
}
