import { parseRoutePrefixInput } from "./internal/route-prefix";

export const DEFAULT_ROUTE_PREFIX = "/api/interfere";

const GLOBAL_OVERRIDE_KEY = "__INTERFERE_ROUTE_PREFIX__";

/** Resolve the normalized proxy route prefix from the build-injected override or the default. */
export function resolveRoutePrefix() {
  const globals: object = globalThis;

  const fromGlobal =
    GLOBAL_OVERRIDE_KEY in globals ? globals[GLOBAL_OVERRIDE_KEY] : undefined;

  const injected = parseRoutePrefixInput(fromGlobal);

  if (injected !== undefined) {
    return normalizeRoutePrefix(injected);
  }

  return DEFAULT_ROUTE_PREFIX;
}

export function normalizeRoutePrefix(prefix: string) {
  const trimmed = prefix.trim();

  if (trimmed === "" || trimmed === "/") {
    return DEFAULT_ROUTE_PREFIX;
  }

  const withLeading = trimmed.startsWith("/") ? trimmed : `/${trimmed}`;

  return withLeading.endsWith("/") ? withLeading.slice(0, -1) : withLeading;
}
