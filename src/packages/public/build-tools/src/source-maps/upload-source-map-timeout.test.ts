import { InterfereClient } from "@interfere/javascript";

import { uploadSourceMapChunks } from "./upload-source-map-chunks";
import { HttpResponse, http } from "msw";
import { setupServer } from "msw/node";
import { afterAll, afterEach, beforeAll, expect, test } from "vitest";

const server = setupServer();

beforeAll(() => server.listen({ onUnhandledFrame: "error" }));

afterEach(() => server.resetHandlers());

afterAll(() => server.close());

const file = {
  path: "a.map",
  content: "é",
  chunkUrl: "a.js",
  hash: "hash",
  debugId: "debug",
};

const capability = { version: 1, maxBodyBytes: 524_288, maxFiles: 32 };

test.each([undefined, capability])(
  "retries one timed-out durable upload with identical bytes: %j",
  async (batchUpload) => {
    const bodies: string[] = [];
    const requests: Request[] = [];

    server.use(
      http.put("https://uploads.test/*", async ({ request }) => {
        requests.push(request);

        bodies.push(await request.text());

        if (bodies.length === 1) {
          await new Promise<void>((resolve) => {
            if (request.signal.aborted) {
              resolve();

              return;
            }

            request.signal.addEventListener(
              "abort",
              () => {
                resolve();
              },
              { once: true },
            );
          });
        }

        return HttpResponse.json(
          batchUpload ? { ok: true, fileCount: 1 } : { objectKey: "map" },
        );
      }),
    );

    const client = new InterfereClient({
      baseUrl: "https://uploads.test",
      apiKey: { token: "secret" },
      timeoutInSeconds: 1,
      maxRetries: 4,
    });

    await expect(
      uploadSourceMapChunks({
        client,
        releaseSlug: "rel_aaaaaaaaaaaaaaaa",
        files: [file],
        signed: {
          expiresAt: Math.floor(Date.now() / 1000) + 600,
          uploads: [file],
          batchUpload,
        },
      }),
    ).resolves.toBe(2);

    expect(requests).toHaveLength(2);
    expect(bodies).toHaveLength(2);
    expect(bodies[0]).toBe(bodies[1]);
  },
);

test("stops after the second timeout despite SDK retry settings", async () => {
  const requests: Request[] = [];

  server.use(
    http.put("https://uploads.test/*", async ({ request }) => {
      requests.push(request);

      await new Promise<void>((resolve) => {
        if (request.signal.aborted) {
          resolve();

          return;
        }

        request.signal.addEventListener("abort", () => resolve(), {
          once: true,
        });
      });

      return HttpResponse.json({ ok: true, fileCount: 1 });
    }),
  );

  const client = new InterfereClient({
    baseUrl: "https://uploads.test",
    apiKey: { token: "secret" },
    timeoutInSeconds: 1,
    maxRetries: 4,
  });

  await expect(
    uploadSourceMapChunks({
      client,
      releaseSlug: "rel_aaaaaaaaaaaaaaaa",
      files: [file],
      signed: {
        expiresAt: Math.floor(Date.now() / 1000) + 600,
        uploads: [file],
        batchUpload: capability,
      },
    }),
  ).rejects.toThrow(/timeout/iu);

  expect(requests).toHaveLength(2);
});
