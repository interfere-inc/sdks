/* oxlint-disable landmines/no-cold-import-in-test -- Each test evaluates the side-effect entry under its own globals; beforeAll warms the ./instrument-client.js graph the entry loads. */
import { BUILD_MARKER_GLOBAL } from "@interfere/types/sdk/injected-globals";

import { setTimeout as sleep } from "node:timers/promises";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

await import("./instrument-client.js");

function clearBuildMarker() {
  Reflect.deleteProperty(globalThis, BUILD_MARKER_GLOBAL);
}

async function flushInitDeferral() {
  await sleep(0);

  await sleep(0);
}

beforeEach(() => {
  vi.resetModules();

  clearBuildMarker();
});

afterEach(() => {
  vi.restoreAllMocks();

  vi.unstubAllEnvs();

  clearBuildMarker();
});

describe("side-effect entry evaluated before the loader-prepended build marker", () => {
  test("contains boot failures in development", async () => {
    vi.stubEnv("NODE_ENV", "development");

    const error = new Error("Cannot read build marker");
    const warn = vi.spyOn(console, "warn");

    Object.defineProperty(globalThis, BUILD_MARKER_GLOBAL, {
      configurable: true,
      get() {
        throw error;
      },
    });

    await import("./instrumentation-client.js");

    await flushInitDeferral();

    expect(warn.mock.calls).toStrictEqual([
      ["[interfere] init() threw during boot", error],
    ]);
  });

  test("contains boot failures in production", async () => {
    vi.stubEnv("NODE_ENV", "production");

    const error = new Error("Cannot read build marker");
    const warn = vi.spyOn(console, "warn");

    Object.defineProperty(globalThis, BUILD_MARKER_GLOBAL, {
      configurable: true,
      get() {
        throw error;
      },
    });

    await import("./instrumentation-client.js");

    await flushInitDeferral();

    expect(warn.mock.calls).toStrictEqual([]);
  });

  test("does not warn when the build marker lands after hoisted imports", async () => {
    const warn = vi.spyOn(console, "warn");

    await import("../test/fixtures/instrumentation-client-hoisted.js");

    await flushInitDeferral();

    expect(warn).not.toHaveBeenCalled();
  });

  test("warns and stays inactive when no build ran and no env credentials exist", async () => {
    vi.stubEnv("INTERFERE_PUBLIC_KEY", "");

    vi.stubEnv("NEXT_PUBLIC_INTERFERE_FORCE_ENABLE", "");

    const warn = vi.spyOn(console, "warn");

    await import("./instrumentation-client.js");

    await flushInitDeferral();

    expect(warn).toHaveBeenCalledTimes(1);
    expect(String(warn.mock.calls[0]?.[0])).toContain("inactive");
  });
});
