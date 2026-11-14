/** Converts exact HTTPS origins to bounded Vercel fetch URL matchers. */
export function serverPropagationTargets(entries: readonly string[]) {
  const origins = new Set<string>();

  for (const entry of entries) {
    try {
      if (!/^https:\/\/[^\s/?#@\\]+$/u.test(entry)) {
        throw new Error("Expected an exact HTTPS origin");
      }

      const url = new URL(entry);

      if (decodeURIComponent(url.hostname).includes("*")) {
        throw new Error("Wildcards are not origins");
      }

      origins.add(url.origin);
    } catch {
      console.warn(`[interfere] Invalid server propagateTo origin: ${entry}`);
    }
  }

  return [...origins].map(
    (origin) =>
      new RegExp(
        `^${origin.replaceAll(/[.*+?^${}()|[\]\\]/gu, String.raw`\$&`)}(?:[/?#]|$)`,
        "iu",
      ),
  );
}
