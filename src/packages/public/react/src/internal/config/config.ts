import { API_URL } from "@interfere/constants/api";
import { omitUndefined } from "@interfere/helpers/omit-undefined";
import {
  API_URL_GLOBAL,
  FORCE_ENABLE_GLOBAL,
  INGEST_URL_GLOBAL,
} from "@interfere/types/sdk/injected-globals";
import { RELEASE_TOKEN_GLOBAL } from "@interfere/types/sdk/release/token";

import { getGlobal, getStringGlobal, setGlobal } from "../../util/global.js";

function resolveReleaseToken() {
  return getStringGlobal(RELEASE_TOKEN_GLOBAL) || undefined;
}

/**
 * Every ingest route rejects requests without a credential, so a kernel with
 * no release token cannot deliver anything.
 */
export function hasIngestCredential() {
  return Boolean(resolveReleaseToken());
}

/**
 * Force enable is the explicit opt-in for telemetry outside production. It
 * also overrides the bot gate, so a browser under test can start a kernel.
 */
export function isForceEnableActive() {
  return Boolean(getGlobal(FORCE_ENABLE_GLOBAL));
}

function resolveApiHost(explicit: string | undefined) {
  return explicit ?? getStringGlobal(API_URL_GLOBAL);
}

function resolveIngestHost(explicit: string | undefined) {
  return explicit ?? getStringGlobal(INGEST_URL_GLOBAL);
}

export function setIngestHost(host: string) {
  setGlobal(INGEST_URL_GLOBAL, host);
}

function normalizeOrigin(host: string) {
  try {
    return new URL(host).origin;
  } catch {
    return API_URL;
  }
}

export function resolveTargets(
  apiHost?: string,
  ingestHost?: string,
  service?: {
    serviceName?: string | undefined;
    serviceNamespace?: string | undefined;
  },
) {
  const releaseToken = resolveReleaseToken();
  const headers: Record<string, string> = {};

  if (releaseToken) {
    headers["x-interfere-release-token"] = releaseToken;
  }

  if (isForceEnableActive()) {
    headers["x-interfere-force-enable"] = "1";
  }

  if (service?.serviceName) {
    headers["x-interfere-service"] = service.serviceName;

    if (service.serviceNamespace) {
      headers["x-interfere-service-namespace"] = service.serviceNamespace;
    }
  }

  const configHost = resolveApiHost(apiHost);
  const configBaseUrl = configHost ? normalizeOrigin(configHost) : API_URL;
  const resolvedIngestHost = resolveIngestHost(ingestHost);

  const ingestBaseUrl = resolvedIngestHost
    ? normalizeOrigin(resolvedIngestHost)
    : configBaseUrl;

  const options = omitUndefined({
    headers,
    releaseTokenHeader: releaseToken ? { apiKey: releaseToken } : undefined,
    timeoutInSeconds: 10,
    maxRetries: 0,
  });

  return {
    config: { ...options, baseUrl: configBaseUrl },
    ingest: { ...options, baseUrl: ingestBaseUrl },
  };
}
