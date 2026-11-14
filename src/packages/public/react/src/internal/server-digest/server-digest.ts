/**
 * True when a thrown value carries a Next.js server `digest` — it originated on
 * the server and was already captured there (`onRequestError`). Client capture
 * sites skip these so one server error isn't recorded again on the client.
 */
export function hasServerDigest(value: unknown): value is { digest: string } {
  return (
    value !== null &&
    typeof value === "object" &&
    "digest" in value &&
    typeof value.digest === "string"
  );
}
