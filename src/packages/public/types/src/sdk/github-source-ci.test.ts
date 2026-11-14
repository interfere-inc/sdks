import {
  resolveGithubReleaseSource,
  resolveReleaseCommitSha,
} from "./github-source.ts";
import { describe, expect, test } from "vitest";

const CI_COMMIT_KEYS = [
  "CI_COMMIT_SHA",
  "COMMIT_REF",
  "RENDER_GIT_COMMIT",
  "CIRCLE_SHA1",
  "BITBUCKET_COMMIT",
  "BUILD_SOURCEVERSION",
  "BUILDKITE_COMMIT",
  "RAILWAY_GIT_COMMIT_SHA",
  "AWS_COMMIT_ID",
  "SOURCE_VERSION",
] as const;

const COMMIT_SHA = "abc0123456789def0123456789abcdef01234567";

const noGit = () => null;

describe("CI commit SHA resolution", () => {
  test.each(CI_COMMIT_KEYS)("reads %s without a git checkout", (key) => {
    const env = { [key]: COMMIT_SHA };

    expect(resolveReleaseCommitSha(env, noGit)).toBe(COMMIT_SHA);
    expect(resolveGithubReleaseSource(env, noGit).commitSha).toBe(COMMIT_SHA);
  });

  test.each(CI_COMMIT_KEYS)("prefers the explicit source ID over %s", (key) => {
    expect(
      resolveReleaseCommitSha(
        { [key]: COMMIT_SHA, INTERFERE_SOURCE_ID: "explicit-source" },
        noGit,
      ),
    ).toBe("explicit-source");
  });

  test.each(CI_COMMIT_KEYS)("falls back to git when %s is blank", (key) => {
    expect(resolveReleaseCommitSha({ [key]: " " }, () => COMMIT_SHA)).toBe(
      COMMIT_SHA,
    );
  });
});
