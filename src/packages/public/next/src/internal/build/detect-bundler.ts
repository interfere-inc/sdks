import type { ManifestBundler } from "@interfere/types/data/source-maps";

const TURBOPACK_FLAGS = ["--turbo", "--turbopack"] as const;

const FIRST_TURBOPACK_DEFAULT_MAJOR = 16;

export type NextBuildBundler = Extract<
  ManifestBundler,
  "webpack" | "turbopack"
>;

export function detectActiveBundler(
  // oxlint-disable-next-line node/no-process-env
  env: Readonly<Record<string, string | undefined>> = process.env,
  argv: readonly string[] = process.argv,
  nextMajor: number | null = null,
) {
  if (argv.includes("--webpack")) {
    return "webpack";
  }

  if (env["TURBOPACK"] === "1") {
    return "turbopack";
  }

  if (TURBOPACK_FLAGS.some((flag) => argv.includes(flag))) {
    return "turbopack";
  }

  return (
    nextMajor !== null && nextMajor >= FIRST_TURBOPACK_DEFAULT_MAJOR
      ? "turbopack"
      : "webpack"
  ) satisfies NextBuildBundler;
}
