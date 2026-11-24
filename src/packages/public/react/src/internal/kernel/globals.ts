import { toReleaseSlug } from "@interfere/types/releases/slug-guard";
import { RELEASE_SLUG_GLOBAL } from "@interfere/types/sdk/injected-globals";

import { getGlobal } from "../../util/global.js";
import { PRODUCER_VERSION } from "../version.js";

export function buildSdkStack(wrapperVersions?: string[]) {
  return [...(wrapperVersions ?? []), PRODUCER_VERSION] satisfies string[];
}

/**
 * Build-time-derived release slug. The build step
 * (`@interfere/next/withInterfere`, `@interfere/vite/plugin`) takes the
 * leading 16 hex chars of the commit SHA (`rel_<16hex>`) and stamps it into
 * the bundle as `__INTERFERE_RELEASE_SLUG__`. The collector derives the same
 * value from the create-release request, so the bundle's runtime slug is
 * guaranteed to match the row without a round-trip.
 *
 * Returns `null` when no slug was injected (dev builds, customers who
 * haven't wired up the plugin).
 */
export function readReleaseSlugFromGlobals() {
  const value = getGlobal(RELEASE_SLUG_GLOBAL);

  // oxlint-disable-next-line anti-slop/no-runtime-typeof
  if (typeof value !== "string") {
    return null;
  }

  return toReleaseSlug(value);
}
