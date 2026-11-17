import { resolveReleaseRequest } from "@interfere/build/release-request";
import { PUBLIC_KEY_HEADER } from "@interfere/constants/api";
import type { Interfere } from "@interfere/javascript";
import { InterfereClient } from "@interfere/javascript";
import { createReleaseResponse } from "@interfere/types/releases/definition";
import type { ReleaseSlug } from "@interfere/types/releases/slug";
import { PRODUCER_VERSION } from "../../version";
import { RELEASE_REQUEST_BUDGET_MS } from "./request-budget";

export async function registerBuildRelease(input: {
  apiUrl: string;
  apiKey: string;
  publicKey: string | undefined;
  serviceName: string | undefined;
  commitSha: string;
  environment: string | null;
  releaseSlug: ReleaseSlug;
  releasesConfig: Interfere.ReleasesConfigResponse;
}) {
  const request = resolveReleaseRequest({
    buildId: input.commitSha,
    config: input.releasesConfig,
    environment: input.environment,
    producerVersion: PRODUCER_VERSION,
    releaseSlug: input.releaseSlug,
  });

  if (!request.ok) {
    return null;
  }

  const client = new InterfereClient({
    baseUrl: input.apiUrl,
    apiKey: { token: input.apiKey },
    headers: {
      [PUBLIC_KEY_HEADER]: input.publicKey,
      "x-interfere-producer-version": PRODUCER_VERSION,
      "x-interfere-service": input.serviceName,
    },
    maxRetries: 0,
    timeoutInSeconds: 10,
  });

  try {
    return createReleaseResponse.parse(
      await client.releases.createRelease(request.request, {
        abortSignal: AbortSignal.timeout(RELEASE_REQUEST_BUDGET_MS),
      }),
    );
  } catch {
    return null;
  }
}
