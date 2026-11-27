import { createReleaseRequest } from "@interfere/types/releases/definition";
import { LOCAL_DEV_RELEASE_SLUG } from "@interfere/types/releases/slug";
import { writeFileSync } from "node:fs";
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

        createReleaseFixture(projectDir, { [provider]: null });

        const config = await withInterfere()(PHASE_PRODUCTION_BUILD);

        const baked = JSON.stringify(config.turbopack);

        expect(baked).not.toContain("__INTERFERE_DISABLED__");
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
});
