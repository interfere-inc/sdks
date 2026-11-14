import { EventType } from "@rrweb/types";
import type { eventWithTime } from "@rrweb/types";

import { toRecordOptions } from "../config.js";
import { startReplayRecorder } from "./recorder.js";
import { expect, test, vi } from "vitest";

test("records native DOM snapshots, restarts with a fresh snapshot, and stops observing", async () => {
  expect.hasAssertions();

  const events: eventWithTime[] = [];
  const onFault = vi.fn<Parameters<typeof startReplayRecorder>[0]["onFault"]>();
  const element = document.createElement("button");

  element.textContent = "recorded button";

  document.body.append(element);

  const recorder = await startReplayRecorder({
    onEvent: (event) => events.push(event),
    onFault,
    onMutationStorm: onFault,
    recordOptions: toRecordOptions({}),
  });

  try {
    expect(recorder).not.toBeNull();

    await vi.waitFor(() => {
      expect(
        events.some((event) => event.type === EventType.FullSnapshot),
      ).toBe(true);
    });

    recorder?.stop();

    recorder?.start();

    expect(
      events.filter((event) => event.type === EventType.FullSnapshot),
    ).toHaveLength(2);

    recorder?.stop();

    const count = events.length;

    element.textContent = "after cleanup";

    await new Promise<void>((resolve) => {
      requestAnimationFrame(() => resolve());
    });

    expect(events).toHaveLength(count);
    expect(onFault).not.toHaveBeenCalled();
  } finally {
    recorder?.stop();

    element.remove();
  }
});
