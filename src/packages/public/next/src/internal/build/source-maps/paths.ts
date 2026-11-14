import { readdir } from "node:fs/promises";
import { isAbsolute, join, resolve } from "node:path";

const NEXT_PUBLIC_PREFIX = "_next/";

const NEXT_DIST_PREFIX = ".next/";

/**
 * Strips Windows path separators and leading `./` segments so the rest of
 * the discovery code can assume POSIX-style paths relative to `projectDir`.
 */
export function normalizeDistDir(distDir = ".next") {
  return distDir.replaceAll("\\", "/").replace(/^(?:\.\/)+/u, "");
}

/**
 * Resolves an absolute filesystem path for the build's `distDir`. Accepts
 * either an absolute path (passed through) or a path relative to
 * `projectDir`.
 */
export function resolveDistDir(projectDir: string, distDir: string) {
  return isAbsolute(distDir) ? distDir : resolve(projectDir, distDir);
}

/**
 * Maps an on-disk path (relative to `projectDir`) to the URL path Next.js
 * will serve it at. Both `_next/` and `.next/` shapes resolve to the public
 * `_next/` prefix the browser sees in `//# sourceMappingURL=...` comments.
 */
export function toPublicPath(rel: string, distDir: string) {
  const p = rel.replaceAll("\\", "/");

  if (p.startsWith(`${distDir}/`)) {
    return `${NEXT_PUBLIC_PREFIX}${p.slice(distDir.length + 1)}`;
  }

  if (p.startsWith(NEXT_DIST_PREFIX)) {
    return `${NEXT_PUBLIC_PREFIX}${p.slice(NEXT_DIST_PREFIX.length)}`;
  }

  return p;
}

/**
 * Recursively collects files under `<absDistDir>/static` and
 * `<absDistDir>/server` matching the given suffix. Both subtrees are
 * optional — partial builds (e.g. server-only chunks during ISR) commonly
 * leave one missing, so a missing directory is treated as empty rather
 * than fatal.
 */
export async function walkDistTrees(
  absDistDir: string,
  suffix: ".js" | ".js.map" | ".map",
) {
  const subtrees = ["static", "server"] as const;

  const results = await Promise.all(
    subtrees.map(async (subtree) => {
      const root = join(absDistDir, subtree);

      const entries = await readdir(root, {
        recursive: true,
        withFileTypes: true,
      }).catch(() => []);

      const out: string[] = [];

      for (const entry of entries) {
        if (entry.isFile() && entry.name.endsWith(suffix)) {
          out.push(join(entry.parentPath, entry.name));
        }
      }

      return out;
    }),
  );

  return results.flat();
}
