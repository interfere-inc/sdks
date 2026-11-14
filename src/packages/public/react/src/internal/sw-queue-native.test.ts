/* oxlint-disable typescript/require-await */
import { HttpResponse } from "msw";
import { expect, onTestFinished, test } from "vitest";
import { createHarness, entries } from "../../test/native-ingest-harness.js";

test("native activation restores a failed export and overlapping background sync retries it once", async () => {
  const runtime = await createHarness();

  await runtime.page.evaluate(async () => {
    const fixture = globalThis.queueFixture;

    await fixture.seed([{ path: "/api/interfere/events", body: "queued" }]);

    await fixture.register();
  });

  await runtime.arrived;

  await expect.poll(async () => entries(runtime.page)).toHaveLength(1);

  expect(runtime.requested).toStrictEqual(["/api/interfere/events"]);

  await expect(
    runtime.page.evaluate(() => globalThis.queueFixture.messages),
  ).resolves.toStrictEqual([]);

  const response = Promise.withResolvers<Response>();

  onTestFinished(() =>
    response.resolve(new HttpResponse(null, { status: 503 })),
  );

  runtime.setResponse(async () => response.promise);

  await runtime.sync();

  await runtime.requestedReached(2);

  expect(runtime.requested).toHaveLength(2);

  await runtime.sync();

  response.resolve(new HttpResponse(null, { status: 200 }));

  await expect
    .poll(async () =>
      runtime.page.evaluate(() => globalThis.queueFixture.messages),
    )
    .toStrictEqual([
      {
        type: "interfere.sw.replayed",
        url: new URL("/api/interfere/events", runtime.page.url()).href,
      },
    ]);

  await expect.poll(async () => entries(runtime.page)).toStrictEqual([]);

  expect(runtime.requested).toStrictEqual([
    "/api/interfere/events",
    "/api/interfere/events",
  ]);
});

test.each([{ bytes: "999999999" }, { bytes: null }, {}])(
  "prunes malformed persisted byte metadata in the native queue: %j",
  async (metadata) => {
    const runtime = await createHarness();

    const status = await runtime.page.evaluate(async (persistedMetadata) => {
      const fixture = globalThis.queueFixture;

      await fixture.register();

      await fixture.seed([
        { path: "/old-malformed", metadata: persistedMetadata },
        { path: "/old-full", metadata: { bytes: 10 * 1024 * 1024 } },
      ]);

      const response = await fetch("/api/interfere/events", {
        method: "POST",
        body: "new",
      });

      return response.status;
    }, metadata);

    expect(status).toBe(202);

    await expect
      .poll(async () =>
        runtime.page.evaluate(() =>
          globalThis.queueFixture.messages.map((message) => [
            message.type,
            new URL(message.url).pathname,
          ]),
        ),
      )
      .toStrictEqual([
        ["interfere.sw.dropped", "/old-malformed"],
        ["interfere.sw.dropped", "/old-full"],
        ["interfere.sw.queued", "/api/interfere/events"],
      ]);

    await expect
      .poll(async () =>
        (await entries(runtime.page)).map((entry) => entry.metadata),
      )
      .toStrictEqual([{ bytes: 3 }]);
  },
);

test("an ingest backoff skips exports on a page no service worker controls", async () => {
  const runtime = await createHarness();

  const uncontrolled = await runtime.page.evaluate(async () => {
    const fixture = globalThis.queueFixture;

    await fixture.backOff();

    return fixture.ingest("/api/interfere/events");
  });

  expect(uncontrolled).toBe("backoff");

  expect(runtime.requested).toStrictEqual([]);
});

test("an ingest backoff still sends offline exports that the controlling ingest queue accepts", async () => {
  const runtime = await createHarness();

  runtime.setResponse(async () => new HttpResponse(null, { status: 200 }));

  const online = await runtime.page.evaluate(async () => {
    const fixture = globalThis.queueFixture;

    await fixture.register();

    return fixture.ingest("/api/interfere/events");
  });

  expect(online).toBe(200);

  await runtime.page.context().setOffline(true);

  const outcomes = await runtime.page.evaluate(async () => {
    const fixture = globalThis.queueFixture;

    await fixture.backOff();

    return {
      queued: await fixture.ingest("/api/interfere/events"),
      unmatched: await fixture.ingest("/elsewhere"),
    };
  });

  expect(outcomes).toStrictEqual({ queued: 202, unmatched: "backoff" });

  await expect
    .poll(async () =>
      (await entries(runtime.page)).map(
        (entry) => new URL(entry.requestData.url).pathname,
      ),
    )
    .toStrictEqual(["/api/interfere/events"]);

  expect(runtime.requested).toStrictEqual(["/api/interfere/events"]);
});

test("native queue drops only the oldest entry when valid payloads exceed its byte cap", async () => {
  const runtime = await createHarness();

  const status = await runtime.page.evaluate(async () => {
    const fixture = globalThis.queueFixture;

    await fixture.register();

    await fixture.seed([
      {
        path: "/oldest",
        body: "a".repeat(6 * 1024 * 1024),
        metadata: { bytes: 6 * 1024 * 1024 },
      },
      {
        path: "/retained",
        body: "b".repeat(4 * 1024 * 1024),
        metadata: { bytes: 4 * 1024 * 1024 },
      },
    ]);

    return (
      await fetch("/api/interfere/events", { method: "POST", body: "new" })
    ).status;
  });

  expect(status).toBe(202);

  await expect
    .poll(async () =>
      (await entries(runtime.page)).map(
        (entry) => new URL(entry.requestData.url).pathname,
      ),
    )
    .toStrictEqual(["/retained", "/api/interfere/events"]);

  await expect
    .poll(async () =>
      runtime.page.evaluate(() =>
        globalThis.queueFixture.messages.map((message) => [
          message.type,
          new URL(message.url).pathname,
        ]),
      ),
    )
    .toStrictEqual([
      ["interfere.sw.dropped", "/oldest"],
      ["interfere.sw.queued", "/api/interfere/events"],
    ]);
});

test.each([400, 401, 403, 404, 413, 422])(
  "native queue passes non-retryable %s through without persistence",
  async (status) => {
    const runtime = await createHarness();

    runtime.setResponse(async () => new HttpResponse("rejected", { status }));

    const response = await runtime.page.evaluate(async () => {
      await globalThis.queueFixture.register();

      const result = await fetch("/api/interfere/events", {
        method: "POST",
        body: "export",
      });

      return { status: result.status, body: await result.text() };
    });

    expect(response).toStrictEqual({ status, body: "rejected" });

    await expect(entries(runtime.page)).resolves.toStrictEqual([]);

    await expect(
      runtime.page.evaluate(() => globalThis.queueFixture.messages),
    ).resolves.toStrictEqual([]);
  },
);

test("an oversized request is rejected without evicting retained requests", async () => {
  const runtime = await createHarness();

  const status = await runtime.page.evaluate(async () => {
    await globalThis.queueFixture.register();

    await globalThis.queueFixture.seed([
      { path: "/retained", body: "retained", metadata: { bytes: 8 } },
    ]);

    const response = await fetch("/api/interfere/events", {
      method: "POST",
      body: "x".repeat(10 * 1024 * 1024 + 1),
    });

    return response.status;
  });

  expect(status).toBe(503);

  await expect
    .poll(async () =>
      (await entries(runtime.page)).map(
        (entry) => new URL(entry.requestData.url).pathname,
      ),
    )
    .toStrictEqual(["/retained"]);

  await expect
    .poll(async () =>
      runtime.page.evaluate(() => globalThis.queueFixture.messages),
    )
    .toStrictEqual([
      {
        type: "interfere.sw.dropped",
        url: `${runtime.origin}/api/interfere/events`,
        reason: "payload_too_large",
      },
    ]);
});

test("queue registration waits for activation replay before callers seed or inspect the queue", async () => {
  const runtime = await createHarness();

  const response = Promise.withResolvers<Response>();

  onTestFinished(() =>
    response.resolve(new HttpResponse(null, { status: 503 })),
  );

  runtime.setResponse(async () => response.promise);

  const registering = runtime.page.evaluate(async () => {
    await globalThis.queueFixture.seed([
      { path: "/activation-pending", body: "retained", metadata: { bytes: 8 } },
    ]);

    await globalThis.queueFixture.register();

    return (await navigator.serviceWorker.ready).active?.state;
  });

  await runtime.arrived;

  await expect(
    runtime.page.evaluate(
      async () => (await navigator.serviceWorker.ready).active?.state,
    ),
  ).resolves.toBe("activating");

  response.resolve(new HttpResponse(null, { status: 503 }));

  await expect(registering).resolves.toBe("activated");

  await expect(entries(runtime.page)).resolves.toHaveLength(1);
});
