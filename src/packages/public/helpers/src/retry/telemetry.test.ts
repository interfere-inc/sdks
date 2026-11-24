/* oxlint-disable typescript/require-await */
import { afterEach, beforeEach, expect, test, vi } from "vitest";
import { retryTelemetry } from "./telemetry";

beforeEach(() => {
  vi.useFakeTimers();

  vi.setSystemTime(new Date("2026-10-23T09:00:00Z"));
});

afterEach(() => {
  vi.useRealTimers();

  vi.restoreAllMocks();
});

function withoutSignalAny() {
  const descriptor = Object.getOwnPropertyDescriptor(AbortSignal, "any");

  // oxlint-disable-next-line landmines/no-test-doubles -- Exercise the same missing platform API as the isolated Edge VM while using real AbortControllers.
  Object.defineProperty(AbortSignal, "any", {
    configurable: true,
    value: undefined,
  });

  return () => {
    if (descriptor) {
      // oxlint-disable-next-line landmines/no-test-doubles -- Restore the host capability removed by this scoped fixture.
      Object.defineProperty(AbortSignal, "any", descriptor);
    }
  };
}

test("delivers with a parent signal when AbortSignal.any is unavailable", async () => {
  const restore = withoutSignalAny();
  const parent = new AbortController();
  const state = responses([200]);

  try {
    await expect(
      retryTelemetry(state.send, parent.signal),
    ).resolves.toHaveProperty("status", 200);

    expect(state.sent).toHaveLength(1);
    expect(vi.getTimerCount()).toBe(0);
  } finally {
    restore();

    vi.clearAllTimers();
  }
});

test("preserves a pre-aborted parent's reason without AbortSignal.any", async () => {
  const restore = withoutSignalAny();
  const parent = new AbortController();
  const reason = new Error("caller cancelled before export");
  const state = responses([200]);

  parent.abort(reason);

  try {
    await expect(retryTelemetry(state.send, parent.signal)).rejects.toBe(
      reason,
    );

    expect(state.sent).toHaveLength(0);
    expect(vi.getTimerCount()).toBe(0);
  } finally {
    restore();

    vi.clearAllTimers();
  }
});

test("forwards parent cancellation to a pending send without AbortSignal.any", async () => {
  const restore = withoutSignalAny();
  const parent = new AbortController();
  const reason = new Error("caller cancelled pending export");
  const removed = vi.spyOn(parent.signal, "removeEventListener");

  const send = async (signal: AbortSignal) =>
    new Promise<Response>((_resolve, reject) => {
      signal.addEventListener(
        "abort",
        () => {
          try {
            signal.throwIfAborted();
          } catch (error) {
            reject(error instanceof Error ? error : new Error(String(error)));
          }
        },
        {
          once: true,
        },
      );
    });

  try {
    const result = retryTelemetry(send, parent.signal);
    const settled = Promise.allSettled([result]);

    parent.abort(reason);

    await expect(settled).resolves.toMatchObject([
      { status: "rejected", reason },
    ]);

    expect(removed).toHaveBeenCalledWith("abort", expect.any(Function));
    expect(vi.getTimerCount()).toBe(0);
  } finally {
    restore();

    vi.clearAllTimers();
  }
});

test("keeps the export deadline and releases the parent listener without AbortSignal.any", async () => {
  const restore = withoutSignalAny();
  const parent = new AbortController();
  const removed = vi.spyOn(parent.signal, "removeEventListener");

  const send = async (signal: AbortSignal) =>
    new Promise<Response>((_resolve, reject) => {
      signal.addEventListener(
        "abort",
        () => {
          try {
            signal.throwIfAborted();
          } catch (error) {
            reject(error instanceof Error ? error : new Error(String(error)));
          }
        },
        {
          once: true,
        },
      );
    });

  try {
    const settled = Promise.allSettled([retryTelemetry(send, parent.signal)]);

    await vi.advanceTimersByTimeAsync(65_000);

    await expect(settled).resolves.toMatchObject([
      {
        status: "rejected",
        reason: { message: "Telemetry export deadline exceeded" },
      },
    ]);

    expect(parent.signal.aborted).toBe(false);
    expect(removed).toHaveBeenCalledWith("abort", expect.any(Function));
    expect(vi.getTimerCount()).toBe(0);
  } finally {
    restore();

    vi.clearAllTimers();
  }
});

function responses(statuses: number[], retryAfter?: string) {
  const sent: number[] = [];

  return {
    sent,
    send: async () => {
      const status = statuses[sent.length] ?? statuses.at(-1) ?? 200;

      sent.push(Date.now());

      return new Response(null, {
        status,
        headers: retryAfter === undefined ? {} : { "Retry-After": retryAfter },
      });
    },
  };
}

test.each([429, 500, 502, 503, 504])(
  "honors Retry-After with bounded positive jitter for %s",
  async (status) => {
    const state = responses([status, 200], "30");
    const result = retryTelemetry(state.send);

    await vi.advanceTimersByTimeAsync(29_999);

    expect(state.sent).toHaveLength(1);

    await vi.advanceTimersByTimeAsync(1001);

    await expect(result).resolves.toHaveProperty("status", 200);

    expect(state.sent).toHaveLength(2);

    expect((state.sent[1] ?? 0) - (state.sent[0] ?? 0)).toBeGreaterThanOrEqual(
      30_000,
    );

    expect((state.sent[1] ?? 0) - (state.sent[0] ?? 0)).toBeLessThan(31_000);
  },
);

test("accepts an HTTP-date Retry-After", async () => {
  const state = responses([503, 200], "Fri, 23 Oct 2026 09:00:30 GMT");
  const result = retryTelemetry(state.send);

  await vi.advanceTimersByTimeAsync(29_999);

  expect(state.sent).toHaveLength(1);

  await vi.advanceTimersByTimeAsync(1001);

  await expect(result).resolves.toHaveProperty("status", 200);
});

test.each([400, 401, 403, 408, 422, 425, 501, 505])(
  "does not retry %s",
  async (status) => {
    const state = responses([status]);

    await expect(retryTelemetry(state.send)).resolves.toHaveProperty(
      "status",
      status,
    );

    expect(state.sent).toHaveLength(1);
  },
);

test("stops after three attempts", async () => {
  const state = responses([503]);
  const result = retryTelemetry(state.send);

  await vi.runAllTimersAsync();

  await expect(result).resolves.toHaveProperty("status", 503);

  expect(state.sent).toHaveLength(3);
});

test("does not shorten Retry-After to fit the remaining budget", async () => {
  const state = responses([503], "40");
  const result = retryTelemetry(state.send);

  await vi.runAllTimersAsync();

  await expect(result).resolves.toHaveProperty("status", 503);

  expect(state.sent).toHaveLength(2);
});

test("aborts waiting when the caller cancels", async () => {
  const controller = new AbortController();
  const state = responses([503], "30");
  const result = retryTelemetry(state.send, controller.signal);
  const settled = Promise.allSettled([result]);

  await vi.advanceTimersByTimeAsync(100);

  controller.abort();

  await expect(settled).resolves.toMatchObject([
    { status: "rejected", reason: expect.any(Error) as Error },
  ]);

  expect(state.sent).toHaveLength(1);
  expect(vi.getTimerCount()).toBe(0);
});

test("aborts an outstanding fetch at the total deadline", async () => {
  const result = retryTelemetry(
    async (signal) =>
      new Promise<Response>((_resolve, reject) => {
        signal.addEventListener("abort", () => reject(new Error("aborted")), {
          once: true,
        });
      }),
  );

  const settled = Promise.allSettled([result]);

  await vi.advanceTimersByTimeAsync(65_000);

  await expect(settled).resolves.toMatchObject([
    { status: "rejected", reason: expect.any(Error) as Error },
  ]);

  expect(vi.getTimerCount()).toBe(0);
});

test.each(["", " ", "-1", "-30", "+30", "1.5", "invalid", "Infinity"])(
  "uses the 30-second fallback for invalid Retry-After %j",
  async (header) => {
    const state = responses([503, 200], header);
    const result = retryTelemetry(state.send);

    await vi.advanceTimersByTimeAsync(29_999);

    expect(state.sent).toHaveLength(1);

    await vi.advanceTimersByTimeAsync(1001);

    await expect(result).resolves.toHaveProperty("status", 200);

    expect(state.sent).toHaveLength(2);
  },
);

test("accepts an explicit zero Retry-After", async () => {
  const state = responses([503, 200], "0");
  const result = retryTelemetry(state.send);

  await vi.advanceTimersByTimeAsync(1000);

  await expect(result).resolves.toHaveProperty("status", 200);

  expect(state.sent).toHaveLength(2);
});
