/* oxlint-disable typescript/require-await */
import { mapWithConcurrency } from "./map-with-concurrency.js";
import { describe, expect, test, vi } from "vitest";

function tasks(count: number) {
  return Array.from({ length: count }, () => {
    // oxlint-disable-next-line typescript/no-invalid-void-type
    const start = Promise.withResolvers<void>();
    // oxlint-disable-next-line typescript/no-invalid-void-type
    const finish = Promise.withResolvers<void>();

    return {
      begin: () => {
        start.resolve();
      },
      finished: finish.promise,
      release: () => {
        finish.resolve();
      },
      started: start.promise,
    };
  });
}

describe(mapWithConcurrency, () => {
  test.each([0, -1, 1.5, Number.NaN, Number.POSITIVE_INFINITY])(
    "rejects invalid concurrency %s without starting work",
    async (concurrency) => {
      const visit = vi.fn<(item: number) => Promise<number>>();

      await expect(mapWithConcurrency([1], concurrency, visit)).rejects.toThrow(
        "Concurrency must be a positive safe integer",
      );

      expect(visit).not.toHaveBeenCalled();
    },
  );

  test("propagates worker failures", async () => {
    const failure = new Error("read failed");

    await expect(
      mapWithConcurrency([1], 1, async () => {
        throw failure;
      }),
    ).rejects.toBe(failure);
  });

  test("starts exactly `concurrency` tasks before any of them finishes", async () => {
    const pending = tasks(10);
    const order: number[] = [];

    const run = mapWithConcurrency(pending, 3, async (task, index) => {
      order.push(index);

      task.begin();

      await task.finished;

      return index;
    });

    await Promise.all(pending.slice(0, 3).map(async (task) => task.started));

    expect(order).toStrictEqual([0, 1, 2]);

    pending[0]?.release();

    await pending[3]?.started;

    expect(order).toStrictEqual([0, 1, 2, 3]);

    for (const task of pending) {
      task.release();
    }

    await expect(run).resolves.toHaveLength(10);
  });

  test("returns results in input order regardless of completion order", async () => {
    const pending = tasks(4);

    const run = mapWithConcurrency(pending, 4, async (task, index) => {
      task.begin();

      await task.finished;

      return index * 10;
    });

    await Promise.all(pending.map(async (task) => task.started));

    for (const task of pending.toReversed()) {
      task.release();
    }

    await expect(run).resolves.toStrictEqual([0, 10, 20, 30]);
  });

  test("visits every item exactly once", async () => {
    const items = Array.from({ length: 200 }, (_, i) => i);
    const seen = new Map<number, number>();

    await mapWithConcurrency(items, 16, async (item) => {
      seen.set(item, (seen.get(item) ?? 0) + 1);

      return item;
    });

    expect(seen).toStrictEqual(new Map(items.map((item) => [item, 1])));
  });

  test("does no work for an empty input", async () => {
    let calls = 0;

    const results = await mapWithConcurrency([], 8, async () => {
      calls += 1;

      return calls;
    });

    expect(results).toStrictEqual([]);
    expect(calls).toBe(0);
  });
});
