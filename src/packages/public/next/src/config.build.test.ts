import { http } from "msw";
import { releaseSourceIdEnvKeys } from "@interfere/types/integrations";
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import {
  PHASE_DEVELOPMENT_SERVER,
  PHASE_PRODUCTION_BUILD,
} from "next/constants";

import {
  capturedReleaseRequest,
  createReleaseFixture,
  releaseNetwork,
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

function compileHookPhase() {
  vi.stubEnv("INTERFERE_PUBLIC_KEY", "interfere_pk_test_123");

  return PHASE_DEVELOPMENT_SERVER;
}

describe("withInterfere build pipeline", () => {
  test.each([
    { apiKey: "interfere_ak_test_123", purge: true },
    { apiKey: "", purge: true },
    { apiKey: "interfere_ak_test_123", purge: false },
  ])(
    "cleans skipped-release source maps according to purge=$purge with apiKey=$apiKey",
    async ({ apiKey, purge }) => {
      expect.hasAssertions();

      await withTempProject("next-skipped-maps-", async (projectDir) => {
        prepareBuildProject(projectDir, "cli", apiKey);

        for (const key of releaseSourceIdEnvKeys) {
          vi.stubEnv(key, undefined);
        }

        vi.stubEnv("GIT_DIR", path.join(projectDir, "missing-git"));

        const { map, chunk, fetcher } = createReleaseFixture(projectDir);

        const output = await withInterfere(
          { sourceMaps: { purge } },
          {},
        )(PHASE_PRODUCTION_BUILD);

        await expect(
          output.compiler?.runAfterProductionCompile?.({
            distDir: ".next",
            projectDir,
          }),
        ).resolves.toBeUndefined();

        expect(existsSync(map)).toBe(!purge);

        expect(readFileSync(chunk, "utf8").includes("sourceMappingURL")).toBe(
          !purge,
        );

        expect(fetcher).not.toHaveBeenCalled();
      });
    },
  );

  test("continues production compile with one warning when the commit SHA is missing", async () => {
    expect.hasAssertions();

    await withTempProject("next-config-no-sha-", async (projectDir) => {
      prepareBuildProject(projectDir, "cli", "interfere_ak_test_123");

      for (const key of releaseSourceIdEnvKeys) {
        vi.stubEnv(key, undefined);
      }

      vi.stubEnv("GIT_DIR", path.join(projectDir, "missing-git"));

      const existingHook = vi.fn<() => Promise<void>>();
      const warn = vi.spyOn(log, "warn");
      const fetchSpy = vi.spyOn(globalThis, "fetch");

      const output = await withInterfere({
        compiler: { runAfterProductionCompile: existingHook },
      })(PHASE_PRODUCTION_BUILD);

      await expect(
        output.compiler?.runAfterProductionCompile?.({
          distDir: ".next",
          projectDir,
        }),
      ).resolves.toBeUndefined();

      expect(existingHook).toHaveBeenCalledTimes(1);
      expect(fetchSpy).not.toHaveBeenCalled();

      expect(
        warn.mock.calls.filter(([title]) => title === "Commit SHA missing"),
      ).toStrictEqual([["Commit SHA missing", expect.any(Array)]]);
    });
  });

  test("skips the release with a warning when the API key is absent", async () => {
    const existingHook = vi.fn<() => Promise<void>>();

    const fetchSpy = vi.spyOn(globalThis, "fetch");

    vi.stubEnv("INTERFERE_ENVIRONMENT", "production");

    vi.stubEnv("INTERFERE_API_KEY", undefined);

    vi.stubEnv("NEXT_PUBLIC_INTERFERE_BUILD_ID", undefined);

    vi.stubEnv("GIT_DIR", "/tmp/missing-git");

    const warn = vi.spyOn(log, "warn");

    const output = await withInterfere({
      compiler: { runAfterProductionCompile: existingHook },
      productionBrowserSourceMaps: false,
    })(PHASE_DEVELOPMENT_SERVER);

    expect(output.productionBrowserSourceMaps).toBe(false);

    await expect(
      output.compiler?.runAfterProductionCompile?.({
        distDir: ".next",
        projectDir: "/tmp/project",
      }),
    ).resolves.toBeUndefined();

    expect(existingHook).toHaveBeenCalledTimes(1);
    expect(fetchSpy).not.toHaveBeenCalled();
    expect(warn).toHaveBeenCalledWith("Skipping release", expect.any(Array));
  });

  test("turns Interfere off when disable is set, even with an invalid secret key", async () => {
    expect.assertions(3);

    await withTempProject("next-config-build-disable-", async (projectDir) => {
      prepareBuildProject(projectDir, "cli", "interfere_pk_wrong_prefix");

      const fetchSpy = vi.spyOn(globalThis, "fetch");

      const compiler = {
        runAfterProductionCompile: vi.fn<() => Promise<void>>(),
      };

      const output = await withInterfere(
        { disable: true },
        { compiler, productionBrowserSourceMaps: false },
      )(PHASE_PRODUCTION_BUILD);

      expect(output.compiler).toBe(compiler);
      expect(output.productionBrowserSourceMaps).toBe(false);

      await output.compiler?.runAfterProductionCompile?.({
        distDir: ".next",
        projectDir,
      });

      expect(fetchSpy).not.toHaveBeenCalled();
    });
  });

  test("throws when API key has the wrong prefix during production compile", async () => {
    const existingHook = vi.fn<() => Promise<void>>();

    const fetchSpy = vi.spyOn(globalThis, "fetch");

    vi.stubEnv("INTERFERE_API_KEY", "interfere_pk_test_123");

    vi.stubEnv("INTERFERE_ENVIRONMENT", "production");

    const output = await withInterfere({
      compiler: {
        runAfterProductionCompile: existingHook,
      },
    })(PHASE_DEVELOPMENT_SERVER);

    const { compiler } = output;

    await expect(
      compiler?.runAfterProductionCompile?.({
        distDir: ".next",
        projectDir: "/tmp/project",
      }),
    ).rejects.toThrow("INTERFERE_API_KEY has wrong prefix");

    expect(existingHook).toHaveBeenCalledTimes(1);
    expect(fetchSpy).not.toHaveBeenCalled();
  });

  test.each([
    ["AUTH_MISSING", 401, "Secret key could not be read", "api_key_unreadable"],
    [
      "AUTH_AK_INVALID",
      401,
      "Secret key rejected by server",
      "api_key_rejected",
    ],
    [
      "AUTH_AK_SCOPE_MISMATCH",
      403,
      "Secret key missing release scope",
      "api_key_scope_mismatch",
    ],
    [
      "AUTH_AK_SURFACE_REQUIRED",
      401,
      "Service needed for a workspace secret key",
      "api_key_surface_required",
    ],
  ] as const)(
    "logs an error and continues the build for %s release pipeline responses",
    async (code, status, expectedMessage, expectedResult) => {
      expect.assertions(5);

      await withTempProject("next-config-build-auth-", async (projectDir) => {
        prepareBuildProject(
          projectDir,
          "vercel",
          "interfere_secret_us_test_123",
        );

        createReleaseFixture(projectDir);

        const fetchSpy = vi.spyOn(globalThis, "fetch");

        releaseNetwork.use(
          http.all("https://in.example.com/*", () =>
            Response.json(
              { code, message: "Authentication failed." },
              { status },
            ),
          ),
        );

        const error = vi.spyOn(log, "error");

        const output = await withInterfere({
          compiler: {
            runAfterProductionCompile: vi.fn<() => Promise<void>>(),
          },
        })(compileHookPhase());

        const { compiler } = output;

        await expect(
          compiler?.runAfterProductionCompile?.({
            distDir: ".next",
            projectDir,
          }),
        ).resolves.toBeUndefined();

        expect(error).toHaveBeenCalledWith(expectedMessage, expect.any(Array));

        expect(fetchSpy).toHaveBeenCalledTimes(3);

        const telemetryRequest = capturedReleaseRequest(2);

        expect(telemetryRequest.url).toBe(
          "https://in.example.com/v3/releases/rel_abc0123456789def/telemetry",
        );

        await expect(telemetryRequest.json()).resolves.toMatchObject({
          result: expectedResult,
        });
      });
    },
  );
});
