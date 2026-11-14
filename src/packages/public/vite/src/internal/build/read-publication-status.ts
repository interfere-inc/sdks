import { sourceMapPublicationStatus } from "@interfere/types/data/source-map-status";

import type { ReleasePipelineInput } from "./pipeline";

export async function readPublicationStatus(
  input: Pick<ReleasePipelineInput, "client" | "releaseSlug" | "environment">,
) {
  try {
    return sourceMapPublicationStatus.parse(
      await input.client.releases.getSourceMapPublicationStatus(
        {
          releaseSlug: input.releaseSlug,
          environment: input.environment ?? undefined,
        },
        { abortSignal: AbortSignal.timeout(10_000) },
      ),
    );
  } catch {
    return null;
  }
}
