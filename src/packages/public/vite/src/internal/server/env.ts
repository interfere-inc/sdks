import { API_URL } from "@interfere/constants/api";
import { parseEnvValue } from "@interfere/types/sdk/env";
import {
  readProcessEnv,
  resolveEnvironment,
} from "@interfere/types/sdk/runtime";

import { bakedReleaseToken } from "../baked-release";

interface InterfereServerEnv {
  readonly apiKey: string | null;
  readonly apiUrl: string;
  readonly nodeEnvironment: string;
  readonly releaseToken: string | null;
}

/**
 * Server-side gate for the `captureError` / `onRequestError` helpers, request
 * tracing and the OTel register path. Server telemetry authenticates with the
 * workspace API key (`INTERFERE_API_KEY`, `org:telemetry:write`), read from the
 * runtime environment and never baked into a bundle, else with the release token
 * the build baked into the bundle. Collector ingress remains the authority on whether local / unknown /
 * production events are allowed.
 */
export function isEnabledOnServer() {
  if (import.meta.env.VITE_INTERFERE_DISABLED) {
    return false;
  }

  if (readProcessEnv()?.["NODE_ENV"] === "production") {
    return true;
  }

  return readApiKey() !== null || readReleaseToken() !== null;
}

export function readInterfereServerEnv() {
  const nodeEnvironment = resolveEnvironment({
    buildMode: import.meta.env.MODE,
    env: readProcessEnv() ?? {},
    explicit: import.meta.env.VITE_INTERFERE_ENVIRONMENT,
  });

  const disabled = Boolean(import.meta.env.VITE_INTERFERE_DISABLED);

  return {
    apiKey: disabled ? null : readApiKey(),
    apiUrl: parseEnvValue(readProcessEnv()?.["INTERFERE_API_URL"]) ?? API_URL,
    nodeEnvironment,
    releaseToken: disabled ? null : readReleaseToken(),
  } satisfies InterfereServerEnv;
}

function readApiKey() {
  return parseEnvValue(readProcessEnv()?.["INTERFERE_API_KEY"]);
}

function readReleaseToken() {
  return bakedReleaseToken() || null;
}
