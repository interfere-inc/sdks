import type { PageSessionId, SessionId } from "@interfere/types/data/session";
import {
  isSessionId,
  toPageSessionId,
  toSessionId,
} from "@interfere/types/data/session-id";

import { v4 as uuidv4, v7 as uuidv7 } from "uuid";

const SESSION_ID_KEY = "interfere:session_id";

const LAST_ACTIVITY_KEY = "interfere:last_activity";

const SESSION_STARTED_KEY = "interfere:session_started";

const WINDOW_ID_KEY = "interfere:window_id";

const SESSION_TIMEOUT_MS = 15 * 60 * 1000;

const SESSION_MAX_AGE_MS = 24 * 60 * 60 * 1000;

function tryStorage(type: "localStorage" | "sessionStorage") {
  try {
    const s = globalThis[type];
    const probeKey = "__interfere_probe__";

    s.setItem(probeKey, "1");

    s.removeItem(probeKey);

    return {
      getItem(key: string) {
        try {
          return s.getItem(key);
        } catch {
          return null;
        }
      },
      removeItem(key: string) {
        try {
          s.removeItem(key);
        } catch {
          return;
        }
      },
      setItem(key: string, value: string) {
        try {
          s.setItem(key, value);
        } catch {
          return;
        }
      },
    };
  } catch {
    return null;
  }
}

export type OnRotate = (sessionId: string) => void;

/** Removes the persisted session, so a later page starts from a fresh id. */
export function clearStoredSession() {
  const local = tryStorage("localStorage");

  for (const key of [SESSION_ID_KEY, SESSION_STARTED_KEY, LAST_ACTIVITY_KEY]) {
    local?.removeItem(key);
  }

  tryStorage("sessionStorage")?.removeItem(WINDOW_ID_KEY);
}

export class SessionManager {
  sessionId: PageSessionId | SessionId | null = null;
  windowId: string | null = null;
  private readonly local: ReturnType<typeof tryStorage>;
  private readonly session: ReturnType<typeof tryStorage>;
  private readonly onRotate: OnRotate | null = null;
  private readonly persistent: boolean;
  private lastActivityMs = 0;
  private sessionStartedMs = 0;

  /** A non-persistent manager neither reads nor writes browser storage and issues page-scoped ids. */
  constructor(onRotate?: OnRotate, persistent = true) {
    this.local = persistent ? tryStorage("localStorage") : null;
    this.session = persistent ? tryStorage("sessionStorage") : null;
    this.onRotate = persistent ? (onRotate ?? null) : null;
    this.persistent = persistent;

    this.restore();
  }

  getSessionId() {
    if (this.sessionId !== null) {
      return this.sessionId;
    }

    return this.rotate();
  }

  markActivity() {
    this.touch();
  }

  getLastActivityAt() {
    const stored = Number(this.local?.getItem(LAST_ACTIVITY_KEY));

    return Number.isFinite(stored)
      ? Math.max(stored, this.lastActivityMs)
      : this.lastActivityMs;
  }

  getSessionStartedAt() {
    return this.sessionStartedMs;
  }

  getWindowId() {
    if (this.windowId) {
      return this.windowId;
    }

    this.windowId = `win_${uuidv4()}`;

    this.session?.setItem(WINDOW_ID_KEY, this.windowId);

    return this.windowId;
  }

  reset() {
    return this.rotate();
  }

  private restore() {
    const stored = this.local?.getItem(SESSION_ID_KEY);
    const startedRaw = this.local?.getItem(SESSION_STARTED_KEY);

    this.sessionStartedMs = startedRaw ? Number(startedRaw) : 0;

    if (isSessionId(stored) && !this.isExpired()) {
      this.sessionId = stored;

      this.touch();
    } else {
      this.rotate();
    }
  }

  private rotate() {
    this.sessionId = this.persistent
      ? toSessionId(uuidv7())
      : toPageSessionId(`page_${uuidv7()}`);

    this.sessionStartedMs = Date.now();

    this.local?.setItem(SESSION_ID_KEY, this.sessionId);

    this.local?.setItem(SESSION_STARTED_KEY, String(this.sessionStartedMs));

    this.touch();

    this.onRotate?.(this.sessionId);

    return this.sessionId;
  }

  private touch() {
    this.lastActivityMs = Date.now();

    this.local?.setItem(LAST_ACTIVITY_KEY, String(this.lastActivityMs));
  }

  private isExpired() {
    if (
      this.sessionStartedMs > 0 &&
      Date.now() - this.sessionStartedMs > SESSION_MAX_AGE_MS
    ) {
      return true;
    }

    const raw = this.local?.getItem(LAST_ACTIVITY_KEY);
    const ts = raw ? Number(raw) : this.lastActivityMs;

    if (ts === 0) {
      return true;
    }

    return Date.now() - ts > SESSION_TIMEOUT_MS;
  }
}

/** OTel semantic-convention event a page emits when its page-scoped session is replaced by the persisted one. */
export const SESSION_START_EVENT_NAME = "session.start";

export const ATTR_SESSION_ID = "session.id";

const ATTR_SESSION_PREVIOUS_ID = "session.previous_id";

/** The `session.start` record linking a page-scoped session to the persisted session that replaced it. */
export function sessionStartRecord(previousId: string, sessionId: string) {
  return {
    attributes: {
      [ATTR_SESSION_ID]: sessionId,
      [ATTR_SESSION_PREVIOUS_ID]: previousId,
    },
    eventName: SESSION_START_EVENT_NAME,
  };
}
