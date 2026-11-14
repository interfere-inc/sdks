import { execSync } from "node:child_process";

/**
 * Runs a git command in `cwd` (the process cwd by default) and returns its
 * stdout, or `null` when git isn't available. Build-time only: inject it
 * into the shared resolvers rather than importing it from runtime server
 * code, which must never spawn subprocesses.
 */
export function runGitCommand(command: string, cwd?: string) {
  try {
    const output = execSync(command, {
      cwd,
      encoding: "utf8",
      stdio: ["ignore", "pipe", "ignore"],
    }).trim();

    return (output.length > 0 ? output : null) satisfies string | null;
  } catch {
    return null;
  }
}
