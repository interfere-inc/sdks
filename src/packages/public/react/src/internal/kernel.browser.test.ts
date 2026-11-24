import { context, propagation, trace } from "@opentelemetry/api";
import type { TracerProvider } from "@opentelemetry/sdk-trace";

import { DISABLE_ALL_PLUGINS } from "../../test/fixtures/disable-all-plugins.js";
import { buildKernel, ingestHandlers } from "../../test/fixtures/kernel.js";
import type { createKernel } from "./kernel.js";
import { HttpResponse, http } from "msw";
import {
  afterEach,
  beforeAll,
  beforeEach,
  describe,
  expect,
  test,
  vi,
} from "vitest";

import { addBaselineHandlers } from "../../test/msw.js";

beforeAll(() => {
  addBaselineHandlers(
    http.get("https://api.interfere.com/v3/config", () =>
      HttpResponse.json({ plugins: {} }),
    ),
    ...ingestHandlers("https://api.interfere.com"),
    ...ingestHandlers("https://in.example.com"),
  );
});

const ownedKernels = new Set<Awaited<ReturnType<typeof createKernel>>>();

const providers = new Set<TracerProvider>();

let priorUrl: string;

let priorHistory: unknown;

let priorCookie: Awaited<ReturnType<CookieStore["get"]>>;

beforeEach(async () => {
  priorUrl = globalThis.location.href;

  priorHistory = globalThis.history.state;

  priorCookie = await cookieStore.get("interfere_did");
});

afterEach(async () => {
  try {
    const results = await Promise.allSettled([
      ...Array.from(ownedKernels, async (kernel) => kernel.dispose()),
      ...Array.from(providers, async (provider) => provider.shutdown()),
    ]);

    const failures = results.filter((result) => result.status === "rejected");

    if (failures.length > 0) {
      throw new AggregateError(
        failures.map((result) => {
          const decodedValue: unknown = result.reason;

          return decodedValue;
        }),
        "Kernel cleanup failed",
      );
    }
  } finally {
    ownedKernels.clear();

    providers.clear();

    context.disable();

    trace.disable();

    propagation.disable();

    globalThis.history.replaceState(priorHistory, "", priorUrl);

    await cookieStore.delete("interfere_did");

    if (priorCookie?.name && priorCookie.value !== undefined) {
      await cookieStore.set({
        ...priorCookie,
        name: priorCookie.name,
        value: priorCookie.value,
      });
    }

    vi.unstubAllGlobals();
  }
});

describe("Kernel", () => {
  test("exposes consent/identity/device/session accessors that delegate to collaborators", () => {
    const { kernel, tracker } = buildKernel({
      pluginOverrides: DISABLE_ALL_PLUGINS,
    });

    ownedKernels.add(kernel);

    tracker.start();

    expect(kernel.session.getId()).toMatch(/\S/u);
    expect(kernel.session.getWindowId()).toMatch(/\S/u);
    expect(kernel.device.getDeviceId()).toBeNull();

    kernel.startDeviceInfoCapture();

    expect(kernel.device.getDeviceId()).toMatch(/\S/u);
    expect(kernel.identity.get()).toBeNull();

    expect(kernel.consent.get()).toStrictEqual({
      analytics: true,
      replay: true,
    });
  });

  test("device capture cleanup purges stored identity only on consent revocation", () => {
    const { kernel, tracker } = buildKernel({
      pluginOverrides: DISABLE_ALL_PLUGINS,
    });

    ownedKernels.add(kernel);

    tracker.start();

    const stop = kernel.startDeviceInfoCapture();
    const deviceId = kernel.device.getDeviceId();

    expect(deviceId).toMatch(/\S/u);
    expect(document.cookie).toContain("interfere_did=");
    expect(localStorage.getItem("interfere:device_id")).toBe(deviceId);

    stop("disabled");

    expect(kernel.device.getDeviceId()).toBe(deviceId);
    expect(localStorage.getItem("interfere:device_id")).toBe(deviceId);

    kernel.startDeviceInfoCapture()("consent-revoked");

    expect(kernel.device.getDeviceId()).toBeNull();
    expect(localStorage.getItem("interfere:device_id")).toBeNull();
    expect(document.cookie).not.toContain("interfere_did=");
  });

  test("clears identity and rotates the session through identity.clear", async () => {
    const { kernel, tracker } = buildKernel();

    ownedKernels.add(kernel);

    tracker.start();

    const initialSessionId = kernel.session.getId();

    await kernel.identity.set({
      identifier: "kernel-clear",
      source: { name: "Test", type: "custom" },
    });

    kernel.identity.clear();

    expect(kernel.identity.get()).toBeNull();
    expect(kernel.session.getId()).toMatch(/\S/u);
    expect(kernel.session.getId()).not.toBe(initialSessionId);
  });

  test("dispose cleans up the tracker", async () => {
    const { kernel, tracker } = buildKernel();

    ownedKernels.add(kernel);

    tracker.start();

    expect(kernel.session.getId()).toMatch(/\S/u);

    await kernel.dispose();

    expect(kernel.session.getId()).toBeNull();
  });
});
