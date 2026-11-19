import { browserStorage } from "@interfere/test-config/browser-storage";
import { DISABLE_ALL_PLUGINS } from "../../../test/fixtures/disable-all-plugins.js";
import { worker } from "../../../test/msw.js";
import { SessionTracker } from "../../tracking/api.js";
import { DeviceManager } from "../../tracking/device.js";
import { Kernel } from "../kernel.js";
import { PluginRuntime } from "./plugin-runtime.js";
import { HttpResponse, http } from "msw";
import { expect, test, vi } from "vitest";

test("keeps analytics dark until provider configuration settles, then activates it", async () => {
  const requests: string[] = [];

  worker.use(
    http.get("https://cloudflare.com/cdn-cgi/trace", ({ request }) => {
      requests.push(`${request.method} ${request.url}`);

      return HttpResponse.text("loc=US\n");
    }),
  );

  const tracker = new SessionTracker({
    device: new DeviceManager(),
    target: {
      baseUrl: "https://plugins.invalid",
      publicKeyHeader: { apiKey: "test-key" },
    },
  });

  const kernel = new Kernel(tracker);
  const runtime = new PluginRuntime(kernel);

  kernel.attachRuntime(runtime);

  onTestFinished(async () => {
    try {
      await kernel.dispose();
    } finally {
      await cookieStore.delete("interfere_did");
    }
  });

  runtime.start();

  runtime.applyRemoteConfigDefaults();

  await vi.dynamicImportSettled();

  expect(runtime.getPluginConfigState("replay")).toBe("enabled");
  expect(kernel.device.getDeviceId()).toBeNull();
  expect(localStorage.getItem("interfere:device_id")).toBeNull();
  expect(requests).toStrictEqual([]);

  runtime.applyProviderConfig({ ...DISABLE_ALL_PLUGINS, device: true });

  await expect
    .poll(() => kernel.device.getDeviceId())
    .toEqual(expect.any(String));

  const id = kernel.device.getDeviceId();

  expect(localStorage.getItem("interfere:device_id")).toBe(id);

  expect(browserStorage.cookies.get("interfere_did")).toBe(id);

  await vi.dynamicImportSettled();

  expect(requests).toStrictEqual([]);
});
