import { setLogLevel } from "../../../util/log.js";
import { createCompressor } from "../compression/compressor-host.js";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace";
import { gunzipSync } from "fflate";
import { HttpResponse, http } from "msw";
import { createPluginContext } from "../../../../test/fixtures/plugin/context.js";
import type { ReplayChunkPayload } from "../chunking/chunking.js";
import { deliverChunk } from "./delivery.js";
import type { DeliverInput } from "./delivery.js";
import { worker } from "../../../../test/msw.js";
import { describe, expect, test } from "vitest";

function payload() {
  return {
    activityCount: 3,
    bodyJson: "[]",
    eventCount: 3,
    firstTs: 1000,
    hasFullSnapshot: true,
    lastTs: 2000,
  } satisfies ReplayChunkPayload;
}

function input(overrides: Partial<DeliverInput> = {}) {
  return {
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
    payload: payload(),
    unload: false,
    ...overrides,
  } satisfies DeliverInput;
}

describe("plugins/replay/delivery", () => {
  test("records no replay.chunk span for a routine chunk", async () => {
    const exporter = new InMemorySpanExporter();
    const ctx = createPluginContext([new SimpleSpanProcessor({ exporter })]);

    worker.use(
      http.post("https://collector.test/v3/replay/upload/:id", () =>
        HttpResponse.json({ objectKey: "replays/x/0000.json.gz" }),
      ),
    );

    await expect(deliverChunk(input({ ctx }))).resolves.toBe("ok");

    expect(
      exporter
        .getFinishedSpans()
        .filter((span) => span.name === "replay.chunk"),
    ).toHaveLength(0);
  });

  test("records the replay.chunk span on the kernel tracer for an exception snapshot", async () => {
    const exporter = new InMemorySpanExporter();
    const ctx = createPluginContext([new SimpleSpanProcessor({ exporter })]);

    worker.use(
      http.post("https://collector.test/v3/replay/upload/:id", () =>
        HttpResponse.json({ objectKey: "replays/x/0000.json.gz" }),
      ),
    );

    await expect(
      deliverChunk(input({ ctx, trigger: "exception" })),
    ).resolves.toBe("ok");

    const spans = exporter
      .getFinishedSpans()
      .filter((span) => span.name === "replay.chunk");

    expect(spans).toHaveLength(1);

    expect(spans[0]?.attributes).toMatchObject({
      "replay.chunk.event_count": 3,
      "replay.chunk.uri": "replays/x/0000.json.gz",
      "replay.chunk.trigger": "exception",
    });
  });

  test("encodes chunk + payload metadata into the x-replay-meta header", async () => {
    setLogLevel("warn");

    let headers = new Headers();
    let handled = false;

    worker.use(
      http.post(
        "https://collector.test/v3/replay/upload/:id",
        ({ request }) => {
          ({ headers } = request);

          handled = true;

          return HttpResponse.json({ ok: true }, { status: 200 });
        },
      ),
    );

    const controllerState =
      navigator.serviceWorker.controller?.state ?? "uncontrolled";

    const result = await deliverChunk(input());

    expect(
      result,
      `Replay upload: controller=${controllerState}, handlerObserved=${handled}`,
    ).toBe("ok");

    expect(JSON.parse(atob(headers.get("x-replay-meta") ?? ""))).toStrictEqual({
      activityCount: 3,
      eventCount: 3,
      firstTs: 1000,
      hasFullSnapshot: true,
      lastTs: 2000,
      segmentId: 7,
      windowId: "win_x",
    });
  });

  test.each(
    [
      null,
      true,
      42,
      "accepted",
      [],
      {},
      { objectKey: null },
      { objectKey: 1 },
      { objectKey: "replays/chunk.jsonl" },
    ].map((response) => ({ response })),
  )(
    "accepts an uploaded chunk with optional artifact metadata %j",
    async ({ response }) => {
      worker.use(
        http.post("https://collector.test/v3/replay/upload/:id", () =>
          HttpResponse.json(response),
        ),
      );

      await expect(deliverChunk(input())).resolves.toBe("ok");
    },
  );

  test.each([
    { unload: true, expectedEncoding: "identity" },
    { unload: false, expectedEncoding: "gzip" },
  ])(
    "preserves arbitrary authentication headers for unload $unload",
    async ({ unload, expectedEncoding }) => {
      const requests: Request[] = [];

      worker.use(
        http.post(
          "https://collector.test/v3/replay/upload/:id",
          ({ request }) => {
            requests.push(request);

            return HttpResponse.json({ ok: true });
          },
        ),
      );

      await expect(
        deliverChunk(
          input({
            target: {
              baseUrl: "https://collector.test",
              publicKeyHeader: { apiKey: "test-key" },
              headers: {
                "x-customer-token": "token",
                "content-encoding": "identity",
              },
              maxRetries: 0,
              timeoutInSeconds: 10,
            },
            unload,
          }),
        ),
      ).resolves.toBe("ok");

      const [request] = requests;

      if (!request) {
        throw new Error("Expected replay upload");
      }

      expect(request.headers.get("x-customer-token")).toBe("token");

      expect(request.headers.get("content-type")).toBe(
        "application/octet-stream",
      );

      expect(request.headers.get("content-encoding")).toBe(expectedEncoding);
    },
  );

  test.each([
    { status: 401, expected: "fatal" },
    { status: 403, expected: "fatal" },
    { status: 413, expected: "fatal" },
    { status: 409, expected: "skipped" },
    { status: 400, expected: "retryable" },
    { status: 408, expected: "retryable" },
    { status: 429, expected: "retryable" },
    { status: 503, expected: "retryable" },
  ])(
    "classifies a $status rejection as $expected",
    async ({ status, expected }) => {
      worker.use(
        http.post(
          "https://collector.test/v3/replay/upload/:id",
          () => new HttpResponse("", { status }),
        ),
      );

      await expect(deliverChunk(input())).resolves.toBe(expected);
    },
  );

  test("classifies a network error as retryable", async () => {
    worker.use(
      http.post("https://collector.test/v3/replay/upload/:id", () =>
        HttpResponse.error(),
      ),
    );

    await expect(deliverChunk(input())).resolves.toBe("retryable");
  });
});

function captureUploads() {
  const requests: Request[] = [];

  worker.use(
    http.post("https://collector.test/v3/replay/upload/:id", ({ request }) => {
      requests.push(request);

      return HttpResponse.json({ objectKey: "replays/x/0000.json.gz" });
    }),
  );

  return requests;
}

function randomJson(bytes: number) {
  const alphabet =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

  let text = "";

  while (text.length < bytes) {
    const values = crypto.getRandomValues(
      new Uint8Array(Math.min(bytes - text.length, 65_536)),
    );

    for (const value of values) {
      text += alphabet[value % alphabet.length];
    }
  }

  return text;
}

describe("plugins/replay/delivery keepalive", () => {
  test("sends an unload chunk with keepalive when it fits the shared body budget", async () => {
    const requests = captureUploads();

    await expect(deliverChunk(input({ unload: true }))).resolves.toBe("ok");

    expect(requests.map((request) => request.keepalive)).toStrictEqual([true]);

    expect(requests[0]?.headers.has("content-encoding")).toBe(false);
  });

  test("sends a routine chunk without keepalive", async () => {
    const requests = captureUploads();

    await expect(deliverChunk(input())).resolves.toBe("ok");

    expect(requests.map((request) => request.keepalive)).toStrictEqual([false]);
  });

  test("gzips an oversized unload chunk synchronously so it fits the keepalive budget", async () => {
    const exporter = new InMemorySpanExporter();
    const ctx = createPluginContext([new SimpleSpanProcessor({ exporter })]);
    const requests = captureUploads();
    const bodyJson = `${"x".repeat(70_000)}\n`;

    await expect(
      deliverChunk(
        input({ ctx, unload: true, payload: { ...payload(), bodyJson } }),
      ),
    ).resolves.toBe("ok");

    const [request] = requests;

    expect(request?.keepalive).toBe(true);

    expect(request?.headers.get("content-encoding")).toBe("gzip");

    const body = new Uint8Array((await request?.arrayBuffer()) ?? []);

    expect(body.byteLength).toBeLessThan(60_000);

    expect(new TextDecoder().decode(gunzipSync(body))).toBe(bodyJson);

    const span = exporter
      .getFinishedSpans()
      .find((item) => item.name === "replay.chunk");

    expect(span?.attributes).toMatchObject({
      "replay.chunk.unload": true,
      "replay.chunk.keepalive": true,
      "replay.chunk.size_bytes": bodyJson.length,
      "replay.chunk.compressed_bytes": body.byteLength,
    });
  });

  test("sends an unload chunk that stays over the budget after gzip without keepalive, records it on the span and releases the budget", async () => {
    const exporter = new InMemorySpanExporter();
    const ctx = createPluginContext([new SimpleSpanProcessor({ exporter })]);
    const requests = captureUploads();

    await expect(
      deliverChunk(
        input({
          ctx,
          unload: true,
          payload: { ...payload(), bodyJson: randomJson(120_000) },
        }),
      ),
    ).resolves.toBe("ok");

    await expect(deliverChunk(input({ ctx, unload: true }))).resolves.toBe(
      "ok",
    );

    expect(requests.map((request) => request.keepalive)).toStrictEqual([
      false,
      true,
    ]);

    expect(requests[0]?.headers.get("content-encoding")).toBe("gzip");

    expect(
      exporter
        .getFinishedSpans()
        .filter((item) => item.name === "replay.chunk")
        .map((item) => item.attributes["replay.chunk.keepalive"]),
    ).toStrictEqual([false, true]);
  });

  test("does not gzip an unload chunk above the synchronous compression ceiling", async () => {
    const requests = captureUploads();

    await expect(
      deliverChunk(
        input({
          unload: true,
          payload: { ...payload(), bodyJson: "x".repeat(2 * 1024 * 1024 + 1) },
        }),
      ),
    ).resolves.toBe("ok");

    expect(requests.map((request) => request.keepalive)).toStrictEqual([false]);

    expect(requests[0]?.headers.has("content-encoding")).toBe(false);
  });
});
