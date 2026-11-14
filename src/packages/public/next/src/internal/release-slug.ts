import {
  deriveReleaseSlug,
  releaseSlugSchema,
} from "@interfere/types/releases/slug";
import type { ReleaseSlug } from "@interfere/types/releases/slug";
import { resolveReleaseCommitSha } from "@interfere/types/sdk/github-source";
import type { RunGitCommand } from "@interfere/types/sdk/github-source";
import { RELEASE_SLUG_GLOBAL } from "@interfere/types/sdk/injected-globals";

const noGit: RunGitCommand = () => null;

function readBakedReleaseSlug() {
  const globals: object = globalThis;

  if (!(RELEASE_SLUG_GLOBAL in globals)) {
    return null;
  }

  const parsed = releaseSlugSchema.safeParse(globals[RELEASE_SLUG_GLOBAL]);

  return (parsed.success ? parsed.data : null) satisfies ReleaseSlug | null;
}

export function resolveReleaseSlug(runGit: RunGitCommand = noGit) {
  const baked = readBakedReleaseSlug();

  if (baked) {
    return { commitSha: null, slug: baked };
  }

  // oxlint-disable-next-line node/no-process-env
  const commitSha = resolveReleaseCommitSha(process.env, runGit);

  return {
    commitSha,
    slug: commitSha ? deriveReleaseSlug(commitSha) : null,
  };
}
