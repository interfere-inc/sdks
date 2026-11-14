import { context, propagation } from "@opentelemetry/api";
import type { SpanProcessor } from "@opentelemetry/sdk-trace-base";
import { HttpResponse, http } from "msw";
import { onTestFinished } from "vitest";

import { Kernel } from "../../src/internal/kernel.js";
import { claimTracingOwnership } from "../../src/internal/otel/ownership.js";
import { buildOtelProvider } from "../../src/internal/otel/provider.js";
import { PluginRuntime } from "../../src/internal/plugins/plugin-runtime.js";
import { SessionTracker } from "../../src/tracking/api.js";
import { addBaselineHandlers } from "../msw.js";

const target = {
  baseUrl: "https://plugin-context.invalid",
  publicKeyHeader: { apiKey: "test-key" },
};

addBaselineHandlers(
  http.post(`${target.baseUrl}/v3/session`, () => HttpResponse.json({})),
  http.post(
    `${target.baseUrl}/drain/v1/traces`,
    () => new HttpResponse(null, { status: 204 }),
  ),
);

export function createPluginContext(
  additionalSpanProcessors: SpanProcessor[] = [],
) {
  const tracker = new SessionTracker({ target });
  const kernel = new Kernel(tracker);
  const runtime = new PluginRuntime(kernel);

  kernel.attachRuntime(runtime);

  runtime.applyRemoteConfigDefaults();

  const otel = buildOtelProvider({
    additionalSpanProcessors,
    getDeviceId: () => kernel.device.getDeviceId(),
    getSessionId: () => kernel.session.getId(),
    clientOptions: target,
    sdkStack: [],
  });

  const ownership = claimTracingOwnership({
    contextManager: otel.contextManager,
    mode: "managed",
    propagator: otel.propagator,
  });

  kernel.attachOtel(otel, () => {
    ownership.release();
  });

  tracker.start();

  onTestFinished(async () => {
    try {
      await kernel.dispose();
    } finally {
      context.disable();

      propagation.disable();
    }
  });

  return kernel;
}
