import { writeFileSync } from "node:fs";
import path from "node:path";
import { PHASE_PRODUCTION_SERVER } from "next/constants";

import {
  createReleaseFixture,
  parseFetchCall,
  withTempProject,
} from "../test/fixtures/build.js";
import { withInterfere } from "./config.js";
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

function compileHookPhase() {
  return PHASE_PRODUCTION_SERVER;
}

describe("withInterfere build pipeline", () => {
  test("does not finalize source maps when signing fails", async () => {
    expect.assertions(2);

    await withTempProject("next-config-build-fail-", async (projectDir) => {
      prepareBuildProject(projectDir, "vercel", "interfere_ak_test_123");

      const { fetcher: fetchSpy } = createReleaseFixture(projectDir, {
        destinationProvider: "vercel",
        failure: "sign",
      });

      const output = await withInterfere({
        compiler: {
          runAfterProductionCompile: vi.fn<() => Promise<void>>(),
        },
      })(compileHookPhase());

      const { compiler } = output;

      await compiler?.runAfterProductionCompile?.({
        distDir: ".next",
        projectDir,
      });

      const visitedUrls = fetchSpy.mock.calls.map(
        (call) => parseFetchCall(call).url,
      );

      expect(
        visitedUrls.some((url) =>
          new URL(url).pathname.endsWith("/source-maps/sign"),
        ),
      ).toBe(true);

      expect(
        visitedUrls.some((url) =>
          new URL(url).pathname.endsWith("/source-maps/finalize"),
        ),
      ).toBe(false);
    });
  });
});
