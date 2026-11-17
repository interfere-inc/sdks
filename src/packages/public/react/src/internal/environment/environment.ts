import { hasIngestCredential, isForceEnableActive } from "../config/config.js";

export function isEnabledByEnvironment() {
  const environment =
    typeof process === "undefined" ? undefined : process.env["NODE_ENV"];

  return (
    environment === undefined ||
    environment === "production" ||
    hasIngestCredential() ||
    isForceEnableActive()
  );
}
