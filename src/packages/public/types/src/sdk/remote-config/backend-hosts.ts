import { z } from "zod";

export const backendHostsSchema = z.array(z.string()).optional();

/** Backend hosts resolved for the application at build time. */
export const BACKEND_HOSTS_GLOBAL = "__INTERFERE_BACKEND_HOSTS__";

export function readBakedBackendHosts() {
  const scope = globalThis;

  const injected =
    BACKEND_HOSTS_GLOBAL in scope ? scope[BACKEND_HOSTS_GLOBAL] : undefined;

  const parsed = backendHostsSchema.safeParse(injected);

  return parsed.success ? parsed.data : undefined;
}
