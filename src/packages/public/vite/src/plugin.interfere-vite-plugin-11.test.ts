import { setupServer } from "msw/node";

import { execFileSync } from "node:child_process";
import { existsSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { createLogger, resolveConfig } from "vite";
import type { ConfigEnv, Logger, Plugin, UserConfig } from "vite";
import {
  afterAll,
  afterEach,
  beforeAll,
  beforeEach,
  describe,
  expect,
  test,
  vi,
} from "vitest";

import {
  configRequests,
  finalizeRequests,
  installVitePluginCase,
  projectDir,
  releaseRequests,
  uploadRequests,
} from "./plugin.test-utils";

const { interfere } = await import("./plugin.js");

const server = setupServer();

const RELEASE_API = "https://vite-release.test";

const SLUG = "rel_deadbeef12345678";

let gitDir: string;

let gitSha: string;

let appRepo: { dir: string; sha: string };

function initGitRepo(prefix: string, message: string) {
  const dir = mkdtempSync(path.join(tmpdir(), prefix));

  execFileSync("git", ["init", dir], { stdio: "ignore" });

  execFileSync(
    "git",
    [
      "-C",
      dir,
      "-c",
      "user.name=SDK Test",
      "-c",
      "user.email=sdk@example.com",
      "-c",
      "core.hooksPath=/dev/null",
      "commit",
      "--allow-empty",
      "--no-gpg-sign",
      "-m",
      message,
    ],
    { stdio: "ignore" },
  );

  return {
    dir,
    sha: execFileSync("git", ["-C", dir, "rev-parse", "HEAD"], {
      encoding: "utf8",
    }).trim(),
  };
}

beforeAll(() => {
  ({ dir: gitDir, sha: gitSha } = initGitRepo(
    "interfere-vite-git-",
    "SDK fixture",
  ));

  appRepo = initGitRepo("interfere-vite-app-root-", "app root");

  server.listen({ onUnhandledFrame: "error" });
});

afterAll(() => {
  rmSync(gitDir, { recursive: true, force: true });

  rmSync(appRepo.dir, { recursive: true, force: true });
});

async function resolvePluginConfig(
  plugin: Plugin,
  input: Pick<UserConfig, "envDir" | "mode" | "plugins" | "root" | "build"> & {
    command?: ConfigEnv["command"];
    env?: Record<string, string>;
    logger?: Logger;
    preview?: boolean;
  } = {},
) {
  if (input.env) {
    vi.stubEnv("VITE_INTERFERE_PUBLIC_KEY", undefined);

    for (const [name, value] of Object.entries(input.env)) {
      vi.stubEnv(name, value);
    }
  }

  return resolveConfig(
    {
      configFile: false,
      build: input.build ?? {},
      envDir: input.envDir ?? "/tmp/__interfere_plugin_test_envdir__",
      customLogger: input.logger ?? createLogger("silent"),
      mode: input.mode ?? "production",
      plugins: [plugin, ...(input.plugins ?? [])],
      root: input.root ?? projectDir,
    },
    input.command ?? "build",
    undefined,
    undefined,
    input.preview,
  );
}

function captureLogger() {
  const logger = createLogger("silent");
  const info = vi.spyOn(logger, "info");
  const warn = vi.spyOn(logger, "warn");

  const error = vi.spyOn(logger, "error");

  return {
    get errors() {
      return error.mock.calls.map(([message]) => message);
    },
    get infos() {
      return info.mock.calls.map(([message]) => message);
    },
    logger,
    get warns() {
      return warn.mock.calls.map(([message]) => message);
    },
  };
}

describe("interfere vite plugin", () => {
  beforeEach(() => {
    installVitePluginCase({
      gitDir,
      gitSha,
      releaseApi: RELEASE_API,
      server,
      slug: SLUG,
    });
  });

  afterEach(() => {
    rmSync(projectDir, { recursive: true, force: true });

    vi.unstubAllEnvs();

    vi.unstubAllGlobals();
  });

  describe("iNTERFERE_API_KEY env-file fallback", () => {
    let envDir: string;

    beforeEach(() => {
      envDir = mkdtempSync(path.join(tmpdir(), "interfere-vite-envdir-"));
    });

    afterEach(() => {
      rmSync(envDir, { force: true, recursive: true });

      vi.unstubAllEnvs();
    });

    test("reads INTERFERE_API_KEY from .env.local when process.env is empty", async () => {
      writeFileSync(
        path.join(envDir, ".env.local"),
        "INTERFERE_API_KEY=interfere_ak_from_dotenv_local\n",
      );

      vi.stubEnv("INTERFERE_API_KEY", undefined);

      const capture = captureLogger();
      const plugin = interfere();

      await resolvePluginConfig(plugin, {
        command: "build",
        envDir,
        logger: capture.logger,
      });

      (plugin.buildStart as () => void)();

      expect(
        capture.warns.some((line) =>
          line.includes("INTERFERE_API_KEY is not set"),
        ),
      ).toBe(false);

      const closeBundle = plugin.closeBundle as () => Promise<void>;

      await closeBundle();

      expect(configRequests).toHaveLength(1);

      expect(configRequests[0]?.headers.get("authorization")).toBe(
        "Bearer interfere_ak_from_dotenv_local",
      );

      expect(releaseRequests).toHaveLength(1);

      expect(releaseRequests[0]?.headers.get("authorization")).toBe(
        "Bearer interfere_ak_from_dotenv_local",
      );

      expect(uploadRequests).toHaveLength(1);
      expect(finalizeRequests).toHaveLength(1);
      expect(existsSync(path.join(projectDir, "dist/app.js.map"))).toBe(false);
    });

    test("prefers process.env over .env.local (shell wins, standard precedence)", async () => {
      writeFileSync(
        path.join(envDir, ".env.local"),
        "INTERFERE_API_KEY=interfere_ak_from_dotenv\n",
      );

      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_from_shell");

      const plugin = interfere();

      await resolvePluginConfig(plugin, { command: "build", envDir });

      const closeBundle = plugin.closeBundle as () => Promise<void>;

      await closeBundle();

      expect(configRequests).toHaveLength(1);

      expect(configRequests[0]?.headers.get("authorization")).toBe(
        "Bearer interfere_ak_from_shell",
      );

      expect(releaseRequests).toHaveLength(1);

      expect(releaseRequests[0]?.headers.get("authorization")).toBe(
        "Bearer interfere_ak_from_shell",
      );

      expect(uploadRequests).toHaveLength(1);
      expect(finalizeRequests).toHaveLength(1);
      expect(existsSync(path.join(projectDir, "dist/app.js.map"))).toBe(false);
    });

    test("treats an explicitly-empty shell value as unset, not a resolved key", async () => {
      writeFileSync(
        path.join(envDir, ".env.local"),
        "INTERFERE_API_KEY=interfere_ak_from_dotenv\n",
      );

      vi.stubEnv("INTERFERE_API_KEY", "");

      await expect(
        resolvePluginConfig(interfere(), { command: "build", envDir }),
      ).resolves.toMatchObject({ build: { sourcemap: false } });
    });
  });
});
