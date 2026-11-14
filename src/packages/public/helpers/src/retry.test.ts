/* oxlint-disable typescript/require-await */
import { backoffMs, retry } from "./retry.js";
import { describe, expect, test } from "vitest";

function harness() {
  const slept: number[] = [];

  return {
    options: {
      random: () => 0.5,
      sleep: async (ms: number) => {
        slept.push(ms);
      },
    },
    slept,
  };
}

function failing(failures: number, error = new Error("boom")) {
  let calls = 0;

  return {
    calls: () => calls,
    run: async () => {
      calls += 1;

      if (calls <= failures) {
        throw error;
      }

      return calls;
    },
  };
}

describe(retry, () => {
  test("returns the first success without waiting", async () => {
    const { options, slept } = harness();

    await expect(retry(async () => "ok", options)).resolves.toBe("ok");

    expect(slept).toStrictEqual([]);
  });

  test("backs off exponentially with jitter between attempts", async () => {
    const { options, slept } = harness();
    const op = failing(2);

    await expect(
      retry(op.run, { ...options, attempts: 3, baseDelayMs: 100 }),
    ).resolves.toBe(3);

    expect(slept).toStrictEqual([75, 150]);
  });

  test("rethrows the last error when attempts run out", async () => {
    const { options } = harness();
    const op = failing(10);

    await expect(retry(op.run, { ...options, attempts: 3 })).rejects.toThrow(
      "boom",
    );

    expect(op.calls()).toBe(3);
  });

  test("stops immediately when retryOn declines", async () => {
    const { options, slept } = harness();
    const op = failing(10);

    await expect(
      retry(op.run, { ...options, retryOn: () => false }),
    ).rejects.toThrow("boom");

    expect(op.calls()).toBe(1);
    expect(slept).toStrictEqual([]);
  });

  test("uses delayMs in place of backoff", async () => {
    const { options, slept } = harness();
    const op = failing(2);

    await retry(op.run, {
      ...options,
      attempts: 3,
      delayMs: (index) => (index + 1) * 1000,
    });

    expect(slept).toStrictEqual([1000, 2000]);
  });

  test("aborts the wait with the signal's reason", async () => {
    const controller = new AbortController();
    const op = failing(10);

    const pending = retry(op.run, {
      baseDelayMs: 60_000,
      signal: controller.signal,
    });

    controller.abort(new Error("stop"));

    await expect(pending).rejects.toThrow("stop");
  });
});

describe(backoffMs, () => {
  test("caps at maxDelayMs", () => {
    expect(backoffMs(10, 100, 1000, () => 1)).toBe(1000);
  });
});
