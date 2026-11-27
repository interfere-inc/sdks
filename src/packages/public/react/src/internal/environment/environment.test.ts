import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { afterEach, expect, test, vi } from "vitest";
import { isEnabledByEnvironment } from "./environment.js";

afterEach(() => {
  vi.unstubAllEnvs();

  vi.unstubAllGlobals();
});

test.each(["development", "test", "production"])(
  "enables a baked release token in %s",
  (environment) => {
    vi.stubEnv("NODE_ENV", environment);

    setInjectedConfiguration("__INTERFERE_RELEASE_TOKEN__", "pk_test_key");

    expect(isEnabledByEnvironment()).toBe(true);
  },
);

test("ignores removed force-enable environment flags without a release token", () => {
  vi.stubEnv("INTERFERE_FORCE_ENABLE", "1");

  vi.stubEnv("NODE_ENV", "development");

  setInjectedConfiguration("__INTERFERE_FORCE_ENABLE__", false);

  setInjectedConfiguration("__INTERFERE_RELEASE_TOKEN__", undefined);

  expect(isEnabledByEnvironment()).toBe(false);
});
