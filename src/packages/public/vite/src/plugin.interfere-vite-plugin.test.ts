import { setupServer } from "msw/node";

import { execFileSync } from "node:child_process";
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { HttpResponse, http } from "msw";
import { tmpdir } from "node:os";
import path from "node:path";
import { createBuilder, createLogger, resolveConfig } from "vite";
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

  describe("mode resolution", () => {
    test.each([
      {
        name: "@acme/storefront",
        option: undefined,
        env: undefined,
        unresolved: false,
        warnings: [],
      },
      {
        name: "@acme/vite",
        option: undefined,
        env: undefined,
        unresolved: false,
        warnings: [],
      },
      {
        name: "@acme/storefront",
        option: "chosen-name",
        env: undefined,
        unresolved: false,
        warnings: [],
      },
      {
        name: "@acme/storefront",
        option: undefined,
        env: "chosen-name",
        unresolved: false,
        warnings: [],
      },
      {
        name: "@acme/storefront",
        option: " ",
        env: " ",
        unresolved: false,
        warnings: [],
      },
      {
        name: "@acme/",
        option: undefined,
        env: undefined,
        unresolved: false,
        warnings: [],
      },
      {
        name: "@acme/storefront",
        option: undefined,
        env: undefined,
        unresolved: true,
        warnings: [],
      },
    ])(
      "never warns about an App for the inferred service name: $name / $option / $env / $unresolved",
      async ({ name, option, env, unresolved, warnings: expected }) => {
        writeFileSync(
          path.join(projectDir, "package.json"),
          JSON.stringify({ name }),
        );

        vi.stubEnv("OTEL_SERVICE_NAME", env);

        if (unresolved) {
          server.use(
            http.get(`${RELEASE_API}/v3/releases/config`, () =>
              HttpResponse.json({ org: { id: "org_1", slug: "acme" } }),
            ),
          );
        }

        const plugin = interfere(
          option === undefined ? {} : { serviceName: option },
        );

        const capture = captureLogger();

        const config = await resolvePluginConfig(plugin, {
          logger: capture.logger,
        });

        (plugin.buildStart as () => void)();

        (plugin.buildStart as () => void)();

        expect(
          config.define?.["import.meta.env.VITE_INTERFERE_DISABLED"],
        ).not.toBe('"true"');

        const warnings = capture.warns.filter(
          (message) =>
            message.includes("inferred service name") ||
            /\bApp\b/u.test(message),
        );

        expect(warnings).toStrictEqual(expected);
      },
    );

    test("announces the auto-detected mode only once across repeated configResolved calls", async () => {
      const plugin = interfere();
      const capture = captureLogger();

      const config = {
        env: { VITE_INTERFERE_PUBLIC_KEY: "interfere_pk_dev" },
        logger: capture.logger,
        plugins: [{ name: "vite:react" }],
      };

      await resolvePluginConfig(plugin, config);

      await resolvePluginConfig(plugin, config);

      await resolvePluginConfig(plugin, config);

      await resolvePluginConfig(plugin, config);

      expect(capture.infos).toHaveLength(0);
      expect(capture.warns).toHaveLength(0);

      (plugin.buildStart as () => void)();

      (plugin.buildStart as () => void)();

      const announcements = capture.infos.filter((msg) =>
        msg.includes("ingest →"),
      );

      expect(announcements).toHaveLength(1);
    });

    test("announces once when Vite resolves and builds multiple environments", async () => {
      const capture = captureLogger();

      const config = {
        configFile: false as const,
        root: projectDir,
        customLogger: capture.logger,
        plugins: [interfere({ sourceMaps: { upload: false } })],
        environments: {
          client: {
            build: { write: false, rolldownOptions: { input: "entry.js" } },
          },
          ssr: {
            build: { write: false, rolldownOptions: { input: "entry.js" } },
          },
        },
      };

      writeFileSync(
        path.join(projectDir, "entry.js"),
        "export const value = 1;",
      );

      await resolveConfig(
        { ...config, plugins: [interfere({ sourceMaps: { upload: false } })] },
        "build",
      );

      await resolveConfig(
        { ...config, plugins: [interfere({ sourceMaps: { upload: false } })] },
        "build",
      );

      expect(capture.infos).toHaveLength(0);
      expect(capture.warns).toHaveLength(0);

      const builder = await createBuilder(config);

      await builder.buildApp();

      expect(
        capture.infos.filter((line) => line.includes("ingest →")),
      ).toHaveLength(1);
    });
  });
});
