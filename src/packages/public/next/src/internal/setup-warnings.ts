/* oxlint-disable unicorn/prefer-number-coercion */
import { existsSync } from "node:fs";
import { createRequire } from "node:module";
import path from "node:path";

import { log } from "./logger.js";

const SERVER_INSTRUMENTATION_FILES = [
  "instrumentation.ts",
  "instrumentation.tsx",
  "instrumentation.js",
  "instrumentation.jsx",
  "src/instrumentation.ts",
  "src/instrumentation.tsx",
  "src/instrumentation.js",
  "src/instrumentation.jsx",
] as const;

function hasServerInstrumentation(projectDir: string) {
  return SERVER_INSTRUMENTATION_FILES.some((c) =>
    existsSync(path.resolve(projectDir, c)),
  );
}

/**
 * Best-effort warning when the customer set `INTERFERE_PUBLIC_KEY` but never
 * created `instrumentation.ts`. Server-side tracing then silently does
 * nothing — that's surprising and there's no other place we get to tell
 * them. Never blocks the build: some customers genuinely don't want
 * server-side tracing.
 */
export function warnIfServerInstrumentationMissing(projectDir: string) {
  if (hasServerInstrumentation(projectDir)) {
    return;
  }

  log.warn("No instrumentation.ts found", [
    "Server-side traces will be skipped. To enable, create instrumentation.ts at the project root with:",
    "  export { register } from '@interfere/next/instrumentation';",
  ]);
}

/**
 * Reads the customer's installed `next` package version. Returns `null` if
 * Next isn't resolvable from `projectDir` (e.g. running outside a Next app),
 * in which case callers should skip version-conditional behaviour.
 */
export function readNextMajorVersion(projectDir: string) {
  try {
    const require = createRequire(`${projectDir}/_`);
    const pkg: unknown = require("next/package.json");

    if (
      pkg === null || // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Customer-installed package manifests are arbitrary JSON; only objects can contain a version field.
      typeof pkg !== "object" ||
      !("version" in pkg) ||
      // oxlint-disable-next-line anti-slop/no-runtime-typeof -- A package version must be a string before splitting its major component; numeric JSON versions are not coerced.
      typeof pkg.version !== "string"
    ) {
      return null;
    }

    const major = Number.parseInt(pkg.version.split(".")[0] ?? "", 10);

    return (Number.isFinite(major) ? major : null) satisfies number | null;
  } catch {
    return null;
  }
}
