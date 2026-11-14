/* oxlint-disable typescript/require-await */
import { afterEach, describe, expect, test, vi } from "vitest";
import { withDeadline } from "./with-deadline.js";

afterEach(() => {
  vi.useRealTimers();
});

describe(withDeadline, () => {
  test("clears its timer once the operation resolves", async () => {
    vi.useFakeTimers();

    await expect(withDeadline(30_000, async () => "done")).resolves.toBe(
      "done",
    );

    expect(vi.getTimerCount()).toBe(0);
  });

  test("clears its timer once the operation rejects", async () => {
    vi.useFakeTimers();

    await expect(
      withDeadline(30_000, async () => {
        throw new Error("boom");
      }),
    ).rejects.toThrow("boom");

    expect(vi.getTimerCount()).toBe(0);
  });

  test("aborts a slow operation with a TimeoutError carrying the message", async () => {
    await expect(
      withDeadline(
        20,
        async (signal) =>
          new Promise<never>((_resolve, reject) => {
            signal.addEventListener("abort", () => {
              reject(
                signal.reason instanceof Error
                  ? signal.reason
                  : new Error("aborted"),
              );
            });
          }),
        "Dispatch exceeded its deadline",
      ),
    ).rejects.toMatchObject({
      name: "TimeoutError",
      message: "Dispatch exceeded its deadline",
    });
  });
});
