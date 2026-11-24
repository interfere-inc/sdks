import { expect, test } from "vitest";
import { compressExportBody } from "./compress-export.js";

test("sends the first export raw when the page began unloading before anything was compressed", async () => {
  globalThis.dispatchEvent(new Event("beforeunload"));

  const body = JSON.stringify({ payload: "x".repeat(4096) });

  await expect(compressExportBody(body)).resolves.toMatchObject({
    encoding: null,
    body,
  });
});
