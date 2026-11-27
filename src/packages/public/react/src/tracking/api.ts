import { InterfereClient, InterfereError } from "@interfere/javascript";

import type { IdentifyParams } from "@interfere/types/sdk/identify";

import { ActivityMonitor } from "../internal/activity/activity.js";
import { fetchIngest } from "../internal/ingest-backoff/ingest-backoff.js";
import type {
  SessionPauseReason,
  SessionResumeReason,
} from "../internal/activity/activity.js";
import { createLogger } from "../util/log.js";
import { DeviceManager } from "./device.js";
import type { SessionManager } from "./session.js";
import { SessionSlot } from "./session-slot.js";
import { SessionRetry, syncKey } from "./session-retry.js";

const log = createLogger("tracking");

const SYNC_COOLDOWN_MS = 5000;

export interface SessionTrackerOptions {
  device?: DeviceManager;
  fetcher?: typeof globalThis.fetch;
  target: InterfereClient.Options;
}

export class SessionTracker {
  private readonly client: InterfereClient;
  private readonly device: DeviceManager;

  readonly slot = new SessionSlot((id) => {
    void this.onRotate(id);
  });
  private readonly activity = new ActivityMonitor();
  private currentIdentity: IdentifyParams | null = null;
  private identifiedKey: string | null = null;
  private lastSyncedKey: string | null = null;
  private inflightSyncKey: string | null = null;
  private syncAttemptMs = 0;
  private generation = 0;
  private readonly syncRetry = new SessionRetry();
  private syncRetryKey: string | null = null;
  private identityStarted = false;
  private identityConfirmed = false;
  private readonly identifiedHandlers = new Set<() => void>();
  private replayAdmittedSessionId: string | null = null;

  constructor(opts: SessionTrackerOptions) {
    this.client = new InterfereClient({
      ...opts.target,
      maxRetries: 0,
      timeoutInSeconds: 10,
      fetch: Object.assign(
        async (
          input: Parameters<typeof fetch>[0],
          init?: Parameters<typeof fetch>[1],
        ) =>
          fetchIngest(input, async () =>
            (opts.fetcher ?? globalThis.fetch)(input, {
              ...init,
              credentials: "omit",
              keepalive: true,
            }),
          ),
        opts.fetcher ?? globalThis.fetch,
      ),
    });

    this.device = opts.device ?? new DeviceManager();
  }

  start() {
    this.slot.start();

    this.activity.start({
      markActivity: () => {
        this.markActivity();
      },
      reset: () => {
        this.rotateSession();
      },
      startedAt: () => this.getSessionStartedAt(),
      lastActivityAt: () => this.slot.manager?.getLastActivityAt() ?? 0,
    });
  }

  /** Switches between the persisted session and a page-scoped in-memory one. Upgrading reports the page id it replaces; downgrading erases the persisted session. */
  setPersistent(persistent: boolean) {
    const switched = this.slot.setPersistent(persistent, () => {
      this.invalidateSync();
    });

    if (switched && persistent && this.currentIdentity) {
      this.identifiedKey = null;

      void this.identify(this.currentIdentity);
    }
  }

  readonly onSessionUpgrade: SessionSlot["onUpgrade"] = (handler) =>
    this.slot.onUpgrade(handler);

  startDeviceIdentity() {
    if (this.identityStarted) {
      return;
    }

    this.invalidateSync();

    this.identityStarted = true;

    this.device.init();

    void this.backfillIdentity();
  }

  discardDeviceIdentity() {
    this.invalidateSync();

    this.identityStarted = false;

    this.device.clear();
  }

  private async backfillIdentity() {
    const gen = this.generation;

    await Promise.resolve();

    if (gen !== this.generation || !this.identityStarted) {
      return;
    }

    const initialSessionId = this.slot.manager?.getSessionId() ?? null;
    const initialFpHash = this.device.getFpHash();

    if (initialSessionId !== null) {
      await this.syncSession(
        initialSessionId,
        this.device.getDeviceId(),
        initialFpHash,
      );
    }

    if (this.currentIdentity) {
      void this.identify(this.currentIdentity);
    }

    const fpHash = await this.device.whenFingerprintReady();

    const sessionId = this.slot.manager?.getSessionId() ?? null;

    if (
      gen !== this.generation ||
      // oxlint-disable-next-line typescript/no-unnecessary-condition -- Consent can change while registration and fingerprinting are pending.
      !this.identityStarted ||
      sessionId === null ||
      fpHash === initialFpHash
    ) {
      return;
    }

    void this.syncSession(sessionId, this.device.getDeviceId(), fpHash);
  }

  onSessionPause(handler: (reason: SessionPauseReason) => void) {
    return this.activity.onPause(handler);
  }

  onSessionExceptionWhilePaused(handler: (reason: SessionPauseReason) => void) {
    return this.activity.onExceptionWhilePaused(handler);
  }

  noteException() {
    this.activity.noteException();
  }

  onSessionException(handler: () => void) {
    return this.activity.onException(handler);
  }

  isIdentified() {
    return this.identityConfirmed;
  }

  onIdentified(handler: () => void) {
    this.identifiedHandlers.add(handler);

    return () => {
      this.identifiedHandlers.delete(handler);
    };
  }

  admitReplay() {
    const sessionId = this.slot.manager?.getSessionId() ?? null;

    if (
      !this.slot.isPersistent() ||
      sessionId === null ||
      this.replayAdmittedSessionId === sessionId
    ) {
      return;
    }

    this.replayAdmittedSessionId = sessionId;

    void this.postReplayAdmission(sessionId);
  }

  private async postReplayAdmission(sessionId: string) {
    try {
      await this.client.sessions.createSession({
        deviceId: this.device.getDeviceId(),
        fpHash: this.device.getFpHash(),
        replayTrigger: "error",
        sessionId,
      });
    } catch {
      if (this.replayAdmittedSessionId === sessionId) {
        this.replayAdmittedSessionId = null;
      }

      log.warn("replay admission failed for session %s", sessionId);
    }
  }

  onSessionResume(handler: (reason: SessionResumeReason) => void) {
    return this.activity.onResume(handler);
  }

  sessionId() {
    const id = this.slot.manager?.getSessionId() ?? null;

    if (id !== null && this.slot.isPersistent()) {
      this.ensureSynced(id);
    }

    return id;
  }

  windowId() {
    return this.slot.manager?.getWindowId() ?? null;
  }

  markActivity() {
    this.slot.manager?.markActivity();
  }

  getSessionStartedAt() {
    return this.slot.manager?.getSessionStartedAt() ?? 0;
  }

  getDeviceId() {
    return this.device.getDeviceId();
  }

  getFpHash() {
    return this.device.getFpHash();
  }

  getIdentity() {
    return this.currentIdentity;
  }

  async identify(params: IdentifyParams) {
    if (!this.slot.manager) {
      return;
    }

    if (!this.slot.isPersistent()) {
      this.currentIdentity = params;

      return;
    }

    const sessionId = this.slot.manager.getSessionId();

    const key = JSON.stringify([
      sessionId,
      this.device.getDeviceId(),
      params.identifier,
      params.source.type,
      params.source.name,
      params.company?.tier,
      params.signature,
    ]);

    if (this.identifiedKey === key) {
      log.debug("skipped, already identified for session %s", sessionId);

      return;
    }

    this.currentIdentity = params;
    this.identifiedKey = key;
    this.identityConfirmed = false;

    log.info("POST session %s → identify", sessionId);

    await this.postIdentify(key, sessionId, {
      deviceId: this.device.getDeviceId(),
      fpHash: this.device.getFpHash(),
      sessionId,
      ...params,
    });
  }

  private async postIdentify(
    key: string,
    sessionId: string | null,
    body: IdentifyParams & {
      deviceId: ReturnType<DeviceManager["getDeviceId"]>;
      fpHash: ReturnType<DeviceManager["getFpHash"]>;
      sessionId: ReturnType<SessionManager["getSessionId"]>;
    },
  ) {
    try {
      await this.client.sessions.identifySession(body);
    } catch {
      if (this.identifiedKey === key) {
        this.identifiedKey = null;
      }

      log.warn("identify failed for session %s", sessionId);

      return;
    }

    if (this.identifiedKey !== key) {
      return;
    }

    this.identityConfirmed = true;

    for (const handler of this.identifiedHandlers) {
      try {
        handler();
      } catch {
        log.warn("identified handler threw for session %s", sessionId);
      }
    }
  }

  clearIdentity() {
    this.currentIdentity = null;
    this.identifiedKey = null;
    this.identityConfirmed = false;
  }

  reset() {
    this.invalidateSync();

    this.clearIdentity();

    if (this.identityStarted) {
      this.device.reset();
    }

    this.slot.manager?.reset();
  }

  rotateSession() {
    this.slot.manager?.reset();
  }

  dispose() {
    this.activity.stop();

    this.invalidateSync();

    this.clearIdentity();

    this.device.dispose();

    this.slot.manager = null;
  }

  private invalidateSync() {
    this.generation += 1;

    this.syncRetry.reset();

    this.syncRetryKey = null;
    this.lastSyncedKey = null;
    this.inflightSyncKey = null;
    this.syncAttemptMs = 0;
  }

  private currentSyncKey() {
    const sessionId = this.slot.manager?.sessionId ?? null;

    return sessionId === null
      ? null
      : syncKey(sessionId, this.device.getDeviceId(), this.device.getFpHash());
  }

  private ensureSynced(sessionId: string) {
    const deviceId = this.device.getDeviceId();
    const fpHash = this.device.getFpHash();
    const key = syncKey(sessionId, deviceId, fpHash);

    if (this.lastSyncedKey === key || this.inflightSyncKey === key) {
      return;
    }

    if (Date.now() - this.syncAttemptMs < SYNC_COOLDOWN_MS) {
      return;
    }

    void this.syncSession(sessionId, deviceId, fpHash);
  }

  private async syncSession(
    sessionId: string,
    deviceId: string | null,
    fpHash: string | null,
  ) {
    const key = syncKey(sessionId, deviceId, fpHash);

    if (this.lastSyncedKey === key || this.inflightSyncKey === key) {
      return;
    }

    if (this.syncRetryKey !== key) {
      this.syncRetry.reset();

      this.syncRetryKey = key;
    }

    this.syncRetry.cancel();

    const { generation } = this;

    this.inflightSyncKey = key;
    this.syncAttemptMs = Date.now();

    try {
      await this.client.sessions.createSession({ sessionId, deviceId, fpHash });

      if (generation === this.generation && key === this.currentSyncKey()) {
        this.lastSyncedKey = key;

        this.syncRetry.reset();
      }
    } catch (error) {
      log.warn("session sync failed");

      if (
        error instanceof InterfereError &&
        generation === this.generation &&
        key === this.currentSyncKey()
      ) {
        this.syncRetry.schedule(error, () => {
          const current = this.slot.manager?.sessionId ?? null;

          if (generation === this.generation && current !== null) {
            this.ensureSynced(current);
          }
        });
      }
    } finally {
      if (generation === this.generation && this.inflightSyncKey === key) {
        this.inflightSyncKey = null;
      }
    }
  }

  private async onRotate(sessionId: string) {
    this.syncAttemptMs = Date.now();

    const gen = this.generation;

    try {
      await Promise.resolve();

      const fpHash = this.device.getFpHash();

      const deviceId = this.device.getDeviceId();

      if (gen !== this.generation) {
        return;
      }

      log.debug(
        "POST session %s (device=%s fp=%s)",
        sessionId,
        deviceId ?? "pending",
        fpHash ?? "none",
      );

      void this.syncSession(sessionId, deviceId, fpHash);

      if (this.currentIdentity) {
        void this.identify(this.currentIdentity);
      }
    } catch (error) {
      void error;
    }
  }
}
