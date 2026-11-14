import { readFile, unlink, writeFile } from "node:fs/promises";

const SOURCE_MAPPING_URL_RE = /\n?\/\/[#@]\s*sourceMappingURL=\S+\s*$/u;

async function failuresOf(absolute: string, operation: Promise<void>) {
  try {
    await operation;

    return [];
  } catch (error) {
    const code =
      error instanceof Error &&
      "code" in error &&
      // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Filesystem failures expose an optional string errno; missing or malformed codes remain reportable failures.
      typeof error.code === "string"
        ? error.code
        : undefined;

    const message = error instanceof Error ? error.message : String(error);

    return code === "ENOENT" ? [] : [{ absolute, code, message }];
  }
}

async function stripSourceMappingUrl(chunk: string) {
  const content = await readFile(chunk, "utf8");

  await writeFile(chunk, content.replace(SOURCE_MAPPING_URL_RE, ""));
}

export async function cleanupSourceMaps(
  files: readonly { absolute: string; chunkAbsolutes: readonly string[] }[],
) {
  // WARNING: Maps can share chunks; two concurrent strips of one chunk can leave it empty.
  const chunks = [...new Set(files.flatMap((file) => file.chunkAbsolutes))];

  const stripFailures = new Map(
    await Promise.all(
      chunks.map(
        async (chunk) =>
          [
            chunk,
            await failuresOf(chunk, stripSourceMappingUrl(chunk)),
          ] as const,
      ),
    ),
  );

  const unlinkFailures = await Promise.all(
    files.map(async ({ absolute, chunkAbsolutes }) =>
      chunkAbsolutes.some(
        (chunk) => (stripFailures.get(chunk)?.length ?? 0) > 0,
      )
        ? []
        : failuresOf(absolute, unlink(absolute)),
    ),
  );

  return [...[...stripFailures.values()].flat(), ...unlinkFailures.flat()];
}
