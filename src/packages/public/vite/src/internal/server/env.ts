import { API_URL } from "@interfere/constants/api";
import { parseEnvValue } from "@interfere/types/sdk/env";
import {
  readProcessEnv,
  resolveEnvironment,
} from "@interfere/types/sdk/runtime";

interface InterfereServerEnv {
  readonly apiUrl: string;
  readonly nodeEnvironment: string;
  readonly publicKey: string | null;
}

/**
 * Server-side gate for the proxy handler, the `captureError` /
 * `onRequestError` helpers, and the OTel register path. Reads the same
 * publishable `VITE_INTERFERE_PUBLIC_KEY` as the browser — the key is
 * not a secret, so there is no reason to demand a second server-only
 * variable. Vite only inlines `VITE_*` into the client bundle, so the
 * plugin mirrors it into `process.env` for the dev server. Collector
 * ingress remains the authority on whether local / unknown / production
 * events are allowed.
 */
export function isEnabledOnServer() {
  if (import.meta.env.VITE_INTERFERE_DISABLED) {
    return false;
  }

  if (readProcessEnv()?.["NODE_ENV"] === "production") {
    return true;
  }

  return readPublicKey() !== null;
}

export function readInterfereServerEnv() {
  const nodeEnvironment = resolveEnvironment({
    env: readProcessEnv() ?? {},
    explicit: import.meta.env.VITE_INTERFERE_ENVIRONMENT,
  });

  return {
    apiUrl: parseEnvValue(readProcessEnv()?.["INTERFERE_API_URL"]) ?? API_URL,
    nodeEnvironment,
    publicKey: import.meta.env.VITE_INTERFERE_DISABLED ? null : readPublicKey(),
  } satisfies InterfereServerEnv;
}

function readPublicKey() {
  return (
    parseEnvValue(readProcessEnv()?.["VITE_INTERFERE_PUBLIC_KEY"]) ??
    parseEnvValue(import.meta.env.VITE_INTERFERE_PUBLIC_KEY)
  );
}
