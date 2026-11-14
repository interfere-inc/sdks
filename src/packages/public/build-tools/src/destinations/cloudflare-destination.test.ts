import { resolve } from "./cloudflare-destination.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

beforeEach(() => {
  for (const key of [
    "CF_VERSION_METADATA",
    "CF_VERSION_ID",
    "CF_WORKER_NAME",
    "CF_ACCOUNT_ID",
    "CLOUDFLARE_ACCOUNT_ID",
    "CF_PAGES_URL",
    "WORKERS_CI_BUILD_UUID",
    "WORKERS_CI_SCRIPT_NAME",
    "WORKERS_CI_BRANCH",
    "WORKERS_CI_DEPLOY_URL",
  ]) {
    vi.stubEnv(key, undefined);
  }
});

afterEach(() => {
  vi.unstubAllEnvs();
});

describe("cloudflare destination resolver", () => {
  test("returns metadata from CF_VERSION_METADATA when present", () => {
    vi.stubEnv(
      "CF_VERSION_METADATA",
      JSON.stringify({ id: "v_abc", tag: "production" }),
    );

    vi.stubEnv("CF_WORKER_NAME", "api");

    vi.stubEnv("CF_ACCOUNT_ID", "acc_123");

    const result = resolve();

    expect(result).toMatchObject({
      accountId: "acc_123",
      destinationReleaseId: "v_abc",
      environment: "production",
      provider: "cloudflare",
      scriptName: "api",
      versionId: "v_abc",
    });
  });

  test("prefers explicit CF_VERSION_ID over CF_VERSION_METADATA", () => {
    vi.stubEnv(
      "CF_VERSION_METADATA",
      JSON.stringify({ id: "v_meta", tag: "production" }),
    );

    vi.stubEnv("CF_VERSION_ID", "v_explicit");

    const result = resolve();

    expect(result.provider).toBe("cloudflare");
    expect(result).toMatchObject({ versionId: "v_explicit" });
  });

  test("falls back to Workers Builds env vars", () => {
    vi.stubEnv("WORKERS_CI_BUILD_UUID", "build_xyz");

    vi.stubEnv("WORKERS_CI_SCRIPT_NAME", "worker-name");

    vi.stubEnv("WORKERS_CI_BRANCH", "main");

    const result = resolve();

    expect(result).toMatchObject({
      environment: "main",
      provider: "cloudflare",
      scriptName: "worker-name",
      versionId: "build_xyz",
    });
  });

  test("returns null fields when no Cloudflare env vars are set", () => {
    expect(resolve()).toStrictEqual({
      accountId: null,
      deploymentUrl: null,
      destinationReleaseId: null,
      environment: null,
      environmentName: null,
      environmentTarget: null,
      provider: "cloudflare",
      scriptName: null,
      versionId: null,
    });
  });

  test("prepends https:// to deployment URL if missing", () => {
    vi.stubEnv("CF_PAGES_URL", "my-app.pages.dev");

    expect(resolve().deploymentUrl).toBe("https://my-app.pages.dev");
  });

  test("returns null when CF_VERSION_METADATA is invalid JSON", () => {
    vi.stubEnv("CF_VERSION_METADATA", "not-json");

    const result = resolve();

    expect(result.provider).toBe("cloudflare");
    expect(result).toMatchObject({ versionId: null });
  });

  test.each(
    [null, false, 42, "version", [], { id: 42, tag: false }].map(
      (metadata) => ({ metadata }),
    ),
  )("ignores malformed version metadata %j", ({ metadata }) => {
    vi.stubEnv("CF_VERSION_METADATA", JSON.stringify(metadata));

    expect(resolve()).toMatchObject({ versionId: null, environment: null });
  });

  test("preserves valid metadata fields independently", () => {
    vi.stubEnv("CF_VERSION_METADATA", JSON.stringify({ id: "v_abc", tag: 42 }));

    expect(resolve()).toMatchObject({ versionId: "v_abc", environment: null });
  });
});

test("preserves an absolute deployment URL", () => {
  vi.stubEnv("CF_PAGES_URL", "https://app.pages.dev");

  expect(resolve().deploymentUrl).toBe("https://app.pages.dev");
});
