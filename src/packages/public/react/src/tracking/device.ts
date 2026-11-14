import { v4 as uuidv4 } from "uuid";
import { createLogger } from "../util/log.js";

const log = createLogger("device");

const LS_KEY = "interfere:device_id";

const COOKIE_NAME = "interfere_did";

const COOKIE_MAX_AGE_DAYS = 400;

/**
 * Producer of an opaque, stable per-browser fingerprint hash. The default
 * dynamic-imports the FingerprintJS OSS library and self-defers via
 * `requestIdleCallback` so the ~1.5s of synchronous font / canvas / WebGL
 * probing the library performs on a cold load doesn't land on the
 * hydration tick. Tests inject a fixed-value provider that resolves
 * immediately — they pay nothing for the production deferral.
 */
export type FingerprintProvider = () => Promise<string | null>;

const FP_IDLE_TIMEOUT_MS = 5000;

/**
 * Resolves on the next browser idle frame, capped at 5s. Falls back to
 * `setTimeout(0)` (next macrotask) on hosts without
 * `requestIdleCallback`. Exported only for the unit test that pins the
 * "default provider gates fingerprint work via RIC" contract — not part
 * of the package's public surface.
 */
export async function whenIdle() {
  return new Promise<undefined>((resolve) => {
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Hosts without the optional idle callback API must use the timer fallback for fingerprint scheduling.
    if (typeof globalThis.requestIdleCallback === "function") {
      globalThis.requestIdleCallback(
        () => {
          resolve(undefined);
        },
        {
          timeout: FP_IDLE_TIMEOUT_MS,
        },
      );
    } else {
      setTimeout(() => {
        resolve(undefined);
      }, 0);
    }
  });
}

const defaultFingerprintProvider: FingerprintProvider = async () => {
  await whenIdle();

  try {
    const FingerprintJS = await import("@fingerprintjs/fingerprintjs");

    const fp = await FingerprintJS.load({ monitoring: false });

    const result = await fp.get();

    if (!result.visitorId) {
      log.error(
        "Fingerprinting returned an empty visitor id; visitor identity will be cookie-only",
      );

      return null;
    }

    return result.visitorId;
  } catch (error) {
    log.error(
      "fingerprint init failed; visitor identity will be cookie-only",
      String(error),
    );

    return null;
  }
};

function tryLocalStorage() {
  try {
    const s = globalThis.localStorage;
    const key = "__interfere_device_probe__";

    s.setItem(key, "1");

    s.removeItem(key);

    return s;
  } catch {
    return null;
  }
}

function getCookie(name: string) {
  if (typeof document === "undefined") {
    return null;
  }

  const match = document.cookie
    .split("; ")
    .find((c) => c.startsWith(`${name}=`));

  return (
    match ? decodeURIComponent(match.split("=")[1] ?? "") : null
  ) satisfies string | null;
}

function setCookie(name: string, value: string, maxAgeSeconds: number) {
  if (typeof document === "undefined") {
    return;
  }

  // oxlint-disable-next-line unicorn/no-document-cookie -- Cookie Store API is async and not universally supported; synchronous access is required here
  document.cookie = `${name}=${encodeURIComponent(value)};path=/;max-age=${maxAgeSeconds};SameSite=Lax`;
}

function persistCookie(name: string, value: string) {
  setCookie(name, value, COOKIE_MAX_AGE_DAYS * 24 * 60 * 60);
}

function deleteCookie(name: string) {
  setCookie(name, "", 0);
}

function generateId() {
  return uuidv4();
}

export class DeviceManager {
  private deviceId: string | null = null;
  private fpHash: string | null = null;
  private fpPending: Promise<string | null> | null = null;
  private epoch = 0;
  private readonly fingerprintProvider: FingerprintProvider;

  constructor(fingerprintProvider?: FingerprintProvider) {
    this.fingerprintProvider =
      fingerprintProvider ?? defaultFingerprintProvider;
  }

  init() {
    if (this.deviceId) {
      return;
    }

    const ls = tryLocalStorage();
    const fromLs = ls?.getItem(LS_KEY) ?? null;
    const fromCookie = getCookie(COOKIE_NAME);

    this.deviceId = fromLs ?? fromCookie ?? generateId();

    if (!fromLs && ls) {
      ls.setItem(LS_KEY, this.deviceId);
    }

    if (!fromCookie) {
      persistCookie(COOKIE_NAME, this.deviceId);
    }

    if (fromLs && !fromCookie) {
      persistCookie(COOKIE_NAME, this.deviceId);
    }

    if (fromCookie && !fromLs && ls) {
      ls.setItem(LS_KEY, this.deviceId);
    }

    log.debug(
      "device %s (ls=%s cookie=%s)",
      this.deviceId,
      Boolean(fromLs),
      Boolean(fromCookie),
    );

    this.startFingerprint();
  }

  private startFingerprint() {
    if (this.fpHash || this.fpPending) {
      return;
    }

    const { epoch } = this;

    this.fpPending = (async () => {
      const hash = await this.fingerprintProvider();

      // WARNING: A consent purge can race this probe; accepting its stale result restores an identity the user cleared.
      if (epoch !== this.epoch) {
        return null;
      }

      this.fpHash = hash;

      if (hash) {
        log.debug("fpHash %s", hash);
      }

      return hash;
    })();
  }

  getDeviceId() {
    return this.deviceId;
  }

  getFpHash() {
    return this.fpHash;
  }

  async whenFingerprintReady() {
    if (this.fpHash) {
      return this.fpHash;
    }

    return this.fpPending ?? Promise.resolve(null);
  }

  reset() {
    this.deviceId = generateId();

    tryLocalStorage()?.setItem(LS_KEY, this.deviceId);

    persistCookie(COOKIE_NAME, this.deviceId);

    log.debug("device %s (reset)", this.deviceId);

    return this.deviceId;
  }

  clear() {
    this.epoch += 1;
    this.deviceId = null;
    this.fpHash = null;
    this.fpPending = null;

    tryLocalStorage()?.removeItem(LS_KEY);

    deleteCookie(COOKIE_NAME);

    log.debug("device identity cleared");
  }
}
