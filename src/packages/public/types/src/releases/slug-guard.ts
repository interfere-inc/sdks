import type { ReleaseSlug } from "./slug.ts";

const releaseSlugPattern = /^rel_[0-9a-f]{16}$/u;

function isReleaseSlug(value: string): value is ReleaseSlug {
  return releaseSlugPattern.test(value);
}

/** Brands a release slug without loading zod into browser bundles, throwing when it is not `rel_<16hex>`. */
export function toReleaseSlug(value: string) {
  if (!isReleaseSlug(value)) {
    throw new TypeError(`Invalid release slug: ${value}`);
  }

  return value;
}
