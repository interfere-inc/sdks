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
  test("stay off on the server in production with a public key and a workspace API key", () => {
    vi.stubEnv("NODE_ENV", "production");

    vi.stubEnv("INTERFERE_PUBLIC_KEY", "interfere_public_us_test");

    vi.stubEnv("INTERFERE_API_KEY", "interfere_secret_us_test");

    setInjectedConfiguration(DISABLED_GLOBAL, true);

    expect(isEnabledOnServer()).toBe(false);
    expect(readInterfereEnv().publicKey).toBeNull();
    expect(readInterfereEnv().apiKey).toBeNull();
  });
});

describe(isEnabledOnServer, () => {
  test("enables server telemetry with only a workspace API key", () => {
    vi.stubEnv("INTERFERE_PUBLIC_KEY", "");

    vi.stubEnv("INTERFERE_API_KEY", "interfere_secret_us_test");

    expect(isEnabledOnServer()).toBe(true);
  });

  test("falls back to the public key alias without a workspace API key", () => {
    vi.stubEnv("INTERFERE_PUBLIC_KEY", "interfere_public_us_test");

    vi.stubEnv("INTERFERE_API_KEY", "");

    expect(isEnabledOnServer()).toBe(true);
  });
});

describe(readInterfereEnv, () => {
  test("reads NEXT_PUBLIC_INTERFERE_BUILD_ID as release.sourceId", () => {
    vi.stubEnv("NEXT_PUBLIC_INTERFERE_BUILD_ID", "build_abc123");

    const env = readInterfereEnv();

    expect(env.release.sourceId).toBe("build_abc123");
  });

  test("returns null sourceId when NEXT_PUBLIC_INTERFERE_BUILD_ID is not set", () => {
    vi.stubEnv("NEXT_PUBLIC_INTERFERE_BUILD_ID", "");

    const env = readInterfereEnv();

    expect(env.release.sourceId).toBeNull();
  });

  test("reads NEXT_PUBLIC_INTERFERE_RELEASE_ID as release.destinationId", () => {
    vi.stubEnv("NEXT_PUBLIC_INTERFERE_RELEASE_ID", "release_xyz");

    const env = readInterfereEnv();

    expect(env.release.destinationId).toBe("release_xyz");
  });

  test("returns null destinationId when not set", () => {
    vi.stubEnv("NEXT_PUBLIC_INTERFERE_RELEASE_ID", "");

    const env = readInterfereEnv();

    expect(env.release.destinationId).toBeNull();
  });

  test("trims whitespace-only values to null", () => {
    vi.stubEnv("NEXT_PUBLIC_INTERFERE_BUILD_ID", "   ");

    const env = readInterfereEnv();

    expect(env.release.sourceId).toBeNull();
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
