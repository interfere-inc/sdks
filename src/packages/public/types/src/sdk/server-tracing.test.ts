import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { PLATFORM_GLOBAL } from "../integrations/platforms/hosting.ts";
import {
  DEFAULT_SERVER_TRACING_MODE,
  SERVER_TRACING_GLOBAL,
  readServerTracing,
  resolveServerTracing,
} from "./server-tracing.ts";
import { afterEach, describe, expect, test, vi } from "vitest";

describe(resolveServerTracing, () => {
  test("follows the injected surface configuration", () => {
    expect(resolveServerTracing({ env: {}, injected: "drain" })).toStrictEqual({
      mode: "drain",
      vercel: false,
    });

    expect(resolveServerTracing({ env: {}, injected: " Full " }).mode).toBe(
      "full",
    );
  });

  test("defaults to full when the surface configuration is absent", () => {
    expect(resolveServerTracing({ env: {} }).mode).toBe(
      DEFAULT_SERVER_TRACING_MODE,
    );

    expect(resolveServerTracing({ env: {}, injected: "sampled" }).mode).toBe(
      "full",
    );
  });

  test("ignores the removed INTERFERE_SERVER_TRACING env var", () => {
    expect(
      resolveServerTracing({ env: { INTERFERE_SERVER_TRACING: "drain" } }).mode,
    ).toBe("full");
  });

  test.each([
    { platform: "vercel", env: {}, vercel: true },
    { platform: " Vercel ", env: {}, vercel: true },
    { platform: "cloudflare", env: { VERCEL: "1" }, vercel: false },
    { platform: null, env: { VERCEL: "1" }, vercel: false },
    { platform: "heroku", env: { VERCEL: "1" }, vercel: false },
  ])(
    "takes Vercel hosting from the baked platform $platform, not VERCEL",
    ({ platform, env, vercel }) => {
      expect(resolveServerTracing({ env, platform }).vercel).toBe(vercel);
    },
  );

  test("falls back to VERCEL only when the build baked no platform", () => {
    expect(resolveServerTracing({ env: { VERCEL: "1" } }).vercel).toBe(true);

    expect(resolveServerTracing({ env: {} }).vercel).toBe(false);
  });
});

describe(readServerTracing, () => {
  afterEach(() => {
    vi.unstubAllGlobals();

    vi.unstubAllEnvs();
  });

  test.each([undefined, null, false, 42, {}, [], "invalid"])(
    "treats invalid injected mode %j as full",
    (value) => {
      setInjectedConfiguration(SERVER_TRACING_GLOBAL, value);

      expect(readServerTracing({}).mode).toBe("full");
    },
  );

  test("reads the build-injected mode and platform globals", () => {
    setInjectedConfiguration(SERVER_TRACING_GLOBAL, "drain");

    setInjectedConfiguration(PLATFORM_GLOBAL, "vercel");

    expect(readServerTracing({})).toStrictEqual({
      mode: "drain",
      vercel: true,
    });
  });

  test("a baked null platform outranks VERCEL in process.env", () => {
    setInjectedConfiguration(PLATFORM_GLOBAL, null);

    vi.stubEnv("VERCEL", "1");

    expect(readServerTracing().vercel).toBe(false);
  });

  test("reads VERCEL from process.env when no platform was baked", () => {
    vi.stubEnv("VERCEL", "1");

    expect(readServerTracing()).toStrictEqual({ mode: "full", vercel: true });
  });
});
