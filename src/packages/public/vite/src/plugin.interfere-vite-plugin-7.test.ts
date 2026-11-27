import { setupServer } from "msw/node";

import { execFileSync } from "node:child_process";
import {
  existsSync,
  mkdtempSync,
  readFileSync,
  rmSync,
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

import { BAKED_RELEASE_TOKEN_DEFINE } from "./internal/baked-release";
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
    test.each(["{", "[]"])(
      "removes every forced map and sourceMappingURL when one map is malformed (%s)",
      async (malformed) => {
        expect.hasAssertions();

        vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test");

        writeFileSync(
          path.join(projectDir, "dist/vendor.js"),
          "console.log('vendor');\n//# sourceMappingURL=vendor.js.map",
        );

        writeFileSync(path.join(projectDir, "dist/vendor.js.map"), malformed);

        const capture = captureLogger();
        const plugin = interfere();

        await resolvePluginConfig(plugin, { logger: capture.logger });

        await expect(
          (plugin.closeBundle as () => Promise<void>)(),
        ).resolves.toBeUndefined();

        expect(releaseRequests).toHaveLength(1);

        for (const chunk of ["app.js", "vendor.js"]) {
          expect(existsSync(path.join(projectDir, `dist/${chunk}.map`))).toBe(
            false,
          );

          expect(
            readFileSync(path.join(projectDir, `dist/${chunk}`), "utf8"),
          ).not.toContain("sourceMappingURL");
        }

        expect(capture.warns).toContainEqual(
          expect.stringContaining("Skipping release pipeline"),
        );
      },
    );

    test("warns when discovery finds no maps and leaves the release unready", async () => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test");

      rmSync(path.join(projectDir, "dist/app.js.map"));

      const capture = captureLogger();
      const plugin = interfere();

      await resolvePluginConfig(plugin, { logger: capture.logger });

      const closeBundle = plugin.closeBundle as () => Promise<void>;

      await closeBundle();

      expect(releaseRequests).toHaveLength(1);

      expect(capture.warns).toContainEqual(
        expect.stringContaining("release remains unready"),
      );
    });

    test("bakes the release token from the build's single release registration without a public key", async () => {
      server.use(
        http.post(`${RELEASE_API}/v3/releases`, ({ request }) => {
          releaseRequests.push(request.clone());

          return HttpResponse.json({
            build: { hash: gitSha },
            destination: {
              environment: "production",
              id: "release_1",
              slug: SLUG,
            },
            org: { id: "org_1", slug: "acme" },
            source: { id: "source_1" },
            surface: { id: "surface_1", slug: "vite" },
            token: "irt_signed",
          });
        }),
      );

      const plugin = interfere({ sourceMaps: { upload: false } });

      const config = await resolvePluginConfig(plugin);

      expect(config.define).toMatchObject({
        [BAKED_RELEASE_TOKEN_DEFINE]: JSON.stringify("irt_signed"),
      });

      expect(releaseRequests).toHaveLength(1);

      expect(
        releaseRequests[0]?.headers.get("x-interfere-public-key"),
      ).toBeNull();

      await (plugin.closeBundle as () => Promise<void>)();

      expect(releaseRequests).toHaveLength(1);
    });

    test("registers the release without uploading when uploads are disabled", async () => {
      const plugin = interfere({ sourceMaps: { upload: false } });
      const scriptPath = path.join(projectDir, "dist/app.js");
      const mapPath = path.join(projectDir, "dist/app.js.map");
      const scriptBefore = readFileSync(scriptPath, "utf8");
      const mapBefore = readFileSync(mapPath, "utf8");

      await resolvePluginConfig(plugin);

      const closeBundle = plugin.closeBundle as () => Promise<void>;

      await expect(closeBundle()).resolves.toBeUndefined();

      expect(configRequests).toHaveLength(1);
      expect(releaseRequests).toHaveLength(1);
      expect(uploadRequests).toHaveLength(0);
      expect(finalizeRequests).toHaveLength(0);
      expect(readFileSync(scriptPath, "utf8")).toBe(scriptBefore);
      expect(readFileSync(mapPath, "utf8")).toBe(mapBefore);
    });

    test("skips upload during vite serve even when an API key is set", async () => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test");

      const plugin = interfere();

      await resolvePluginConfig(plugin, { command: "serve" });

      const closeBundle = plugin.closeBundle as () => Promise<void>;

      await closeBundle();

      expect(configRequests).toHaveLength(0);
      expect(releaseRequests).toHaveLength(0);
    });

    test("registers no release for vite preview", async () => {
      vi.stubEnv("VITEST", "");

      vi.stubEnv("VITEST_WORKER_ID", "");

      await resolvePluginConfig(interfere(), {
        command: "serve",
        preview: true,
      });

      expect(releaseRequests).toHaveLength(0);
    });

    test("mints one local dev release during vite serve when an API key is set", async () => {
      vi.stubEnv("VITEST", "");

      vi.stubEnv("VITEST_WORKER_ID", "");

      await resolvePluginConfig(interfere(), { command: "serve" });

      expect(releaseRequests).toHaveLength(1);
    });
  });
});
