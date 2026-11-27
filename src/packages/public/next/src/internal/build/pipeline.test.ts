import { releaseSlugSchema } from "@interfere/types/releases/slug";
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";
import {
  RELEASE_RESPONSE,
  RELEASE_SLUG,
  createReleaseFixture,
  releaseRequests,
  withTempProject,
} from "../../../test/fixtures/build.js";
import { log } from "../logger.js";
import { runBuildPipeline } from "./pipeline.js";

const metadata = {
  apiKey: "interfere_ak_test_123",
  apiUrl: "https://in.example.com",
  buildId: "abc123def4567890abc123def4567890abc123de",
  environment: "staging",
  purgeSourceMaps: true,
  releaseSlug: releaseSlugSchema.parse(RELEASE_SLUG),
  uploadSourceMaps: true,
};

beforeEach(() => {
  vi.stubEnv("TURBOPACK", undefined);

  vi.stubEnv("GITHUB_REPOSITORY", "acme/app");

  vi.stubEnv("GITHUB_SERVER_URL", "https://github.com");
});

afterEach(() => {
  vi.restoreAllMocks();

  vi.unstubAllEnvs();

  vi.unstubAllGlobals();
});

describe("source-map pipeline cleanup", () => {
  test.each(["webpack", "turbopack"])(
    "purges unreferenced maps before %s output is copied for deployment",
    async (bundler) => {
      expect.hasAssertions();

      vi.stubEnv("TURBOPACK", bundler === "turbopack" ? "1" : undefined);

      await withTempProject("pipeline-orphan-map-", async (projectDir) => {
        const { map } = createReleaseFixture(projectDir);
        const orphan = map.replace("app.js.map", "unreferenced.js.map");

        writeFileSync(orphan, '{"version":3,"sources":[],"mappings":""}');

        await runBuildPipeline({ distDir: ".next", projectDir }, metadata);

        expect(existsSync(map)).toBe(false);
        expect(existsSync(orphan)).toBe(false);
      });
    },
  );

  test("purges generated maps even when discovery rejects a malformed map", async () => {
    expect.hasAssertions();

    await withTempProject("pipeline-malformed-map-", async (projectDir) => {
      const { chunk, map } = createReleaseFixture(projectDir);

      writeFileSync(map, "{");

      await expect(
        runBuildPipeline({ distDir: ".next", projectDir }, metadata),
      ).rejects.toThrow(SyntaxError);

      expect(existsSync(map)).toBe(false);
      expect(readFileSync(chunk, "utf8")).not.toContain("sourceMappingURL");
    });
  });

  test("reports uncovered chunks before returning for an empty map inventory", async () => {
    expect.hasAssertions();

    await withTempProject("pipeline-no-maps-", async (projectDir) => {
      const { chunk } = createReleaseFixture(projectDir, { maps: false });

      writeFileSync(chunk, "function action(){}\n//# debugId=abcdef-abcd\n");

      const error = vi.spyOn(log, "error");

      await expect(
        runBuildPipeline({ distDir: ".next", projectDir }, metadata),
      ).resolves.toMatchObject({ accepted: false, reason: "no_source_maps" });

      expect(error).toHaveBeenCalledWith(
        "Source-map coverage is incomplete",
        expect.arrayContaining(["static/chunks/app.js"]),
      );
    });
  });

  test("registers under the build environment's deploy target when the config lookup fails", async () => {
    expect.hasAssertions();

    await withTempProject("pipeline-keyless-", async (projectDir) => {
      createReleaseFixture(projectDir, { failure: "config" });

      await expect(
        runBuildPipeline({ distDir: ".next", projectDir }, metadata),
      ).resolves.toMatchObject({ accepted: true });

      const registration = releaseRequests.find(
        (request) =>
          request.method === "POST" &&
          new URL(request.url).pathname === "/v3/releases",
      );

      await expect(registration?.json()).resolves.toMatchObject({
        destination: { provider: "cli" },
      });
    });
  });

  test.each([
    "register",
    "sign",
    "upload",
    "finalize",
    "timeout",
    "incomplete",
  ] as const)("purges maps after %s failure", async (failure) => {
    expect.hasAssertions();

    await withTempProject("pipeline-failure-", async (projectDir) => {
      const { chunk, map } = createReleaseFixture(projectDir, { failure });

      const result = runBuildPipeline(
        { distDir: ".next", projectDir },
        metadata,
      );

      await expect(result).rejects.toThrow(
        /original pipeline failure|complete batch|timeout|timed out|abort/iu,
      );

      expect(existsSync(map)).toBe(false);
      expect(readFileSync(chunk, "utf8")).not.toContain("sourceMappingURL");
    });
  });

  test("keeps maps after failure when purge is false", async () => {
    expect.hasAssertions();

    await withTempProject("pipeline-keep-", async (projectDir) => {
      const { chunk, map } = createReleaseFixture(projectDir, {
        failure: "upload",
      });

      await expect(
        runBuildPipeline(
          { distDir: ".next", projectDir },
          { ...metadata, purgeSourceMaps: false },
        ),
      ).rejects.toMatchObject({ statusCode: 422 });

      expect(existsSync(map)).toBe(true);
      expect(readFileSync(chunk, "utf8")).toContain("sourceMappingURL");
    });
  });

  test("reports cleanup failure without replacing the original finalize error", async () => {
    expect.hasAssertions();

    await withTempProject("pipeline-cleanup-failure-", async (projectDir) => {
      const { chunk } = createReleaseFixture(projectDir, {
        failure: "finalize",
        cleanupFailure: true,
      });

      const warning = vi.spyOn(log, "warn");

      await expect(
        runBuildPipeline({ distDir: ".next", projectDir }, metadata),
      ).rejects.toMatchObject({ statusCode: 422 });

      expect(warning).toHaveBeenCalledWith(
        "Source-map cleanup left files on disk",
        [expect.stringContaining(chunk)],
      );
    });
  });

  test("keeps the upload error when telemetry reporting also fails", async () => {
    expect.hasAssertions();

    await withTempProject("pipeline-report-failure-", async (projectDir) => {
      const { map } = createReleaseFixture(projectDir, {
        failure: "upload",
        telemetryFailure: true,
      });

      await expect(
        runBuildPipeline({ distDir: ".next", projectDir }, metadata),
      ).rejects.toMatchObject({ statusCode: 422 });

      expect(existsSync(map)).toBe(false);

      expect(
        releaseRequests.some((request) =>
          new URL(request.url).pathname.endsWith("/telemetry"),
        ),
      ).toBe(true);
    });
  });

  test("returns cleanup failures after successful upload", async () => {
    expect.hasAssertions();

    await withTempProject("pipeline-cleanup-success-", async (projectDir) => {
      const { chunk } = createReleaseFixture(projectDir, {
        cleanupFailure: true,
      });

      const result = await runBuildPipeline(
        { distDir: ".next", projectDir },
        metadata,
      );

      expect(result).toMatchObject({
        accepted: true,
        cleanupFailures: [{ absolute: chunk }],
      });
    });
  });
});

describe("disabled source-map uploads", () => {
  test("skips malformed maps when uploads and purge are both disabled", async () => {
    expect.hasAssertions();

    await withTempProject("pipeline-no-discovery-", async (projectDir) => {
      const { map } = createReleaseFixture(projectDir);

      writeFileSync(map, "not a source map");

      await expect(
        runBuildPipeline(
          { distDir: ".next", projectDir },
          { ...metadata, uploadSourceMaps: false, purgeSourceMaps: false },
        ),
      ).resolves.toMatchObject({ accepted: true });

      expect(readFileSync(map, "utf8")).toBe("not a source map");
    });
  });

  test.each([true, false])(
    "registers without source-map requests with purge=%s",
    async (purgeSourceMaps) => {
      expect.hasAssertions();

      await withTempProject("pipeline-disabled-", async (projectDir) => {
        const { chunk, map } = createReleaseFixture(projectDir);

        const result = await runBuildPipeline(
          { distDir: ".next", projectDir },
          { ...metadata, purgeSourceMaps, uploadSourceMaps: false },
        );

        const requests = releaseRequests;

        expect(
          requests.some((request) => request.url.includes("source-maps")),
        ).toBe(false);

        expect(
          requests.some(
            (request) => new URL(request.url).pathname === "/v3/releases",
          ),
        ).toBe(true);

        expect(result).toMatchObject({
          accepted: true,
          fileCount: 0,
          release: RELEASE_RESPONSE,
        });

        const telemetry = requests.find((request) =>
          new URL(request.url).pathname.endsWith("/telemetry"),
        );

        expect(telemetry?.url).toBe(
          `https://in.example.com/v3/releases/${RELEASE_SLUG}/telemetry`,
        );

        await expect(telemetry?.json()).resolves.toMatchObject({
          result: "ok",
          totalBytes: 0,
        });

        expect(existsSync(map)).toBe(!purgeSourceMaps);

        expect(readFileSync(chunk, "utf8").includes("sourceMappingURL")).toBe(
          !purgeSourceMaps,
        );
      });
    },
  );

  test("registers a release even without source-map files", async () => {
    expect.hasAssertions();

    await withTempProject("pipeline-no-maps-", async (projectDir) => {
      createReleaseFixture(projectDir, { maps: false });

      const result = await runBuildPipeline(
        { distDir: ".next", projectDir },
        { ...metadata, uploadSourceMaps: false },
      );

      expect(result).toMatchObject({
        accepted: true,
        release: RELEASE_RESPONSE,
      });

      expect(
        releaseRequests.some((request) => request.url.includes("source-maps")),
      ).toBe(false);
    });
  });
});

describe("incomplete Surface setup", () => {
  test.each(["sourceProvider", "destinationProvider"] as const)(
    "registers the release and uploads source maps without %s",
    async (provider) => {
      expect.hasAssertions();

      await withTempProject("pipeline-unmapped-", async (projectDir) => {
        const { map } = createReleaseFixture(projectDir, {
          [provider]: null,
        });

        await expect(
          runBuildPipeline({ distDir: ".next", projectDir }, metadata),
        ).resolves.toMatchObject({ accepted: true, fileCount: 1 });

        expect(
          releaseRequests.map((request) => new URL(request.url).pathname),
        ).toContain("/v3/releases");

        expect(existsSync(map)).toBe(false);
      });
    },
  );
});
