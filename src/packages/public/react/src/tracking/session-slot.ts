import { SessionManager, clearStoredSession } from "./session.js";
import type { OnRotate } from "./session.js";

type UpgradeHandler = (previousId: string, sessionId: string) => void;

/** Holds the current session, persisted or page-scoped, and reports when a page-scoped session is replaced by the persisted one. */
export class SessionSlot {
  manager: SessionManager | null = null;
  private persistent = true;
  private readonly onRotate: OnRotate;
  private readonly upgradeHandlers = new Set<UpgradeHandler>();

  constructor(onRotate: OnRotate) {
    this.onRotate = onRotate;
  }

  isPersistent() {
    return this.persistent;
  }

  start() {
    this.manager = this.create();
  }

  /** Switches mode; `beforeSwitch` runs only when a started session is replaced. Downgrading erases the persisted session. */
  setPersistent(persistent: boolean, beforeSwitch: () => void) {
    if (persistent === this.persistent) {
      return false;
    }

    this.persistent = persistent;

    if (!this.manager) {
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
      for (const handler of this.upgradeHandlers) {
        handler(previousId, sessionId);
      }
    }

    return true;
  }

  onUpgrade(handler: UpgradeHandler) {
    this.upgradeHandlers.add(handler);

    return () => {
      this.upgradeHandlers.delete(handler);
    };
  }

  private create() {
    return new SessionManager(this.onRotate, this.persistent);
  }
}
