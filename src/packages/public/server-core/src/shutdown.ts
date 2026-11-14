import { containAsync } from "./contain.js";
import {
  isInterfereListener,
  removeInterfereListeners,
  tagListener,
} from "./process-listeners.js";

const FLUSH_CAP_MS = 2000;

const SHUTDOWN_SIGNALS = ["SIGTERM", "SIGINT"] as const;

type ShutdownSignal = (typeof SHUTDOWN_SIGNALS)[number];

/** Waits at most 2 s for `flush` and never rejects. */
export async function settleFlush(flush: () => Promise<void>) {
  const { promise: capped, resolve: release } =
    Promise.withResolvers<undefined>();

  const timer = setTimeout(() => {
    release(undefined);
  }, FLUSH_CAP_MS);

  try {
    await Promise.race([containAsync("Telemetry flush", flush), capped]);
  } finally {
    clearTimeout(timer);
  }
}

async function flushThenTerminate(
  flush: () => Promise<void>,
  signal: ShutdownSignal,
) {
  const hostOwned = process
    .listeners(signal)
    .some((listener) => !isInterfereListener(listener));

  await settleFlush(flush);

  if (!hostOwned) {
    uninstallShutdownFlush();

    process.kill(process.pid, signal);
  }
}

/** Flushes telemetry on SIGTERM, SIGINT and a drained event loop; never delays the host's own shutdown past the flush cap. */
export function installShutdownFlush(flush: () => Promise<void>) {
  uninstallShutdownFlush();

  let drained = false;

  const onSignal = tagListener((signal: ShutdownSignal) => {
    void flushThenTerminate(flush, signal);
  });

  const onBeforeExit = tagListener(() => {
    if (drained) {
      return;
    }

    drained = true;

    void settleFlush(flush);
  });

  for (const signal of SHUTDOWN_SIGNALS) {
    process.on(signal, onSignal);
  }

  process.on("beforeExit", onBeforeExit);
}

export function uninstallShutdownFlush() {
  for (const signal of SHUTDOWN_SIGNALS) {
    removeInterfereListeners(signal);
  }

  removeInterfereListeners("beforeExit");
}
