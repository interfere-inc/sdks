import { setupServer } from "msw/node";
import { createReleaseRequest } from "@interfere/types/releases/definition";

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

import { BAKED_RELEASE_SLUG_DEFINE } from "./internal/baked-release";
import { PRODUCER_VERSION } from "./version";
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

const RELEASE_SLUG_KEY = BAKED_RELEASE_SLUG_DEFINE;

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
    test("publishes separate Vercel artifacts with matching baked identities and unchanged commit provenance", async () => {
      expect.assertions(11);

      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test");

      const slugs: unknown[] = [];
      const chunkPath = path.join(projectDir, "dist/app.js");
      const mapPath = `${chunkPath}.map`;
      const chunk = readFileSync(chunkPath, "utf8");
      const map = readFileSync(mapPath, "utf8");

      for (const deploymentId of ["dpl_first", "dpl_second"]) {
        vi.stubEnv("VERCEL_DEPLOYMENT_ID", deploymentId);

        writeFileSync(chunkPath, chunk);

        writeFileSync(mapPath, map);

        const plugin = interfere();

        // oxlint-disable-next-line eslint/no-await-in-loop -- Each simulated deployment is configured and published before the next.
        const config = await resolvePluginConfig(plugin);

        // oxlint-disable-next-line eslint/no-await-in-loop -- Each simulated deployment is configured and published before the next.
        await (plugin.closeBundle as () => Promise<void>)();

        expect(releaseRequests).toHaveLength(slugs.length + 1);

        const request = releaseRequests.at(-1);

        expect(request).toBeDefined();

        // oxlint-disable-next-line eslint/no-await-in-loop -- Inspect the request belonging to this deployment.
        const body = createReleaseRequest.parse(await request?.json());

        expect(body.buildId).toBe(gitSha);
        expect(body.source.commitSha).toBe(gitSha);

        expect(JSON.stringify(body.slug)).toBe(
          config.define?.[RELEASE_SLUG_KEY],
        );

        slugs.push(body.slug);
      }

      expect(slugs[0]).not.toBe(slugs[1]);
    });

    test("accepts current regional secret keys for release uploads", async () => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_secret_us_test_123");

      const plugin = interfere();

      await resolvePluginConfig(plugin, { command: "build" });

      const closeBundle = plugin.closeBundle as () => Promise<void>;

      await closeBundle();

      expect(releaseRequests).toHaveLength(1);

      expect(releaseRequests[0]?.headers.get("authorization")).toBe(
        "Bearer interfere_secret_us_test_123",
      );
    });

    test("names the app with its public key and service on every release request", async () => {
      const plugin = interfere({ serviceName: "storefront" });

      await resolvePluginConfig(plugin, { command: "build" });

      const closeBundle = plugin.closeBundle as () => Promise<void>;

      await closeBundle();

      const requests = [
        ...releaseRequests,
        ...uploadRequests,
        ...finalizeRequests,
      ];

      expect(requests).toHaveLength(3);

      expect(
        requests.map((request) =>
          request.headers.get("x-interfere-public-key"),
        ),
      ).toStrictEqual(Array.from(requests, () => "interfere_pk_test_fake"));

      expect(
        requests.map((request) => request.headers.get("x-interfere-service")),
      ).toStrictEqual(Array.from(requests, () => "storefront"));

      expect(
        configRequests.map((request) =>
          request.headers.get("x-interfere-producer-version"),
        ),
      ).toStrictEqual([PRODUCER_VERSION]);
    });

    test.each([
      ["AUTH_MISSING", "Secret key could not be read"],
      ["AUTH_AK_INVALID", "Secret key rejected by server"],
      ["AUTH_AK_SCOPE_MISMATCH", "Secret key missing release scope"],
      ["AUTH_AK_SURFACE_REQUIRED", "Service needed for a workspace secret key"],
    ] as const)(
      "logs an error and continues the build for %s release pipeline responses",
      async (code, expectedMessage) => {
        vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test_123");

        const failedRequests: Request[] = [];

        server.use(
          http.post(`${RELEASE_API}/v3/releases`, ({ request }) => {
            failedRequests.push(request);

            return HttpResponse.json({ code }, { status: 401 });
          }),
        );

        const capture = captureLogger();
        const plugin = interfere();

        await resolvePluginConfig(plugin, { logger: capture.logger });

        const closeBundle = plugin.closeBundle as () => Promise<void>;

        await expect(closeBundle()).resolves.toBeUndefined();

        expect(capture.errors).toContainEqual(
          expect.stringContaining(expectedMessage),
        );

        expect(failedRequests).toHaveLength(2);
        expect(finalizeRequests).toHaveLength(0);

        expect(existsSync(path.join(projectDir, "dist/app.js.map"))).toBe(
          false,
        );
      },
    );
  });
});
