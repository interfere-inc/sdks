import { StackContextManager } from "@opentelemetry/sdk-trace";
import { context, propagation } from "@opentelemetry/api";
import type { SpanProcessor } from "@opentelemetry/sdk-trace";
import { HttpResponse, http } from "msw";
import { onTestFinished } from "vitest";

import { Kernel } from "../../src/internal/kernel.js";
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

  context.setGlobalContextManager(new StackContextManager().enable());

  kernel.attachOtel(otel, () => {
    context.disable();
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
