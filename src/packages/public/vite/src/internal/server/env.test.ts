import { isEnabledOnServer, readInterfereServerEnv } from "./env";
import { afterEach, describe, expect, test, vi } from "vitest";

afterEach(() => {
  vi.unstubAllEnvs();
});

describe("server env", () => {
  test("enables server telemetry in development when a workspace API key is configured", () => {
    vi.stubEnv("NODE_ENV", "development");

    vi.stubEnv("INTERFERE_API_KEY", "interfere_secret_us_test");

    expect(isEnabledOnServer()).toBe(true);
    expect(readInterfereServerEnv().apiKey).toBe("interfere_secret_us_test");
  });

  test("falls back to the public key alias in development without a workspace API key", () => {
    vi.stubEnv("NODE_ENV", "development");

    vi.stubEnv("INTERFERE_API_KEY", "");

    vi.stubEnv("VITE_INTERFERE_PUBLIC_KEY", "interfere_pub_us_test");

    expect(isEnabledOnServer()).toBe(true);
  });

  test("stays off in production with a public key when the build disabled Interfere", () => {
    vi.stubEnv("NODE_ENV", "production");

    vi.stubEnv("INTERFERE_API_KEY", "interfere_secret_us_test");

    vi.stubEnv("VITE_INTERFERE_PUBLIC_KEY", "interfere_pub_us_test");

    vi.stubEnv("VITE_INTERFERE_DISABLED", "true");

    expect(isEnabledOnServer()).toBe(false);
    expect(readInterfereServerEnv().publicKey).toBeNull();
    expect(readInterfereServerEnv().apiKey).toBeNull();
  });

  test("falls back to development when no environment signal is set", () => {
    vi.stubEnv("NODE_ENV", "");

    vi.stubEnv("INTERFERE_ENVIRONMENT", "");

    vi.stubEnv("VERCEL_ENV", "");

    vi.stubEnv("MODE", "");

    const env = readInterfereServerEnv();

    expect(env.nodeEnvironment).toBe("development");
  });

  test.each(["production", "development"])(
    "reports the %s build mode when no environment variable is set",
    (mode) => {
      vi.stubEnv("NODE_ENV", "");

      vi.stubEnv("INTERFERE_ENVIRONMENT", "");

      vi.stubEnv("VERCEL_ENV", "");

      vi.stubEnv("MODE", mode);

      expect(readInterfereServerEnv().nodeEnvironment).toBe(mode);
    },
  );

  test("prefers INTERFERE_ENVIRONMENT and the baked deploy environment over the build mode", () => {
    vi.stubEnv("NODE_ENV", "");

    vi.stubEnv("MODE", "production");

    vi.stubEnv("INTERFERE_ENVIRONMENT", "staging");

    expect(readInterfereServerEnv().nodeEnvironment).toBe("staging");

    vi.stubEnv("VITE_INTERFERE_ENVIRONMENT", "preview");

    expect(readInterfereServerEnv().nodeEnvironment).toBe("preview");
  });

  test("prefers INTERFERE_ENVIRONMENT over NODE_ENV", () => {
    vi.stubEnv("NODE_ENV", "production");

    vi.stubEnv("INTERFERE_ENVIRONMENT", "staging");

    const env = readInterfereServerEnv();

    expect(env.nodeEnvironment).toBe("staging");
  });

  test("reports the deploy environment that the build baked in", () => {
    vi.stubEnv("NODE_ENV", "production");

    vi.stubEnv("VITE_INTERFERE_ENVIRONMENT", "preview");

    expect(readInterfereServerEnv().nodeEnvironment).toBe("preview");
  });
});
