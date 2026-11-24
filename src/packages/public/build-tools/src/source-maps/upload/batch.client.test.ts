import { InterfereClient } from "@interfere/javascript";
import { sourceMapUploadBatch } from "@interfere/types/data/source-map-upload-batch";

import { Buffer } from "node:buffer";
import { HttpResponse, delay, http } from "msw";
import { setupServer } from "msw/node";

import { uploadSourceMapChunks } from "./chunks";
import { afterAll, afterEach, beforeAll, expect, test, vi } from "vitest";

const MILLIS_PER_SECOND = 1000;

const origin = "https://uploads.test";
const releaseSlug = "rel_aaaaaaaaaaaaaaaa";
const base = `${origin}/v3/releases/${releaseSlug}/source-maps`;
const server = setupServer();

const client = new InterfereClient({
  baseUrl: origin,
  apiKey: { token: "secret" },
});

const capability = { version: 1, maxBodyBytes: 524_288, maxFiles: 2 };

const file = {
  path: "a.map",
  content: "é",
  chunkUrl: "a.js",
  hash: "hash",
  debugId: "debug",
};

beforeAll(() => server.listen({ onUnhandledFrame: "error" }));

afterEach(() => server.resetHandlers());

afterAll(() => server.close());

test.each([
  { concurrency: undefined, expectedPeak: 20 },
  { concurrency: 2, expectedPeak: 2 },
  { concurrency: 7, expectedPeak: 7 },
])(
  "bounds uploads with configured concurrency $concurrency",
  async ({ concurrency, expectedPeak }) => {
    expect.hasAssertions();

    vi.stubEnv("INTERFERE_SOURCEMAP_UPLOAD_CONCURRENCY", "1");

    const files = [
      ...Array.from({ length: 60 }, (_, index) => ({
        ...file,
        path: `日本語/${index}.map`,
      })),
      { ...file, path: "large.map", content: "x".repeat(1_200_000) },
    ];

    const chunks = new Map<string, Uint8Array[]>();
    let active = 0;
    let peak = 0;
    let batches = 0;
    const offsets: number[] = [];

    server.use(
      http.put(`${base}/batch`, async ({ request }) => {
        active += 1;

        peak = Math.max(peak, active);

        const raw = await request.text();

        const body = sourceMapUploadBatch.parse(JSON.parse(raw));

        expect(Buffer.byteLength(raw)).toBeLessThanOrEqual(
          capability.maxBodyBytes,
        );

        expect(body.chunks.length).toBeLessThanOrEqual(capability.maxFiles);
        expect(request.headers.get("authorization")).toBe("Bearer secret");

        expect(new URL(request.url).searchParams.get("environment")).toBe(
          "preview",
        );

        for (const chunk of body.chunks) {
          expect(chunk.offset).toBe(0);

          chunks.set(chunk.path, [Buffer.from(chunk.bytes, "base64")]);
        }

        await delay(5);

        active -= 1;

        batches += 1;

        return HttpResponse.json({ ok: true, fileCount: body.chunks.length });
      }),
      http.put(base, async ({ request }) => {
        expect(request.headers.get("content-encoding")).toBeNull();

        active += 1;

        peak = Math.max(peak, active);

        offsets.push(Number(new URL(request.url).searchParams.get("offset")));

        const prior = chunks.get("large.map") ?? [];

        prior.push(new Uint8Array(await request.arrayBuffer()));

        chunks.set("large.map", prior);

        await delay(5);

        active -= 1;

        return HttpResponse.json({ objectKey: "map" });
      }),
    );

    const total = await uploadSourceMapChunks({
      client,
      releaseSlug,
      files,
      environment: "preview",
      concurrency,
      compression: "gzip",
      signed: {
        expiresAt: Math.floor(Date.now() / MILLIS_PER_SECOND) + 600,
        uploads: files,
        batchUpload: capability,
      },
    });

    expect(total).toBe(1_200_120);
    expect(batches).toBe(30);
    expect(peak).toBe(expectedPeak);
    expect(offsets).toStrictEqual([0, 524_288, 1_048_576]);

    for (const input of files) {
      expect(Buffer.concat(chunks.get(input.path) ?? []).toString()).toBe(
        input.content,
      );
    }
  },
);

test.each([0, -1, 1.5, Number.NaN, Infinity])(
  "rejects invalid concurrency %j before upload",
  async (value) => {
    vi.stubEnv("INTERFERE_SOURCEMAP_UPLOAD_CONCURRENCY", "1");

    await expect(
      uploadSourceMapChunks({
        client,
        releaseSlug,
        files: [file],
        concurrency: value,
        signed: {
          expiresAt: Math.floor(Date.now() / MILLIS_PER_SECOND) + 600,
          uploads: [file],
        },
      }),
    ).rejects.toThrow("positive integer");
  },
);
