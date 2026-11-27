import { setupServer } from "msw/node";

import { execFileSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
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
  installVitePluginCase,
  projectDir,
  releaseRequests,
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

  describe("missing secret key", () => {
    test("skips the release with a warning when INTERFERE_API_KEY is unset", async () => {
      vi.stubEnv("NODE_ENV", "production");

      vi.stubEnv("INTERFERE_API_KEY", "");

      vi.stubEnv("GIT_DIR", path.join(projectDir, "missing-git"));

      const capture = captureLogger();
      const mapPath = path.join(projectDir, "dist/app.js.map");
      const mapBefore = readFileSync(mapPath, "utf8");
      const plugin = interfere();

      const config = await resolvePluginConfig(plugin, {
        logger: capture.logger,
      });

      (plugin.buildStart as () => void)();

      await expect(
        (plugin.closeBundle as () => Promise<void>)(),
      ).resolves.toBeUndefined();

      expect(config.build.sourcemap).toBe(false);
      expect(configRequests).toHaveLength(0);
      expect(releaseRequests).toHaveLength(0);
      expect(readFileSync(mapPath, "utf8")).toBe(mapBefore);

      expect(capture.warns).toContainEqual(
        expect.stringContaining("Skipping release"),
      );
    });

    test("turns Interfere off when disable is set, even with an invalid secret key", async () => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_pk_wrong_prefix");

      const config = await resolvePluginConfig(interfere({ disable: true }));

      expect(config.build.sourcemap).toBe(false);

      expect(Object.keys(config.define ?? {})).toStrictEqual([
        "import.meta.env.VITE_INTERFERE_DISABLED",
      ]);

      expect(configRequests).toHaveLength(0);
      expect(releaseRequests).toHaveLength(0);
    });

    test("does not require a key during dev (command=serve)", async () => {
      vi.stubEnv("INTERFERE_API_KEY", "");

      await expect(
        resolvePluginConfig(interfere(), { command: "serve" }),
      ).resolves.toBeDefined();
    });

    test("fails a production build when INTERFERE_API_KEY has the wrong prefix", async () => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_pk_test_123");

      await expect(resolvePluginConfig(interfere())).rejects.toThrow(
        "INTERFERE_API_KEY has wrong prefix",
      );
    });
  });
});
