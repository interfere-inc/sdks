import { setupServer } from "msw/node";
import { deriveReleaseSlug } from "@interfere/types/releases/slug";

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

import { BAKED_RELEASE_SLUG_DEFINE } from "./internal/baked-release";
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

const API_HOST_KEY = "import.meta.env.VITE_INTERFERE_API_HOST";

const ENVIRONMENT_KEY = "import.meta.env.VITE_INTERFERE_ENVIRONMENT";

const RELEASE_SLUG_KEY = BAKED_RELEASE_SLUG_DEFINE;

const DEV_RELEASE_SLUG_KEY = "import.meta.env.VITE_INTERFERE_RELEASE_SLUG";

const SERVICE_NAME_KEY = "import.meta.env.VITE_INTERFERE_SERVICE_NAME";

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

  test("returns a plugin with the correct name", () => {
    const plugin = interfere();

    expect(plugin.name).toBe("interfere");
  });

  describe("browser define", () => {
    test("defaults to the Interfere endpoint (null apiHost)", async () => {
      const define = await callConfigDefine(interfere());

      expect(define[API_HOST_KEY]).toBe("null");
      expect(define[ENVIRONMENT_KEY]).toBe('"production"');

      expect(define[RELEASE_SLUG_KEY]).toBe(
        JSON.stringify(deriveReleaseSlug(gitSha)),
      );
    });

    test("inlines a normalised apiHost origin", async () => {
      const define = await callConfigDefine(
        interfere({ apiHost: "https://in.example.com/ignored" }),
      );

      expect(define[API_HOST_KEY]).toBe('"https://in.example.com"');
    });

    test("rejects an apiHost that is not a full URL", async () => {
      await expect(
        callConfigDefine(interfere({ apiHost: "in.example.com" })),
      ).rejects.toThrow("apiHost");
    });

    test("does not depend on the surrounding plugin set", async () => {
      const define = await callConfigDefine(interfere(), {
        plugins: [{ name: "vite:react" }, { name: "tanstack-start" }],
      });

      expect(define[API_HOST_KEY]).toBe("null");
    });

    test("uses the commitSha override for the release slug", async () => {
      const define = await callConfigDefine(
        interfere({ commitSha: "abcd0000abcd0000abcd0000abcd0000abcd0000" }),
      );

      expect(define[RELEASE_SLUG_KEY]).toBe(
        JSON.stringify(
          deriveReleaseSlug("abcd0000abcd0000abcd0000abcd0000abcd0000"),
        ),
      );
    });

    test("derives the release from the git checkout at the Vite root, not the process cwd", async () => {
      vi.stubEnv("GIT_DIR", undefined);

      const config = await resolvePluginConfig(interfere(), {
        root: appRepo.dir,
      });

      expect(config.define?.[RELEASE_SLUG_KEY]).toBe(
        JSON.stringify(deriveReleaseSlug(appRepo.sha)),
      );
    });

    test("gives bundle variants stable distinct release identities", async () => {
      const web = await callConfigDefine(interfere());

      const desktop = await callConfigDefine(
        interfere({ buildVariant: "desktop" }),
      );

      const repeated = await callConfigDefine(
        interfere({ buildVariant: "desktop" }),
      );

      const other = await callConfigDefine(
        interfere({ buildVariant: "other" }),
      );

      expect(desktop[RELEASE_SLUG_KEY]).not.toBe(web[RELEASE_SLUG_KEY]);
      expect(desktop[RELEASE_SLUG_KEY]).toBe(repeated[RELEASE_SLUG_KEY]);
      expect(desktop[RELEASE_SLUG_KEY]).not.toBe(other[RELEASE_SLUG_KEY]);

      expect(web[RELEASE_SLUG_KEY]).toBe(
        JSON.stringify(deriveReleaseSlug(gitSha)),
      );
    });

    test("stamps the unscoped package name as the default service name", async () => {
      writeFileSync(
        path.join(projectDir, "package.json"),
        JSON.stringify({ name: "@acme/storefront" }),
      );

      const define = await callConfigDefine(interfere());

      expect(define[SERVICE_NAME_KEY]).toBe(JSON.stringify("storefront"));

      const dev = await callConfigDefine(interfere(), { command: "serve" });

      expect(dev[SERVICE_NAME_KEY]).toBe(JSON.stringify("storefront"));
    });

    test("uses shared service-name precedence and ignores the removed Vite environment alias", async () => {
      writeFileSync(
        path.join(projectDir, "package.json"),
        JSON.stringify({ name: "@acme/storefront" }),
      );

      vi.stubEnv("VITE_INTERFERE_SERVICE_NAME", "checkout");

      const fromEnv = await callConfigDefine(interfere());

      expect(fromEnv[SERVICE_NAME_KEY]).toBe(JSON.stringify("storefront"));

      vi.stubEnv("OTEL_SERVICE_NAME", "otel-checkout");

      const fromOtel = await callConfigDefine(interfere());

      expect(fromOtel[SERVICE_NAME_KEY]).toBe(JSON.stringify("otel-checkout"));

      const fromOption = await callConfigDefine(
        interfere({ serviceName: "storefront-web" }),
      );

      expect(fromOption[SERVICE_NAME_KEY]).toBe(
        JSON.stringify("storefront-web"),
      );
    });

    test("stamps null when no name can be derived", async () => {
      writeFileSync(
        path.join(projectDir, "package.json"),
        JSON.stringify({ name: "@acme/" }),
      );

      vi.stubEnv("VITE_INTERFERE_SERVICE_NAME", "   ");

      const define = await callConfigDefine(interfere({ serviceName: " " }));

      expect(define[SERVICE_NAME_KEY]).toBe("null");
    });

    test("emits a null environment during dev (command=serve)", async () => {
      const define = await callConfigDefine(interfere(), { command: "serve" });

      expect(define[ENVIRONMENT_KEY]).toBe("null");

      expect(define[DEV_RELEASE_SLUG_KEY]).toBe(
        JSON.stringify(deriveReleaseSlug(gitSha)),
      );
    });
  });
});
