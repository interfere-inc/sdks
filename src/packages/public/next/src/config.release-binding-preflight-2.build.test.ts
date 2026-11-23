import { rmSync, writeFileSync } from "node:fs";
import path from "node:path";
import {
  PHASE_DEVELOPMENT_SERVER,
  PHASE_PRODUCTION_BUILD,
} from "next/constants";

import {
  createReleaseFixture,
  releaseRequests,
  withTempProject,
} from "../test/fixtures/build.js";
import { withInterfere } from "./config.js";
import { log } from "./internal/logger.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

beforeEach(() => {
  vi.stubEnv("CI", undefined);

  vi.stubEnv("VERCEL", undefined);

  vi.stubEnv("WORKERS_CI", undefined);

  vi.stubEnv("CF_PAGES", undefined);

  vi.stubEnv("INTERFERE_ENVIRONMENT", undefined);

  vi.stubEnv("CLOUDFLARE_ENV", undefined);
});

afterEach(() => {
  vi.restoreAllMocks();

  vi.unstubAllEnvs();

  vi.unstubAllGlobals();
});

function prepareBuildProject(
  projectDir: string,
  provider: "cli" | "cloudflare" | "vercel",
  apiKey: string,
) {
  const environment =
    provider === "vercel"
      ? {
          VERCEL_DEPLOYMENT_ID: "dpl_test_123",
          VERCEL_ENV: "production",
          VERCEL_URL: "test.vercel.app",
        }
      : {
          INTERFERE_ENVIRONMENT: provider === "cli" ? "production" : "preview",
          INTERFERE_DEPLOYMENT_URL:
            provider === "cli" ? "https://self-hosted.test" : undefined,
          GITHUB_ACTIONS: "true",
          GITHUB_SERVER_URL: "https://github.com",
          GITHUB_REPOSITORY: "acme/app",
          GITHUB_RUN_ID: "42",
        };

  for (const [name, value] of Object.entries({
    ...environment,
    INTERFERE_API_KEY: apiKey,
    INTERFERE_API_URL: "https://in.example.com",
    NEXT_PUBLIC_INTERFERE_BUILD_ID: "abc0123456789def0123456789abcdef01234567",
  })) {
    vi.stubEnv(name, value);
  }

  writeFileSync(path.join(projectDir, "instrumentation-client.ts"), "");

  process.chdir(projectDir);
}

describe("release binding preflight", () => {
  test("the dev server warns when INTERFERE_API_KEY is not a valid key and mints nothing", async () => {
    expect.hasAssertions();

    await withTempProject("next-dev-bad-key-", async (projectDir) => {
      prepareBuildProject(projectDir, "cli", "not-an-interfere-key");

      vi.stubEnv("INTERFERE_PUBLIC_KEY", undefined);

      createReleaseFixture(projectDir);

      const warn = vi.spyOn(log, "warn");

      await withInterfere()(PHASE_DEVELOPMENT_SERVER);

      expect(
        warn.mock.calls.filter(
          ([title]) =>
            title === "Could not mint a local development release token",
        ),
      ).toHaveLength(1);

      expect(
        releaseRequests.filter((request) => request.method === "POST"),
      ).toHaveLength(0);
    });
  });

  test("the dev server warns once when it cannot mint a local-development token", async () => {
    expect.hasAssertions();

    await withTempProject("next-dev-mint-failure-", async (projectDir) => {
      prepareBuildProject(projectDir, "cli", "interfere_ak_test_123");

      vi.stubEnv("INTERFERE_PUBLIC_KEY", undefined);

      createReleaseFixture(projectDir, { failure: "register" });

      const warn = vi.spyOn(log, "warn");

      const nextConfig = withInterfere();

      await nextConfig(PHASE_DEVELOPMENT_SERVER);

      const config = await nextConfig(PHASE_DEVELOPMENT_SERVER);

      expect(JSON.stringify(config.turbopack)).not.toContain(
        "__INTERFERE_RELEASE_TOKEN__",
      );

      expect(
        warn.mock.calls.filter(
          ([title]) =>
            title === "Could not mint a local development release token",
        ),
      ).toHaveLength(1);
    });
  });

  test("warns instead of failing a release-token build that has no instrumentation-client file", async () => {
    expect.hasAssertions();

    await withTempProject(
      "next-release-token-server-only-",
      async (projectDir) => {
        prepareBuildProject(projectDir, "cli", "interfere_ak_test_123");

        vi.stubEnv("INTERFERE_PUBLIC_KEY", undefined);

        rmSync(path.join(projectDir, "instrumentation-client.ts"));

        createReleaseFixture(projectDir, { releaseToken: "irt_next_build" });

        const warn = vi.spyOn(log, "warn");

        await expect(
          withInterfere()(PHASE_PRODUCTION_BUILD),
        ).resolves.toBeDefined();

        expect(warn).toHaveBeenCalledWith(
          "Missing instrumentation client",
          expect.any(Array),
        );
      },
    );
  });

  test("keeps instrumentation enabled when the binding lookup fails", async () => {
    expect.hasAssertions();

    await withTempProject("next-binding-outage-", async (projectDir) => {
      prepareBuildProject(projectDir, "vercel", "interfere_secret_us_test_123");

      vi.stubEnv("INTERFERE_PUBLIC_KEY", "interfere_pk_next");

      createReleaseFixture(projectDir, { failure: "config" });

      const config = await withInterfere()(PHASE_PRODUCTION_BUILD);

      const baked = JSON.stringify(config.turbopack);

      expect(baked).toContain("__INTERFERE_PUBLIC_KEY__");
      expect(baked).toContain("__INTERFERE_RELEASE_SLUG__");
      expect(baked).not.toContain("__INTERFERE_DISABLED__");
      expect(baked).not.toContain("interfere_secret_us_test_123");
    });
  });
});
