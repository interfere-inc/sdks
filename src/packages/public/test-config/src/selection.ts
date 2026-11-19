import { resolve } from "node:path";

export function selection(
  suiteArgs: string[],
  entries: string[],
  repository: string,
  packagePath: string,
  expand: (directory: string) => string[],
) {
  const root = `${packagePath}/`;

  if (entries.some((entry) => entry.endsWith("/") && root.startsWith(entry))) {
    return suiteArgs;
  }

  const patterns = suiteArgs.filter((arg) => !arg.startsWith("-"));
  const flags = suiteArgs.filter((arg) => arg.startsWith("-"));
  const owned = entries.filter((entry) => entry.startsWith(root));

  const files =
    patterns.length === 0
      ? owned
      : [
          ...new Set(
            owned.flatMap((entry) =>
              entry.endsWith("/") ? expand(entry) : [entry],
            ),
          ),
        ].filter((file) => patterns.some((pattern) => file.includes(pattern)));

  return files.length === 0
    ? undefined
    : [...flags, ...files.map((file) => resolve(repository, file))];
}
