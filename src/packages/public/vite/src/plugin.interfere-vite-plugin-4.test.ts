import { setupServer } from "msw/node";

import { execFileSync } from "node:child_process";
import { existsSync, mkdtempSync, rmSync } from "node:fs";
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

import { installVitePluginCase, projectDir } from "./plugin.test-utils";

const MILLIS_PER_SECOND = 1000;

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

  describe("closeBundle release metadata gate", () => {
    test.each([
      ["release", HttpResponse.error()],
      ["sign", HttpResponse.json({}, { status: 403 })],
      ["upload", new HttpResponse(null, { status: 400 })],
      ["finalize", HttpResponse.json({}, { status: 404 })],
    ] as const)(
      "continues the build when the provider fails %s",
      async (step, response) => {
        vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test");

        const endpoints = {
          release: "/v3/releases",
          sign: `/v3/releases/${SLUG}/source-maps/sign`,
          upload: `/v3/releases/${SLUG}/source-maps`,
          finalize: `/v3/releases/${SLUG}/source-maps/finalize`,
        };

        const method = step === "upload" ? http.put : http.post;

        server.use(
          method(`${RELEASE_API}${endpoints[step]}`, () => response.clone()),
        );

        const capture = captureLogger();
        const plugin = interfere();

        await resolvePluginConfig(plugin, { logger: capture.logger });

        await expect(
          (plugin.closeBundle as () => Promise<void>)(),
        ).resolves.toBeUndefined();

        expect(existsSync(path.join(projectDir, "dist/app.js.map"))).toBe(
          false,
        );

        expect(capture.warns).toContainEqual(
          expect.stringContaining("Skipping release pipeline"),
        );
      },
    );

    test("continues the build when finalize times out", async () => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test");

      server.use(
        http.post(`${RELEASE_API}/v3/releases/${SLUG}/source-maps/sign`, () =>
          HttpResponse.json({
            expiresAt: Math.floor(Date.now() / MILLIS_PER_SECOND) + 2,
            uploads: [{ path: "app.js.map" }],
          }),
        ),
        http.post(
          `${RELEASE_API}/v3/releases/${SLUG}/source-maps/finalize`,
          async ({ request }) => {
            const { promise, reject } = Promise.withResolvers<Response>();

            request.signal.addEventListener("abort", () => {
              reject(request.signal.reason);
            });

            return promise;
          },
        ),
      );

      const capture = captureLogger();
      const plugin = interfere();

      await resolvePluginConfig(plugin, { logger: capture.logger });

      await expect(
        (plugin.closeBundle as () => Promise<void>)(),
      ).resolves.toBeUndefined();

      expect(existsSync(path.join(projectDir, "dist/app.js.map"))).toBe(false);

      expect(capture.warns).toContainEqual(
        expect.stringContaining("Skipping release pipeline"),
      );
    });

    test("continues a production build with one warning without a commit SHA", async () => {
      vi.stubEnv("GIT_DIR", path.join(projectDir, "missing-git"));

      const capture = captureLogger();
      const fetchSpy = vi.spyOn(globalThis, "fetch");
      const plugin = interfere();

      await resolvePluginConfig(plugin, { logger: capture.logger });

      (plugin.buildStart as () => void)();

      (plugin.buildStart as () => void)();

      await expect(
        (plugin.closeBundle as () => Promise<void>)(),
      ).resolves.toBeUndefined();

      expect(fetchSpy).not.toHaveBeenCalled();

      expect(capture.warns).toStrictEqual([
        expect.stringContaining("Commit SHA missing"),
      ]);
    });
  });
});
