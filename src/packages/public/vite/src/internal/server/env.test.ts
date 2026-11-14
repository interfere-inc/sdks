import { isEnabledOnServer, readInterfereServerEnv } from "./env";
import { afterEach, describe, expect, test, vi } from "vitest";

afterEach(() => {
  vi.unstubAllEnvs();
});

describe("server env", () => {
  test("enables server/proxy mode when a public key is configured", () => {
    vi.stubEnv("NODE_ENV", "development");

    vi.stubEnv("VITE_INTERFERE_PUBLIC_KEY", "interfere_pub_us_test");

    expect(isEnabledOnServer()).toBe(true);
  });

  test("stays off in production with a public key when the build disabled Interfere", () => {
    vi.stubEnv("NODE_ENV", "production");

    vi.stubEnv("VITE_INTERFERE_PUBLIC_KEY", "interfere_pub_us_test");

    vi.stubEnv("VITE_INTERFERE_DISABLED", "true");

    expect(isEnabledOnServer()).toBe(false);
    expect(readInterfereServerEnv().publicKey).toBeNull();
  });

  test("falls back to development when no environment signal is set", () => {
    vi.stubEnv("NODE_ENV", "");

    vi.stubEnv("INTERFERE_ENVIRONMENT", "");

    vi.stubEnv("VERCEL_ENV", "");

    const env = readInterfereServerEnv();

    expect(env.nodeEnvironment).toBe("development");
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
