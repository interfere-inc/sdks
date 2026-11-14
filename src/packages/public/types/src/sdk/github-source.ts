import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { releaseSourceIdEnvKeys } from "../integrations/index.ts";
import type { ReleaseSourceMetadata } from "../integrations/index.ts";
import { parseEnvValue, readFirstEnvValue } from "./env.ts";

type GithubReleaseSource = Extract<
  ReleaseSourceMetadata,
  { provider: "github" }
>;

/**
 * Runs a git command, returning `null` when git is unavailable. Injected
 * rather than imported so this module stays free of `node:child_process` —
 * it is reachable from bundles that never run in Node.
 */
export type RunGitCommand = (command: string) => string | null;

/**
 * The single answer to "what commit is this build?". The release slug and
 * `source_metadata.commitSha` must resolve identically or the release
 * registers with a working slug and a null ref, and every source tool 404s
 * against it for the life of that release.
 */
export function resolveReleaseCommitSha(
  env: Record<string, string | undefined>,
  runGit: RunGitCommand,
) {
  return (
    readFirstEnvValue(env, releaseSourceIdEnvKeys) ??
    runGit("git rev-parse HEAD")
  );
}

export function resolveGithubReleaseSource(
  env: Record<string, string | undefined>,
  runGit: RunGitCommand,
) {
  return omitUndefined({
    authorLogin:
      parseEnvValue(env["VERCEL_GIT_COMMIT_AUTHOR_LOGIN"]) ?? undefined,
    branch:
      parseEnvValue(env["VERCEL_GIT_COMMIT_REF"]) ??
      parseEnvValue(env["GITHUB_REF_NAME"]) ??
      parseEnvValue(env["GITHUB_HEAD_REF"]) ??
      runGit("git rev-parse --abbrev-ref HEAD") ??
      "unknown",
    commitMessage:
      parseEnvValue(env["VERCEL_GIT_COMMIT_MESSAGE"]) ??
      runGit("git log -1 --pretty=%B") ??
      "",
    commitSha: resolveReleaseCommitSha(env, runGit),
    provider: "github" as const,
  }) satisfies GithubReleaseSource;
}
