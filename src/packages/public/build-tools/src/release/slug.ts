import { createHash } from "node:crypto";
import { deriveReleaseSlug } from "@interfere/types/releases/slug";
import { parseEnvValue } from "@interfere/types/sdk/env";

/** Derives a build-time artifact slug while preserving legacy identities without a deployment ID. */
export function resolveBuildReleaseSlug(input: {
  commitSha: string | null;
  deploymentId?: string | undefined;
  variant?: string | undefined;
}) {
  if (!input.commitSha) {
    return null;
  }

  const deploymentId = parseEnvValue(input.deploymentId);

  let identity: (string | null)[] | null = null;

  if (deploymentId) {
    identity = [
      "vercel-deployment",
      input.commitSha,
      deploymentId,
      input.variant ?? null,
    ];
  } else if (input.variant) {
    identity = [input.commitSha, input.variant];
  }

  return deriveReleaseSlug(
    identity
      ? createHash("sha256").update(JSON.stringify(identity)).digest("hex")
      : input.commitSha,
  );
}
