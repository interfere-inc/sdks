import type { SessionId } from "@interfere/types/data/session";
import { sessionIdSchema } from "@interfere/types/data/session";

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

export class SessionManager {
  sessionId: SessionId | null = null;
  windowId: string | null = null;
  private readonly local: ReturnType<typeof tryStorage>;
  private readonly session: ReturnType<typeof tryStorage>;
  private readonly onRotate: OnRotate | null = null;
  private lastActivityMs = 0;
  private sessionStartedMs = 0;

  constructor(onRotate?: OnRotate) {
    this.local = tryStorage("localStorage");
    this.session = tryStorage("sessionStorage");
    this.onRotate = onRotate ?? null;

    this.restore();
  }

  getSessionId() {
    if (this.sessionId) {
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
    const parsed = sessionIdSchema.safeParse(stored);

    this.sessionStartedMs = startedRaw ? Number(startedRaw) : 0;

    if (parsed.success && !this.isExpired()) {
      this.sessionId = parsed.data;

      this.touch();
    } else {
      this.rotate();
    }
  }

  private rotate() {
    this.sessionId = sessionIdSchema.parse(uuidv7());
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
