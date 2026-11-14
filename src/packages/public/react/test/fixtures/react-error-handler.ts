import { context, propagation } from "@opentelemetry/api";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";

import { Kernel } from "../../src/internal/kernel.js";
import { buildOtelProvider } from "../../src/internal/otel/provider.js";
import { SessionTracker } from "../../src/tracking/api.js";
import { HttpResponse, http } from "msw";
import { onTestFinished } from "vitest";

import { addBaselineHandlers } from "../msw.js";

const TARGET = {
  baseUrl: "https://sdk.invalid",
  publicKeyHeader: { apiKey: "test-key" },
};

addBaselineHandlers(
  http.post(
    `${TARGET.baseUrl}/drain/v1/traces`,
    () => new HttpResponse(null, { status: 204 }),
  ),
);

export function createTracedKernel() {
  const exporter = new InMemorySpanExporter();
  const kernel = new Kernel(new SessionTracker({ target: TARGET }));

  const handle = buildOtelProvider({
    additionalSpanProcessors: [new SimpleSpanProcessor(exporter)],
    clientOptions: TARGET,
    getDeviceId: () => kernel.device.getDeviceId(),
    getSessionId: () => kernel.session.getId(),
    sdkStack: [],
  });

  kernel.attachOtel(handle, () => handle.contextManager.disable());

  onTestFinished(async () => {
    try {
      await kernel.dispose();
    } finally {
      context.disable();

      propagation.disable();
    }
  });

  return { exporter, kernel };
}
