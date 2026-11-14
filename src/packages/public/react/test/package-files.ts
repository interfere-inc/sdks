import { Minimatch } from "minimatch";
import { readdirSync } from "node:fs";
import { join } from "node:path";

/** Evaluates this package's files allow-list; npm CLI semantics are covered by the hosted packed-installation matrix. */
export function packageFiles(directory: string, patterns: string[]) {
  const included: Minimatch[] = [];
  const excluded: Minimatch[] = [];

  for (const pattern of patterns) {
    const negative = pattern.startsWith("!");

    const normalized = (negative ? pattern.slice(1) : pattern)
      .replace(/^\.\//u, "")
      .replace(/\/$/u, "");

    (negative ? excluded : included).push(
      new Minimatch(normalized, { dot: true, nonegate: true }),
    );
  }

  const files: string[] = [];
  const pending = [""];

  const ignored = new Set([
    "node_modules",
    ".git",
    ".svn",
    ".hg",
    "CVS",
    ".npmrc",
    ".npmignore",
    ".gitignore",
    ".DS_Store",
  ]);

  for (const relative of pending) {
    for (const entry of readdirSync(join(directory, relative), {
      withFileTypes: true,
    })) {
      if (entry.isSymbolicLink() || ignored.has(entry.name)) {
        continue;
      }

      const path = relative ? `${relative}/${entry.name}` : entry.name;

      if (entry.isDirectory()) {
        pending.push(path);

        continue;
      }

      if (!entry.isFile()) {
        continue;
      }

      const parts = path.split("/");

      const ancestors = parts.map((_, index) =>
        parts.slice(0, index + 1).join("/"),
      );

      const matches = (rules: Minimatch[]) =>
        rules.some((rule) =>
          ancestors.some((ancestor) => rule.match(ancestor)),
        );

      const required =
        relative === "" &&
        /^(?:package\.json|(?:readme|licen[cs]e|changelog)(?:\..*)?)$/iu.test(
          entry.name,
        );

      if (required || (matches(included) && !matches(excluded))) {
        files.push(path);
      }
    }
  }

  return files.toSorted();
}
