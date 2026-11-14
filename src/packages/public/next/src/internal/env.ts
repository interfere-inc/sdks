import { API_URL } from "@interfere/constants/api";
import { getStringGlobal } from "@interfere/react/util/global";
import { parseEnvValue } from "@interfere/types/sdk/env";
import { ENVIRONMENT_GLOBAL } from "@interfere/types/sdk/injected-globals";
import { resolveEnvironment } from "@interfere/types/sdk/runtime";
import { SERVICE_NAME_GLOBAL } from "@interfere/types/sdk/service-name";

import { interfereDisabled } from "./build-marker-guard.js";

interface InterfereEnv {
  readonly apiKey: string | null;
  readonly apiUrl: string;
  readonly nextRuntime: string | null;
  readonly nodeEnvironment: string;
  readonly publicKey: string | null;
  readonly serviceName: string | null;
  readonly release: {
    readonly sourceId: string | null;
    readonly destinationId: string | null;
  };
}

/** Enable server telemetry when its public key is configured and the build has not disabled it. */
export function isEnabledOnServer() {
  if (interfereDisabled()) {
    return false;
  }

  return Boolean(parseEnvValue(process.env["INTERFERE_PUBLIC_KEY"]));
}

export function readInterfereEnv() {
  const nodeEnvironment = resolveEnvironment({
    env: process.env,
    explicit: getStringGlobal(ENVIRONMENT_GLOBAL),
  });

  return {
    serviceName: getStringGlobal(SERVICE_NAME_GLOBAL) ?? null,
    apiKey: parseEnvValue(process.env["INTERFERE_API_KEY"]),
    apiUrl: parseEnvValue(process.env["INTERFERE_API_URL"]) ?? API_URL,
    nextRuntime: parseEnvValue(process.env["NEXT_RUNTIME"]),
    nodeEnvironment,
    publicKey: interfereDisabled()
      ? null
      : parseEnvValue(process.env["INTERFERE_PUBLIC_KEY"]),
    release: {
      destinationId: parseEnvValue(
        process.env["NEXT_PUBLIC_INTERFERE_RELEASE_ID"],
      ),
      sourceId: parseEnvValue(process.env["NEXT_PUBLIC_INTERFERE_BUILD_ID"]),
    },
  } satisfies InterfereEnv;
}
