import { setupServer } from "msw/node";
import {
  LOCAL_DEV_RELEASE_SLUG,
  deriveReleaseSlug,
} from "@interfere/types/releases/slug";
import { createReleaseRequest } from "@interfere/types/releases/definition";

import { execFileSync } from "node:child_process";
import { mkdtempSync, rmSync } from "node:fs";
import { HttpResponse, http } from "msw";
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

const RELEASE_SLUG_KEY = BAKED_RELEASE_SLUG_DEFINE;

const DEV_RELEASE_SLUG_KEY = "import.meta.env.VITE_INTERFERE_RELEASE_SLUG";

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

  describe("closeBundle release metadata gate", () => {
    test("warns once when the dev server cannot mint a local development release token", async () => {
      vi.stubEnv("VITEST", "");

      vi.stubEnv("VITEST_WORKER_ID", "");

      const capture = captureLogger();

      await resolvePluginConfig(interfere(), {
        command: "serve",
        logger: capture.logger,
      });

      expect(
        capture.warns.filter((message) =>
          message.includes("could not mint a local development release token"),
        ),
      ).toHaveLength(1);
    });

    test("registers only the local development release during vite serve and uploads nothing", async () => {
      vi.stubEnv("VITEST", "");

      vi.stubEnv("VITEST_WORKER_ID", "");

      const plugin = interfere();

      await resolvePluginConfig(plugin, { command: "serve" });

      const closeBundle = plugin.closeBundle as () => Promise<void>;

      await closeBundle();

      expect(configRequests).toHaveLength(0);
      expect(releaseRequests).toHaveLength(1);
      expect(uploadRequests).toHaveLength(0);
      expect(finalizeRequests).toHaveLength(0);

      expect(
        releaseRequests[0]?.headers.get("x-interfere-public-key"),
      ).toBeNull();

      await expect(releaseRequests[0]?.json()).resolves.toMatchObject({
        buildId: LOCAL_DEV_RELEASE_SLUG,
        destination: null,
        slug: LOCAL_DEV_RELEASE_SLUG,
        source: { commitSha: null },
      });
    });

    test("bakes the local development release token and slug into the dev server", async () => {
      vi.stubEnv("VITEST", "");

      vi.stubEnv("VITEST_WORKER_ID", "");

      server.use(
        http.post(`${RELEASE_API}/v3/releases`, () =>
          HttpResponse.json({
            build: { hash: gitSha },
            destination: {
              environment: null,
              id: "release_local",
              slug: LOCAL_DEV_RELEASE_SLUG,
            },
            org: { id: "org_1", slug: "acme" },
            source: { id: "source_1" },
            surface: null,
            token: "interfere_release_local",
          }),
        ),
      );

      const define = await callConfigDefine(interfere(), { command: "serve" });

      expect(define["import.meta.env.VITE_INTERFERE_RELEASE_TOKEN"]).toBe(
        JSON.stringify("interfere_release_local"),
      );

      expect(define[DEV_RELEASE_SLUG_KEY]).toBe(
        JSON.stringify(LOCAL_DEV_RELEASE_SLUG),
      );
    });

    test("passes the resolved deploy environment to the release pipeline", async () => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test");

      vi.stubEnv("INTERFERE_ENVIRONMENT", "staging");

      const plugin = interfere();

      await resolvePluginConfig(plugin, { command: "build" });

      const closeBundle = plugin.closeBundle as () => Promise<void>;

      await closeBundle();

      expect(releaseRequests).toHaveLength(1);

      await expect(releaseRequests[0]?.json()).resolves.toMatchObject({
        destination: { environment: "staging" },
      });
    });

    test("publishes a variant using its runtime slug and the original commit", async () => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test");

      const plugin = interfere({ buildVariant: "desktop" });

      const config = await resolvePluginConfig(plugin);

      await (plugin.closeBundle as () => Promise<void>)();

      expect(releaseRequests).toHaveLength(1);

      const request = createReleaseRequest.parse(
        await releaseRequests[0]?.json(),
      );

      expect(request.buildId).toBe(gitSha);

      expect(JSON.stringify(request.slug)).toBe(
        config.define?.[RELEASE_SLUG_KEY],
      );

      expect(config.define?.[RELEASE_SLUG_KEY]).not.toBe(
        JSON.stringify(deriveReleaseSlug(gitSha)),
      );

      expect(uploadRequests).toHaveLength(1);
      expect(finalizeRequests).toHaveLength(1);
    });
  });
});
