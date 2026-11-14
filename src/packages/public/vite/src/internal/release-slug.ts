/// <reference types="vite/client" />
import {
  deriveReleaseSlug,
  releaseSlugSchema,
} from "@interfere/types/releases/slug";
import { resolveReleaseCommitSha } from "@interfere/types/sdk/github-source";
import type { RunGitCommand } from "@interfere/types/sdk/github-source";

const noGit: RunGitCommand = () => null;

export function resolveCommitSha(runGit: RunGitCommand = noGit) {
  // oxlint-disable-next-line node/no-process-env
  return resolveReleaseCommitSha(process.env, runGit);
}

export function resolveReleaseSlug(runGit: RunGitCommand = noGit) {
  const baked = releaseSlugSchema.safeParse(
    // oxlint-disable-next-line typescript/no-unnecessary-condition, typescript/dot-notation -- Native Node omits env; downstream source consumers see only Vite's index signature.
    import.meta.env?.["VITE_INTERFERE_RELEASE_SLUG"],
  );

  if (baked.success) {
    return { commitSha: null, slug: baked.data };
  }

  const commitSha = resolveCommitSha(runGit);

  return {
    commitSha,
    slug: commitSha ? deriveReleaseSlug(commitSha) : null,
  };
}
