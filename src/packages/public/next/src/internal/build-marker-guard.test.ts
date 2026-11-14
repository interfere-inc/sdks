import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { BUILD_MARKER_GLOBAL } from "@interfere/types/sdk/injected-globals";

import {
  buildIntegrationRan,
  hasEnvCredentials,
  warnBuildIntegrationDegraded,
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

describe(hasEnvCredentials, () => {
  test("returns false when neither credential env var is set", () => {
    vi.stubEnv("INTERFERE_PUBLIC_KEY", "");

    vi.stubEnv("NEXT_PUBLIC_INTERFERE_FORCE_ENABLE", "");

    expect(hasEnvCredentials()).toBe(false);
  });

  test("returns true when INTERFERE_PUBLIC_KEY is set", () => {
    vi.stubEnv("INTERFERE_PUBLIC_KEY", "pk_test_123");

    expect(hasEnvCredentials()).toBe(true);
  });

  test("ignores the removed NEXT_PUBLIC_INTERFERE_FORCE_ENABLE setting", () => {
    vi.stubEnv("INTERFERE_PUBLIC_KEY", "");

    vi.stubEnv("NEXT_PUBLIC_INTERFERE_FORCE_ENABLE", "1");

    expect(hasEnvCredentials()).toBe(false);
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

describe(warnBuildIntegrationDegraded, () => {
  test("warns once naming withInterfere and the missing capabilities", () => {
    const warn = vi.spyOn(console, "warn");

    warnBuildIntegrationDegraded();

    expect(warn).toHaveBeenCalledTimes(1);
    expect(String(warn.mock.calls[0]?.[0])).toContain("withInterfere");
    expect(String(warn.mock.calls[0]?.[0])).toContain("sourcemaps");
  });
});
