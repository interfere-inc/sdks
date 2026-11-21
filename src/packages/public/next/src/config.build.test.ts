import { http } from "msw";
import { releaseSourceIdEnvKeys } from "@interfere/types/integrations";
import { createReleaseRequest } from "@interfere/types/releases/definition";
import { LOCAL_DEV_RELEASE_SLUG } from "@interfere/types/releases/slug";
import { existsSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import path from "node:path";
import {
  PHASE_DEVELOPMENT_SERVER,
  PHASE_PRODUCTION_BUILD,
} from "next/constants";

import {
  RELEASE_SLUG,
  capturedReleaseRequest,
  createReleaseFixture,
  parseFetchCall,
  releaseNetwork,
  releaseRequests,
  withTempProject,
} from "../test/fixtures/build.js";
import { withInterfere } from "./config.js";
import { log } from "./internal/logger.js";
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

        vi.stubEnv("INTERFERE_PUBLIC_KEY", "interfere_pk_next");

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

  test("names the app with its public key and service on every release request", async () => {
    expect.assertions(4);

    await withTempProject("next-config-build-selector-", async (projectDir) => {
      prepareBuildProject(projectDir, "vercel", "interfere_secret_us_test_123");

      vi.stubEnv("INTERFERE_PUBLIC_KEY", "interfere_pk_next");

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
        requests.map((request) =>
          request.headers.get("x-interfere-public-key"),
        ),
      ).toStrictEqual(Array.from(requests, () => "interfere_pk_next"));

      expect(
        requests.map((request) =>
          request.headers.get("x-interfere-producer-version"),
        ),
      ).toStrictEqual(Array.from(requests, () => PRODUCER_VERSION));
    });
  });

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

describe("release binding preflight", () => {
  test.each([
    {
      name: "@acme/storefront",
      option: undefined,
      env: undefined,
      failure: false,
      warnings: [],
    },
    {
      name: "@acme/frontend",
      option: undefined,
      env: undefined,
      failure: false,
      warnings: [],
    },
    {
      name: "@acme/storefront",
      option: "chosen-name",
      env: undefined,
      failure: false,
      warnings: [],
    },
    {
      name: "@acme/storefront",
      option: undefined,
      env: "chosen-name",
      failure: false,
      warnings: [],
    },
    {
      name: "@acme/storefront",
      option: " ",
      env: " ",
      failure: false,
      warnings: [],
    },
    {
      name: "@acme/",
      option: undefined,
      env: undefined,
      failure: false,
      warnings: [],
    },
    {
      name: "@acme/storefront",
      option: undefined,
      env: undefined,
      failure: true,
      warnings: [],
    },
  ])(
    "never warns about an App for the inferred service name: $name / $option / $env / $failure",
    async ({ name, option, env, failure, warnings: expected }) => {
      expect.hasAssertions();

      await withTempProject("next-service-name-", async (projectDir) => {
        prepareBuildProject(
          projectDir,
          "vercel",
          "interfere_secret_us_test_123",
        );

        vi.stubEnv("INTERFERE_PUBLIC_KEY", "interfere_pk_next");

        vi.stubEnv("OTEL_SERVICE_NAME", env);

        writeFileSync(
          path.join(projectDir, "package.json"),
          JSON.stringify({ name }),
        );

        createReleaseFixture(projectDir, failure ? { failure: "config" } : {});

        const warn = vi.spyOn(log, "warn");

        const config = await withInterfere(
          option === undefined ? {} : { serviceName: option },
          {},
        )(PHASE_PRODUCTION_BUILD);

        expect(config.compiler?.runAfterProductionCompile).toBeTypeOf(
          "function",
        );

        expect(JSON.stringify(config.turbopack)).not.toContain(
          "__INTERFERE_DISABLED__",
        );

        const warnings = warn.mock.calls.filter(([title, details]) =>
          /\bApp\b/u.test([title, ...details].join(" ")),
        );

        expect(warnings.map(([, details]) => details)).toStrictEqual(expected);
      });
    },
  );

  test.each(["destinationProvider", "sourceProvider"] as const)(
    "keeps instrumentation and registers the release when %s is missing",
    async (provider) => {
      expect.hasAssertions();

      await withTempProject("next-binding-", async (projectDir) => {
        prepareBuildProject(
          projectDir,
          "vercel",
          "interfere_secret_us_test_123",
        );

        vi.stubEnv("INTERFERE_PUBLIC_KEY", "interfere_pk_next");

        createReleaseFixture(projectDir, { [provider]: null });

        const config = await withInterfere()(PHASE_PRODUCTION_BUILD);

        const baked = JSON.stringify(config.turbopack);

        expect(baked).not.toContain("__INTERFERE_DISABLED__");
        expect(baked).toContain("__INTERFERE_PUBLIC_KEY__");
        expect(baked).toContain("__INTERFERE_RELEASE_SLUG__");
        expect(baked).not.toContain("interfere_secret_us_test_123");

        expect(
          releaseRequests
            .filter((request) => request.method === "POST")
            .map((request) => new URL(request.url).pathname),
        ).toStrictEqual(["/v3/releases"]);
      });
    },
  );

  test("bakes the release token for a workspace secret whose release config is refused", async () => {
    expect.hasAssertions();

    await withTempProject("next-keyless-", async (projectDir) => {
      prepareBuildProject(projectDir, "cli", "interfere_secret_us_test_123");

      vi.stubEnv("INTERFERE_PUBLIC_KEY", undefined);

      createReleaseFixture(projectDir, {
        failure: "config",
        releaseToken: "irt_next_keyless",
      });

      const config = await withInterfere()(PHASE_PRODUCTION_BUILD);

      expect(JSON.stringify(config.turbopack)).toContain(
        String.raw`\"__INTERFERE_RELEASE_TOKEN__\":\"irt_next_keyless\"`,
      );

      await config.compiler?.runAfterProductionCompile?.({
        distDir: ".next",
        projectDir,
      });

      const registrations = releaseRequests.filter(
        (request) =>
          request.method === "POST" &&
          new URL(request.url).pathname === "/v3/releases",
      );

      expect(registrations).toHaveLength(1);

      await expect(registrations[0]?.json()).resolves.toMatchObject({
        destination: { provider: "cli" },
      });

      expect(
        releaseRequests.filter((request) =>
          request.headers.has("x-interfere-public-key"),
        ),
      ).toStrictEqual([]);
    });
  });

  test("bakes the registered release token without a public key and creates the release once across config evaluations", async () => {
    expect.hasAssertions();

    await withTempProject("next-release-token-", async (projectDir) => {
      prepareBuildProject(projectDir, "cli", "interfere_ak_test_123");

      vi.stubEnv("INTERFERE_PUBLIC_KEY", undefined);

      createReleaseFixture(projectDir, { releaseToken: "irt_next_build" });

      const nextConfig = withInterfere();

      await nextConfig(PHASE_PRODUCTION_BUILD);

      const config = await nextConfig(PHASE_PRODUCTION_BUILD);

      const baked = JSON.stringify(config.turbopack);

      expect(baked).toContain(
        String.raw`\"__INTERFERE_RELEASE_TOKEN__\":\"irt_next_build\"`,
      );

      expect(baked).toContain(
        String.raw`\"__INTERFERE_API_URL__\":\"https://in.example.com\"`,
      );

      expect(baked).not.toContain("__INTERFERE_PUBLIC_KEY__");
      expect(baked).not.toContain("__INTERFERE_DISABLED__");
      expect(config.experimental?.clientTraceMetadata).toContain("traceparent");

      await config.compiler?.runAfterProductionCompile?.({
        distDir: ".next",
        projectDir,
      });

      expect(
        releaseRequests
          .filter((request) => request.method === "POST")
          .map((request) => new URL(request.url).pathname)
          .filter((pathname) => pathname === "/v3/releases"),
      ).toStrictEqual(["/v3/releases"]);

      expect(
        releaseRequests.filter((request) =>
          request.headers.has("x-interfere-public-key"),
        ),
      ).toStrictEqual([]);
    });
  });

  test("the dev server mints the shared local-development release and bakes its token and slug without a public key", async () => {
    expect.hasAssertions();

    await withTempProject("next-dev-release-token-", async (projectDir) => {
      prepareBuildProject(projectDir, "cli", "interfere_ak_test_123");

      vi.stubEnv("INTERFERE_PUBLIC_KEY", undefined);

      createReleaseFixture(projectDir, { releaseToken: "irt_next_dev" });

      const nextConfig = withInterfere();

      await nextConfig(PHASE_DEVELOPMENT_SERVER);

      const config = await nextConfig(PHASE_DEVELOPMENT_SERVER);

      const baked = JSON.stringify(config.turbopack);

      expect(baked).toContain(
        String.raw`\"__INTERFERE_RELEASE_TOKEN__\":\"irt_next_dev\"`,
      );

      expect(baked).toContain(
        String.raw`\"__INTERFERE_RELEASE_SLUG__\":\"${LOCAL_DEV_RELEASE_SLUG}\"`,
      );

      expect(baked).not.toContain("__INTERFERE_PUBLIC_KEY__");

      const registrations = releaseRequests.filter(
        (request) =>
          request.method === "POST" &&
          new URL(request.url).pathname === "/v3/releases",
      );

      expect(registrations).toHaveLength(1);

      expect(registrations[0]?.headers.has("x-interfere-public-key")).toBe(
        false,
      );

      expect(
        createReleaseRequest.parse(await registrations[0]?.clone().json()),
      ).toMatchObject({ destination: null, slug: LOCAL_DEV_RELEASE_SLUG });
    });
  });

  test("the dev server mints no release when a public key is set", async () => {
    expect.hasAssertions();

    await withTempProject("next-dev-public-key-", async (projectDir) => {
      prepareBuildProject(projectDir, "cli", "interfere_ak_test_123");

      vi.stubEnv("INTERFERE_PUBLIC_KEY", "interfere_pk_test_123");

      createReleaseFixture(projectDir, { releaseToken: "irt_next_dev" });

      const config = await withInterfere()(PHASE_DEVELOPMENT_SERVER);

      expect(JSON.stringify(config.turbopack)).not.toContain(
        "__INTERFERE_RELEASE_TOKEN__",
      );

      expect(
        releaseRequests.filter((request) => request.method === "POST"),
      ).toHaveLength(0);
    });
  });

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
