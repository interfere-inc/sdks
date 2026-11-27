import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { BUILD_MARKER_GLOBAL } from "@interfere/types/sdk/injected-globals";

import {
  buildIntegrationRan,
  warnBuildIntegrationMissing,
} from "./build-marker-guard.js";
import { afterEach, describe, expect, test, vi } from "vitest";

afterEach(() => {
  vi.restoreAllMocks();

  vi.unstubAllEnvs();

  vi.unstubAllGlobals();
});

describe(buildIntegrationRan, () => {
  test("returns false when the build marker global is absent", () => {
    setInjectedConfiguration(BUILD_MARKER_GLOBAL, undefined);

    expect(buildIntegrationRan()).toBe(false);
  });

  test("returns true when the build marker global was baked into the build", () => {
    setInjectedConfiguration(BUILD_MARKER_GLOBAL, true);

    expect(buildIntegrationRan()).toBe(true);
  });

  test.each([false, null, 0, "", {}, []])(
    "treats defined marker %j as a completed build",
    (value) => {
      setInjectedConfiguration(BUILD_MARKER_GLOBAL, value);

      expect(buildIntegrationRan()).toBe(true);
    },
  );

  test("returns false when the marker property does not exist", () => {
    setInjectedConfiguration(BUILD_MARKER_GLOBAL, undefined);

    Reflect.deleteProperty(globalThis, BUILD_MARKER_GLOBAL);

    expect(buildIntegrationRan()).toBe(false);
  });
});

describe(warnBuildIntegrationMissing, () => {
  test("warns once naming withInterfere", () => {
    const warn = vi.spyOn(console, "warn");

    warnBuildIntegrationMissing();

    expect(warn).toHaveBeenCalledTimes(1);
    expect(String(warn.mock.calls[0]?.[0])).toContain("withInterfere");
  });
});
