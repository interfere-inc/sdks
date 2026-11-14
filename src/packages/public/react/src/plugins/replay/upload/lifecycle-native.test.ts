import { expect, onTestFinished, test } from "vitest";
import { HttpResponse } from "msw";
import { createHarness } from "../../../../test/native-ingest-harness.js";

const body = JSON.stringify([
  { type: 2, timestamp: 123, data: "customer-view" },
]);

test.each([408, 429, 500, 502, 504])(
  "a replay rejected with HTTP %i recovers after reload with its exact payload and attribution",
  async (status) => {
    const runtime = await createHarness();

    runtime.setResponse(() => new HttpResponse(null, { status }));

    await runtime.page.goto(`${runtime.origin}/replay-carryover`);

    await runtime.page.evaluate(
      async (payload) =>
        globalThis.replayCarryoverFixture.send({ body: payload }),
      body,
    );

    expect(runtime.received).toHaveLength(1);

    const [first] = runtime.received;

    expect(first?.body.toString()).toBe(body);

    await expect(
      runtime.page.evaluate(() => localStorage.length),
    ).resolves.toBe(1);

    runtime.setResponse(() => HttpResponse.json({ objectKey: "recovered" }));

    await runtime.page.reload();

    await expect
      .poll(async () => runtime.page.evaluate(() => localStorage.length))
      .toBe(0);

    expect(runtime.received).toHaveLength(2);

    const [, second] = runtime.received;

    expect(second?.body).toStrictEqual(first?.body);
    expect(second?.path).toBe("/v3/replay/upload/original-session");

    expect(second?.headers.get("x-interfere-public-key")).toBe(
      "replay-public-key",
    );

    expect(second?.headers.get("x-replay-meta")).toBe(
      first?.headers.get("x-replay-meta"),
    );
  },
);

test.each([
  { status: 401, result: "fatal" },
  { status: 403, result: "fatal" },
  { status: 413, result: "fatal" },
  { status: 409, result: "skipped" },
  { status: 422, result: "skipped" },
])(
  "terminal replay HTTP $status is removed and classified as $result",
  async ({ status, result }) => {
    const runtime = await createHarness();

    runtime.setResponse(() => new HttpResponse(null, { status }));

    await runtime.page.goto(`${runtime.origin}/replay-carryover`);

    await expect(
      runtime.page.evaluate(async () =>
        globalThis.replayCarryoverFixture.send(),
      ),
    ).resolves.toStrictEqual([]);

    await expect(
      runtime.page.evaluate(() => globalThis.replayCarryoverFixture.result()),
    ).resolves.toBe(result);

    await expect(
      runtime.page.evaluate(() => localStorage.length),
    ).resolves.toBe(0);

    runtime.setResponse(() => HttpResponse.json({}));

    await runtime.page.reload();

    await runtime.page.evaluate(async () =>
      globalThis.replayCarryoverFixture.send({ sessionId: "new-session" }),
    );

    expect(runtime.received.map((request) => request.path)).toStrictEqual([
      "/v3/replay/upload/original-session",
      "/v3/replay/upload/new-session",
    ]);
  },
);

test("an offline page retains replay for navigation and the next page delivers it", async () => {
  const runtime = await createHarness();

  await runtime.page.goto(`${runtime.origin}/replay-carryover`);

  await runtime.context.setOffline(true);

  await runtime.page.evaluate(
    async (payload) =>
      globalThis.replayCarryoverFixture.send({ body: payload }),
    body,
  );

  expect(runtime.received).toStrictEqual([]);

  await expect(runtime.page.evaluate(() => localStorage.length)).resolves.toBe(
    1,
  );

  await runtime.page.goto("about:blank");

  await runtime.context.setOffline(false);

  runtime.setResponse(() => HttpResponse.json({}));

  await runtime.page.goto(`${runtime.origin}/replay-carryover`);

  await expect
    .poll(async () => runtime.page.evaluate(() => localStorage.length))
    .toBe(0);

  expect(runtime.received).toHaveLength(1);
  expect(runtime.received[0]?.body.toString()).toBe(body);
});

test("two pages recover one retained replay under a shared browser lock without duplicate delivery", async () => {
  const runtime = await createHarness();

  await runtime.page.goto(`${runtime.origin}/replay-carryover`);

  await runtime.page.evaluate(async () =>
    globalThis.replayCarryoverFixture.send(),
  );

  const scope = await runtime.page.evaluate(() => {
    const key = localStorage.key(0);

    if (!key) {
      throw new Error("Missing retained replay");
    }

    return (
      JSON.parse(localStorage.getItem(key) ?? "null") as { scope: string }
    ).scope;
  });

  await runtime.page.goto("about:blank");

  const acknowledged = Promise.withResolvers<Response>();

  onTestFinished(() =>
    acknowledged.resolve(new HttpResponse(null, { status: 503 })),
  );

  runtime.setResponse(async () => acknowledged.promise);

  const other = await runtime.context.newPage();

  await Promise.all([
    runtime.page.goto(`${runtime.origin}/replay-carryover`),
    other.goto(`${runtime.origin}/replay-carryover`),
  ]);

  await runtime.requestedReached(2);

  acknowledged.resolve(HttpResponse.json({}));

  await expect
    .poll(async () => other.evaluate(() => localStorage.length))
    .toBe(0);

  await other.evaluate(async (lockScope) => {
    await navigator.locks.request(
      `interfere:export:v1:${lockScope}`,
      () => undefined,
    );
  }, scope);

  expect(runtime.received).toHaveLength(2);
  expect(runtime.received[1]?.body).toStrictEqual(runtime.received[0]?.body);
});

test("changing the collector destination cannot drain another destination's retained replay", async () => {
  const runtime = await createHarness();

  await runtime.page.goto(`${runtime.origin}/replay-carryover`);

  await runtime.page.evaluate(async () =>
    globalThis.replayCarryoverFixture.send(),
  );

  await runtime.page.goto(
    `${runtime.origin}/replay-carryover?destination=${encodeURIComponent("https://different-collector.invalid")}`,
  );

  await expect(runtime.page.evaluate(() => localStorage.length)).resolves.toBe(
    1,
  );

  expect(runtime.received).toHaveLength(1);

  runtime.setResponse(() => HttpResponse.json({}));

  await runtime.page.goto(`${runtime.origin}/replay-carryover`);

  await expect
    .poll(async () => runtime.page.evaluate(() => localStorage.length))
    .toBe(0);

  expect(runtime.received).toHaveLength(2);
});

test("corrupt and expired retained replay is discarded while a valid later segment still drains", async () => {
  const runtime = await createHarness();

  await runtime.page.goto(`${runtime.origin}/replay-carryover`);

  await runtime.page.evaluate(
    async (payload) =>
      globalThis.replayCarryoverFixture.send({ body: payload }),
    body,
  );

  await runtime.page.goto(`${runtime.origin}/replay-carryover?allowed=false`);

  await runtime.page.evaluate(() => {
    const key = localStorage.key(0);

    if (!key) {
      throw new Error("Missing retained replay");
    }

    const valid = JSON.parse(localStorage.getItem(key) ?? "null") as {
      body: string;
      created: number;
      scope: string;
    };

    const envelope = JSON.parse(valid.body) as {
      body: string;
      id: string;
      sessionId: string;
      meta: string;
      encoding: "gzip" | null;
    };

    localStorage.setItem("interfere:export:v1:broken-json", "not-json");

    localStorage.setItem(
      "interfere:export:v1:expired",
      JSON.stringify({ ...valid, created: Date.now() - 25 * 60 * 60 * 1000 }),
    );

    localStorage.setItem(
      "interfere:export:v1:broken-envelope",
      JSON.stringify({
        ...valid,
        created: valid.created - 1,
        body: JSON.stringify({ ...envelope, body: "!invalid base64!" }),
      }),
    );
  });

  runtime.setResponse(() => HttpResponse.json({}));

  await runtime.page.goto(`${runtime.origin}/replay-carryover`);

  await expect
    .poll(async () => runtime.page.evaluate(() => localStorage.length))
    .toBe(0);

  expect(runtime.received).toHaveLength(2);
  expect(runtime.received[1]?.body.toString()).toBe(body);
});

test("queued segments keep their distinct session and segment identities through recovery", async () => {
  const runtime = await createHarness();

  await runtime.page.goto(`${runtime.origin}/replay-carryover`);

  await runtime.page.evaluate(async () => {
    await globalThis.replayCarryoverFixture.send({
      body: "first-view",
      sessionId: "first-session",
      segmentId: 2,
    });

    await globalThis.replayCarryoverFixture.send({
      body: "second-view",
      sessionId: "second-session",
      segmentId: 8,
    });
  });

  const before = runtime.received.length;

  runtime.setResponse(() => HttpResponse.json({}));

  await runtime.page.reload();

  await expect
    .poll(async () => runtime.page.evaluate(() => localStorage.length))
    .toBe(0);

  expect(
    runtime.received.slice(before).map((request) => ({
      path: request.path,
      body: request.body.toString(),
      meta: JSON.parse(
        Buffer.from(
          request.headers.get("x-replay-meta") ?? "",
          "base64",
        ).toString(),
      ) as unknown,
    })),
  ).toMatchObject([
    {
      path: "/v3/replay/upload/first-session",
      body: "first-view",
      meta: { windowId: "original-window", segmentId: 2 },
    },
    {
      path: "/v3/replay/upload/second-session",
      body: "second-view",
      meta: { windowId: "original-window", segmentId: 8 },
    },
  ]);
});
