import { InterfereClient } from "@interfere/javascript";

import { HttpResponse, http } from "msw";
import { setupServer } from "msw/node";

import { uploadSourceMapChunks } from "./chunks";
import { afterAll, afterEach, beforeAll, expect, test, vi } from "vitest";

const MILLIS_PER_SECOND = 1000;

const origin = "https://uploads.test";

const slug = "rel_aaaaaaaaaaaaaaaa";

const base = `${origin}/v3/releases/${slug}/source-maps`;

const grantExpiresAt = () => Math.floor(Date.now() / MILLIS_PER_SECOND) + 600;

const server = setupServer();

beforeAll(() => server.listen({ onUnhandledFrame: "error" }));

afterEach(() => {
  server.resetHandlers();

  vi.useRealTimers();
});

afterAll(() => server.close());

test.each([
  { auth: "bearer", expectedCredential: "Bearer secret" },
  { auth: "header", expectedCredential: "secret" },
])(
  "uploads ordered binary chunks with $auth auth and retries a throttled chunk",
  async ({ auth, expectedCredential }) => {
    expect.hasAssertions();

    const client = new InterfereClient({
      baseUrl: origin,
      ...(auth === "bearer"
        ? { apiKey: { token: "secret" } }
        : { apiKeyHeader: { apiKey: "secret" } }),
    });

    const file = {
      path: "assets/a + b/é?.js.map",
      chunkUrl: "assets/app.js",
      hash: "hash",
      debugId: "debug",
      content: JSON.stringify({
        version: 3,
        sources: ["app.ts"],
        sourcesContent: ["é".repeat(4_200_000)],
        mappings: "AAAA",
      }),
    };

    const bytes = new TextEncoder().encode(file.content);
    const offsets: number[] = [];
    const uploaded: Uint8Array[] = [];
    const credentials: (string | null)[] = [];
    const paths: (string | null)[] = [];
    const environments: (string | null)[] = [];
    const contentTypes: (string | null)[] = [];

    server.use(
      http.put(base, async ({ request }) => {
        const url = new URL(request.url);

        offsets.push(Number(url.searchParams.get("offset")));

        paths.push(url.searchParams.get("path"));

        environments.push(url.searchParams.get("environment"));

        contentTypes.push(request.headers.get("content-type"));

        credentials.push(
          request.headers.get(
            auth === "bearer" ? "authorization" : "x-api-key",
          ),
        );

        if (offsets.length === 1) {
          return HttpResponse.json(
            { code: "INGEST_BACKPRESSURE" },
            { status: 429, headers: { "retry-after": "1" } },
          );
        }

        const received = new Uint8Array(await request.arrayBuffer());

        expect(Number(request.headers.get("x-interfere-source-map-size"))).toBe(
          received.byteLength,
        );

        uploaded.push(received);

        return HttpResponse.json({ objectKey: "map" });
      }),
    );

    vi.useFakeTimers({ toFake: ["setTimeout"] });

    const upload = uploadSourceMapChunks({
      environment: "preview",
      client,
      releaseSlug: slug,
      files: [file],
      signed: {
        expiresAt: grantExpiresAt(),
        uploads: [{ path: file.path }],
        batchUpload: {
          version: 1,
          maxBodyBytes: 524_288,
          maxFiles: 32,
          compression: "gzip",
        },
      },
    });

    await vi.waitFor(async () => {
      await vi.advanceTimersByTimeAsync(1200);

      expect(offsets.length).toBeGreaterThan(1);
    });

    await expect(upload).resolves.toBe(bytes.byteLength);

    expect(offsets).toStrictEqual([0, 0, 8 * 1024 * 1024]);
    expect(paths).toStrictEqual(Array.from({ length: 3 }, () => file.path));

    expect(environments).toStrictEqual(
      Array.from({ length: 3 }, () => "preview"),
    );

    expect(credentials).toStrictEqual(
      Array.from({ length: 3 }, () => expectedCredential),
    );

    expect(contentTypes).toStrictEqual(
      Array.from({ length: 3 }, () => "application/octet-stream"),
    );

    expect(Buffer.concat(uploaded).equals(Buffer.from(bytes))).toBe(true);
  },
);

test("sends the session upload grant on every chunk and batch request", async () => {
  expect.hasAssertions();

  const client = new InterfereClient({
    baseUrl: origin,
    apiKey: { token: "secret" },
  });

  const grants: (string | null)[] = [];

  server.use(
    http.put(base, ({ request }) => {
      grants.push(request.headers.get("x-interfere-upload-grant"));

      return HttpResponse.json({ objectKey: "key" });
    }),
    http.put(`${base}/batch`, ({ request }) => {
      grants.push(request.headers.get("x-interfere-upload-grant"));

      return HttpResponse.json({ ok: true, fileCount: 1 });
    }),
  );

  await uploadSourceMapChunks({
    client,
    releaseSlug: slug,
    signed: {
      batchUpload: { version: 1, maxBodyBytes: 524_288, maxFiles: 32 },
      expiresAt: grantExpiresAt(),
      uploadGrant: "grant",
      uploads: [{ path: "small.js.map" }, { path: "large.js.map" }],
    },
    files: [
      {
        path: "small.js.map",
        chunkUrl: "small.js",
        hash: "hash",
        debugId: "debug-small",
        content: JSON.stringify({ version: 3, mappings: "AAAA" }),
      },
      {
        path: "large.js.map",
        chunkUrl: "large.js",
        hash: "hash",
        debugId: "debug-large",
        content: JSON.stringify({
          version: 3,
          mappings: "AAAA",
          sourcesContent: ["x".repeat(1_200_000)],
        }),
      },
    ],
  });

  expect(grants.length).toBeGreaterThan(1);
  expect(new Set(grants)).toStrictEqual(new Set(["grant"]));
});

test("sends no bytes for files the server marks present", async () => {
  const client = new InterfereClient({
    baseUrl: origin,
    apiKey: { token: "secret" },
  });

  const [stored, fresh] = ["stored.js.map", "fresh.js.map"].map((path) => ({
    path,
    chunkUrl: path.replace(/\.map$/u, ""),
    hash: "hash",
    debugId: "debug",
    content: JSON.stringify({ version: 3, file: path, mappings: "AAAA" }),
  }));

  if (!stored || !fresh) {
    throw new Error("Fixture files are missing");
  }

  const paths: (string | null)[] = [];

  server.use(
    http.put(base, ({ request }) => {
      paths.push(new URL(request.url).searchParams.get("path"));

      return HttpResponse.json({ objectKey: "map" });
    }),
  );

  await expect(
    uploadSourceMapChunks({
      client,
      releaseSlug: slug,
      files: [stored, fresh],
      signed: {
        expiresAt: grantExpiresAt(),
        uploads: [{ path: stored.path, present: true }, { path: fresh.path }],
      },
    }),
  ).resolves.toBe(new TextEncoder().encode(fresh.content).byteLength);

  expect(paths).toStrictEqual([fresh.path]);
});
