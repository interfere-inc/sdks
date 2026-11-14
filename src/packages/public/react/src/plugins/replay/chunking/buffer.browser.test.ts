/* oxlint-disable unicorn/prefer-single-call */
import { EventType } from "@rrweb/types";
import type { eventWithTime } from "@rrweb/types";

import { createReplayBuffer } from "./buffer.js";
import { describe, expect, test, vi } from "vitest";

describe("plugins/replay/buffer", () => {
  test("accumulates events and drains them in order", () => {
    const buffer = createReplayBuffer({
      onSoftLimit: vi.fn<() => void>(),
      softLimitBytes: 1024,
    });

    const first = {
      type: EventType.Custom,
      timestamp: 1,
      data: { tag: "click", payload: {} },
    } satisfies eventWithTime;

    const second = {
      type: EventType.Custom,
      timestamp: 2,
      data: { tag: "scroll", payload: {} },
    } satisfies eventWithTime;

    buffer.push(first);

    buffer.push(second);

    const drained = buffer.drain();

    expect(drained.map((entry) => entry.json)).toStrictEqual([
      JSON.stringify(first),
      JSON.stringify(second),
    ]);

    expect(drained[0]?.bytes).toBe(JSON.stringify(first).length);
    expect(buffer.drain()).toStrictEqual([]);
  });

  test("accounts bytes in utf-8, not code units", () => {
    const buffer = createReplayBuffer({
      onSoftLimit: vi.fn<() => void>(),
      softLimitBytes: 1024,
    });

    const event = {
      type: EventType.Custom,
      timestamp: 1,
      data: { tag: "emoji", payload: "😀" },
    } satisfies eventWithTime;

    buffer.push(event);

    const drained = buffer.drain();

    expect(drained[0]?.bytes).toBe(
      new TextEncoder().encode(JSON.stringify(event)).length,
    );
  });

  test("fires onSoftLimit once the accumulated bytes cross the threshold", () => {
    const onSoftLimit = vi.fn<() => void>();

    const event = {
      type: EventType.Custom,
      timestamp: 1,
      data: { tag: "payload", payload: "small" },
    } satisfies eventWithTime;

    const buffer = createReplayBuffer({
      onSoftLimit,
      softLimitBytes: JSON.stringify(event).length + 1,
    });

    buffer.push(event);

    expect(onSoftLimit).not.toHaveBeenCalled();

    buffer.push(event);

    expect(onSoftLimit).toHaveBeenCalledTimes(1);
  });

  test("resets byte accounting on drain", () => {
    const onSoftLimit = vi.fn<() => void>();

    const event = {
      type: EventType.Custom,
      timestamp: 1,
      data: { tag: "payload", payload: "0123456789012345" },
    } satisfies eventWithTime;

    const buffer = createReplayBuffer({
      onSoftLimit,
      softLimitBytes: JSON.stringify(event).length + 1,
    });

    buffer.push(event);

    buffer.drain();

    buffer.push(event);

    expect(onSoftLimit).not.toHaveBeenCalled();
  });
});
