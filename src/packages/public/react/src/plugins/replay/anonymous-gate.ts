import { DEFAULT_ANONYMOUS_REPLAY_POLICY } from "@interfere/types/sdk/remote-config/anonymous-replay";

import type { PluginContext } from "../lib/types.js";
import type { ReplayBuffer } from "./chunking/buffer.js";
import type { ReplayRecorder } from "./recorder/recorder.js";

const ANONYMOUS_BUFFER_MS = 60_000;

const ANONYMOUS_CHECKPOINT_MS = 30_000;

export type ReplayAdmission = "error";

type RecordingMode = "buffer" | "continuous" | "off";

interface AnonymousReplayGateInput {
  buffer: ReplayBuffer;
  ctx: PluginContext;
  flush: () => void;
  getRecorder: () => ReplayRecorder | null;
  isHalted: () => boolean;
  isPaused: () => boolean;
  softLimitBytes: number;
}

export function createAnonymousReplayGate({
  buffer,
  ctx,
  flush,
  getRecorder,
  isHalted,
  isPaused,
  softLimitBytes,
}: AnonymousReplayGateInput) {
  let admittedSessionId: string | null = null;
  let recordingInBufferMode = false;

  function mode() {
    if (ctx.isIdentified()) {
      return "continuous" satisfies RecordingMode;
    }

    const policy =
      ctx.getAnonymousReplayPolicy() ?? DEFAULT_ANONYMOUS_REPLAY_POLICY;

    const sessionId = ctx.getSessionId();

    if (policy === "always" || (sessionId && sessionId === admittedSessionId)) {
      return "continuous" satisfies RecordingMode;
    }

    return (policy === "on_error" ? "buffer" : "off") satisfies RecordingMode;
  }

  function syncRecording() {
    const recorder = getRecorder();

    if (!recorder || isHalted() || isPaused()) {
      return;
    }

    if (mode() === "off") {
      recorder.stop();
    } else {
      recorder.start();
    }
  }

  function restartRecording() {
    getRecorder()?.stop();

    syncRecording();
  }

  function leaveBufferMode() {
    if (recordingInBufferMode) {
      restartRecording();
    }
  }

  function admitOnError() {
    const sessionId = ctx.getSessionId();

    if (isHalted() || !sessionId || mode() !== "buffer") {
      return;
    }

    admittedSessionId = sessionId;

    ctx.admitReplay();

    flush();

    leaveBufferMode();
  }

  function recordIdentified() {
    if (isHalted()) {
      return;
    }

    if (ctx.getAnonymousReplayPolicy() === "never") {
      buffer.drain();

      restartRecording();
    }

    flush();

    leaveBufferMode();
  }

  return {
    admission(sessionId: string) {
      return (admittedSessionId === sessionId ? "error" : undefined) satisfies
        | ReplayAdmission
        | undefined;
    },

    allowsRecording() {
      return mode() !== "off";
    },

    checkoutEveryNms() {
      recordingInBufferMode = mode() === "buffer";

      return recordingInBufferMode ? ANONYMOUS_CHECKPOINT_MS : undefined;
    },

    syncRecording,

    holdsUploads() {
      const current = mode();

      if (current === "buffer") {
        const now = Date.now();

        buffer.retainFromCheckpoint(
          buffer.totalBytes >= softLimitBytes ? now : now - ANONYMOUS_BUFFER_MS,
        );
      } else if (current === "off") {
        buffer.drain();
      }

      return current !== "continuous";
    },

    start() {
      const unsubscribeException = ctx.onSessionException(admitOnError);

      const unsubscribeIdentified = ctx.onIdentified(recordIdentified);

      return () => {
        unsubscribeException();

        unsubscribeIdentified();
      };
    },
  };
}
