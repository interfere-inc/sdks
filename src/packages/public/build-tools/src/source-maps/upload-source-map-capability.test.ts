import { InterfereClient } from "@interfere/javascript";

import { uploadSourceMapChunks } from "./upload-source-map-chunks";
import { HttpResponse, http } from "msw";
import { setupServer } from "msw/node";
import { afterAll, afterEach, beforeAll, expect, test } from "vitest";

const MILLIS_PER_SECOND = 1000;

const server = setupServer();
const requests: Request[] = [];

beforeAll(() => {
  server.listen({ onUnhandledFrame: "error" });

  server.events.on("request:start", ({ request }) => requests.push(request));
});

afterEach(() => {
  server.resetHandlers();

  requests.length = 0;
});

afterAll(() => server.close());

const releaseSlug = "rel_aaaaaaaaaaaaaaaa";

const file = {
  path: "a.map",
  content: "é",
  chunkUrl: "a.js",
  hash: "hash",
  debugId: "debug",
};

const inventory = [
  {
    path: "a.map",
    sizeBytes: 2,
    hasFile: true,
    hasNames: true,
    hasSourcesContent: true,
    mappingsPresent: true,
  },
];

test.each([
  null,
  { version: 2, maxBodyBytes: 524_288, maxFiles: 32 },
  { version: 1, maxBodyBytes: 524_289, maxFiles: 32 },
])(
  "native SDK preserves malformed present capability for rejection: %j",
  async (batchUpload) => {
    server.use(
      http.post(
        `https://uploads.test/v3/releases/${releaseSlug}/source-maps/sign`,
        () =>
          HttpResponse.json({
            expiresAt: Math.floor(Date.now() / MILLIS_PER_SECOND) + 600,
            uploads: [{ path: "a.map" }],
            batchUpload,
          }),
      ),
    );

    const client = new InterfereClient({
      baseUrl: "https://uploads.test",
      apiKey: { token: "secret" },
    });

    const signed = await client.releases.signSourceMapUploads({
      releaseSlug,
      files: inventory,
    });

    expect(signed.batchUpload).toStrictEqual(batchUpload);

    await expect(
      uploadSourceMapChunks({ client, releaseSlug, files: [file], signed }),
    ).rejects.toThrow(/Invalid|Too big/u);

    expect(requests).toHaveLength(1);

    expect(requests[0]?.url).toBe(
      `https://uploads.test/v3/releases/${releaseSlug}/source-maps/sign`,
    );

    expect(requests[0]?.method).toBe("POST");
  },
);

test("native SDK preserves absent capability and sends the legacy binary request", async () => {
  server.use(
    http.post(
      `https://uploads.test/v3/releases/${releaseSlug}/source-maps/sign`,
      () =>
        HttpResponse.json({
          expiresAt: Math.floor(Date.now() / MILLIS_PER_SECOND) + 600,
          uploads: [{ path: "a.map" }],
        }),
    ),
    http.put(
      `https://uploads.test/v3/releases/${releaseSlug}/source-maps`,
      () => HttpResponse.json({ objectKey: "map" }),
    ),
  );

  const client = new InterfereClient({
    baseUrl: "https://uploads.test",
    apiKey: { token: "secret" },
  });

  const signed = await client.releases.signSourceMapUploads({
    releaseSlug,
    files: inventory,
  });

  expect(signed).not.toHaveProperty("batchUpload");

  await expect(
    uploadSourceMapChunks({ client, releaseSlug, files: [file], signed }),
  ).resolves.toBe(2);

  expect(requests).toHaveLength(2);

  expect(requests[1]?.url).toBe(
    `https://uploads.test/v3/releases/${releaseSlug}/source-maps?offset=0&path=a.map`,
  );

  expect(requests[1]?.method).toBe("PUT");
});
