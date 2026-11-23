import { InterfereClient } from "@interfere/javascript";

import { HttpResponse, http } from "msw";
import { setupServer } from "msw/node";

import { uploadSourceMapChunks } from "./upload-source-map-chunks";
import { afterAll, afterEach, beforeAll, expect, test } from "vitest";

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

test("retries the same typed JSON batch and counts decoded bytes once", async () => {
  const bodies: string[] = [];

  server.use(
    http.put(`${base}/batch`, async ({ request }) => {
      bodies.push(await request.text());

      if (bodies.length === 1) {
        return HttpResponse.json(
          {},
          { status: 429, headers: { "retry-after": "0" } },
        );
      }

      return HttpResponse.json({ ok: true, fileCount: 1 });
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
        batchUpload: capability,
      },
    }),
  ).resolves.toBe(2);

  expect(bodies).toHaveLength(2);
  expect(bodies[0]).toBe(bodies[1]);

  expect(JSON.parse(bodies[0] ?? "")).toStrictEqual({
    chunks: [{ path: "a.map", offset: 0, bytes: "w6k=" }],
  });
});
