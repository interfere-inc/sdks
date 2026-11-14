import { releaseSlugSchema } from "@interfere/types/releases/slug";
import { releaseSourceIdEnvKeys } from "@interfere/types/integrations";

import { execFileSync } from "node:child_process";
import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { resolveReleaseRequest } from "./release-request";
import {
  afterAll,
  afterEach,
  beforeAll,
  beforeEach,
  expect,
  test,
  vi,
} from "vitest";

let directory: string;

let gitSha: string;

beforeAll(() => {
  directory = mkdtempSync(path.join(tmpdir(), "interfere-release-git-"));

  execFileSync(
    "git",
    ["init", "--initial-branch=main", path.join(directory, "present")],
    { stdio: "ignore" },
  );

  execFileSync("git", ["init", path.join(directory, "empty")], {
    stdio: "ignore",
  });

  execFileSync(
    "git",
    [
      "-C",
      path.join(directory, "present"),
      "-c",
      "user.name=SDK Test",
      "-c",
      "user.email=sdk@example.com",
      "-c",
      "core.hooksPath=/dev/null",
      "commit",
      "--allow-empty",
      "--no-gpg-sign",
      "-m",
      "ship release",
    ],
    { stdio: "ignore" },
  );

  gitSha = execFileSync(
    "git",
    ["-C", path.join(directory, "present"), "rev-parse", "HEAD"],
    { encoding: "utf8" },
  ).trim();
});

beforeEach(() => {
  for (const key of releaseSourceIdEnvKeys) {
    vi.stubEnv(key, undefined);
  }

  vi.stubEnv("GIT_DIR", path.join(directory, "present/.git"));

  vi.stubEnv("VERCEL_GIT_COMMIT_MESSAGE", undefined);

  vi.stubEnv("VERCEL_GIT_COMMIT_REF", undefined);
});

afterEach(() => {
  vi.unstubAllEnvs();
});

afterAll(() => {
  rmSync(directory, { recursive: true, force: true });
});

const input = {
  buildId: "build",
  config: {
    org: { id: "org", slug: "org" },
    surface: {
      destinationProvider: "vercel" as const,
      environmentFiltering: true,
      id: "surface",
      sdkConfig: null,
      slug: "web",
      sourceProvider: "github" as const,
    },
  },
  environment: "staging",
  producerVersion: "@interfere/next@1.0.0",
  releaseSlug: releaseSlugSchema.parse("rel_aaaaaaaaaaaaaaaa"),
};

test.each([
  { field: "sourceProvider", reason: "missing_source_provider" },
  { field: "destinationProvider", reason: "missing_destination_provider" },
] as const)("rejects missing $field", ({ field, reason }) => {
  expect(
    resolveReleaseRequest({
      ...input,
      config: {
        ...input.config,
        surface: { ...input.config.surface, [field]: null },
      },
    }),
  ).toStrictEqual({
    ok: false,
    reason,
  });
});

test.each(["vercel", "cloudflare", "cli"] as const)(
  "resolves %s metadata with the explicit deploy environment",
  (destinationProvider) => {
    vi.stubEnv("GITHUB_SHA", "a".repeat(40));

    vi.stubEnv("GITHUB_REF_NAME", "main");

    vi.stubEnv("VERCEL_ENV", "production");

    expect(
      resolveReleaseRequest({
        ...input,
        config: {
          ...input.config,
          surface: { ...input.config.surface, destinationProvider },
        },
      }),
    ).toMatchObject({
      ok: true,
      request: {
        buildId: "build",
        destination: { environment: "staging", provider: destinationProvider },
        producerVersion: input.producerVersion,
        slug: input.releaseSlug,
        source: { commitSha: "a".repeat(40) },
      },
    });
  },
);

test.each(["ghe_cloud", "ghe_server"] as const)(
  "resolves %s commits through the shared GitHub resolver",
  (sourceProvider) => {
    vi.stubEnv("GITHUB_SHA", "b".repeat(40));

    expect(
      resolveReleaseRequest({
        ...input,
        config: {
          ...input.config,
          surface: { ...input.config.surface, sourceProvider },
        },
      }),
    ).toMatchObject({
      ok: true,
      request: { source: { commitSha: "b".repeat(40) } },
    });
  },
);

test("handles empty git metadata without CI variables", () => {
  for (const key of [
    "GITHUB_SHA",
    "VERCEL_GIT_COMMIT_SHA",
    "CF_PAGES_COMMIT_SHA",
    "WORKERS_CI_COMMIT_SHA",
    "GITHUB_REF_NAME",
    "GITHUB_REF",
    "VERCEL_GIT_COMMIT_REF",
  ]) {
    vi.stubEnv(key, "");
  }

  vi.stubEnv("GIT_DIR", path.join(directory, "empty/.git"));

  expect(resolveReleaseRequest(input)).toMatchObject({
    ok: true,
    request: {
      source: { commitSha: null },
    },
  });
});

test("handles unavailable git metadata without CI variables", () => {
  for (const key of [
    "GITHUB_SHA",
    "VERCEL_GIT_COMMIT_SHA",
    "CF_PAGES_COMMIT_SHA",
    "WORKERS_CI_COMMIT_SHA",
    "GITHUB_REF_NAME",
    "GITHUB_REF",
    "VERCEL_GIT_COMMIT_REF",
  ]) {
    vi.stubEnv(key, "");
  }

  vi.stubEnv("GIT_DIR", path.join(directory, "missing"));

  expect(resolveReleaseRequest(input)).toMatchObject({
    ok: true,
    request: {
      source: { commitSha: null },
    },
  });
});

test("handles present git metadata without CI variables", () => {
  for (const key of [
    "GITHUB_SHA",
    "VERCEL_GIT_COMMIT_SHA",
    "CF_PAGES_COMMIT_SHA",
    "WORKERS_CI_COMMIT_SHA",
    "GITHUB_REF_NAME",
    "GITHUB_REF",
    "VERCEL_GIT_COMMIT_REF",
  ]) {
    vi.stubEnv(key, "");
  }

  vi.stubEnv("GIT_DIR", path.join(directory, "present/.git"));

  expect(resolveReleaseRequest(input)).toMatchObject({
    ok: true,
    request: {
      source: { commitSha: gitSha },
    },
  });
});
