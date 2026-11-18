import { HttpResponse } from "msw";
import { expect, onTestFinished, test } from "vitest";
import { createHarness } from "../../../test/native-ingest-harness.js";

test("hidden exports survive tab close, drain in order on the next page, and hold the one in flight at close until its replay window opens", async () => {
  const harness = await createHarness();

  await harness.page.goto(`${harness.origin}/carryover`);

  await harness.page.evaluate(() => {
    for (let index = 0; index < 8; index += 1) {
      globalThis.exportCarryoverFixture.enqueue(
        `${index}:${"x".repeat(15_000)}`,
      );
    }
  });

  await expect(
    harness.page.evaluate(() =>
      Array.from({ length: localStorage.length }, (_, index) =>
        localStorage.key(index),
      ).filter((key) => key !== null),
    ),
  ).resolves.toHaveLength(8);

  await harness.page.close();

  const reopened = await harness.context.newPage();

  await reopened.goto(`${harness.origin}/carryover`);

  await expect
    .poll(async () =>
      reopened.evaluate(
        () => globalThis.exportCarryoverFixture.delivered.length,
      ),
    )
    .toBe(7);

  await expect(
    reopened.evaluate(() =>
      globalThis.exportCarryoverFixture.delivered.map(
        (body) => body.split(":")[0],
      ),
    ),
  ).resolves.toStrictEqual(["1", "2", "3", "4", "5", "6", "7"]);

  const held = await reopened.evaluate(() =>
    Array.from({ length: localStorage.length }, (_, index) =>
      localStorage.key(index),
    )
      .filter((key) => key !== null)
      .map((key) => {
        const entry = JSON.parse(localStorage.getItem(key) ?? "{}") as {
          body: string;
          dispatched?: number;
        };

        localStorage.setItem(key, JSON.stringify({ ...entry, dispatched: 0 }));

        return {
          body: entry.body.split(":")[0],
          dispatched: entry.dispatched !== undefined,
        };
      }),
  );

  expect(held).toStrictEqual([{ body: "0", dispatched: true }]);

  await reopened.reload();

  await expect
    .poll(async () =>
      reopened.evaluate(() => globalThis.exportCarryoverFixture.delivered),
    )
    .toStrictEqual([expect.stringMatching(/^0:/u)]);

  await expect(
    reopened.evaluate(() =>
      Array.from({ length: localStorage.length }, (_, index) =>
        localStorage.key(index),
      ).filter((key) => key !== null),
    ),
  ).resolves.toHaveLength(0);
});

test("carryover stays within its public-key scope and clears before a later page drains", async () => {
  const harness = await createHarness();

  await harness.page.goto(`${harness.origin}/carryover?scope=one`);

  await harness.page.evaluate(() =>
    globalThis.exportCarryoverFixture.enqueue("private payload"),
  );

  await harness.page.close();

  const reopened = await harness.context.newPage();

  await reopened.goto(`${harness.origin}/carryover?scope=two`);

  await expect(
    reopened.evaluate(() => globalThis.exportCarryoverFixture.delivered),
  ).resolves.toStrictEqual([]);

  await expect(
    reopened.evaluate(() =>
      Array.from({ length: localStorage.length }, (_, index) =>
        localStorage.key(index),
      ).filter((key) => key !== null),
    ),
  ).resolves.toHaveLength(1);

  await reopened.evaluate(() => globalThis.exportCarryoverFixture.deny());

  await reopened.goto(`${harness.origin}/carryover?scope=one`);

  await expect(
    reopened.evaluate(() => globalThis.exportCarryoverFixture.delivered),
  ).resolves.toStrictEqual([]);

  await expect(
    reopened.evaluate(() =>
      Array.from({ length: localStorage.length }, (_, index) =>
        localStorage.key(index),
      ).filter((key) => key !== null),
    ),
  ).resolves.toHaveLength(0);
});

test("carryover rejects new payloads beyond its count and byte bounds", async () => {
  const harness = await createHarness();

  await harness.page.goto(`${harness.origin}/carryover`);

  await harness.page.evaluate(() => {
    for (let index = 0; index < 70; index += 1) {
      globalThis.exportCarryoverFixture.enqueue("x".repeat(20_000));
    }
  });

  const retained = await harness.page.evaluate(() =>
    Array.from(
      { length: localStorage.length },
      (_, index) => localStorage.getItem(localStorage.key(index) ?? "") ?? "{}",
    ).map((value) => (JSON.parse(value) as { body: string }).body),
  );

  expect(retained.length).toBeLessThanOrEqual(64);

  expect(
    retained.reduce((bytes, body) => bytes + body.length, 0),
  ).toBeLessThanOrEqual(1024 * 1024);

  expect(retained.length).toBeGreaterThan(0);
});

test("a payload added during an active drain gets a subsequent wakeup", async () => {
  const harness = await createHarness();

  await harness.page.goto(`${harness.origin}/carryover`);

  await harness.page.evaluate(() =>
    globalThis.exportCarryoverFixture.enqueue("first"),
  );

  await expect
    .poll(async () =>
      harness.page.evaluate(
        () => globalThis.exportCarryoverFixture.pending.length,
      ),
    )
    .toBe(1);

  await harness.page.evaluate(() => {
    globalThis.exportCarryoverFixture.enqueue("second");

    globalThis.exportCarryoverFixture.release();
  });

  await expect
    .poll(async () =>
      harness.page.evaluate(() => globalThis.exportCarryoverFixture.delivered),
    )
    .toStrictEqual(["first", "second"]);
});

test("revoking consent aborts the pending drain and clears retained payloads", async () => {
  const harness = await createHarness();

  await harness.page.goto(`${harness.origin}/carryover`);

  await harness.page.evaluate(() =>
    globalThis.exportCarryoverFixture.enqueue("old identity"),
  );

  await expect
    .poll(async () =>
      harness.page.evaluate(
        () => globalThis.exportCarryoverFixture.pending.length,
      ),
    )
    .toBe(1);

  await harness.page.evaluate(() => globalThis.exportCarryoverFixture.deny());

  await expect(
    harness.page.evaluate(() => globalThis.exportCarryoverFixture.aborted()),
  ).resolves.toBe(1);

  await expect(harness.page.evaluate(() => localStorage.length)).resolves.toBe(
    0,
  );

  await expect(
    harness.page.evaluate(() => globalThis.exportCarryoverFixture.delivered),
  ).resolves.toStrictEqual([]);
});

test.each([408, 429, 503])(
  "hTTP %s remains durable until a later successful page",
  async (status) => {
    const harness = await createHarness();

    await harness.page.goto(`${harness.origin}/carryover?status=${status}`);

    await harness.page.evaluate(() =>
      globalThis.exportCarryoverFixture.enqueue("retry me"),
    );

    await expect
      .poll(async () =>
        harness.page.evaluate(
          () => globalThis.exportCarryoverFixture.pending.length,
        ),
      )
      .toBe(1);

    await harness.page.evaluate(() =>
      globalThis.exportCarryoverFixture.release(),
    );

    await expect(
      harness.page.evaluate(() => localStorage.length),
    ).resolves.toBe(1);

    await harness.page.goto(`${harness.origin}/carryover`);

    await expect
      .poll(async () =>
        harness.page.evaluate(
          () => globalThis.exportCarryoverFixture.delivered,
        ),
      )
      .toStrictEqual(["retry me"]);

    await expect(
      harness.page.evaluate(() => localStorage.length),
    ).resolves.toBe(0);
  },
);

test("scoped revoke aborts replay across tabs while preserving other telemetry", async () => {
  const harness = await createHarness();

  await harness.page.goto(`${harness.origin}/carryover?scope=replay:one`);

  const other = await harness.context.newPage();

  await other.goto(`${harness.origin}/carryover?scope=otlp:one`);

  await harness.page.evaluate(() =>
    globalThis.exportCarryoverFixture.enqueue("replay"),
  );

  await other.evaluate(() =>
    globalThis.exportCarryoverFixture.enqueue("trace"),
  );

  await expect
    .poll(async () =>
      harness.page.evaluate(
        () => globalThis.exportCarryoverFixture.pending.length,
      ),
    )
    .toBe(1);

  await expect
    .poll(async () =>
      other.evaluate(() => globalThis.exportCarryoverFixture.pending.length),
    )
    .toBe(1);

  await other.evaluate(() =>
    globalThis.exportCarryoverFixture.clear("replay:"),
  );

  await expect
    .poll(async () =>
      harness.page.evaluate(() => globalThis.exportCarryoverFixture.aborted()),
    )
    .toBe(1);

  await expect(
    other.evaluate(() => globalThis.exportCarryoverFixture.aborted()),
  ).resolves.toBe(0);

  await expect(other.evaluate(() => localStorage.length)).resolves.toBe(1);

  await other.evaluate(() => globalThis.exportCarryoverFixture.release());

  await expect
    .poll(async () =>
      other.evaluate(() => globalThis.exportCarryoverFixture.delivered),
    )
    .toStrictEqual(["trace"]);
});

test("a keepalive export the browser rejects while the page unloads is delivered once, even when a host pagehide listener runs first", async () => {
  const harness = await createHarness();

  const context = await harness.context.browser()?.newContext();

  if (!context) {
    throw new Error("Missing browser");
  }

  onTestFinished(async () => {
    await context.close();
  });

  harness.setResponse(() => new HttpResponse(null));

  const page = await context.newPage();

  await page.goto(`${harness.origin}/unload-export`);

  await page.goto("about:blank");

  await page.goto(`${harness.origin}/after-unload`);

  await expect.poll(() => harness.requested).toContain("/after-unload");

  expect(
    harness.requested.filter((path) => path === "/drain/v1/traces"),
  ).toHaveLength(1);
});
