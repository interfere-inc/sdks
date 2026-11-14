import {
  configuredDestinationProviderSchema,
  configuredSourceProviderSchema,
  releaseDestinationIdEnvKeys,
  releaseDestinationMetadataSchema,
  releaseDestinationProviderSchema,
  releaseSourceIdEnvKeys,
  releaseSourceMetadataSchema,
  releaseSourceProviderSchema,
} from "./index.ts";
import { describe, expect, test } from "vitest";

describe("release source providers", () => {
  test("keeps CLI release metadata separate from configured source providers", () => {
    expect(releaseSourceProviderSchema.options).toStrictEqual([
      "github",
      "cli",
    ]);

    expect(configuredSourceProviderSchema.options).toStrictEqual([
      "github",
      "ghe_cloud",
      "ghe_server",
    ]);

    expect(configuredSourceProviderSchema.safeParse("cli").success).toBe(false);

    expect(
      releaseSourceMetadataSchema.safeParse({
        commitMessage: "feat: cli release",
        commitRef: "main",
        commitSha: "abc123",
        commitUrl: null,
        provider: "cli",
        runId: "42",
        runUrl: "https://github.com/acme/app/actions/runs/42",
        runnerName: "github-actions",
      }).success,
    ).toBe(true);
  });
});

describe("release destination providers", () => {
  test("accepts CLI destination metadata for deployment metadata providers", () => {
    expect(releaseDestinationProviderSchema.options).toStrictEqual([
      "vercel",
      "cloudflare",
      "cli",
    ]);

    expect(configuredDestinationProviderSchema.options).toStrictEqual([
      "vercel",
      "cloudflare",
      "cli",
    ]);

    expect(
      releaseDestinationMetadataSchema.safeParse({
        deploymentUrl: null,
        destinationReleaseId: "rel_abc123",
        environment: "production",
        provider: "cli",
      }).success,
    ).toBe(true);
  });
});

describe("releaseSourceIdEnvKeys contract", () => {
  test("includes source override and source integration keys", () => {
    expect(releaseSourceIdEnvKeys).toContain("NEXT_PUBLIC_INTERFERE_BUILD_ID");
    expect(releaseSourceIdEnvKeys).toContain("INTERFERE_SOURCE_ID");
    expect(releaseSourceIdEnvKeys).toContain("NEXT_BUILD_ID");
    expect(releaseSourceIdEnvKeys).toContain("VERCEL_GIT_COMMIT_SHA");
    expect(releaseSourceIdEnvKeys).toContain("GITHUB_SHA");
  });
});

describe("releaseDestinationIdEnvKeys contract", () => {
  test("includes destination override and integration keys", () => {
    expect(releaseDestinationIdEnvKeys).toContain(
      "NEXT_PUBLIC_INTERFERE_RELEASE_ID",
    );

    expect(releaseDestinationIdEnvKeys).not.toContain(
      "INTERFERE_DESTINATION_ID",
    );

    expect(releaseDestinationIdEnvKeys).toContain("VERCEL_DEPLOYMENT_ID");
    expect(releaseDestinationIdEnvKeys).toContain("VERCEL_GIT_COMMIT_SHA");
    expect(releaseDestinationIdEnvKeys).toContain("GITHUB_SHA");
  });
});
