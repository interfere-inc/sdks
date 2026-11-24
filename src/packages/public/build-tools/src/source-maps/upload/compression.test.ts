import { sourceMapUploadBatch } from "@interfere/types/data/source-map-upload-batch";
import { InterfereClient } from "@interfere/javascript";
import { randomBytes } from "node:crypto";
import { gunzipSync } from "node:zlib";
import { HttpResponse, http } from "msw";
import { setupServer } from "msw/node";
import { afterAll, afterEach, beforeAll, expect, test } from "vitest";
import { uploadSourceMapChunks } from "./chunks";
import { sourceMapUploadFetch } from "../source-map-upload-fetch";

const MILLIS_PER_SECOND = 1000;

const origin = "https://uploads.test";
const releaseSlug = "rel_aaaaaaaaaaaaaaaa";
const base = `${origin}/v3/releases/${releaseSlug}/source-maps`;
const server = setupServer();

beforeAll(() => server.listen({ onUnhandledFrame: "error" }));

afterEach(() => server.resetHandlers());

afterAll(() => server.close());

test("uploads exact gzip bytes and retries", async () => {
  expect.hasAssertions();

  const files = [
    {
      path: "日本語.map",
      content: "é".repeat(10_000),
      chunkUrl: "a.js",
      hash: "hash",
      debugId: "debug",
    },
    {
      path: "large.map",
      content: "x".repeat(1_200_000),
      chunkUrl: "b.js",
      hash: "hash",
      debugId: "debug",
    },
  ];

  const client = new InterfereClient({
    baseUrl: origin,
    apiKey: { token: "secret" },
    fetch: sourceMapUploadFetch,
  });

  const chunks = new Map<string, Buffer[]>();
  const offsets: number[] = [];
  let wireBytes = 0;
  let attempts = 0;

  async function decode(request: Request) {
    expect(request.headers.get("authorization")).toBe("Bearer secret");

    expect(request.headers.get("content-encoding")).toBe("gzip");

    const bytes = Buffer.from(await request.arrayBuffer());

    wireBytes += bytes.length;

    return gunzipSync(bytes);
  }

  server.use(
    http.put(`${base}/batch`, async ({ request }) => {
      const bytes = await decode(request);

      attempts += 1;

      if (attempts === 1) {
        return HttpResponse.json(
          {},
          { status: 503, headers: { "retry-after": "0" } },
        );
      }

      const body = sourceMapUploadBatch.parse(JSON.parse(bytes.toString()));

      for (const chunk of body.chunks) {
        chunks.set(chunk.path, [Buffer.from(chunk.bytes, "base64")]);
      }

      return HttpResponse.json({ ok: true, fileCount: body.chunks.length });
    }),
    http.put(base, async ({ request }) => {
      const bytes = await decode(request);

      expect(Number(request.headers.get("x-interfere-source-map-size"))).toBe(
        bytes.byteLength,
      );

      offsets.push(Number(new URL(request.url).searchParams.get("offset")));

      const prior = chunks.get("large.map") ?? [];

      chunks.set("large.map", [...prior, bytes]);

      return HttpResponse.json({ objectKey: "map" });
    }),
  );

  const signed: Parameters<typeof uploadSourceMapChunks>[0]["signed"] = {
    expiresAt: Math.floor(Date.now() / MILLIS_PER_SECOND) + 600,
    uploads: files,
    batchUpload: {
      version: 1,
      maxBodyBytes: 524_288,
      maxFiles: 32,
      compression: "gzip",
    },
  };

  const total = await uploadSourceMapChunks({
    compression: "gzip",
    client,
    releaseSlug,
    files,
    signed,
  });

  expect(total).toBe(1_220_000);
  expect(attempts).toBe(2);
  expect(offsets).toStrictEqual([0]);

  for (const file of files) {
    expect(Buffer.concat(chunks.get(file.path) ?? []).toString()).toBe(
      file.content,
    );
  }

  expect(wireBytes).toBeLessThan(total / 10);
});

test("sends identity bytes when gzip would increase the payload", async () => {
  expect.hasAssertions();

  const bytes = randomBytes(524_288);

  server.use(
    http.put(base, async ({ request }) => {
      expect(request.headers.get("content-encoding")).toBeNull();

      expect(Buffer.from(await request.arrayBuffer())).toStrictEqual(bytes);

      return HttpResponse.json({ objectKey: "map" });
    }),
  );

  const response = await sourceMapUploadFetch(base, {
    method: "PUT",
    headers: { "content-encoding": "gzip" },
    body: bytes,
  });

  expect(response.status).toBe(200);
});

test("keeps plain API clients uncompressed without the gzip option", async () => {
  expect.hasAssertions();

  const client = new InterfereClient({
    baseUrl: origin,
    apiKey: { token: "secret" },
  });

  const file = {
    path: "a.map",
    content: "hello",
    chunkUrl: "a.js",
    hash: "hash",
    debugId: "debug",
  };

  server.use(
    http.put(base, async ({ request }) => {
      expect(request.headers.get("content-encoding")).toBeNull();

      await expect(request.text()).resolves.toBe(file.content);

      return HttpResponse.json({ objectKey: "map" });
    }),
  );

  await expect(
    uploadSourceMapChunks({
      client,
      releaseSlug,
      files: [file],
      signed: {
        expiresAt: Math.floor(Date.now() / MILLIS_PER_SECOND) + 600,
        uploads: [file],
      },
    }),
  ).resolves.toBe(5);
});

test("negotiates compression when signing through the gzip-capable transport", async () => {
  expect.hasAssertions();

  server.use(
    http.post(`${base}/sign`, async ({ request }) => {
      expect(request.headers.get("x-interfere-source-map-compression")).toBe(
        "gzip",
      );

      expect(request.headers.get("content-encoding")).toBeNull();
      expect(request.headers.get("authorization")).toBe("Bearer secret");

      await expect(request.json()).resolves.toMatchObject({
        files: [{ path: "a.map" }],
      });

      return HttpResponse.json({
        expiresAt: Math.floor(Date.now() / MILLIS_PER_SECOND) + 600,
        uploads: [{ path: "a.map" }],
        batchUpload: {
          version: 1,
          maxBodyBytes: 524_288,
          maxFiles: 32,
          compression: "gzip",
        },
      });
    }),
  );

  const client = new InterfereClient({
    baseUrl: origin,
    apiKey: { token: "secret" },
    fetch: sourceMapUploadFetch,
  });

  await expect(
    client.releases.signSourceMapUploads({
      releaseSlug,
      files: [
        {
          path: "a.map",
          sizeBytes: 1,
          hasFile: true,
          hasNames: true,
          hasSourcesContent: true,
          mappingsPresent: true,
        },
      ],
    }),
  ).resolves.toMatchObject({ batchUpload: { compression: "gzip" } });
});
