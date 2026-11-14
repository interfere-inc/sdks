import { ROOT_CONTEXT } from "@opentelemetry/api";
import { afterEach, expect, test, vi } from "vitest";

import { StartupExceptionBuffer } from "./startup-buffer.js";
import type { PendingException } from "./startup-buffer.js";

afterEach(() => {
  vi.useRealTimers();
});

function entry(index: number) {
  return {
    attrs: {},
    handled: true,
    origin: { at: index, context: ROOT_CONTEXT },
    value: new Error(`held ${String(index)}`),
  } satisfies PendingException;
}

function setup() {
  const delivered: number[][] = [];

  const buffer = new StartupExceptionBuffer((pending) => {
    delivered.push(pending.map(({ origin }) => origin.at));
  });

  return { buffer, delivered };
}

test("does not buffer before hold or after release", () => {
  const { buffer, delivered } = setup();

  expect(buffer.push(entry(0))).toBe(false);

  buffer.hold();

  buffer.release(false);

  expect(buffer.holding).toBe(false);
  expect(buffer.push(entry(1))).toBe(false);
  expect(delivered).toStrictEqual([[]]);
});

test("caps the startup buffer while no release has been requested", () => {
  expect.hasAssertions();

  const { buffer, delivered } = setup();

  buffer.hold();

  for (let index = 0; index < 60; index += 1) {
    expect(buffer.push(entry(index))).toBe(true);
  }

  buffer.release(false);

  expect(delivered[0]).toHaveLength(50);
});

test("delivers a full deferred buffer before buffering more", () => {
  const { buffer, delivered } = setup();

  buffer.hold();

  buffer.release(true);

  for (let index = 0; index < 51; index += 1) {
    buffer.push(entry(index));
  }

  expect(delivered).toHaveLength(1);
  expect(delivered[0]).toHaveLength(50);
  expect(buffer.holding).toBe(false);
});

test("delivers a deferred release after the grace period", () => {
  vi.useFakeTimers();

  const { buffer, delivered } = setup();

  buffer.hold();

  buffer.push(entry(0));

  buffer.release(true);

  buffer.release(true);

  buffer.releaseDeferred();

  buffer.hold();

  buffer.push(entry(1));

  buffer.release(true);

  vi.advanceTimersByTime(4999);

  expect(delivered).toStrictEqual([[0]]);

  vi.advanceTimersByTime(1);

  expect(delivered).toStrictEqual([[0], [1]]);
});

test("ignores releaseDeferred without a deferred release and discards on demand", () => {
  vi.useFakeTimers();

  const { buffer, delivered } = setup();

  buffer.hold();

  buffer.push(entry(0));

  buffer.releaseDeferred();

  expect(buffer.holding).toBe(true);

  buffer.release(true);

  buffer.discard();

  vi.runAllTimers();

  expect(delivered).toStrictEqual([]);
});
