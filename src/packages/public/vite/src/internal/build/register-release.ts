import { resolveReleaseRequest } from "@interfere/build/release-request";
import type { Interfere } from "@interfere/javascript";
import { InterfereClient } from "@interfere/javascript";
import { createReleaseResponse } from "@interfere/types/releases/definition";
import { localDevReleaseRequest } from "@interfere/types/releases/local-dev";
import type { CreateReleaseRequest } from "@interfere/types/releases/definition";
import type { ReleaseSlug } from "@interfere/types/releases/slug";
import { PRODUCER_VERSION } from "../../version";
import {
  LOCAL_DEV_RELEASE_BUDGET_MS,
  RELEASE_REQUEST_BUDGET_MS,
} from "./request-budget";

interface ReleaseCredentials {
  apiUrl: string;
  apiKey: string;
  serviceName: string | undefined;
}

async function createRelease(
  input: ReleaseCredentials,
  request: CreateReleaseRequest,
  budgetMs: number,
) {
  const client = new InterfereClient({
    baseUrl: input.apiUrl,
    apiKey: { token: input.apiKey },
    headers: {
      "x-interfere-producer-version": PRODUCER_VERSION,
      "x-interfere-service": input.serviceName,
    },
    maxRetries: 0,
    timeoutInSeconds: 10,
  });

  try {
    return createReleaseResponse.parse(
      await client.releases.createRelease(request, {
        abortSignal: AbortSignal.timeout(budgetMs),
      }),
    );
  } catch {
    return null;
  }
}

export async function registerBuildRelease(
  input: ReleaseCredentials & {
    commitSha: string;
    environment: string | null;
    releaseSlug: ReleaseSlug;
    releasesConfig: Interfere.ReleasesConfigResponse | null;
  },
) {
  return createRelease(
    input,
    resolveReleaseRequest({
      buildId: input.commitSha,
      config: input.releasesConfig,
      environment: input.environment,
      producerVersion: PRODUCER_VERSION,
      releaseSlug: input.releaseSlug,
      service: input.serviceName ? { name: input.serviceName } : undefined,
    }),
    RELEASE_REQUEST_BUDGET_MS,
  );
}

export async function registerLocalDevRelease(input: ReleaseCredentials) {
  return createRelease(
    input,
    localDevReleaseRequest({
      producerVersion: PRODUCER_VERSION,
      service: input.serviceName ? { name: input.serviceName } : undefined,
    }),
    LOCAL_DEV_RELEASE_BUDGET_MS,
  );
}
