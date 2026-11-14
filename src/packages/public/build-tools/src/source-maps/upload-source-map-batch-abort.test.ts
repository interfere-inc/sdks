import { InterfereClient } from "@interfere/javascript";

import { uploadSourceMapChunks } from "./upload-source-map-chunks";
import { HttpResponse, http } from "msw";
import { setupServer } from "msw/node";
import { afterAll, afterEach, beforeAll, expect, test } from "vitest";

const server = setupServer();

beforeAll(() => server.listen({ onUnhandledRequest: "error" }));

afterEach(() => server.resetHandlers());

afterAll(() => server.close());

const file = {
  path: "a.map",
  content: "é",
  chunkUrl: "a.js",
  hash: "hash",
  debugId: "debug",
};

const batchUpload = { version: 1, maxBodyBytes: 524_288, maxFiles: 1 };
const releaseSlug = "rel_aaaaaaaaaaaaaaaa";

test("cancels the other active batches and never starts queued requests after rejection", async () => {
  let started = 0;
  let cancelled = 0;
  const barrier = Promise.withResolvers<undefined>();

  server.use(
    http.put("https://uploads.test/*", async ({ request }) => {
      const first = started === 0;

      started += 1;

      if (started === 20) {
        barrier.resolve(undefined);
      }

      await barrier.promise;

      if (first) {
        return HttpResponse.json({}, { status: 401 });
      }

      await new Promise<void>((resolve) => {
        request.signal.addEventListener(
          "abort",
          () => {
            cancelled += 1;

            resolve();
          },
          { once: true },
        );
      });

      return HttpResponse.json({ ok: true, fileCount: 1 });
    }),
  );

  const client = new InterfereClient({
    baseUrl: "https://uploads.test",
    apiKey: { token: "secret" },
  });

  const files = Array.from({ length: 30 }, (_, index) => ({
    ...file,
    path: `${index}.map`,
  }));

  await expect(
    uploadSourceMapChunks({
      client,
      releaseSlug,
      files,
      signed: {
        expiresAt: Math.floor(Date.now() / 1000) + 600,
        uploads: files,
        batchUpload,
      },
    }),
  ).rejects.toThrow("Status code: 401");

  expect(started).toBe(20);
  expect(cancelled).toBe(19);
});

test("rejects an expired capable session before any request", async () => {
  const requests: Request[] = [];

  server.use(
    http.put("https://uploads.test/*", ({ request }) => {
      requests.push(request);

      return HttpResponse.json({ ok: true, fileCount: 1 });
    }),
  );

  const client = new InterfereClient({
    baseUrl: "https://uploads.test",
    apiKey: { token: "secret" },
  });

  await expect(
    uploadSourceMapChunks({
      client,
      releaseSlug,
      files: [file],
      signed: {
        expiresAt: Math.floor(Date.now() / 1000) - 1,
        uploads: [file],
        batchUpload,
      },
    }),
  ).rejects.toThrow("Source-map upload session expired");

  expect(requests).toHaveLength(0);
});
