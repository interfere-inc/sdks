import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { close, consent, init } from "./init";
import { PRODUCER_VERSION } from "./version";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

const VITE_VERSION_RE = /^@interfere\/vite@/u;

const BUILD_ID_KEY = "__INTERFERE_BUILD_ID__";

const STACK_KEY = "__INTERFERE_SDK_STACK__";

beforeEach(async () => {
  await close();

  vi.unstubAllEnvs();

  vi.unstubAllGlobals();

  setInjectedConfiguration(BUILD_ID_KEY, undefined);

  setInjectedConfiguration(STACK_KEY, undefined);
});

afterEach(async () => {
  await close();

  vi.unstubAllGlobals();
});

describe("init (Vite entrypoint)", () => {
  test("re-exports init", () => {
    expect(init).toBeTypeOf("function");
  });

  test("re-exports consent", () => {
    expect(consent.get).toBeTypeOf("function");
    expect(consent.set).toBeTypeOf("function");
  });

  test("returns null when the SDK is disabled", async () => {
    await expect(init({ enabled: false })).resolves.toBeNull();
  });

  test("returns null when the build disabled Interfere, even with force enable", async () => {
    vi.stubEnv("INTERFERE_FORCE_ENABLE", "1");

    vi.stubEnv("VITE_INTERFERE_FORCE_ENABLE", "1");

    vi.stubEnv("VITE_INTERFERE_DISABLED", "true");

    await expect(init()).resolves.toBeNull();
  });

  test("uses the correct vite package version string", () => {
    expect(PRODUCER_VERSION).toMatch(VITE_VERSION_RE);
  });
});
