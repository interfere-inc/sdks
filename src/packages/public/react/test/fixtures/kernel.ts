import { Kernel } from "../../src/internal/kernel.js";
import { PluginRuntime } from "../../src/internal/plugins/plugin-runtime.js";
import type { PluginOverrides } from "../../src/plugins/lib/loader.js";
import { SessionTracker } from "../../src/tracking/api.js";
import { DeviceManager } from "../../src/tracking/device.js";
import { HttpResponse, http } from "msw";

import { addBaselineHandlers } from "../msw.js";

export function ingestHandlers(origin: string) {
  return [
    ...["/v3/session", "/v3/session/identify"].map((path) =>
      http.post(`${origin}${path}`, () => new HttpResponse("ok")),
    ),
    http.post(`${origin}/v3/replay/upload/:id`, () =>
      HttpResponse.json({ ok: true }),
    ),
    ...["traces", "logs", "metrics"].map((signal) =>
      http.post(
        `${origin}/drain/v1/${signal}`,
        () => new HttpResponse(null, { status: 204 }),
      ),
    ),
  ];
}

addBaselineHandlers(...ingestHandlers("https://test.local"));

interface BuildKernelInput {
  pluginOverrides?: PluginOverrides;
}

export function buildKernel(input: BuildKernelInput = {}) {
  const tracker = new SessionTracker({
    device: new DeviceManager(),
    target: {
      baseUrl: "https://test.local",
      publicKeyHeader: { apiKey: "test-key" },
    },
  });

  const kernel = new Kernel(tracker);

  const runtime = new PluginRuntime(kernel);

  kernel.attachRuntime(runtime);

  runtime.applyProviderConfig(input.pluginOverrides);

  return { kernel, runtime, tracker };
}
