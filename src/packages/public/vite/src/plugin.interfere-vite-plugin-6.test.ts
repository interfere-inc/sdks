/* oxlint-disable typescript/no-unsafe-assignment */
import { setupServer } from "msw/node";

import { execFileSync } from "node:child_process";
import {
  existsSync,
  mkdirSync,
  mkdtempSync,
  rmSync,
  utimesSync,
  writeFileSync,
} from "node:fs";
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
    test.each(["upload", "finalize"])(
      "removes maps when the provider rejects %s",
      async (step) => {
        vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test");

        server.use(
          step === "upload"
            ? http.put(
                `${RELEASE_API}/v3/releases/${SLUG}/source-maps`,
                () => new HttpResponse(null, { status: 400 }),
              )
            : http.post(
                `${RELEASE_API}/v3/releases/${SLUG}/source-maps/finalize`,
                () => HttpResponse.json({ ok: false, fileCount: 0 }),
              ),
        );

        const capture = captureLogger();
        const plugin = interfere();

        await resolvePluginConfig(plugin, { logger: capture.logger });

        const closeBundle = plugin.closeBundle as () => Promise<void>;

        await expect(closeBundle()).resolves.toBeUndefined();

        expect(releaseRequests).toHaveLength(1);

        expect(existsSync(path.join(projectDir, "dist/app.js.map"))).toBe(
          false,
        );

        expect(capture.warns).toContainEqual(
          expect.stringContaining("release"),
        );
      },
    );

    test("waits for server maps at serverSourcemapsDir before uploading on a server build", async () => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test");

      const serverDir = mkdtempSync(
        path.join(tmpdir(), "interfere-server-maps-"),
      );

      const plugin = interfere({ serverSourcemapsDir: serverDir });

      await resolvePluginConfig(plugin, {
        command: "build",
        plugins: [{ name: "tanstack-start" }],
      });

      const closeBundle = plugin.closeBundle as () => Promise<void>;

      await closeBundle();

      expect(configRequests).toHaveLength(1);

      expect(releaseRequests).toHaveLength(1);

      expect(uploadRequests).toHaveLength(0);

      rmSync(serverDir, { force: true, recursive: true });
    });

    test("uploads once server maps appear at serverSourcemapsDir on a server build", async () => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test");

      const serverDir = mkdtempSync(
        path.join(tmpdir(), "interfere-server-maps-"),
      );

      const plugin = interfere({ serverSourcemapsDir: serverDir });

      await resolvePluginConfig(plugin, {
        command: "build",
        plugins: [{ name: "tanstack-start" }],
      });

      writeFileSync(path.join(serverDir, "index.js.map"), "{}");

      const fresh = new Date(Date.now() + 1000);

      utimesSync(path.join(serverDir, "index.js.map"), fresh, fresh);

      const closeBundle = plugin.closeBundle as () => Promise<void>;

      await closeBundle();

      expect(releaseRequests).toHaveLength(1);
      expect(finalizeRequests).toHaveLength(1);
      expect(uploadRequests).toHaveLength(1);

      expect(uploadRequests[0]?.headers.get("content-encoding")).toBeNull();

      const body = uploadRequests[0]?.body;

      await expect(new Response(body).json()).resolves.toMatchObject({
        version: 3,
        sources: ["src/app.ts"],
        debugId: expect.any(String),
      });

      expect(existsSync(path.join(projectDir, "dist/app.js.map"))).toBe(false);

      await closeBundle();

      expect(releaseRequests).toHaveLength(1);

      rmSync(serverDir, { force: true, recursive: true });
    });

    test("uploads once server maps appear under the top-level outDir after the SSR environment re-resolves config", async () => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test");

      const plugin = interfere();
      const serverPlugins = [{ name: "tanstack-start" }];

      await resolvePluginConfig(plugin, {
        build: { outDir: "dist" },
        plugins: serverPlugins,
      });

      await resolvePluginConfig(plugin, {
        build: { outDir: "dist/server" },
        plugins: serverPlugins,
      });

      const serverMap = path.join(projectDir, "dist/server/index.js.map");

      mkdirSync(path.dirname(serverMap), { recursive: true });

      writeFileSync(serverMap, "{}");

      const fresh = new Date(Date.now() + 1000);

      utimesSync(serverMap, fresh, fresh);

      await (plugin.closeBundle as () => Promise<void>)();

      expect(releaseRequests).toHaveLength(1);
    });
  });
});
