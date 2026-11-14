import { execSync } from "node:child_process";

/**
 * Runs a git command and returns its stdout, or `null` when git isn't
 * available (no `.git` checkout, git binary missing, command errored).
 *
 * The ignore-on-failure shape lets callers compose this with env-var
 * reads as a fallback chain — env vars first (CI environments where
 * git isn't checked out), git as a last resort (local dev).
 *
 * Build-time only. `node:child_process` isn't available in the Edge
 * runtime, so this helper must never end up in `instrumentation.ts`'s
 * import graph (the SDK ships that entrypoint to both Node + Edge
 * runtimes via conditional exports).
 */
export function runGitCommand(command: string) {
  try {
    const output = execSync(command, {
      encoding: "utf8",
      stdio: ["ignore", "pipe", "ignore"],
    }).trim();

    return (output.length > 0 ? output : null) satisfies string | null;
  } catch {
    return null;
  }
}
