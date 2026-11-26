import { expect, test } from "vitest";
import { HttpResponse } from "msw";
import { gunzipSync } from "node:zlib";
import { createHarness } from "../../../../test/native-ingest-harness.js";

test("the actual closing page retains its oversized final segment and the next page delivers original bytes and identity", async () => {
  const harness = await createHarness();

  await harness.page.goto(`${harness.origin}/replay-carryover`);

  const original = await harness.page.evaluate(() =>
    globalThis.replayCarryoverFixture.prepare(),
  );

  const closed = harness.page.waitForEvent("close");

  await harness.page.close({ runBeforeUnload: true });

  await closed;

  const reopened = await harness.context.newPage();

  await reopened.goto(`${harness.origin}/replay-carryover?allowed=false`);

  const retained = await reopened.evaluate(() =>
    Array.from({ length: localStorage.length }, (_, index) =>
      localStorage.key(index),
    )
      .filter((key) => key !== null)
      .map(
        (key) =>
          JSON.parse(localStorage.getItem(key) ?? "null") as { body: string },
      ),
  );

  expect(retained).toHaveLength(1);

  const [first] = retained;

  if (!first) {
    throw new Error("Missing durable replay segment");
  }

  const envelope = JSON.parse(first.body) as {
    body: string;
    meta: string;
    sessionId: string;
  };

  const prepared = Buffer.from(envelope.body, "base64");

  expect(prepared.byteLength).toBeGreaterThan(64 * 1024);
  expect(gunzipSync(prepared).toString()).toBe(original);

  harness.setResponse(() => HttpResponse.json({}));

  await reopened.goto(`${harness.origin}/replay-carryover`);

  await expect
    .poll(async () => reopened.evaluate(() => localStorage.length))
    .toBe(0);

  const delivered = harness.received.at(-1);

  if (!delivered) {
    throw new Error("Missing received replay segment");
  }

  expect(delivered.path).toBe("/v3/replay/upload/original-session");
  expect(delivered.body).toStrictEqual(prepared);
  expect(delivered.headers.get("x-replay-meta")).toBe(envelope.meta);

  expect(
    JSON.parse(Buffer.from(envelope.meta, "base64").toString()),
  ).toMatchObject({ windowId: "original-window", segmentId: 7 });
});

test("replay carryover requires the original public key and consent, and revoke purges it", async () => {
  const harness = await createHarness();

  await harness.page.goto(`${harness.origin}/replay-carryover`);

  await harness.page.evaluate(() =>
    globalThis.replayCarryoverFixture.prepare(),
  );

  const closed = harness.page.waitForEvent("close");

  await harness.page.close({ runBeforeUnload: true });

  await closed;

  const reopened = await harness.context.newPage();

  await reopened.goto(`${harness.origin}/replay-carryover?key=another-key`);

  await expect(reopened.evaluate(() => localStorage.length)).resolves.toBe(1);

  await reopened.evaluate(() => globalThis.replayCarryoverFixture.deny());

  await expect(reopened.evaluate(() => localStorage.length)).resolves.toBe(0);

  await reopened.goto(`${harness.origin}/replay-carryover`);

  await expect(reopened.evaluate(() => localStorage.length)).resolves.toBe(0);
});

test("an acknowledged unload preserves the real replay object URI", async () => {
  const harness = await createHarness();

  harness.setResponse(() =>
    HttpResponse.json({ objectKey: "replays/acknowledged.json.gz" }),
  );

  await harness.page.goto(`${harness.origin}/replay-carryover`);

  const spans = await harness.page.evaluate(async () =>
    globalThis.replayCarryoverFixture.send(),
  );

  expect(spans).toHaveLength(1);

  expect(spans[0]).toMatchObject({
    "replay.chunk.uri": "replays/acknowledged.json.gz",
  });

  expect(spans[0]).not.toHaveProperty("replay.chunk.queued");

  await expect(harness.page.evaluate(() => localStorage.length)).resolves.toBe(
    0,
  );
});

test("durable custody is explicitly queued and never reports a delivered replay URI", async () => {
  const harness = await createHarness();

  await harness.page.goto(`${harness.origin}/replay-carryover`);

  const spans = await harness.page.evaluate(async () =>
    globalThis.replayCarryoverFixture.send(),
  );

  expect(spans).toHaveLength(1);
  expect(spans[0]).toMatchObject({ "replay.chunk.queued": true });
  expect(spans[0]).not.toHaveProperty("replay.chunk.uri");

  await expect(harness.page.evaluate(() => localStorage.length)).resolves.toBe(
    1,
  );
});

test("a terminal carryover rejection drops the segment without requeueing", async () => {
  const harness = await createHarness();

  harness.setResponse(() => new HttpResponse(null, { status: 400 }));

  await harness.page.goto(`${harness.origin}/replay-carryover`);

  await expect(
    harness.page.evaluate(async () => globalThis.replayCarryoverFixture.send()),
  ).resolves.toStrictEqual([]);

  await expect(
    harness.page.evaluate(() => globalThis.replayCarryoverFixture.result()),
  ).resolves.toBe("skipped");

  await expect(harness.page.evaluate(() => localStorage.length)).resolves.toBe(
    0,
  );
});

test("an unloading page delivers a keepalive-sized final segment at once while another tab holds the replay drain lock", async () => {
  const harness = await createHarness();

  harness.setResponse(() => HttpResponse.json({}));

  const holder = await harness.context.newPage();

  await holder.goto(`${harness.origin}/replay-carryover?allowed=false`);

  await holder.evaluate(
    async (name) =>
      new Promise((resolve) => {
        void navigator.locks.request(name, async () => {
          resolve(null);

          await Promise.withResolvers<null>().promise;
        });
      }),
    `interfere:export:v1:replay:${JSON.stringify([harness.origin, "replay-public-key"])}`,
  );

  await harness.page.goto(`${harness.origin}/replay-carryover`);

  const body = JSON.stringify({ type: 2, timestamp: 123, data: ["tail"] });

  const spans = await harness.page.evaluate(
    async (segment) =>
      globalThis.replayCarryoverFixture.send({ body: segment }),
    body,
  );

  expect(spans).toHaveLength(1);

  expect(spans[0]).toMatchObject({
    "replay.chunk.unload": true,
    "replay.chunk.keepalive": true,
  });

  expect(spans[0]).not.toHaveProperty("replay.chunk.queued");

  const delivered = harness.received.filter(
    (request) => request.path === "/v3/replay/upload/original-session",
  );

  expect(delivered).toHaveLength(1);

  expect(delivered[0]?.body.toString()).toBe(body);

  expect(
    JSON.parse(
      Buffer.from(
        delivered[0]?.headers.get("x-replay-meta") ?? "",
        "base64",
      ).toString(),
    ),
  ).toMatchObject({ windowId: "original-window", segmentId: 7 });

  await expect(harness.page.evaluate(() => localStorage.length)).resolves.toBe(
    0,
  );
});
