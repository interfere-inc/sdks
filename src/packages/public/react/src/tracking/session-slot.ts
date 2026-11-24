import { SessionManager, clearStoredSession } from "./session.js";
import type { OnRotate } from "./session.js";

type UpgradeHandler = (previousId: string, sessionId: string) => void;

/** Holds the current session, persisted or page-scoped, and reports when a page-scoped session is replaced by the persisted one. */
export class SessionSlot {
  manager: SessionManager | null = null;
  private pending: SessionManager | null = null;
  private heldRotation: string | null = null;
  private upgradedFrom: string | null = null;
  private started = false;
  private persistent = true;
  private readonly onRotate: OnRotate;
  private readonly upgradeHandlers = new Set<UpgradeHandler>();

  constructor(onRotate: OnRotate) {
    this.onRotate = onRotate;
  }

  isPersistent() {
    return this.persistent;
  }

  /** Creates the session `start()` adopts, so its id is readable while the kernel starts; a rotation it makes is reported only from `start()`. */
  prepare() {
    if (this.manager || this.pending) {
      return;
    }

    this.pending = new SessionManager((sessionId) => {
      if (this.started) {
        this.onRotate(sessionId);
      } else {
        this.heldRotation = sessionId;
      }
    }, this.persistent);
  }

  pendingSessionId() {
    return this.pending?.getSessionId() ?? null;
  }

  start() {
    this.started = true;

    this.manager = this.pending ?? this.create();

    this.pending = null;

    const held = this.heldRotation;

    this.heldRotation = null;

    if (held !== null && held === this.manager.sessionId) {
      this.onRotate(held);
    }

    const previousId = this.upgradedFrom;

    this.upgradedFrom = null;

    if (previousId !== null) {
      this.upgraded(previousId, this.manager.getSessionId());
    }
  }

  /** Switches mode; `beforeSwitch` runs only when a started session is replaced. Downgrading erases the persisted session. */
  setPersistent(persistent: boolean, beforeSwitch: () => void) {
    if (persistent === this.persistent) {
      return false;
    }

    this.persistent = persistent;

    if (!this.manager) {
      if (this.pending) {
        this.replacePending(persistent);
      }

      return false;
    }

    const previousId = this.manager.sessionId;

    beforeSwitch();

    if (!persistent) {
      clearStoredSession();
    }

    this.manager = this.create();

    const sessionId = this.manager.getSessionId();

    if (persistent && previousId !== null) {
      this.upgraded(previousId, sessionId);
    }

    return true;
  }

  onUpgrade(handler: UpgradeHandler) {
    this.upgradeHandlers.add(handler);

    return () => {
      this.upgradeHandlers.delete(handler);
    };
  }

  private replacePending(persistent: boolean) {
    const previousId = this.pending?.sessionId ?? null;

    if (!persistent) {
      clearStoredSession();
    }

    this.pending = null;

    this.heldRotation = null;

    this.prepare();

    this.upgradedFrom = persistent ? previousId : null;
  }

  private upgraded(previousId: string, sessionId: string) {
    for (const handler of this.upgradeHandlers) {
      handler(previousId, sessionId);
    }
  }

  private create() {
    return new SessionManager(this.onRotate, this.persistent);
  }
}
