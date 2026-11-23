import { InterfereClient } from "@interfere/javascript";
import { sourceMapBatchDigest } from "@interfere/types/data/source-map-batch";
import { releaseSlugSchema } from "@interfere/types/releases/slug";
import { mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { HttpResponse, http } from "msw";
import { setupServer } from "msw/node";
import { afterAll, afterEach, beforeAll, expect, test, vi } from "vitest";
import { runReleasePipeline } from "./pipeline";

const MILLIS_PER_SECOND = 1000;

const server = setupServer();
const origin = "https://receipt.test";
const slug = releaseSlugSchema.parse("rel_bbbbbbbbbbbbbbbb");
const sessionId = "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa";

beforeAll(() => server.listen({ onUnhandledFrame: "error" }));

afterEach(() => {
  server.resetHandlers();

  vi.unstubAllEnvs();
});

afterAll(() => server.close());

test.each([
  {
    state: "accepted",
    expectedSessionId: sessionId,
    expectedStatus: { state: "accepted", sessionId },
  },
  {
    state: "legacy",
    expectedSessionId: null,
    expectedStatus: { state: "accepted", sessionId: null },
  },
  { state: "unavailable", expectedSessionId: null, expectedStatus: null },
])(
  "captures $state publication without claiming readiness",
  async ({ state, expectedSessionId, expectedStatus }) => {
    vi.stubEnv("GITHUB_SHA", "a".repeat(40));

    vi.stubEnv("GITHUB_REF_NAME", "main");

    const directory = await mkdtemp(path.join(tmpdir(), "source-receipt-"));

    const absolute = path.join(directory, "app.js.map");

    const file = {
      absolute,
      chunkAbsolutes: [],
      chunkUrl: "app.js",
      content: "{}",
      debugId: "debug",
      hash: "hash",
      path: "app.js.map",
    };

    await writeFile(absolute, file.content);

    const batchDigest = await sourceMapBatchDigest({
      bundler: "rollup",
      sourceFileCount: 1,
      files: [file],
    });

    let statusReads = 0;
    let finalized = false;

    server.use(
      http.post(`${origin}/v3/releases`, () =>
        HttpResponse.json({
          build: { hash: "registered-build" },
          destination: { environment: "preview", id: "release", slug },
          org: { id: "org", slug: "acme" },
          source: { id: "source" },
          surface: { id: "surface", slug: "web" },
        }),
      ),
      http.post(`${origin}/v3/releases/${slug}/source-maps/sign`, () =>
        HttpResponse.json({
          expiresAt: Math.floor(Date.now() / MILLIS_PER_SECOND) + 600,
          uploads: [{ path: file.path }],
        }),
      ),
      http.put(
        `${origin}/v3/releases/${slug}/source-maps`,
        () => new HttpResponse(null),
      ),
      http.post(`${origin}/v3/releases/${slug}/source-maps/finalize`, () => {
        finalized = true;

        return HttpResponse.json({ ok: true, fileCount: 1 });
      }),
      http.get(`${origin}/v3/releases/${slug}/source-maps`, () => {
        statusReads += 1;

        if (state === "unavailable") {
          return new HttpResponse(null, { status: 503 });
        }

        return HttpResponse.json({
          releaseSlug: slug,
          environment: "preview",
          sessionId: state === "legacy" ? null : sessionId,
          state: finalized ? "accepted" : "uploading",
          batchDigest: finalized ? batchDigest : null,
          fileCount: 1,
          publishedFileCount: 0,
          manifestReady: false,
        });
      }),
    );

    try {
      const result = await runReleasePipeline({
        client: new InterfereClient({
          baseUrl: origin,
          apiKey: { token: "test" },
          maxRetries: 0,
        }),
        buildId: "requested-build",
        bundler: "rollup",
        discovery: { files: [file], sourceFileCount: 1 },
        environment: "preview",
        producerVersion: "test",
        releaseSlug: releaseSlugSchema.parse("rel_aaaaaaaaaaaaaaaa"),
        capturePublication: true,
        uploadSourceMaps: true,
        releasesConfig: {
          org: { id: "org", slug: "acme" },
          surface: {
            id: "surface",
            slug: "web",
            sourceProvider: "github",
            destinationProvider: "cli",
            environmentFiltering: true,
            sdkConfig: null,
          },
        },
      });

      expect(result.publication).toMatchObject({
        releaseSlug: slug,
        buildId: "registered-build",
        batchDigest,
        sessionId: expectedSessionId,
        status: expectedStatus,
      });

      expect(statusReads).toBe(2);
    } finally {
      await rm(directory, { recursive: true, force: true });
    }
  },
);
