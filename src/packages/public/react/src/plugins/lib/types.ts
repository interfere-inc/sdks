import type { NonErrorException } from "@interfere/types/sdk/errors";
import type { PluginKey } from "@interfere/types/sdk/plugins/manifest";
import type { AnonymousReplayPolicy } from "@interfere/types/sdk/remote-config/anonymous-replay";

import type { Tracer } from "@opentelemetry/api";

import type {
  SessionPauseReason,
  SessionResumeReason,
} from "../../internal/activity/activity.js";
import type { PrivacyPolicy } from "../../internal/dom/privacy.js";
import type { RecordExceptionOpts } from "../../internal/kernel.js";
import type { ReplayConfig } from "../replay/config.js";

// WARNING: Consent revocation must discard buffered data; uploading it violates the withdrawn consent.
export type PluginTeardownReason = "consent-revoked" | "disabled" | "dispose";

export type PluginCleanup = (reason?: PluginTeardownReason) => void;

export type PluginConfigState = "disabled" | "enabled" | "pending";

export interface PluginContext {
  admitReplay: () => void;
  getAnonymousReplayPolicy: () => AnonymousReplayPolicy | null;
  getPluginConfigState: (key: PluginKey) => PluginConfigState;
  getReplayConfig: () => ReplayConfig;
  readonly privacyPolicy: PrivacyPolicy;
  readonly serviceIdentity?: {
    serviceName?: string | undefined;
    serviceNamespace?: string | undefined;
  };
  getSessionId: () => string;
  getTracer: (name: string) => Tracer;
  getWindowId: () => string;
  isIdentified: () => boolean;
  onIdentified: (handler: () => void) => () => void;
  onSessionException: (handler: () => void) => () => void;
  onSessionExceptionWhilePaused: (
    handler: (reason: SessionPauseReason) => void,
  ) => () => void;
  onSessionPause: (handler: (reason: SessionPauseReason) => void) => () => void;
  onSessionResume: (
    handler: (reason: SessionResumeReason) => void,
  ) => () => void;
  recordException: (
    value: Error | NonErrorException,
    opts: RecordExceptionOpts,
  ) => void;
  redactUrl: (url: string) => string;
  startDeviceInfoCapture: () => PluginCleanup | null;
  startPageEventsCapture: () => PluginCleanup | null;
  subscribePluginConfigState: (
    key: PluginKey,
    listener: (state: PluginConfigState) => void,
  ) => () => void;
}

export interface Plugin {
  readonly name: string;
  setup: (ctx: PluginContext) => PluginCleanup | undefined;
}
