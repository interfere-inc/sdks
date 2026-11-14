import { resolveReleaseRequest as resolve } from "@interfere/build/release-request";

import type { ReleaseSlug } from "@interfere/types/releases/slug";

import { log } from "../../logger.js";
import { PRODUCER_VERSION } from "../../version.js";

export function resolveReleaseRequest(
  buildId: string,
  environment: string | null,
  releaseSlug: ReleaseSlug,
  config: Parameters<typeof resolve>[0]["config"],
) {
  const result = resolve({
    buildId,
    config,
    environment,
    producerVersion: PRODUCER_VERSION,
    releaseSlug,
  });

  if (result.ok) {
    return result;
  }

  const sourceMissing = result.reason === "missing_source_provider";

  log.warn(
    sourceMissing
      ? "Release skipped: missing version control provider"
      : "Release skipped: missing deployment target",
    [
      sourceMissing
        ? "This surface does not have a version control provider configured."
        : "This surface does not have a deployment target integration configured.",
      `Configure one at https://interfere.com/~/${config.org.slug}/apps/${config.surface.slug}`,
    ],
  );

  return result;
}
