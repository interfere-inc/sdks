import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
  StackContextManager,
} from "@opentelemetry/sdk-trace";
import { context, propagation } from "@opentelemetry/api";
import { HttpResponse, http } from "msw";
import { onTestFinished, vi } from "vitest";

import { Kernel } from "../../src/internal/kernel.js";
import { buildOtelProvider } from "../../src/internal/otel/provider.js";
import errorsPlugin from "../../src/plugins/errors.js";
import { SessionTracker } from "../../src/tracking/api.js";
import { addBaselineHandlers } from "../msw.js";

const target = {
  baseUrl: "https://errors.invalid",
  releaseTokenHeader: { apiKey: "test-key" },
};

addBaselineHandlers(
  http.post(
    "https://errors.invalid/drain/v1/traces",
    () => new HttpResponse(null, { status: 204 }),
  ),
);

export function createErrorsKernel() {
  vi.spyOn(console, "error");

  const kernel = new Kernel(new SessionTracker({ target }));

  const exporter = new InMemorySpanExporter();

  const handle = buildOtelProvider({
    additionalSpanProcessors: [new SimpleSpanProcessor({ exporter })],

    getDeviceId: () => null,
    getSessionId: () => null,
    getViewId: () => kernel.views.viewId(),
    getActionId: () => kernel.views.actionId(),
    clientOptions: target,
    sdkStack: [],
  });

  context.setGlobalContextManager(new StackContextManager().enable());

  kernel.attachOtel(handle, () => {
    context.disable();
  });

  onTestFinished(async () => {
    try {
      await kernel.dispose();
    } finally {
      context.disable();

      propagation.disable();
    }
  });

  return { kernel, exporter, handle };
}

export function setupErrorsPlugin() {
  const harness = createErrorsKernel();

  let cleanup = errorsPlugin.setup(harness.kernel);

  const stop = () => {
    cleanup?.();

    cleanup = undefined;
  };

  onTestFinished(stop);

  return { ...harness, stop };
}
