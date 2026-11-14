import { resolve } from "./vercel-destination.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

beforeEach(() => {
  for (const key of [
    "VERCEL_DEPLOYMENT_ID",
    "VERCEL_ENV",
    "VERCEL_TARGET_ENV",
    "VERCEL_URL",
  ]) {
    vi.stubEnv(key, undefined);
  }
});

afterEach(() => {
  vi.unstubAllEnvs();
});

describe("vercel destination resolver", () => {
  test("returns full metadata when Vercel env vars are set", () => {
    vi.stubEnv("VERCEL_DEPLOYMENT_ID", "dpl_123");

    vi.stubEnv("VERCEL_ENV", "production");

    vi.stubEnv("VERCEL_URL", "example.vercel.app");

    const result = resolve();

    expect(result).toStrictEqual({
      deploymentId: "dpl_123",
      deploymentUrl: "https://example.vercel.app",
      destinationReleaseId: "dpl_123",
      environment: "production",
      environmentName: "production",
      environmentTarget: "production",
      provider: "vercel",
    });
  });

  test("returns null fields when no Vercel env vars are set", () => {
    vi.stubEnv("VERCEL_DEPLOYMENT_ID", "");

    vi.stubEnv("VERCEL_ENV", "");

    expect(resolve()).toStrictEqual({
      deploymentId: null,
      deploymentUrl: null,
      destinationReleaseId: null,
      environment: null,
      environmentName: null,
      environmentTarget: null,
      provider: "vercel",
    });
  });

  test("prepends https:// to deployment URL if missing", () => {
    vi.stubEnv("VERCEL_DEPLOYMENT_ID", "dpl_123");

    vi.stubEnv("VERCEL_ENV", "preview");

    vi.stubEnv("VERCEL_URL", "my-app-abc123.vercel.app");

    expect(resolve().deploymentUrl).toBe("https://my-app-abc123.vercel.app");
  });

  test("preserves existing protocol in deployment URL", () => {
    vi.stubEnv("VERCEL_DEPLOYMENT_ID", "dpl_123");

    vi.stubEnv("VERCEL_ENV", "preview");

    vi.stubEnv("VERCEL_URL", "https://already-prefixed.vercel.app");

    expect(resolve().deploymentUrl).toBe("https://already-prefixed.vercel.app");
  });
});
