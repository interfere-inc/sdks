import { browserStorage } from "@interfere/test-config/browser-storage";
import { HttpResponse, http } from "msw";
import { addBaselineHandlers } from "../../test/msw";
import { ingestHandlers } from "../../test/fixtures/kernel";
import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { context, propagation, trace } from "@opentelemetry/api";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace";
import { afterEach, beforeAll, expect, onTestFinished, test, vi } from "vitest";

import { DISABLE_ALL_PLUGINS } from "../../test/fixtures/disable-all-plugins.js";
import { createKernel } from "./kernel.js";

beforeAll(() => {
  addBaselineHandlers(
    http.get("https://api.interfere.com/v3/config", () =>
      HttpResponse.json({ plugins: {} }),
    ),
    ...ingestHandlers("https://api.interfere.com"),
  );
});

const deviceId = "00000000-0000-4000-8000-000000000001";

afterEach(async () => {
  trace.disable();

  context.disable();

  propagation.disable();

  vi.unstubAllGlobals();

  await cookieStore.delete({ name: "interfere_did", path: "/" });
});

test.each([
  {
    consent: "required" as const,
    enabled: true,
    state: { analytics: false, replay: false },
  },
  {
    consent: undefined,
    enabled: true,
    state: { analytics: true, replay: true },
  },
])(
  "resolves startup device identity with consent $consent and device enabled $enabled",
  async ({ consent, enabled, state }) => {
    setInjectedConfiguration("__INTERFERE_PLUGINS__", {
      ...DISABLE_ALL_PLUGINS,
      device: enabled,
    });

    browserStorage.local.set("interfere:device_id", deviceId);

    const kernel = await createKernel({ opts: { consent, tracing: false } });

    onTestFinished(async () => {
      await kernel.dispose();
    });

    expect(kernel.device.getDeviceId()).toBeNull();
    expect(kernel.isDeviceInfoCaptureEnabled()).toBe(false);

    expect(kernel.consent.get()).toStrictEqual(state);
  },
);

test("attaches the saved device to spans created during tracing startup", async () => {
  setInjectedConfiguration("__INTERFERE_PLUGINS__", {
    ...DISABLE_ALL_PLUGINS,
    device: true,
  });

  browserStorage.local.set("interfere:device_id", deviceId);

  const processor = new SimpleSpanProcessor({
    exporter: new InMemorySpanExporter(),
  });

  const started = vi.spyOn(processor, "onStart");

  const kernel = await createKernel({
    onCreated: (created) => {
      created.consent.set({ replay: false });
    },
    opts: { _internalAdditionalSpanProcessors: [processor] },
  });

  kernel.getTracer("startup").startSpan("startup-work").end();

  onTestFinished(async () => {
    await kernel.dispose();
  });

  kernel.syncProviderConfig({});

  expect(kernel.consent.get()).toStrictEqual({
    analytics: true,
    replay: false,
  });

  kernel.consent.set(false);

  expect(kernel.device.getDeviceId()).toBeNull();

  kernel.syncProviderConfig({});

  expect(kernel.consent.get()).toStrictEqual({
    analytics: false,
    replay: false,
  });

  expect(started.mock.calls.length).toBeGreaterThan(0);
  expect(started.mock.calls[0]?.[0].attributes["device.id"]).toBe(deviceId);
});

test("stamps device attributes on exceptions captured before the provider settles consent", async () => {
  setInjectedConfiguration("__INTERFERE_PLUGINS__", {
    ...DISABLE_ALL_PLUGINS,
    device: true,
    errors: true,
  });

  browserStorage.local.set("interfere:device_id", deviceId);

  const exporter = new InMemorySpanExporter();

  const kernel = await createKernel({
    opts: {
      _internalAdditionalSpanProcessors: [
        new SimpleSpanProcessor({ exporter }),
      ],
    },
  });

  onTestFinished(async () => {
    await kernel.dispose();
  });

  kernel.recordException(
    {
      kind: "non-error",
      serialized: { url: "https://cdn.app.test/gtm.js" },
      type: "ResourceLoadError",
      value: "Failed to load script: https://cdn.app.test/gtm.js",
    },
    { mechanism: { handled: false, type: "auto.browser.resource.error" } },
  );

  kernel.syncProviderConfig({});

  const spans = exporter
    .getFinishedSpans()
    .filter(({ name }) => name === "ResourceLoadError");

  expect(spans.map(({ attributes }) => attributes)).toStrictEqual([
    expect.objectContaining({
      "device.id": deviceId,
      "user_agent.original": navigator.userAgent,
    }),
  ]);
});

async function createHeldKernel() {
  setInjectedConfiguration("__INTERFERE_PLUGINS__", {
    ...DISABLE_ALL_PLUGINS,
    errors: true,
  });

  const processor = new SimpleSpanProcessor({
    exporter: new InMemorySpanExporter(),
  });

  const ended = vi.spyOn(processor, "onEnd");

  const kernel = await createKernel({
    opts: { _internalAdditionalSpanProcessors: [processor] },
  });

  const record = (index: number) => {
    kernel.recordException(new Error(`held ${String(index)}`), {
      mechanism: { handled: true, type: "manual.capture" },
    });
  };

  const exported = () =>
    ended.mock.calls
      .flatMap(([span]) => span.events)
      .filter(({ name }) => name === "exception")
      .map(({ attributes }) => attributes?.["exception.message"]);

  return { exported, kernel, record };
}

test("exports every exception of a storm captured while startup waits for consent", async () => {
  const { exported, kernel, record } = await createHeldKernel();

  onTestFinished(async () => {
    await kernel.dispose();
  });

  const messages = Array.from({ length: 120 }, (_, index) => {
    record(index);

    return `held ${String(index)}`;
  });

  kernel.syncProviderConfig({});

  expect(exported()).toStrictEqual(messages);
});

test("exports exceptions held for consent when the kernel is disposed", async () => {
  const { exported, kernel, record } = await createHeldKernel();

  record(0);

  expect(exported()).toStrictEqual([]);

  await kernel.dispose();

  expect(exported()).toStrictEqual(["held 0"]);
});

test("restores the saved device only once required consent is granted", async () => {
  setInjectedConfiguration("__INTERFERE_PLUGINS__", {
    ...DISABLE_ALL_PLUGINS,
    device: true,
  });

  browserStorage.local.set("interfere:device_id", deviceId);

  const kernel = await createKernel({
    opts: { consent: "required", tracing: false },
  });

  onTestFinished(async () => {
    await kernel.dispose();
  });

  expect(kernel.device.getDeviceId()).toBeNull();

  kernel.consent.set(true);

  await expect.poll(() => kernel.device.getDeviceId()).toBe(deviceId);
});
