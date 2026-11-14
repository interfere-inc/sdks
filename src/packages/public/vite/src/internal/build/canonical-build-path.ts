import { realpath } from "node:fs/promises";
import path from "node:path";

export const canonicalBuildPath: (file: string) => Promise<string> = async (
  file,
) => {
  const existing = await realpath(file).catch(() => null);

  if (existing) {
    return existing;
  }

  const parent = path.dirname(file);

  return parent === file
    ? file
    : path.join(await canonicalBuildPath(parent), path.basename(file));
};
