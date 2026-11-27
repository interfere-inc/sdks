import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { isEnabledOnServer, readInterfereEnv } from "./env.js";
import {
  DISABLED_GLOBAL,
  ENVIRONMENT_GLOBAL,
} from "@interfere/types/sdk/injected-globals";

import { afterEach, describe, expect, test, vi } from "vitest";

afterEach(() => {
  vi.unstubAllEnvs();

  vi.unstubAllGlobals();
});

describe("disabled builds", () => {
  test("stay off on the server in production with a workspace API key", () => {
    vi.stubEnv("NODE_ENV", "production");

    vi.stubEnv("INTERFERE_API_KEY", "interfere_secret_us_test");

    setInjectedConfiguration(DISABLED_GLOBAL, true);

    expect(isEnabledOnServer()).toBe(false);
    expect(readInterfereEnv().apiKey).toBeNull();
  });
});

describe(isEnabledOnServer, () => {
  test("enables server telemetry with only a workspace API key", () => {
    vi.stubEnv("INTERFERE_API_KEY", "interfere_secret_us_test");

    expect(isEnabledOnServer()).toBe(true);
  });

  test("stays off without a workspace API key or a baked release token", () => {
    vi.stubEnv("INTERFERE_API_KEY", "");

    expect(isEnabledOnServer()).toBe(false);
  });
});

describe(readInterfereEnv, () => {
  test("trims whitespace-only values to null", () => {
    vi.stubEnv("INTERFERE_API_KEY", "   ");

    const env = readInterfereEnv();

    expect(env.apiKey).toBeNull();
  });

  test("falls back to development when no environment signal is set", () => {
    vi.stubEnv("NODE_ENV", "");

    vi.stubEnv("INTERFERE_ENVIRONMENT", "");

    vi.stubEnv("VERCEL_ENV", "");

    const env = readInterfereEnv();

    expect(env.nodeEnvironment).toBe("development");
  });

  test("prefers INTERFERE_ENVIRONMENT over NODE_ENV", () => {
    vi.stubEnv("NODE_ENV", "production");

    vi.stubEnv("INTERFERE_ENVIRONMENT", "staging");

    const env = readInterfereEnv();

    expect(env.nodeEnvironment).toBe("staging");
  });

  test("reports the deploy environment that the build baked in", () => {
    vi.stubEnv("NODE_ENV", "production");

    setInjectedConfiguration(ENVIRONMENT_GLOBAL, "preview");

    try {
      expect(readInterfereEnv().nodeEnvironment).toBe("preview");
    } finally {
      vi.unstubAllGlobals();
    }
  });
});
