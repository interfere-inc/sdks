const HASHED_ASSET_RE =
  /[-./](?<hash>[A-Za-z0-9_-]{8}|[{]hash[}])[.](?<extension>(?:m?js|cjs|css)(?:[.]map)?|woff2?|ttf|otf|eot|png|jpe?g|gif|svg|webp|avif|ico)$/u;

const UUID_SEGMENT_RE =
  /[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}/gu;

const LONG_HEX_RE = /[0-9a-fA-F]{16,}/gu;

const DIGIT_RUN_RE = /[0-9]{2,}/gu;

function isBuildHash(segment: string) {
  return (
    segment === "{hash}" ||
    /[0-9_-]/u.test(segment) ||
    (/[a-z]/u.test(segment) &&
      (segment.slice(1).match(/[A-Z]/gu)?.length ?? 0) >= 2)
  );
}

function collapseHashedAsset(pathname: string) {
  const { hash, extension } = HASHED_ASSET_RE.exec(pathname)?.groups ?? {};

  return hash !== undefined && extension !== undefined && isBuildHash(hash)
    ? `${pathname.slice(0, pathname.lastIndexOf("/") + 1)}{asset}.${extension}`
    : pathname;
}

/**
 * Low-cardinality fallback for span names when the app supplies no
 * `resolveRoute` template, applied in order: a content-hashed static asset
 * filename (an 8-character build hash before a script, style, source map,
 * font or image extension) → `{asset}.<ext>`, UUIDs → `{id}`, 16+-char hex
 * runs → `{hash}`, 2+-digit runs → `{n}`. MUST stay in lockstep with
 * `normalizeHttpPath` in fingerprinting and `NORMALIZED_PATH` in the tinybird
 * request-failures pipe — the sides group by this shape.
 */
export function normalizePath(pathname: string) {
  return collapseHashedAsset(pathname)
    .replaceAll(UUID_SEGMENT_RE, "{id}")
    .replaceAll(LONG_HEX_RE, "{hash}")
    .replaceAll(DIGIT_RUN_RE, "{n}");
}
