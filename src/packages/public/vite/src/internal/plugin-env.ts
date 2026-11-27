import path from "node:path";
import type { UserConfig } from "vite";

export function isTestEnv() {
  // oxlint-disable-next-line node/no-process-env
  return Boolean(process.env["VITEST"] || process.env["VITEST_WORKER_ID"]);
}

export function pickEnvValue(
  env: Record<string, string | undefined>,
  key: string,
) {
  const value = env[key];

  return (value && value.length > 0 ? value : undefined) satisfies
    | string
    | undefined;
}

export function mirrorKeysForDevServer(
  command: "build" | "serve",
  env: Record<string, string | undefined>,
) {
  if (command !== "serve") {
    return;
  }

  const value = env["INTERFERE_API_KEY"];

  // oxlint-disable-next-line node/no-process-env
  if (value && !process.env["INTERFERE_API_KEY"]) {
    // oxlint-disable-next-line node/no-process-env
    process.env["INTERFERE_API_KEY"] = value;
  }
}

export function resolveRoot(userConfig: UserConfig) {
  return userConfig.root
    ? path.resolve(process.cwd(), userConfig.root)
    : process.cwd();
}

export function resolveEnvDir(userConfig: UserConfig) {
  const root = resolveRoot(userConfig);

  return userConfig.envDir === false ||
    userConfig.envDir === undefined ||
    userConfig.envDir === ""
    ? root
    : path.resolve(root, userConfig.envDir);
}
