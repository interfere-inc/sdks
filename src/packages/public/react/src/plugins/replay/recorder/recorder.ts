import type { eventWithTime, listenerHandler } from "@rrweb/types";
import type { record as rrwebRecord } from "rrweb";

import { createLogger } from "../../../util/log.js";
import type { ReplayRecordOptions } from "../config.js";

const log = createLogger("replay");

const MOUSEMOVE_SAMPLE_MS = 50;

const SCROLL_SAMPLE_MS = 150;

const MEDIA_SAMPLE_MS = 800;

const CHECKOUT_EVERY_MS = 60_000;

const MUTATION_STORM_RECORD_LIMIT = 10_000;

const MUTATION_STORM_NODE_LIMIT = 100_000;

function isElement(node: Node): node is Element {
  return node.nodeType === Node.ELEMENT_NODE;
}

function countAddedNodes(records: MutationRecord[]) {
  let nodes = 0;

  for (const { addedNodes } of records) {
    for (const node of addedNodes) {
      nodes += 1;

      if (isElement(node)) {
        // oxlint-disable-next-line unicorn/prefer-query-selector -- The live collection counts descendants without materialising a static list inside the observer task this breaker protects.
        nodes += node.getElementsByTagName("*").length;
      }

      if (nodes > MUTATION_STORM_NODE_LIMIT) {
        return nodes;
      }
    }
  }

  return nodes;
}

function describeMutationStorm(records: MutationRecord[]) {
  if (records.length > MUTATION_STORM_RECORD_LIMIT) {
    return `${records.length} mutations`;
  }

  const nodes = countAddedNodes(records);

  return nodes > MUTATION_STORM_NODE_LIMIT ? `${nodes} added nodes` : undefined;
}

export interface ReplayRecorder {
  addCustomEvent: (tag: string, payload: Record<string, string>) => void;
  start: () => boolean;
  stop: () => void;
  takeFullSnapshot: () => void;
}

export interface StartReplayRecorderOptions {
  onEvent: (event: eventWithTime, isCheckout?: boolean) => void;
  // oxlint-disable-next-line anti-slop/no-unknown-parameters -- rrweb forwards arbitrary thrown values to its fault callback, including non-Error rejections.
  onFault: (error: unknown) => void;
  onMutationStorm: (detail: string) => void;
  recordOptions: ReplayRecordOptions;
  signal?: AbortSignal;
}

export async function startReplayRecorder({
  onEvent,
  onFault,
  onMutationStorm,
  recordOptions,
  signal,
}: StartReplayRecorderOptions) {
  let record: typeof rrwebRecord;

  try {
    ({ record } = await import("rrweb"));
  } catch {
    log.error("rrweb failed to load, replay disabled");

    return null;
  }

  if (signal?.aborted) {
    return null;
  }

  let stopRecording: listenerHandler | undefined;
  let stormObserver: MutationObserver | undefined;

  const recorder: ReplayRecorder = {
    addCustomEvent: (tag, payload) => {
      if (stopRecording) {
        record.addCustomEvent(tag, payload);
      }
    },
    start: () => {
      if (stopRecording) {
        return true;
      }

      // WARNING: This observer must be created before rrweb's so it sees each delivery first; stopping rrweb here drops the records rrweb would otherwise serialise in one long task.
      stormObserver = new MutationObserver((records) => {
        const detail = describeMutationStorm(records);

        if (detail) {
          recorder.stop();

          onMutationStorm(detail);
        }
      });

      stormObserver.observe(document, {
        attributes: true,
        characterData: true,
        childList: true,
        subtree: true,
      });

      try {
        stopRecording = record({
          emit: onEvent,
          recordAfter: "DOMContentLoaded",
          inlineStylesheet: true,
          slimDOMOptions: "all",
          checkoutEveryNms: CHECKOUT_EVERY_MS,
          sampling: {
            media: MEDIA_SAMPLE_MS,
            mousemove: MOUSEMOVE_SAMPLE_MS,
            scroll: SCROLL_SAMPLE_MS,
          },
          ...recordOptions,
          errorHandler: (error) => {
            onFault(error);

            return true;
          },
        });
      } finally {
        if (!stopRecording) {
          stormObserver.disconnect();

          stormObserver = undefined;
        }
      }

      return stopRecording !== undefined;
    },
    stop: () => {
      stormObserver?.disconnect();

      stormObserver = undefined;

      stopRecording?.();

      stopRecording = undefined;
    },
    takeFullSnapshot: () => {
      if (stopRecording) {
        record.takeFullSnapshot(true);
      }
    },
  };

  if (!recorder.start()) {
    log.error("rrweb failed to start, replay disabled");

    return null;
  }

  return recorder;
}
