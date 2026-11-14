import assert from "node:assert/strict";
import { Buffer } from "node:buffer";
import { createRequire } from "node:module";
import process from "node:process";
import { Readable } from "node:stream";

const [format, runtime, path] = process.argv.slice(2);

const requireSdk: (
  specifier: string,
) => typeof import("@interfere/javascript") = createRequire(import.meta.url);

const { InterfereClient } =
  format === "require"
    ? requireSdk("@interfere/javascript")
    : await import("@interfere/javascript");

assert.ok(path);

const uploads: { headers: Headers; text: string }[] = [];

const client = new InterfereClient({
  baseUrl: "https://api.interfere.test",
  apiKeyHeader: { apiKey: "test-key" },
  maxRetries: 0,
  fetch: Object.assign(
    async (
      _url: Parameters<typeof fetch>[0],
      init?: Parameters<typeof fetch>[1],
    ) => {
      assert.ok(init !== undefined);

      const { body } = init;
      let text: string;

      if (body instanceof Readable) {
        const chunks: Uint8Array[] = [];

        for await (const chunk of body) {
          assert.ok(chunk instanceof Uint8Array);

          chunks.push(chunk);
        }

        text = Buffer.concat(chunks).toString();
      } else {
        text = await new globalThis.Response(body).text();
      }

      uploads.push({ headers: new globalThis.Headers(init.headers), text });

      return globalThis.Response.json({ objectKey: "source-map" });
    },
    globalThis.fetch,
  ),
});

const request = { environment: "production", path: "bundle.map", offset: 0 };
const content = "source-map-content";

const result = await client.releases.uploadSourceMap(
  new globalThis.Blob([content]),
  "release",
  request,
);

assert.equal(result.objectKey, "source-map");

assert.equal(uploads.length, 1);

assert.equal(uploads[0].text, content);

assert.equal(uploads[0].headers.get("x-api-key"), "test-key");

assert.equal(uploads[0].headers.get("content-length"), String(content.length));

const bytes = new TextEncoder().encode(content);

const inputs = [
  new globalThis.File([content], "bundle.map"),
  bytes,
  new ReadableStream<Uint8Array>({
    start(controller) {
      controller.enqueue(bytes);

      controller.close();
    },
  }),
];

await Promise.all(
  inputs.map(
    async (input) =>
      await client.releases.uploadSourceMap(input, "release", request),
  ),
);

assert.equal(uploads.length, 4);

for (const upload of uploads) {
  assert.equal(upload.text, content);
}

if (runtime === "portable") {
  await assert.rejects(
    client.releases.uploadSourceMap({ path }, "release", request),
    /File path uploads are not supported in this environment\./u,
  );

  assert.equal(uploads.length, 4);
} else {
  await client.releases.uploadSourceMap({ path }, "release", request);

  assert.equal(uploads.length, 5);

  assert.equal(uploads[4].text, content);

  assert.equal(
    uploads[4].headers.get("content-length"),
    String(content.length),
  );

  assert.match(
    uploads[4].headers.get("content-disposition") ?? "",
    /upload\.map/u,
  );

  await client.releases.uploadSourceMap(
    Readable.from([bytes]),
    "release",
    request,
  );

  assert.equal(uploads.length, 6);

  assert.equal(uploads[5].text, content);
}

process.stdout.write("uploads verified");
