/* oxlint-disable typescript/no-unsafe-assignment */
import { setupServer } from "msw/node";
import { deriveReleaseSlug } from "@interfere/types/releases/slug";
import { createReleaseRequest } from "@interfere/types/releases/definition";
import { releaseSourceIdEnvKeys } from "@interfere/types/integrations";

import { execFileSync } from "node:child_process";
import {
  existsSync,
  mkdirSync,
  mkdtempSync,
  readFileSync,
  rmSync,
  utimesSync,
  writeFileSync,
} from "node:fs";
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

import { PRODUCER_VERSION } from "./version";

const { interfere } = await import("./plugin.js");

const server = setupServer();

const RELEASE_API = "https://vite-release.test";

const SLUG = "rel_deadbeef12345678";

let projectDir: string;

let configRequests: Request[];

let releaseRequests: Request[];

let uploadRequests: Request[];

let finalizeRequests: Request[];

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

  server.listen({ onUnhandledRequest: "error" });
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

const API_HOST_KEY = "import.meta.env.VITE_INTERFERE_API_HOST";

const ENVIRONMENT_KEY = "import.meta.env.VITE_INTERFERE_ENVIRONMENT";

const RELEASE_SLUG_KEY = "import.meta.env.VITE_INTERFERE_RELEASE_SLUG";

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
    vi.clearAllMocks();

    vi.stubEnv("GIT_DIR", path.join(gitDir, ".git"));

    for (const key of releaseSourceIdEnvKeys) {
      vi.stubEnv(key, undefined);
    }

    vi.stubEnv("VITE_INTERFERE_PUBLIC_KEY", "interfere_pk_test_fake");

    vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test");

    vi.stubEnv("NEXT_PUBLIC_INTERFERE_BUILD_ID", undefined);

    vi.stubEnv("OTEL_SERVICE_NAME", undefined);

    vi.stubEnv("INTERFERE_SOURCE_ID", undefined);

    vi.stubEnv("NEXT_BUILD_ID", undefined);

    vi.stubEnv("VERCEL_GIT_COMMIT_SHA", undefined);

    vi.stubEnv("GITHUB_SHA", undefined);

    vi.stubEnv("VERCEL_ENV", undefined);

    vi.stubEnv("VERCEL_TARGET_ENV", undefined);

    vi.stubEnv("CLOUDFLARE_ENV", undefined);

    vi.stubEnv("INTERFERE_ENVIRONMENT", "production");

    projectDir = mkdtempSync(path.join(tmpdir(), "interfere-vite-build-"));

    mkdirSync(path.join(projectDir, "dist"));

    writeFileSync(
      path.join(projectDir, "dist/app.js"),
      "console.log('app');\n//# sourceMappingURL=app.js.map",
    );

    writeFileSync(
      path.join(projectDir, "dist/app.js.map"),
      JSON.stringify({
        version: 3,
        sources: ["src/app.ts"],
        names: [],
        mappings: "AAAA",
        sourcesContent: ["console.log('app');"],
      }),
    );

    vi.stubEnv("INTERFERE_API_URL", RELEASE_API);

    configRequests = [];

    server.use(
      http.get(`${RELEASE_API}/v3/releases/config`, ({ request }) => {
        configRequests.push(request);

        return HttpResponse.json(
          {
            org: { id: "org_1", slug: "acme" },
            surface: {
              destinationProvider: "cloudflare",
              environmentFiltering: false,
              sdkConfig: null,
              id: "surface_1",
              slug: "vite",
              sourceProvider: "github",
            },
          },
          { status: 200 },
        );
      }),
    );

    releaseRequests = [];

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
        });
      }),
    );

    server.use(
      http.post(`${RELEASE_API}/v3/releases/${SLUG}/source-maps/sign`, () =>
        HttpResponse.json(
          {
            expiresAt: Math.floor(Date.now() / 1000) + 600,
            uploads: [
              {
                path: "app.js.map",
              },
            ],
          },
          { status: 200 },
        ),
      ),
    );

    uploadRequests = [];

    server.use(
      http.put(
        `${RELEASE_API}/v3/releases/${SLUG}/source-maps`,
        ({ request }) => {
          uploadRequests.push(request.clone());

          return new HttpResponse(null);
        },
      ),
    );

    finalizeRequests = [];

    server.use(
      http.post(
        `${RELEASE_API}/v3/releases/${SLUG}/source-maps/finalize`,
        ({ request }) => {
          finalizeRequests.push(request);

          return HttpResponse.json(
            {
              fileCount: 1,
              ok: true,
            },
            { status: 200 },
          );
        },
      ),
    );
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

      expect(define[RELEASE_SLUG_KEY]).toBe(
        JSON.stringify(deriveReleaseSlug(gitSha)),
      );
    });
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

    test("warns when a server-rendered build has no apiHost set", async () => {
      const plugin = interfere();
      const capture = captureLogger();

      await resolvePluginConfig(plugin, {
        env: { VITE_INTERFERE_PUBLIC_KEY: "interfere_pk_dev" },
        logger: capture.logger,
        plugins: [{ name: "vite:react" }, { name: "tanstack-start" }],
      });

      (plugin.buildStart as () => void)();

      expect(
        capture.warns.some((line) => line.includes("server-rendered")),
      ).toBe(true);
    });

    test("does not warn when a vanilla vite app has no apiHost", async () => {
      const plugin = interfere();
      const capture = captureLogger();

      await resolvePluginConfig(plugin, {
        env: { VITE_INTERFERE_PUBLIC_KEY: "interfere_pk_dev" },
        logger: capture.logger,
        plugins: [{ name: "vite:react" }],
      });

      (plugin.buildStart as () => void)();

      expect(
        capture.warns.some((line) => line.includes("server-rendered")),
      ).toBe(false);
    });

    test("does not warn about SSR when an apiHost is set", async () => {
      const plugin = interfere({ apiHost: "https://in.example.com" });
      const capture = captureLogger();

      await resolvePluginConfig(plugin, {
        env: { VITE_INTERFERE_PUBLIC_KEY: "interfere_pk_dev" },
        logger: capture.logger,
        plugins: [{ name: "vite:react" }, { name: "tanstack-start" }],
      });

      (plugin.buildStart as () => void)();

      expect(
        capture.warns.some((line) => line.includes("server-rendered")),
      ).toBe(false);
    });
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
            expiresAt: Math.floor(Date.now() / 1000) + 2,
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

    test.each([
      {
        accepted: true,
        frameworkBuilder: false,
        expectedSource: {
          releaseSlug: SLUG,
          status: { state: "accepted", manifestReady: false },
        },
      },
      { accepted: false, frameworkBuilder: false, expectedSource: null },
      {
        accepted: true,
        frameworkBuilder: true,
        expectedSource: {
          releaseSlug: SLUG,
          status: { state: "accepted", manifestReady: false },
        },
      },
      { accepted: false, frameworkBuilder: true, expectedSource: null },
    ])(
      "records native client and server publication only when accepted: %j",
      async ({ accepted, frameworkBuilder, expectedSource }) => {
        vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test");

        rmSync(path.join(projectDir, "dist"), { recursive: true });

        writeFileSync(
          path.join(projectDir, "entry.js"),
          "globalThis.app = 'client and server';",
        );

        server.use(
          http.post(
            `${RELEASE_API}/v3/releases/${SLUG}/source-maps/sign`,
            async ({ request }) => {
              const body = (await request.json()) as {
                files: { path: string }[];
              };

              return HttpResponse.json({
                expiresAt: Math.floor(Date.now() / 1000) + 600,
                uploads: body.files.map(({ path: mapPath }) => ({
                  path: mapPath,
                })),
              });
            },
          ),
        );

        server.use(
          http.post(
            `${RELEASE_API}/v3/releases/${SLUG}/source-maps/finalize`,
            ({ request }) => {
              finalizeRequests.push(request.clone());

              return HttpResponse.json({
                ok: accepted,
                fileCount: accepted ? 2 : 0,
              });
            },
          ),
        );

        server.use(
          http.get(`${RELEASE_API}/v3/releases/${SLUG}/source-maps`, () =>
            HttpResponse.json({
              releaseSlug: SLUG,
              environment: "production",
              sessionId: "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa",
              state: finalizeRequests.length > 0 ? "accepted" : "uploading",
              batchDigest: null,
              fileCount: 2,
              publishedFileCount: 0,
              manifestReady: false,
            }),
          ),
        );

        const config: NonNullable<Parameters<typeof createBuilder>[0]> = {
          configFile: false,
          root: projectDir,
          logLevel: "silent",
          plugins: [
            interfere({ buildReceipt: ".cache/receipt.json" }),
            { name: "tanstack-start" },
          ],
          build: { outDir: "dist", sourcemap: true },
          environments: {
            client: {
              build: {
                outDir: "dist/client",
                rolldownOptions: { input: "entry.js" },
              },
            },
            ssr: {
              build: {
                outDir: "dist/server",
                rolldownOptions: { input: "entry.js" },
              },
            },
          },
        };

        if (frameworkBuilder) {
          config.builder = {
            async buildApp(appBuilder) {
              for (const environment of Object.values(
                appBuilder.environments,
              ).toReversed()) {
                // oxlint-disable-next-line eslint/no-await-in-loop
                await appBuilder.build(environment);
              }
            },
          };
        }

        const builder = await createBuilder(config);

        await expect(builder.buildApp()).resolves.toBeUndefined();

        expect(releaseRequests).toHaveLength(1);
        expect(uploadRequests).toHaveLength(2);
        expect(finalizeRequests).toHaveLength(1);

        expect(
          JSON.parse(
            readFileSync(path.join(projectDir, ".cache/receipt.json"), "utf8"),
          ),
        ).toMatchObject({
          config: { status: "unavailable", digest: null },
          source: expectedSource,
        });

        const upload = (await finalizeRequests[0]?.json()) as {
          files: { chunkUrl: string }[];
        };

        expect(upload.files).toHaveLength(2);

        expect(
          upload.files.map((entry: { chunkUrl: string }) => entry.chunkUrl),
        ).toStrictEqual(
          expect.arrayContaining([
            expect.stringMatching(/^assets\/entry-[\w-]+\.js$/u),
            expect.stringMatching(/^server\//u),
          ]),
        );
      },
    );

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
      vi.stubEnv("VITE_INTERFERE_PUBLIC_KEY", undefined);

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
        "import.meta.env.VITE_INTERFERE_RELEASE_TOKEN":
          JSON.stringify("irt_signed"),
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
    test("warns without failing when a production build has no public key", async () => {
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
        expect.stringContaining(
          "no release token and no VITE_INTERFERE_PUBLIC_KEY",
        ),
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

  describe("iNTERFERE_API_KEY env-file fallback", () => {
    let envDir: string;

    beforeEach(() => {
      envDir = mkdtempSync(path.join(tmpdir(), "interfere-vite-envdir-"));
    });

    afterEach(() => {
      rmSync(envDir, { force: true, recursive: true });

      vi.unstubAllEnvs();
    });

    test("reads INTERFERE_API_KEY from .env.local when process.env is empty", async () => {
      writeFileSync(
        path.join(envDir, ".env.local"),
        "INTERFERE_API_KEY=interfere_ak_from_dotenv_local\n",
      );

      vi.stubEnv("INTERFERE_API_KEY", undefined);

      const capture = captureLogger();
      const plugin = interfere();

      await resolvePluginConfig(plugin, {
        command: "build",
        envDir,
        logger: capture.logger,
      });

      (plugin.buildStart as () => void)();

      expect(
        capture.warns.some((line) =>
          line.includes("INTERFERE_API_KEY is not set"),
        ),
      ).toBe(false);

      const closeBundle = plugin.closeBundle as () => Promise<void>;

      await closeBundle();

      expect(configRequests).toHaveLength(1);

      expect(configRequests[0]?.headers.get("authorization")).toBe(
        "Bearer interfere_ak_from_dotenv_local",
      );

      expect(releaseRequests).toHaveLength(1);

      expect(releaseRequests[0]?.headers.get("authorization")).toBe(
        "Bearer interfere_ak_from_dotenv_local",
      );

      expect(uploadRequests).toHaveLength(1);
      expect(finalizeRequests).toHaveLength(1);
      expect(existsSync(path.join(projectDir, "dist/app.js.map"))).toBe(false);
    });

    test("prefers process.env over .env.local (shell wins, standard precedence)", async () => {
      writeFileSync(
        path.join(envDir, ".env.local"),
        "INTERFERE_API_KEY=interfere_ak_from_dotenv\n",
      );

      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_from_shell");

      const plugin = interfere();

      await resolvePluginConfig(plugin, { command: "build", envDir });

      const closeBundle = plugin.closeBundle as () => Promise<void>;

      await closeBundle();

      expect(configRequests).toHaveLength(1);

      expect(configRequests[0]?.headers.get("authorization")).toBe(
        "Bearer interfere_ak_from_shell",
      );

      expect(releaseRequests).toHaveLength(1);

      expect(releaseRequests[0]?.headers.get("authorization")).toBe(
        "Bearer interfere_ak_from_shell",
      );

      expect(uploadRequests).toHaveLength(1);
      expect(finalizeRequests).toHaveLength(1);
      expect(existsSync(path.join(projectDir, "dist/app.js.map"))).toBe(false);
    });

    test("treats an explicitly-empty shell value as unset, not a resolved key", async () => {
      writeFileSync(
        path.join(envDir, ".env.local"),
        "INTERFERE_API_KEY=interfere_ak_from_dotenv\n",
      );

      vi.stubEnv("INTERFERE_API_KEY", "");

      await expect(
        resolvePluginConfig(interfere(), { command: "build", envDir }),
      ).resolves.toMatchObject({ build: { sourcemap: false } });
    });
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

  test.each(["sourceProvider", "destinationProvider"] as const)(
    "disables the compiled SDK before a build with no %s",
    async (provider) => {
      server.use(
        http.get(`${RELEASE_API}/v3/releases/config`, () =>
          HttpResponse.json({
            org: { id: "org_1", slug: "acme" },
            surface: {
              id: "surface_1",
              slug: "vite",
              sourceProvider: "github",
              destinationProvider: "cloudflare",
              environmentFiltering: false,
              sdkConfig: null,
              [provider]: null,
            },
          }),
        ),
      );

      const capture = captureLogger();
      const plugin = interfere();

      const config = await resolvePluginConfig(plugin, {
        logger: capture.logger,
      });

      expect(config.define?.["import.meta.env.VITE_INTERFERE_DISABLED"]).toBe(
        JSON.stringify("true"),
      );

      expect(config.define?.["import.meta.env.VITE_INTERFERE_PUBLIC_KEY"]).toBe(
        JSON.stringify(""),
      );

      expect(config.define?.[RELEASE_SLUG_KEY]).toBe("null");
      expect(JSON.stringify(config.define)).not.toContain("interfere_ak_test");

      (plugin.buildStart as () => void)();

      await (plugin.closeBundle as () => Promise<void>)();

      expect(releaseRequests).toHaveLength(0);

      expect(capture.warns).toStrictEqual([
        expect.stringContaining("https://interfere.com/~/acme/apps/vite"),
      ]);

      expect(capture.infos).toHaveLength(0);
    },
  );

  test("keeps browser instrumentation when binding lookup is unavailable", async () => {
    server.use(
      http.get(`${RELEASE_API}/v3/releases/config`, () => HttpResponse.error()),
    );

    const config = await resolvePluginConfig(interfere());

    expect(
      config.define?.["import.meta.env.VITE_INTERFERE_DISABLED"],
    ).toBeUndefined();

    expect(config.define?.[RELEASE_SLUG_KEY]).toBe(
      JSON.stringify(deriveReleaseSlug(gitSha)),
    );
  });
});
