import { setupServer } from "msw/node";

import { execFileSync } from "node:child_process";
import { mkdtempSync, rmSync } from "node:fs";
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

import { installVitePluginCase, projectDir } from "./plugin.test-utils";

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

  describe("removed options", () => {
    test.each([false, true, "required"])(
      "fails fast for the removed sourceMaps: %j form instead of uploading",
      (sourceMaps) => {
        // @ts-expect-error -- JavaScript configs can still pass the removed forms.
        expect(() => interfere({ sourceMaps })).toThrow(
          "Removed: interfere({ sourceMaps })",
        );
      },
    );

    test("fails fast for the removed sourceMapReadinessTimeoutMs option", () => {
      // @ts-expect-error -- JavaScript configs can still pass the removed option.
      expect(() => interfere({ sourceMapReadinessTimeoutMs: 15_000 })).toThrow(
        "Removed: interfere({ sourceMapReadinessTimeoutMs })",
      );
    });
  });

  describe("public key requirement", () => {
    test("warns without failing when a production build has no release token", async () => {
      const capture = captureLogger();
      const plugin = interfere();

      await expect(
        resolvePluginConfig(plugin, {
          command: "build",
          env: {},
          logger: capture.logger,
        }),
      ).resolves.toMatchObject({ command: "build" });

      (plugin.buildStart as () => void)();

      expect(capture.warns).toContainEqual(
        expect.stringContaining("no release token"),
      );
    });

    test("does not fatal during dev (command=serve)", async () => {
      const plugin = interfere();

      await expect(
        resolvePluginConfig(plugin, { command: "serve", env: {} }),
      ).resolves.toMatchObject({ command: "serve" });
    });
  });

  describe("forces build.sourcemap for symbolication", () => {
    const NO_ENV_DIR = "/tmp/__interfere_cfg_no_env__";

    afterEach(() => {
      vi.unstubAllEnvs();
    });

    test("forces build.sourcemap to true on build when an API key is present", async () => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_fake");

      const config = await resolvePluginConfig(interfere(), {
        envDir: NO_ENV_DIR,
      });

      expect(config.build.sourcemap).toBe(true);
    });

    test("does not touch build.sourcemap during dev (command=serve)", async () => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_fake");

      const config = await resolvePluginConfig(interfere(), {
        envDir: NO_ENV_DIR,
        command: "serve",
        mode: "development",
      });

      expect(config.build.sourcemap).toBe(false);
    });

    test("does not touch build.sourcemap when uploads are disabled", async () => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_fake");

      const config = await resolvePluginConfig(
        interfere({ sourceMaps: { upload: false } }),
        { envDir: NO_ENV_DIR },
      );

      expect(config.build.sourcemap).toBe(false);
    });

    test("overrides an incompatible build.sourcemap, preserves other build options, and warns", async () => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_fake");

      const capture = captureLogger();

      const plugin = interfere();

      const config = await resolvePluginConfig(plugin, {
        build: { outDir: "build", sourcemap: "hidden" },
        envDir: NO_ENV_DIR,
        root: projectDir,
        logger: capture.logger,
      });

      expect(config.build.sourcemap).toBe(true);
      expect(config.build.outDir).toBe("build");

      (plugin.buildStart as () => void)();

      expect(
        capture.warns.some((line) => line.includes("Forced build.sourcemap")),
      ).toBe(true);
    });
  });
});
