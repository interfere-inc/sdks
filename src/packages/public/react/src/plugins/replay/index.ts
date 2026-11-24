/* oxlint-disable import/no-default-export */
import { omitUndefined } from "@interfere/helpers/omit-undefined";

import { EventType } from "@rrweb/types";

import { createReplayCarryover } from "./upload/carryover.js";
import { clearExportCarryover } from "../../internal/otel/export/carryover.js";
import { resolveTargets } from "../../internal/config/config.js";
import type { SessionPauseReason } from "../../internal/activity/activity.js";
import { onPageUnloading } from "../../internal/page-lifecycle/page-lifecycle.js";
import { createLogger } from "../../util/log.js";
import type { Plugin } from "../lib/types.js";
import { createAnonymousReplayGate } from "./anonymous-gate.js";
import {
  capPendingReplayBuffer,
  createReplayBuffer,
} from "./chunking/buffer.js";
import {
  splitReplayEvents,
  toReplayChunkPayload,
} from "./chunking/chunking.js";
import { createCompressor } from "./compression/compressor-host.js";
import { toRecordOptions } from "./config.js";
import { REPLAY_TRACER_NAME, deliverChunk } from "./upload/delivery.js";
import type { ChunkTrigger, DeliverResult } from "./upload/delivery.js";
import { createReplayRetryQueue } from "./upload/retry-queue.js";
import type { PendingReplayChunk } from "./upload/retry-queue.js";
import { startReplayRecorder } from "./recorder/recorder.js";
import { redactReplayEvent } from "./recorder/redact-urls.js";
import type { ReplayRecorder } from "./recorder/recorder.js";
import { createSegmentSequence } from "./sequence/sequence.js";

const log = createLogger("replay");

// WARNING: The first flush moves the full snapshot out while the page is alive; a bounce session shorter than this has to carry the snapshot through the unload path.
const FIRST_FLUSH_DELAY_MS = 2000;

const FLUSH_INTERVAL_MS = 10_000;

const SOFT_FLUSH_BYTES = 4 * 1024 * 1024;

const MAX_RECORDER_FAULTS = 10;

const PAUSED_SNAPSHOT_TAG = "interfere.paused-exception-snapshot";

const STOPPED_TAG = "interfere.replay-stopped";

interface FlushOptions {
  trigger?: ChunkTrigger;
  unload?: boolean;
  retryPending?: boolean;
}

type ReplayStopReason = SessionPauseReason | ReplayHaltReason | "unload";

type ReplayHaltReason =
  | "init-failed"
  | "mutation-storm"
  | "recorder-faults"
  | "upload-rejected";

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- rrweb fault callbacks carry arbitrary thrown values, including DOMExceptions from other realms.
function isBrowserApiRejection(error: unknown) {
  return Object.prototype.toString.call(error) === "[object DOMException]";
}

const replayPlugin: Plugin = {
  name: "replay",

  setup(ctx) {
    const targets = resolveTargets(undefined, undefined, ctx.serviceIdentity);

    let recorder: ReplayRecorder | null = null;
    let flushTimer: ReturnType<typeof setInterval> | null = null;
    let unsubscribeConfig: (() => void) | null = null;
    let unsubscribeUnloading: (() => void) | null = null;
    let unsubscribePause: (() => void) | null = null;
    let unsubscribeResume: (() => void) | null = null;
    let unsubscribeException: (() => void) | null = null;
    let stopAnonymousGate: (() => void) | null = null;
    let isPaused = false;
    let stopMarked = false;
    let awaitingKeyframe = false;

    const startup = new AbortController();
    const consent = new AbortController();
    const compressor = createCompressor();
    const windowId = ctx.getWindowId();
    const sequence = createSegmentSequence(windowId);

    const retryQueue = createReplayRetryQueue({
      onKeyframeLost: requestKeyframe,
    });

    let recorderFaults = 0;
    let halted = false;

    const carryover = createReplayCarryover(
      targets.ingest,
      () =>
        !halted &&
        !startup.signal.aborted &&
        ctx.getPluginConfigState("replay") === "enabled",
    );

    function halt(reason: ReplayHaltReason, detail?: string) {
      if (halted) {
        return;
      }

      halted = true;

      ctx
        .getTracer(REPLAY_TRACER_NAME)
        .startSpan("replay.halt", {
          attributes: omitUndefined({
            "replay.halt.reason": reason,
            "replay.halt.detail": detail,
            "replay.halt.recorder_faults": recorderFaults,
          }),
        })
        .end();

      retryQueue.clear();

      startup.abort();

      carryover.dispose();

      clearFlushTimer();

      stopAnonymousGate?.();

      recorder?.stop();

      recorder = null;
    }

    function clearFlushTimer() {
      if (flushTimer) {
        clearInterval(flushTimer);

        flushTimer = null;
      }
    }

    function armFlushTimer() {
      clearFlushTimer();

      flushTimer = setInterval(() => {
        clearFlushTimer();

        flushTimer = setInterval(
          () => flush({ retryPending: true }),
          FLUSH_INTERVAL_MS,
        );

        flush();
      }, FIRST_FLUSH_DELAY_MS);
    }

    function handleResult(result: DeliverResult, chunk: PendingReplayChunk) {
      if (result === "retryable" && !startup.signal.aborted) {
        retryQueue.push(chunk);
      } else if (result === "skipped") {
        log.warn("replay chunk rejected as a segment conflict; skipping it");
      } else if (result === "fatal") {
        log.warn("replay halted: upload rejected");

        halt("upload-rejected");
      }
    }

    // oxlint-disable-next-line anti-slop/no-unknown-parameters -- rrweb fault callbacks can carry arbitrary thrown values, which are logged without assuming Error instances.
    function handleRecorderFault(error: unknown) {
      if (isBrowserApiRejection(error)) {
        log.debug(
          "recorder passed through a browser API rejection: %s",
          error instanceof Error ? error.name : "DOMException",
        );

        return;
      }

      recorderFaults += 1;

      if (recorderFaults > MAX_RECORDER_FAULTS) {
        return;
      }

      log.warn(
        "recorder fault: %s",
        error instanceof Error ? error.constructor.name : "UnknownError",
      );

      if (recorderFaults === MAX_RECORDER_FAULTS) {
        log.warn("replay halted after %d recorder faults", MAX_RECORDER_FAULTS);

        markStopped("recorder-faults");

        flush();

        halt(
          "recorder-faults",
          error instanceof Error ? error.constructor.name : "UnknownError",
        );
      }
    }

    const buffer = createReplayBuffer({
      onSoftLimit: flush,
      softLimitBytes: SOFT_FLUSH_BYTES,
    });

    const anonymousGate = createAnonymousReplayGate({
      buffer,
      ctx,
      flush,
      getRecorder: () => recorder,
      isHalted: () => halted,
      isPaused: () => isPaused,
      softLimitBytes: SOFT_FLUSH_BYTES,
    });

    function requestKeyframe() {
      awaitingKeyframe = true;

      recorder?.takeFullSnapshot();
    }

    function markStopped(reason: ReplayStopReason) {
      if (stopMarked) {
        return;
      }

      stopMarked = true;

      buffer.push({
        type: EventType.Custom,
        data: { tag: STOPPED_TAG, payload: { reason } },
        timestamp: Date.now(),
      });
    }

    function flush(opts?: FlushOptions) {
      const state = ctx.getPluginConfigState("replay");

      if (!halted && state === "pending") {
        awaitingKeyframe ||= capPendingReplayBuffer(buffer, SOFT_FLUSH_BYTES);
      }

      if (halted || state !== "enabled" || anonymousGate.holdsUploads()) {
        return;
      }

      if (opts?.retryPending) {
        for (const chunk of retryQueue.drain()) {
          void sendChunk(chunk, opts.unload ?? false);
        }
      }

      const drained = buffer.drain();
      const sessionId = ctx.getSessionId();

      if (drained.length === 0 || !sessionId) {
        return;
      }

      const split = splitReplayEvents(drained, awaitingKeyframe);

      ({ awaitingKeyframe } = split);

      for (const bytes of split.droppedEventBytes) {
        log.warn(
          "dropping replay event of ~%d bytes: too large to upload",
          bytes,
        );
      }

      for (const group of split.chunks) {
        void sendChunk(
          {
            payload: toReplayChunkPayload(group),
            chunk: { segmentId: sequence.take(), windowId },
            admission: anonymousGate.admission(sessionId),
            sessionId,
            trigger: opts?.trigger,
          },
          opts?.unload ?? false,
        );
      }

      if (split.requestKeyframe) {
        requestKeyframe();
      }
    }

    // WARNING: Unload uploads bypass the worker compressor (its replies never arrive) and ask for keepalive, which Chromium grants only within a 64 KiB shared body budget; delivery gzips an oversized body synchronously to fit.
    async function sendChunk(pending: PendingReplayChunk, unload: boolean) {
      try {
        const result = await deliverChunk({
          target: targets.ingest,
          carryover,
          ...pending,
          compressor,
          ctx,
          signal: consent.signal,
          unload,
        });

        handleResult(result, pending);
      } catch (error) {
        log.warn(
          "replay chunk upload failed: %s",
          error instanceof Error ? error.constructor.name : "UnknownError",
        );

        handleResult("retryable", pending);
      }
    }

    const pauseRecording = (reason: SessionPauseReason) => {
      if (isPaused) {
        return;
      }

      isPaused = true;

      recorder?.stop();

      markStopped(reason);

      flush(reason === "hidden" ? { unload: true } : undefined);
    };

    // WARNING: A hidden or idle page keeps its session id and can keep throwing for hours; without this snapshot those failures have no replay evidence.
    const snapshotWhilePaused = (reason: SessionPauseReason) => {
      if (
        !isPaused ||
        halted ||
        !anonymousGate.allowsRecording() ||
        !recorder?.start()
      ) {
        return;
      }

      recorder.addCustomEvent(PAUSED_SNAPSHOT_TAG, { reason });

      recorder.stop();

      flush({ trigger: "exception" });
    };

    const resumeRecording = () => {
      if (!isPaused) {
        return;
      }

      isPaused = false;

      anonymousGate.syncRecording();

      armFlushTimer();
    };

    const recordOptions = toRecordOptions(
      ctx.getReplayConfig(),
      ctx.privacyPolicy,
    );

    const init = async () => {
      recorder = await startReplayRecorder({
        checkoutEveryNms: () => anonymousGate.checkoutEveryNms(),
        deferStart: !anonymousGate.allowsRecording(),
        onEvent: (event, isCheckout) => {
          if (startup.signal.aborted) {
            return;
          }

          if (isCheckout && event.type === EventType.Meta) {
            flush();
          }

          redactReplayEvent(event, ctx.redactUrl);

          stopMarked = false;

          buffer.push(event);
        },
        onFault: handleRecorderFault,
        onMutationStorm: (detail) => {
          log.warn("replay halted after a mutation storm: %s", detail);

          markStopped("mutation-storm");

          flush();

          halt("mutation-storm", detail);
        },
        recordOptions,
        signal: startup.signal,
      });

      if (startup.signal.aborted) {
        recorder?.stop();

        recorder = null;

        return;
      }

      if (!recorder) {
        return;
      }

      unsubscribeConfig = ctx.subscribePluginConfigState("replay", (state) => {
        if (state === "enabled") {
          anonymousGate.syncRecording();

          if (awaitingKeyframe) {
            requestKeyframe();
          }

          flush();
        }
      });

      anonymousGate.syncRecording();

      armFlushTimer();

      unsubscribeUnloading = onPageUnloading(() => {
        markStopped("unload");

        flush({ unload: true });
      });

      unsubscribePause = ctx.onSessionPause(pauseRecording);

      unsubscribeResume = ctx.onSessionResume(resumeRecording);

      unsubscribeException =
        ctx.onSessionExceptionWhilePaused(snapshotWhilePaused);

      stopAnonymousGate = anonymousGate.start();

      log.debug("recording started");
    };

    // oxlint-disable-next-line promise/prefer-await-to-then, anti-slop/no-unknown-parameters -- Recorder initialization can reject with any JavaScript value and must still halt replay safely.
    init().catch((error: unknown) => {
      const detail =
        error instanceof Error ? error.constructor.name : "UnknownError";

      log.warn("replay initialization failed: %s", detail);

      halt("init-failed", detail);
    });

    return (reason) => {
      if (reason === "consent-revoked") {
        consent.abort();

        clearExportCarryover("replay:");

        buffer.drain();

        log.debug("replay buffer discarded: consent revoked");
      } else {
        flush();
      }

      retryQueue.clear();

      startup.abort();

      carryover.dispose();

      recorder?.stop();

      clearFlushTimer();

      unsubscribeConfig?.();

      unsubscribeUnloading?.();

      unsubscribePause?.();

      unsubscribeResume?.();

      unsubscribeException?.();

      stopAnonymousGate?.();

      compressor.dispose();
    };
  },
};

export default replayPlugin;
