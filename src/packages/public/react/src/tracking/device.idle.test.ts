import { whenIdle } from "./device.js";
import { expect, test, vi } from "vitest";

test("defers fingerprint work to a real timer when the host has no idle API", async () => {
  expect(globalThis.requestIdleCallback).toBeUndefined();

  const schedule = vi.spyOn(globalThis, "setTimeout");
  const idle = whenIdle();

  expect(schedule).toHaveBeenCalledExactlyOnceWith(expect.any(Function), 0);

  let resolved = false;

  async function observeIdle() {
    await idle;

    resolved = true;
  }

  const observed = observeIdle();

  await Promise.resolve();

  expect(resolved).toBe(false);

  await expect(idle).resolves.toBeUndefined();

  await observed;

  expect(resolved).toBe(true);
});
