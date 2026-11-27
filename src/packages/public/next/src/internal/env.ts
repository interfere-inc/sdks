import { API_URL } from "@interfere/constants/api";
import { getStringGlobal } from "@interfere/react/util/global";
import { parseEnvValue } from "@interfere/types/sdk/env";
import { ENVIRONMENT_GLOBAL } from "@interfere/types/sdk/injected-globals";
import { RELEASE_TOKEN_GLOBAL } from "@interfere/types/sdk/release/token";
import { resolveEnvironment } from "@interfere/types/sdk/runtime";
import { SERVICE_NAME_GLOBAL } from "@interfere/types/sdk/service-name";

import { interfereDisabled } from "./build-marker-guard.js";

interface InterfereEnv {
  readonly apiKey: string | null;
  readonly apiUrl: string;
  readonly nextRuntime: string | null;
  readonly nodeEnvironment: string;
  readonly releaseToken: string | null;
  readonly serviceName: string | null;
}

/** Enable server telemetry when a server credential (workspace API key or baked release token) is configured and the build has not disabled it. */
export function isEnabledOnServer() {
  if (interfereDisabled()) {
    return false;
  }

  return Boolean(
    parseEnvValue(process.env["INTERFERE_API_KEY"]) ?? bakedReleaseToken(),
  );
}

export function readInterfereEnv() {
  const nodeEnvironment = resolveEnvironment({
    env: process.env,
    explicit: getStringGlobal(ENVIRONMENT_GLOBAL),
  });

  return {
    serviceName: getStringGlobal(SERVICE_NAME_GLOBAL) ?? null,
    apiKey: interfereDisabled()
      ? null
      : parseEnvValue(process.env["INTERFERE_API_KEY"]),
    apiUrl: parseEnvValue(process.env["INTERFERE_API_URL"]) ?? API_URL,
    nextRuntime: parseEnvValue(process.env["NEXT_RUNTIME"]),
    nodeEnvironment,
    releaseToken: interfereDisabled() ? null : bakedReleaseToken(),
  } satisfies InterfereEnv;
}

function bakedReleaseToken() {
  return getStringGlobal(RELEASE_TOKEN_GLOBAL) || null;
}
