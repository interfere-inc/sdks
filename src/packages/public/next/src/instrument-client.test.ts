import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import {
  BUILD_MARKER_GLOBAL,
  DISABLED_GLOBAL,
} from "@interfere/types/sdk/injected-globals";

import { getKernelOrNull, init } from "./instrument-client.js";
import { afterEach, describe, expect, test, vi } from "vitest";

afterEach(() => {
  vi.restoreAllMocks();

  vi.unstubAllEnvs();

  vi.unstubAllGlobals();
});

describe("init when the build integration did not run", () => {
  test("warns, resolves to null, and never boots the kernel", async () => {
    setInjectedConfiguration(BUILD_MARKER_GLOBAL, undefined);

    vi.stubEnv("NEXT_PUBLIC_INTERFERE_FORCE_ENABLE", "");

    const warn = vi.spyOn(console, "warn");

    const kernel = await init();

    expect(kernel).toBeNull();
    expect(warn).toHaveBeenCalledTimes(1);
    expect(String(warn.mock.calls[0]?.[0])).toContain("inactive");
    expect(getKernelOrNull()).toBeNull();
  });

  test("accepts a marker set during init deferral without warning while explicitly disabled", async () => {
    setInjectedConfiguration(BUILD_MARKER_GLOBAL, undefined);

    const warn = vi.spyOn(console, "warn");
    const pending = init({ enabled: false });

    setInjectedConfiguration(BUILD_MARKER_GLOBAL, true);

    const kernel = await pending;

    expect(kernel).toBeNull();
    expect(warn).not.toHaveBeenCalled();
  });
});

describe("init when the build disabled Interfere", () => {
  test("resolves to null without warning, even with force enable", async () => {
    setInjectedConfiguration(BUILD_MARKER_GLOBAL, true);

    setInjectedConfiguration(DISABLED_GLOBAL, true);

    vi.stubEnv("NEXT_PUBLIC_INTERFERE_FORCE_ENABLE", "1");

    const warn = vi.spyOn(console, "warn");

    await expect(init()).resolves.toBeNull();

    expect(getKernelOrNull()).toBeNull();
    expect(warn).not.toHaveBeenCalled();
  });
});
