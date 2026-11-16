import { EventType, IncrementalSource } from "@rrweb/types";
import type { eventWithTime } from "@rrweb/types";

import { toRecordOptions } from "../config.js";
import { startReplayRecorder } from "./recorder.js";
import { expect, onTestFinished, test, vi } from "vitest";

interface Emitted {
  event: eventWithTime;
  isCheckout: boolean | undefined;
}

function fullSnapshots(emitted: Emitted[]) {
  return emitted.filter(({ event }) => event.type === EventType.FullSnapshot);
}

function mutations(emitted: Emitted[]) {
  return emitted.filter(
    ({ event }) =>
      event.type === EventType.IncrementalSnapshot &&
      event.data.source === IncrementalSource.Mutation,
  );
}

test("a recording older than five minutes checks out a fresh full snapshot on the next change", async () => {
  expect.hasAssertions();

  // WARNING: rrweb captures Date.now when its module loads, so the fake clock must exist before startReplayRecorder imports rrweb; keep this test in its own file.
  vi.useFakeTimers({ toFake: ["Date"] });

  onTestFinished(() => {
    vi.useRealTimers();
  });

  const emitted: Emitted[] = [];
  const onFault = vi.fn<Parameters<typeof startReplayRecorder>[0]["onFault"]>();
  const element = document.createElement("p");

  element.textContent = "first";

  document.body.append(element);

  const recorder = await startReplayRecorder({
    onEvent: (event, isCheckout) => emitted.push({ event, isCheckout }),
    onFault,
    onMutationStorm: onFault,
    recordOptions: toRecordOptions({}),
  });

  onTestFinished(() => {
    recorder?.stop();

    element.remove();
  });

  expect(recorder).not.toBeNull();

  await vi.waitFor(() => {
    expect(fullSnapshots(emitted)).toHaveLength(1);
  });

  const started = Date.now();

  vi.setSystemTime(started + 4 * 60_000);

  element.textContent = "four minutes in";

  await vi.waitFor(() => {
    expect(mutations(emitted).length).toBeGreaterThan(0);
  });

  expect(fullSnapshots(emitted)).toHaveLength(1);

  vi.setSystemTime(started + 5 * 60_000 + 1000);

  element.textContent = "five minutes in";

  await vi.waitFor(() => {
    expect(fullSnapshots(emitted)).toHaveLength(2);
  });

  const [, checkout] = fullSnapshots(emitted);

  expect(checkout?.isCheckout).toBe(true);
  expect(checkout?.event.timestamp).toBeGreaterThan(started + 5 * 60_000);
  expect(onFault).not.toHaveBeenCalled();
});
