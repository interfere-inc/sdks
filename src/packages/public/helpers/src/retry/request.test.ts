/* oxlint-disable typescript/require-await, unicorn/no-useless-promise-resolve-reject */
import { isRetryableStatus, retryRequest } from "./request.js";
import { describe, expect, test } from "vitest";

function harness() {
  const slept: number[] = [];

  return {
    options: {
      baseDelayMs: 100,
      now: () => 0,
      random: () => 0.5,
      sleep: async (ms: number) => {
        slept.push(ms);

        return Promise.resolve();
      },
    },
    slept,
  };
}

async function respond(status: number, headers?: Record<string, string>) {
  return new Response(null, headers ? { headers, status } : { status });
}

describe(isRetryableStatus, () => {
  test("retries throttling and transient server statuses only", () => {
    expect(isRetryableStatus(429)).toBe(true);
    expect(isRetryableStatus(408)).toBe(true);
    expect(isRetryableStatus(500)).toBe(true);
    expect(isRetryableStatus(503)).toBe(true);

    expect(isRetryableStatus(200)).toBe(false);
    expect(isRetryableStatus(400)).toBe(false);
    expect(isRetryableStatus(403)).toBe(false);
    expect(isRetryableStatus(404)).toBe(false);
  });
});

describe(retryRequest, () => {
  test("returns a success without sleeping", async () => {
    const { options, slept } = harness();
    let calls = 0;

    const response = await retryRequest(async () => {
      calls += 1;

      return respond(200);
    }, options);

    expect(response.status).toBe(200);
    expect(calls).toBe(1);
    expect(slept).toStrictEqual([]);
  });

  test("recovers a throttled request", async () => {
    const { options, slept } = harness();
    const statuses = [429, 429, 200];

    let calls = 0;

    const response = await retryRequest(async () => {
      const status = statuses[calls] ?? 200;

      calls += 1;

      return respond(status);
    }, options);

    expect(response.status).toBe(200);
    expect(calls).toBe(3);
    expect(slept).toHaveLength(2);
  });

  test("backs off exponentially", async () => {
    const { options, slept } = harness();

    await retryRequest(async () => respond(503), { ...options, attempts: 4 });

    expect(slept).toStrictEqual([75, 150, 300]);
  });

  test("honours retry-after over backoff", async () => {
    const { options, slept } = harness();
    let calls = 0;

    await retryRequest(async () => {
      calls += 1;

      return calls === 1 ? respond(429, { "retry-after": "2" }) : respond(200);
    }, options);

    expect(slept).toStrictEqual([2000]);
  });

  test("does not retry a client error", async () => {
    const { options, slept } = harness();
    let calls = 0;

    const response = await retryRequest(async () => {
      calls += 1;

      return respond(403);
    }, options);

    expect(response.status).toBe(403);
    expect(calls).toBe(1);
    expect(slept).toStrictEqual([]);
  });

  test("retries a network error and surfaces the last one when exhausted", async () => {
    const { options } = harness();
    let calls = 0;

    await expect(
      retryRequest(
        () => {
          calls += 1;

          throw new Error(`socket hang up ${calls}`);
        },
        { ...options, attempts: 3 },
      ),
    ).rejects.toThrow("socket hang up 3");

    expect(calls).toBe(3);
  });

  test("returns the last response when retries are exhausted", async () => {
    const { options } = harness();
    let calls = 0;

    const response = await retryRequest(
      async () => {
        calls += 1;

        return respond(500);
      },
      { ...options, attempts: 2 },
    );

    expect(response.status).toBe(500);
    expect(calls).toBe(2);
  });
});
