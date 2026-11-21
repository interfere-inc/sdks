/// <reference types="vite/client" />
import {
  deriveReleaseSlug,
  releaseSlugSchema,
} from "@interfere/types/releases/slug";
import { resolveReleaseCommitSha } from "@interfere/types/sdk/github-source";
import type { RunGitCommand } from "@interfere/types/sdk/github-source";

import { bakedReleaseSlug } from "./baked-release";

const noGit: RunGitCommand = () => null;

export function resolveCommitSha(runGit: RunGitCommand = noGit) {
  // oxlint-disable-next-line node/no-process-env
  return resolveReleaseCommitSha(process.env, runGit);
}

export function resolveReleaseSlug(runGit: RunGitCommand = noGit) {
  const baked = releaseSlugSchema.safeParse(bakedReleaseSlug());

  if (baked.success) {
    return { commitSha: null, slug: baked.data };
  }

  const commitSha = resolveCommitSha(runGit);

  return {
    commitSha,
    slug: commitSha ? deriveReleaseSlug(commitSha) : null,
  };
}
