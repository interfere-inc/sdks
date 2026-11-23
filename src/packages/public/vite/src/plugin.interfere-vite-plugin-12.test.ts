import { setupServer } from "msw/node";

import { execFileSync } from "node:child_process";
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
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

const ENVIRONMENT_KEY = "import.meta.env.VITE_INTERFERE_ENVIRONMENT";

async function callConfigDefine(
  plugin: Plugin,
  input: {
    command?: "build" | "serve";
    mode?: string;
    plugins?: { name: string }[];
  } = {},
) {
  const result = await resolvePluginConfig(plugin, input);

  return result.define ?? {};
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

  describe("dev-server key mirror", () => {
    let envDir: string;
    const KEY = "VITE_INTERFERE_PUBLIC_KEY";

    async function callConfig(command: "build" | "serve") {
      await resolvePluginConfig(interfere(), {
        envDir,
        command,
        mode: command === "serve" ? "development" : "production",
      });
    }

    beforeEach(() => {
      envDir = mkdtempSync(path.join(tmpdir(), "interfere-vite-mirror-"));

      delete process.env["VITE_INTERFERE_PUBLIC_KEY"];

      delete process.env["INTERFERE_API_KEY"];
    });

    afterEach(() => {
      rmSync(envDir, { force: true, recursive: true });

      delete process.env["VITE_INTERFERE_PUBLIC_KEY"];

      delete process.env["INTERFERE_API_KEY"];

      vi.unstubAllEnvs();
    });

    test("mirrors the .env public key into process.env for the dev server", async () => {
      writeFileSync(
        path.join(envDir, ".env.local"),
        `${KEY}=interfere_pub_us_dev\n`,
      );

      await callConfig("serve");

      expect(process.env[KEY]).toBe("interfere_pub_us_dev");
    });

    test("does not mirror during a production build", async () => {
      writeFileSync(
        path.join(envDir, ".env.local"),
        `${KEY}=interfere_pub_us_dev\n`,
      );

      await callConfig("build");

      expect(process.env[KEY]).toBeUndefined();
    });

    test("does not overwrite a value already set in the shell", async () => {
      writeFileSync(
        path.join(envDir, ".env.local"),
        `${KEY}=interfere_pub_us_dev\n`,
      );

      process.env[KEY] = "interfere_pub_us_shell";

      await callConfig("serve");

      expect(process.env[KEY]).toBe("interfere_pub_us_shell");
    });

    test("mirrors the .env workspace API key for server telemetry in the dev server", async () => {
      writeFileSync(
        path.join(envDir, ".env.local"),
        "INTERFERE_API_KEY=interfere_secret_us_dev\n",
      );

      await callConfig("serve");

      expect(process.env["INTERFERE_API_KEY"]).toBe("interfere_secret_us_dev");
    });
  });

  describe("__INTERFERE_ENVIRONMENT__ global", () => {
    test("inlines the deploy environment from INTERFERE_ENVIRONMENT", async () => {
      vi.stubEnv("INTERFERE_ENVIRONMENT", "production");

      vi.stubEnv("VERCEL_ENV", undefined);

      const define = await callConfigDefine(interfere());

      expect(define[ENVIRONMENT_KEY]).toBe('"production"');
    });

    test("falls back to VERCEL_ENV when INTERFERE_ENVIRONMENT is unset", async () => {
      vi.stubEnv("INTERFERE_ENVIRONMENT", undefined);

      vi.stubEnv("VERCEL_ENV", "preview");

      const define = await callConfigDefine(interfere());

      expect(define[ENVIRONMENT_KEY]).toBe('"preview"');
    });

    test("inlines the Wrangler environment a CLOUDFLARE_ENV build targets", async () => {
      vi.stubEnv("INTERFERE_ENVIRONMENT", undefined);

      vi.stubEnv("VERCEL_ENV", undefined);

      vi.stubEnv("CLOUDFLARE_ENV", "preview");

      const define = await callConfigDefine(interfere());

      expect(define[ENVIRONMENT_KEY]).toBe('"preview"');
    });

    test("prefers INTERFERE_ENVIRONMENT over VERCEL_ENV", async () => {
      vi.stubEnv("INTERFERE_ENVIRONMENT", "staging");

      vi.stubEnv("VERCEL_ENV", "production");

      const define = await callConfigDefine(interfere());

      expect(define[ENVIRONMENT_KEY]).toBe('"staging"');
    });

    test("does not throw in dev even when the environment is unresolvable", async () => {
      vi.stubEnv("INTERFERE_ENVIRONMENT", undefined);

      vi.stubEnv("VERCEL_ENV", undefined);

      const plugin = interfere();

      await expect(
        resolvePluginConfig(plugin, { command: "serve", env: {} }),
      ).resolves.toMatchObject({ command: "serve" });
    });
  });
});

afterEach(() => {
  server.resetHandlers();
});

// oxlint-disable-next-line vitest/no-duplicate-hooks -- Independent teardown must still run after another cleanup fails, in native reverse registration order.
afterAll(() => {
  server.close();
});
