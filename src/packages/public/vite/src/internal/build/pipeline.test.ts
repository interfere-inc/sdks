import { InterfereClient } from "@interfere/javascript";

import { setupServer } from "msw/node";

import { releaseSlugSchema } from "@interfere/types/releases/slug";

import { access, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { HttpResponse, http } from "msw";

import { runReleasePipeline } from "./pipeline";
import { afterAll, afterEach, beforeAll, expect, test, vi } from "vitest";

const MILLIS_PER_SECOND = 1000;

const server = setupServer();

beforeAll(() => {
  server.listen({ onUnhandledFrame: "error" });
});

afterEach(() => vi.unstubAllEnvs());

// oxlint-disable-next-line vitest/no-duplicate-hooks -- Independent teardown must still run after another cleanup fails, in native reverse registration order.
afterEach(() => {
  server.resetHandlers();
});

afterAll(() => {
  server.close();
});

test("uploads the canonical release and cleans its artifacts", async () => {
  const hash = "registered-build";

  const directory = await mkdtemp(path.join(tmpdir(), "vite-release-"));

  try {
    vi.stubEnv("GITHUB_SHA", "a".repeat(40));

    vi.stubEnv("GITHUB_REF_NAME", "main");

    const absolute = path.join(directory, "app.js.map");
    const chunkAbsolute = path.join(directory, "app.js");

    await writeFile(absolute, "{}");

    await writeFile(chunkAbsolute, "a();\n//# sourceMappingURL=app.js.map\n");

    const slug = releaseSlugSchema.parse("rel_aaaaaaaaaaaaaaaa");
    const canonical = releaseSlugSchema.parse("rel_bbbbbbbbbbbbbbbb");
    const origin = "https://build.test";

    const registrations: Request[] = [];

    server.use(
      http.post(`${origin}/v3/releases`, ({ request }) => {
        registrations.push(request);

        return HttpResponse.json(
          {
            build: { hash },
            destination: {
              environment: "production",
              id: "release",
              slug: canonical,
            },
            org: { id: "org", slug: "acme" },
            source: { id: "source" },
            surface: { id: "surface", slug: "web" },
          },
          { status: 200 },
        );
      }),
    );

    const signed: Request[] = [];

    server.use(
      http.post(
        `${origin}/v3/releases/${canonical}/source-maps/sign`,
        ({ request }) => {
          signed.push(request);

          return HttpResponse.json(
            {
              expiresAt: Math.floor(Date.now() / MILLIS_PER_SECOND) + 600,
              uploads: [
                {
                  path: "app.js.map",
                },
              ],
            },
            { status: 200 },
          );
        },
      ),
    );

    server.use(
      http.put(
        `${origin}/v3/releases/${canonical}/source-maps`,
        () => new HttpResponse(null),
      ),
    );

    server.use(
      http.post(
        `${origin}/v3/releases/${canonical}/source-maps/finalize`,
        async () => {
          await access(absolute);

          return HttpResponse.json({ fileCount: 1, ok: true });
        },
      ),
    );

    const result = await runReleasePipeline({
      client: new InterfereClient({
        baseUrl: origin,
        apiKey: { token: "interfere_ak_test" },
      }),
      buildId: "requested-build",
      bundler: "rollup",
      discovery: {
        files: [
          {
            absolute,
            chunkAbsolutes: [chunkAbsolute],
            chunkUrl: "app.js",
            content: "{}",
            debugId: "debug",
            hash: "hash",
            path: "app.js.map",
          },
        ],
        sourceFileCount: 1,
      },
      environment: "production",
      producerVersion: "@interfere/vite@11.2.1",
      uploadSourceMaps: true,
      releaseSlug: slug,
      service: { name: "web", namespace: "shop" },
      releasesConfig: {
        org: { id: "org", slug: "acme" },
        surface: {
          destinationProvider: "cli",
          environmentFiltering: true,
          id: "surface",
          sdkConfig: null,
          slug: "web",
          sourceProvider: "github",
        },
      },
    });

    expect(result).toMatchObject({
      cleanupFailures: [],
      fileCount: 1,
      releaseSlug: canonical,
    });

    expect(signed).toHaveLength(1);

    expect(
      registrations.map((call) => new URL(call.url).pathname),
    ).toStrictEqual(["/v3/releases"]);

    await expect(registrations[0]?.json()).resolves.toMatchObject({
      service: { name: "web", namespace: "shop" },
    });

    await expect(access(absolute)).rejects.toMatchObject({ code: "ENOENT" });

    await expect(readFile(chunkAbsolute, "utf8")).resolves.toBe("a();");
  } finally {
    await rm(directory, { force: true, recursive: true });
  }
});
