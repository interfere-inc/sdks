import { resolveReleaseRequest as resolve } from "@interfere/build/release-request";

import type { ReleaseSlug } from "@interfere/types/releases/slug";

import { PRODUCER_VERSION } from "../../version.js";

export function resolveReleaseRequest(
  buildId: string,
  environment: string | null,
  releaseSlug: ReleaseSlug,
  config: Parameters<typeof resolve>[0]["config"],
  service?: Parameters<typeof resolve>[0]["service"],
) {
  return resolve({
    buildId,
    config,
    environment,
    producerVersion: PRODUCER_VERSION,
    releaseSlug,
    service,
  });
}
