import { existsSync, writeFileSync } from "node:fs";
import path from "node:path";
import {
  PHASE_PRODUCTION_BUILD,
  PHASE_PRODUCTION_SERVER,
} from "next/constants";

import {
  RELEASE_SLUG,
  capturedReleaseRequest,
  createReleaseFixture,
  parseFetchCall,
  releaseRequests,
  withTempProject,
} from "../test/fixtures/build.js";
import { withInterfere } from "./config.js";
import { PRODUCER_VERSION } from "./internal/version.js";
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
  test("creates release metadata for paired .js + .js.map files", async () => {
    expect.assertions(18);

    await withTempProject("next-config-build-", async (projectDir) => {
      prepareBuildProject(projectDir, "vercel", "interfere_secret_us_test_123");

      const { fetcher: fetchSpy } = createReleaseFixture(projectDir, {
        destinationProvider: "vercel",
      });

      const existingHook = vi.fn<() => Promise<void>>();

      const output = await withInterfere({
        compiler: {
          runAfterProductionCompile: existingHook,
        },
      })(compileHookPhase());

      expect(output.productionBrowserSourceMaps).toBe(true);

      const { compiler } = output;

      await compiler?.runAfterProductionCompile?.({
        distDir: ".next",
        projectDir,
      });

      expect(existingHook).toHaveBeenCalledTimes(1);
      expect(fetchSpy).toHaveBeenCalledTimes(6);

      const configRequest = capturedReleaseRequest(0);

      expect(configRequest.url).toBe(
        "https://in.example.com/v3/releases/config",
      );

      const releaseRequest = capturedReleaseRequest(1);

      expect(releaseRequest.url).toBe("https://in.example.com/v3/releases");

      expect(releaseRequest.headers.get("Authorization")).toBe(
        "Bearer interfere_secret_us_test_123",
      );

      const signRequest = capturedReleaseRequest(2);

      expect(new URL(signRequest.url).searchParams.has("environment")).toBe(
        false,
      );

      expect(signRequest.url).toBe(
        `https://in.example.com/v3/releases/${RELEASE_SLUG}/source-maps/sign`,
      );

      expect(signRequest.headers.get("Authorization")).toBe(
        "Bearer interfere_secret_us_test_123",
      );

      const r2PutRequest = capturedReleaseRequest(3);

      const uploadUrl = new URL(r2PutRequest.url);

      uploadUrl.searchParams.sort();

      expect(uploadUrl.toString()).toBe(
        `https://in.example.com/v3/releases/${RELEASE_SLUG}/source-maps?offset=0&path=_next%2Fstatic%2Fchunks%2Fapp.js.map`,
      );

      expect(r2PutRequest.method).toBe("PUT");

      expect(r2PutRequest.headers.get("Authorization")).toBe(
        "Bearer interfere_secret_us_test_123",
      );

      expect(r2PutRequest.headers.get("content-type")).toBe(
        "application/octet-stream",
      );

      const completeRequest = capturedReleaseRequest(4);

      expect(completeRequest.url).toBe(
        `https://in.example.com/v3/releases/${RELEASE_SLUG}/source-maps/finalize`,
      );

      expect(completeRequest.headers.get("Authorization")).toBe(
        "Bearer interfere_secret_us_test_123",
      );

      const telemetryRequest = capturedReleaseRequest(5);

      expect(telemetryRequest.url).toBe(
        `https://in.example.com/v3/releases/${RELEASE_SLUG}/telemetry`,
      );

      expect(telemetryRequest.method).toBe("POST");

      await expect(telemetryRequest.json()).resolves.toMatchObject({
        fileCount: 1,
        result: "ok",
      });
    });
  });

  test("publishes and cleans source maps for CLI-managed surfaces", async () => {
    expect.assertions(9);

    await withTempProject("next-config-build-cli-", async (projectDir) => {
      prepareBuildProject(projectDir, "cli", "interfere_ak_test_123");

      const { map: sourceMapPath, fetcher: fetchSpy } = createReleaseFixture(
        projectDir,
        {
          destinationProvider: "cli",
        },
      );

      const existingHook = vi.fn<() => Promise<void>>();

      const output = await withInterfere({
        compiler: {
          runAfterProductionCompile: existingHook,
        },
      })(compileHookPhase());

      expect(output.productionBrowserSourceMaps).toBe(true);

      const { compiler } = output;

      await compiler?.runAfterProductionCompile?.({
        distDir: ".next",
        projectDir,
      });

      expect(existingHook).toHaveBeenCalledTimes(1);
      expect(fetchSpy).toHaveBeenCalledTimes(6);

      const configRequest = capturedReleaseRequest(0);

      expect(configRequest.url).toBe(
        "https://in.example.com/v3/releases/config",
      );

      const releaseRequest = capturedReleaseRequest(1);

      expect(releaseRequest.url).toBe("https://in.example.com/v3/releases");

      await expect(releaseRequest.json()).resolves.toMatchObject({
        destination: {
          deploymentUrl: "https://self-hosted.test",
          destinationReleaseId: "rel_abc0123456789def",
          environment: "production",
          provider: "cli",
        },
        slug: "rel_abc0123456789def",
      });

      const signRequest = capturedReleaseRequest(2);

      expect(new URL(signRequest.url).pathname).toBe(
        `/v3/releases/${RELEASE_SLUG}/source-maps/sign`,
      );

      const telemetryRequest = capturedReleaseRequest(5);

      expect(telemetryRequest.url).toBe(
        `https://in.example.com/v3/releases/${RELEASE_SLUG}/telemetry`,
      );

      expect(existsSync(sourceMapPath)).toBe(false);
    });
  });

  test.each([
    { upload: true, expectedFetches: 6 },
    { upload: false, expectedFetches: 3 },
  ])(
    "keeps source maps on disk with purge=false and upload=$upload",
    async ({ upload, expectedFetches }) => {
      expect.assertions(3);

      await withTempProject("next-config-build-keep-", async (projectDir) => {
        prepareBuildProject(projectDir, "cli", "interfere_ak_test_123");

        const { map: sourceMapPath, fetcher: fetchSpy } = createReleaseFixture(
          projectDir,
          { destinationProvider: "cli" },
        );

        const output = await withInterfere(
          { sourceMaps: { purge: false, upload } },
          {
            compiler: {
              runAfterProductionCompile: vi.fn<() => Promise<void>>(),
            },
          },
        )(compileHookPhase());

        const { compiler } = output;

        await compiler?.runAfterProductionCompile?.({
          distDir: ".next",
          projectDir,
        });

        expect(fetchSpy).toHaveBeenCalledTimes(expectedFetches);

        expect(
          fetchSpy.mock.calls.some((call) =>
            parseFetchCall(call).url.includes("source-maps"),
          ),
        ).toBe(upload);

        expect(existsSync(sourceMapPath)).toBe(true);
      });
    },
  );

  test("seeds cloudflare release metadata with the SDK deploy environment", async () => {
    expect.assertions(1);

    await withTempProject(
      "next-config-build-cloudflare-",
      async (projectDir) => {
        prepareBuildProject(projectDir, "cloudflare", "interfere_ak_test_123");

        createReleaseFixture(projectDir, {
          destinationProvider: "cloudflare",
        });

        const output = await withInterfere()(PHASE_PRODUCTION_BUILD);

        const { compiler } = output;

        await compiler?.runAfterProductionCompile?.({
          distDir: ".next",
          projectDir,
        });

        const releaseRequest = capturedReleaseRequest(1);

        await expect(releaseRequest.json()).resolves.toMatchObject({
          destination: {
            environment: "preview",
            environmentName: "preview",
            environmentTarget: "preview",
            provider: "cloudflare",
          },
        });
      },
    );
  });

  test("names the service on every release request", async () => {
    expect.assertions(4);

    await withTempProject("next-config-build-selector-", async (projectDir) => {
      prepareBuildProject(projectDir, "vercel", "interfere_secret_us_test_123");

      createReleaseFixture(projectDir, { destinationProvider: "vercel" });

      await (
        await withInterfere(
          { serviceName: "storefront" },
          {},
        )(PHASE_PRODUCTION_BUILD)
      ).compiler?.runAfterProductionCompile?.({ distDir: ".next", projectDir });

      const requests = releaseRequests.filter((request) =>
        new URL(request.url).pathname.startsWith("/v3/releases"),
      );

      expect(
        requests.map((request) => request.headers.get("x-interfere-service")),
      ).toStrictEqual(Array.from(requests, () => "storefront"));

      expect(requests.length).toBeGreaterThanOrEqual(5);

      expect(
        requests.every(
          (request) => !request.headers.has("x-interfere-public-key"),
        ),
      ).toBe(true);

      expect(
        requests.map((request) =>
          request.headers.get("x-interfere-producer-version"),
        ),
      ).toStrictEqual(Array.from(requests, () => PRODUCER_VERSION));
    });
  });
});
