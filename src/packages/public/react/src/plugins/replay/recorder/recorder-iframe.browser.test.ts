import { EventType, IncrementalSource } from "@rrweb/types";
import type { eventWithTime } from "@rrweb/types";

import { expect, onTestFinished, test, vi } from "vitest";

import { toRecordOptions } from "../config.js";
import { startReplayRecorder } from "./recorder.js";

test("keeps recording when a measurement iframe has no Element constructor", async () => {
  const events: eventWithTime[] = [];
  const onFault = vi.fn<Parameters<typeof startReplayRecorder>[0]["onFault"]>();

  const recorder = await startReplayRecorder({
    onEvent: (event) => events.push(event),
    onFault,
    onMutationStorm: onFault,
    recordOptions: toRecordOptions({}),
  });

  const iframe = document.createElement("iframe");
  const marker = document.createElement("p");

  onTestFinished(() => {
    recorder?.stop();

    iframe.remove();

    marker.remove();
  });

  document.body.append(iframe);

  const iframeWindow = iframe.contentWindow;

  if (!iframeWindow) {
    throw new Error("The fixture iframe has no window");
  }

  Object.defineProperty(iframeWindow, "Element", {
    configurable: true,
    value: undefined,
    writable: true,
  });

  await vi.waitUntil(() =>
    events.some(
      (event) =>
        event.type === EventType.IncrementalSnapshot &&
        event.data.source === IncrementalSource.Mutation &&
        event.data.isAttachIframe,
    ),
  );

  marker.textContent = "main document still records";

  document.body.append(marker);

  await vi.waitUntil(() =>
    JSON.stringify(events).includes("main document still records"),
  );

  expect(onFault).not.toHaveBeenCalled();
});

test("records same-origin iframe shadow roots and restores instrumentation", async () => {
  const iframe = document.createElement("iframe");

  document.body.append(iframe);

  onTestFinished(() => iframe.remove());

  const iframeDocument = iframe.contentDocument;

  if (!iframeDocument) {
    throw new Error("The fixture iframe has no document");
  }

  const host = iframeDocument.createElement("div");
  // oxlint-disable-next-line typescript/unbound-method -- Compare method identity without invoking it.
  const originalAttachShadow = host.attachShadow;
  const events: eventWithTime[] = [];
  const onFault = vi.fn<Parameters<typeof startReplayRecorder>[0]["onFault"]>();

  const recorder = await startReplayRecorder({
    onEvent: (event) => events.push(event),
    onFault,
    onMutationStorm: onFault,
    recordOptions: toRecordOptions({}),
  });

  onTestFinished(() => recorder?.stop());

  await vi.waitUntil(() => host.attachShadow !== originalAttachShadow);

  iframeDocument.body.append(host);

  const shadow = host.attachShadow({ mode: "open" });
  const marker = iframeDocument.createElement("p");

  marker.textContent = "iframe shadow content";

  shadow.append(marker);

  await vi.waitUntil(() =>
    JSON.stringify(events).includes("iframe shadow content"),
  );

  recorder?.stop();

  // oxlint-disable-next-line typescript/unbound-method -- Assert the original method was restored without invoking it.
  expect(host.attachShadow).toBe(originalAttachShadow);
  expect(onFault).not.toHaveBeenCalled();

  const count = events.length;

  marker.textContent = "after cleanup";

  await new Promise<void>((resolve) => {
    requestAnimationFrame(() => resolve());
  });

  expect(events).toHaveLength(count);
});
