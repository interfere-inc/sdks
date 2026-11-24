import { HttpResponse, http } from "msw";
import { createPluginContext } from "../../../../test/fixtures/plugin/context.js";
import { worker } from "../../../../test/msw.js";
import { createCompressor } from "../compression/compressor-host.js";
import { deliverChunk } from "./delivery.js";
import { expect, test } from "vitest";

const UPLOAD_URL = "https://collector.test/v3/replay/upload/:id";

async function deliver(signal: AbortSignal) {
  return deliverChunk({
    target: {
      baseUrl: "https://collector.test",
      publicKeyHeader: { apiKey: "test-key" },
      maxRetries: 0,
      timeoutInSeconds: 10,
      headers: {},
    },
    chunk: { segmentId: 7, windowId: "win_x" },
    compressor: createCompressor(),
    ctx: createPluginContext(),
    payload: {
      activityCount: 0,
      bodyJson: "x".repeat(64 * 1024),
      eventCount: 1,
      firstTs: 1000,
      hasFullSnapshot: true,
      lastTs: 2000,
    },
    signal,
    unload: false,
  });
}

test("an abort while the chunk is compressing sends nothing", async () => {
  expect.hasAssertions();

  const requests: Request[] = [];

  worker.use(
    http.post(UPLOAD_URL, ({ request }) => {
      requests.push(request);

      return HttpResponse.json({});
    }),
  );

  const controller = new AbortController();

  const delivery = deliver(controller.signal);

  controller.abort();

  await expect(delivery).resolves.toBe("aborted");

  expect(requests).toStrictEqual([]);
});
