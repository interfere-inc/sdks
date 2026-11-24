export const DENIED_KEY = "interfere:propagation_denied";

/** Origins this browser session stopped sending trace headers to after a header-bearing request failed. */
export function readDenied() {
  try {
    const parsed: unknown = JSON.parse(
      sessionStorage.getItem(DENIED_KEY) ?? "[]",
    );

    return new Set(
      Array.isArray(parsed)
        ? parsed.filter((entry): entry is string => typeof entry === "string")
        : [],
    );
  } catch {
    return new Set<string>();
  }
}
