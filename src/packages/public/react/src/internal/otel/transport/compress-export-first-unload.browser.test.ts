import { expect, test } from "vitest";

test("sends the first export raw when the page began unloading before anything was compressed", async () => {
  const { compressExportBody } = await import("./compress-export.js");

  globalThis.dispatchEvent(new Event("beforeunload"));

  const body = JSON.stringify({ payload: "x".repeat(4096) });

  await expect(compressExportBody(body)).resolves.toMatchObject({
    encoding: null,
    body,
  });
});
