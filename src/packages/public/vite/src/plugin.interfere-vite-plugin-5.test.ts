import { setupServer } from "msw/node";

import { execFileSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { HttpResponse, http } from "msw";
import { tmpdir } from "node:os";
import path from "node:path";
import { createBuilder } from "vite";
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
  finalizeRequests,
  installVitePluginCase,
  projectDir,
  releaseRequests,
  uploadRequests,
} from "./plugin.test-utils";

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
                expiresAt: Math.floor(Date.now() / MILLIS_PER_SECOND) + 600,
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
  });
});
