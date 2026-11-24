/** Input-idle time after which the session pauses (replay stops recording); the session id is kept. */
export const IDLE_AFTER_MS = 5 * 60 * 1000;

/** Input gap after which the next input rotates to a new session id. */
export const EXPIRE_AFTER_MS = 15 * 60 * 1000;

export const MAX_AGE_MS = 24 * 60 * 60 * 1000;

export const CHECK_INTERVAL_MS = 30 * 1000;
