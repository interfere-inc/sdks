import { InterfereClient } from "@interfere/javascript";

import { HttpResponse, http } from "msw";
import { setupServer } from "msw/node";

import { uploadSourceMapRequest } from "./request";
import { afterAll, afterEach, beforeAll, expect, test } from "vitest";

const server = setupServer();
const url = "https://uploads.test/v3/releases/rel_aaaaaaaaaaaaaaaa/source-maps";

const client = new InterfereClient({
  baseUrl: "https://uploads.test",
  apiKey: { token: "secret" },
  timeoutInSeconds: 1,
  maxRetries: 0,
});

beforeAll(() => server.listen({ onUnhandledFrame: "error" }));

afterEach(() => server.resetHandlers());

afterAll(() => server.close());

async function upload(signal: AbortSignal) {
  return client.releases.uploadSourceMap(
    new Blob(["source map"]),
    "rel_aaaaaaaaaaaaaaaa",
    { path: "a.map", offset: 0 },
    { abortSignal: signal },
  );
}

test.each([401, 403, 409, 422, 503])(
  "does not add another retry for HTTP %i",
  async (statusCode) => {
    let requests = 0;

    server.use(
      http.put(url, () => {
        requests += 1;

        return HttpResponse.json({}, { status: statusCode });
      }),
    );

    const controller = new AbortController();

    await expect(
      uploadSourceMapRequest(
        async () => upload(controller.signal),
        controller.signal,
      ),
    ).rejects.toMatchObject({ statusCode });

    expect(requests).toBe(1);
  },
);

test("retries an SDK timeout once", async () => {
  let requests = 0;

  server.use(
    http.put(url, async ({ request }) => {
      requests += 1;

      if (requests === 1) {
        await new Promise<void>((resolve) => {
          if (request.signal.aborted) {
            resolve();

            return;
          }

          request.signal.addEventListener("abort", () => resolve(), {
            once: true,
          });
        });
      }

      return HttpResponse.json({ objectKey: "map" });
    }),
  );

  const controller = new AbortController();

  await uploadSourceMapRequest(
    async () => upload(controller.signal),
    controller.signal,
  );

  expect(requests).toBe(2);
});

test("never starts a request after cancellation", async () => {
  let requests = 0;

  server.use(
    http.put(url, () => {
      requests += 1;

      return HttpResponse.json({ objectKey: "map" });
    }),
  );

  const controller = new AbortController();

  controller.abort();

  await expect(
    uploadSourceMapRequest(
      async () => upload(controller.signal),
      controller.signal,
    ),
  ).rejects.toBe(controller.signal.reason);

  expect(requests).toBe(0);
});

test("never retries a timeout after its upload session is cancelled", async () => {
  const controller = new AbortController();
  let requests = 0;

  server.use(
    http.put(url, async ({ request }) => {
      requests += 1;

      await new Promise<void>((resolve) => {
        if (request.signal.aborted) {
          controller.abort();

          resolve();

          return;
        }

        request.signal.addEventListener(
          "abort",
          () => {
            controller.abort();

            resolve();
          },
          { once: true },
        );
      });

      return HttpResponse.json({ objectKey: "map" });
    }),
  );

  await expect(
    uploadSourceMapRequest(
      async () => upload(controller.signal),
      controller.signal,
    ),
  ).rejects.toThrow(/timeout|aborted/iu);

  expect(controller.signal.aborted).toBe(true);
  expect(requests).toBe(1);
});
