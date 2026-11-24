import type { PageSessionId, SessionId } from "@interfere/types/data/session";
import type { IdentifyParams } from "@interfere/types/sdk/identify";
import type {
  ConsentDecision,
  ConsentState,
} from "@interfere/types/sdk/plugins/manifest";

export interface KernelConsent {
  get: () => ConsentState;
  /** Records the visitor's decision: `true` grants analytics and replay, `false` denies both, `{ replay: false }` grants analytics only. */
  set: (decision: ConsentDecision) => void;
}

export interface KernelIdentity {
  clear: () => void;
  get: () => IdentifyParams | null;
  set: (params: IdentifyParams) => Promise<void>;
}

export interface KernelDevice {
  getDeviceId: () => string | null;
  getFpHash: () => string | null;
}

export interface KernelSession {
  getId: () => PageSessionId | SessionId | null;
  getWindowId: () => string | null;
}
