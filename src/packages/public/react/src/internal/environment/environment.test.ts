import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { afterEach, expect, test, vi } from "vitest";
import { isEnabledByEnvironment } from "./environment.js";

afterEach(() => {
  vi.unstubAllEnvs();

  vi.unstubAllGlobals();
});

test.each(["development", "test", "production"])(
  "enables a configured public key in %s",
  (environment) => {
    vi.stubEnv("NODE_ENV", environment);

    vi.stubEnv("INTERFERE_PUBLIC_KEY", undefined);

    setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "pk_test_key");

    expect(isEnabledByEnvironment()).toBe(true);
  },
);

test("enables a server environment public key in development", () => {
  vi.stubEnv("NODE_ENV", "development");

  vi.stubEnv("INTERFERE_PUBLIC_KEY", "pk_test_key");

  setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", undefined);

  expect(isEnabledByEnvironment()).toBe(true);
});

test("ignores removed force-enable environment flags without a public key", () => {
  vi.stubEnv("INTERFERE_PUBLIC_KEY", undefined);

  vi.stubEnv("INTERFERE_FORCE_ENABLE", "1");

  vi.stubEnv("NODE_ENV", "development");

  setInjectedConfiguration("__INTERFERE_FORCE_ENABLE__", false);

  setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", undefined);

  expect(isEnabledByEnvironment()).toBe(false);
});
