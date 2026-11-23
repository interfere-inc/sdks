import { execSync } from "node:child_process";
import {
  githubReleaseSourceSchema,
  releaseSourceIdEnvKeys,
} from "../integrations/index.ts";
import {
  resolveGithubReleaseSource,
  resolveReleaseCommitSha,
} from "./github-source.ts";
import { beforeAll, describe, expect, test, vi } from "vitest";

const noGit = () => null;

const gitHead = { sha: "" };

beforeAll(() => {
  gitHead.sha = execSync("git rev-parse HEAD", { encoding: "utf8" }).trim();
});

describe(resolveReleaseCommitSha, () => {
  test.each(releaseSourceIdEnvKeys)("reads %s", (key) => {
    expect(resolveReleaseCommitSha({ [key]: "abc123" }, noGit)).toBe("abc123");
  });

  test("falls back to git when no env key is set", () => {
    const git = {
      run() {
        return gitHead.sha;
      },
    };

    const runGit = vi.spyOn(git, "run");

    expect(resolveReleaseCommitSha({}, runGit)).toBe(gitHead.sha);

    expect(runGit).toHaveBeenCalledWith("git rev-parse HEAD");
  });

  test("returns null when neither env nor git can answer", () => {
    expect(resolveReleaseCommitSha({}, noGit)).toBeNull();
  });
});

describe(resolveGithubReleaseSource, () => {
  test.each(releaseSourceIdEnvKeys)(
    "agrees with the slug resolver when the SHA comes from %s",
    (key) => {
      const env = { [key]: "abc123" };

      expect(resolveGithubReleaseSource(env, noGit).commitSha).toBe(
        resolveReleaseCommitSha(env, noGit),
      );
    },
  );

  test("registers a SHA from the documented INTERFERE_SOURCE_ID override", () => {
    const source = resolveGithubReleaseSource(
      { INTERFERE_SOURCE_ID: "0e0702a126ab0e0702a126ab0e0702a126ab0e07" },
      noGit,
    );

    expect(source.commitSha).toBe("0e0702a126ab0e0702a126ab0e0702a126ab0e07");
  });

  test("prefers platform git context for branch and message", () => {
    const source = resolveGithubReleaseSource(
      {
        VERCEL_GIT_COMMIT_MESSAGE: "ship it",
        VERCEL_GIT_COMMIT_REF: "main",
        VERCEL_GIT_COMMIT_SHA: "deadbeef",
      },
      noGit,
    );

    expect(source).toStrictEqual({
      branch: "main",
      commitMessage: "ship it",
      commitSha: "deadbeef",
      provider: "github",
    });
  });

  test("degrades to its literal fallbacks when nothing is available", () => {
    expect(resolveGithubReleaseSource({}, noGit)).toStrictEqual({
      branch: "unknown",
      commitMessage: "",
      commitSha: null,
      provider: "github",
    });
  });
});

test("captures the Vercel commit author without substituting the workflow actor", () => {
  expect(
    resolveGithubReleaseSource(
      { VERCEL_GIT_COMMIT_AUTHOR_LOGIN: "ada", GITHUB_ACTOR: "release-bot" },
      noGit,
    ).authorLogin,
  ).toBe("ada");

  expect(
    resolveGithubReleaseSource({ GITHUB_ACTOR: "release-bot" }, noGit)
      .authorLogin,
  ).toBeUndefined();
});

test("retains provider-reported GitHub Enterprise author metadata through the release contract", () => {
  const source = {
    provider: "github",
    branch: "main",
    commitMessage: "ship",
    commitSha: "abc123",
    authorLogin: "ada",
    authorAvatarUrl: "https://git.example.com/avatars/u/42",
  };

  expect(githubReleaseSourceSchema.parse(source)).toStrictEqual(source);
});
