import { isString } from "../guards/json.ts";

/** Backend hosts resolved for the application at build time. */
export const BACKEND_HOSTS_GLOBAL = "__INTERFERE_BACKEND_HOSTS__";

/** Whether a value is a list of backend hosts. */
export function isBackendHosts(value: unknown): value is string[] {
  return Array.isArray(value) && value.every((host) => isString(host));
}

export function readBakedBackendHosts() {
  const scope = globalThis;

  const injected =
    BACKEND_HOSTS_GLOBAL in scope ? scope[BACKEND_HOSTS_GLOBAL] : undefined;

  return isBackendHosts(injected) ? injected : undefined;
}
