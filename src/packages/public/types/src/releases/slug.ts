import { z } from "zod";

const SLUG_PREFIX = "rel_";

const SLUG_HEX_LEN = 16;

export const releaseSlugSchema = z
  .templateLiteral([SLUG_PREFIX, z.string().regex(/^[0-9a-f]{16}$/u)])
  .brand<"ReleaseSlug">();

export type ReleaseSlug = z.infer<typeof releaseSlugSchema>;

export const LOCAL_DEV_RELEASE_SLUG = releaseSlugSchema.parse(
  "rel_0000000000000000",
);

/**
 * Derives a release slug from a commit SHA. Both ends of the wire — the
 * build step that stamps the slug into the bundle, and any server-side
 * upsert that reaches the same commit — compute the same `rel_<16hex>`
 * shape with no shared secret, so no round-trip negotiation is needed.
 *
 * Git commit SHAs are uniformly distributed hex; the leading 16 chars give
 * 64 bits of entropy, which is the same width the collector regex enforces.
 * Cross-surface collisions on the same commit are intentional (eyeball-
 * correlate slug to commit) — lookups are scoped by `(surfaceId, slug)`.
 *
 * Return type is `ReleaseSlug` (template-literal branded) so consumers
 * can pass the result into branded sites without a re-validate.
 */
export function deriveReleaseSlug(commitSha: string) {
  return releaseSlugSchema.parse(
    `${SLUG_PREFIX}${commitSha.toLowerCase().slice(0, SLUG_HEX_LEN)}`,
  );
}
