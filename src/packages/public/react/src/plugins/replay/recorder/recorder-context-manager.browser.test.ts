import { EventType } from "@rrweb/types";
import type { eventWithTime } from "@rrweb/types";
import { expect, test, vi } from "vitest";

import { StackContextManager } from "@opentelemetry/sdk-trace-web";
import { toRecordOptions } from "../config.js";
import { startReplayRecorder } from "./recorder.js";

test("records snapshots and mutations under the stack context manager using at most two temporary frames", async () => {
  expect.hasAssertions();

  const manager = new StackContextManager().enable();
  const events: eventWithTime[] = [];
  const onFault = vi.fn<Parameters<typeof startReplayRecorder>[0]["onFault"]>();
  const host = document.createElement("div");
  const shadow = host.attachShadow({ mode: "open" });
  const marker = document.createElement("p");

  marker.textContent = "initial shadow content";

  shadow.append(marker);

  document.body.append(host);

  await import("rrweb");

  const appended = vi.spyOn(document.body, "appendChild");

  const recorder = await startReplayRecorder({
    onEvent: (event) => events.push(event),
    onFault,
    onMutationStorm: onFault,
    recordOptions: toRecordOptions({}),
  });

  try {
    expect(recorder).not.toBeNull();

    expect(events.some((event) => event.type === EventType.FullSnapshot)).toBe(
      true,
    );

    expect(JSON.stringify(events)).toContain("initial shadow content");

    const frames = appended.mock.calls
      .map(([node]) => node)
      .filter((node) => node instanceof HTMLIFrameElement);

    expect(frames.length).toBeGreaterThan(0);
    expect(frames.length).toBeLessThanOrEqual(2);
    expect(frames.every((frame) => !frame.isConnected)).toBe(true);

    marker.textContent = "first recorded mutation";

    await vi.waitFor(() =>
      expect(JSON.stringify(events)).toContain("first recorded mutation"),
    );

    recorder?.stop();

    events.length = 0;

    recorder?.start();

    expect(
      events.filter((event) => event.type === EventType.FullSnapshot),
    ).toHaveLength(1);

    marker.textContent = "mutation after resume";

    await vi.waitFor(() =>
      expect(JSON.stringify(events)).toContain("mutation after resume"),
    );

    expect(onFault).not.toHaveBeenCalled();
  } finally {
    recorder?.stop();

    appended.mockRestore();

    manager.disable();

    host.remove();
  }
});
