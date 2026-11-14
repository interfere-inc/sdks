import { MECHANISM_TYPE } from "@interfere/types/sdk/errors";
import type { ErrorMechanism } from "@interfere/types/sdk/plugins/payload/errors";

import { contain } from "./contain.js";
import {
  isInterfereListener,
  removeInterfereListeners,
  tagListener,
} from "./process-listeners.js";
import {
  installShutdownFlush,
  settleFlush,
  uninstallShutdownFlush,
} from "./shutdown.js";

export interface GlobalHandlerOptions {
  readonly captureUncaughtException: boolean;
  readonly captureUnhandledRejection: boolean;
  // oxlint-disable-next-line anti-slop/no-unknown-parameters -- Process listeners receive arbitrary thrown values, which adapters normalize.
  readonly capture: (error: unknown, mechanism: ErrorMechanism) => void;
  readonly flush: () => Promise<void>;
}

let exiting = false;

async function handleUncaughtException(
  error: Error,
  options: GlobalHandlerOptions,
) {
  if (exiting) {
    return;
  }

  const fatal = process
    .listeners("uncaughtException")
    .every((listener) => isInterfereListener(listener));

  exiting = fatal;

  contain("captureError()", () => {
    options.capture(error, {
      handled: false,
      type: MECHANISM_TYPE.node.uncaughtException,
    });
  });

  await settleFlush(options.flush);

  if (fatal) {
    // WARNING: With no other uncaughtException listener Node would have exited; staying alive would leave a corrupted process serving.
    // oxlint-disable-next-line unicorn/no-process-exit
    process.exit(1);
  }
}

async function handleUnhandledRejection(
  // oxlint-disable-next-line anti-slop/no-unknown-parameters -- Node rejection listeners receive arbitrary JavaScript values, which error capture normalizes without changing their identity.
  reason: unknown,
  options: GlobalHandlerOptions,
) {
  contain("captureError()", () => {
    options.capture(reason, {
      handled: false,
      type: MECHANISM_TYPE.node.unhandledRejection,
    });
  });

  await settleFlush(options.flush);
}

/** Captures process-level errors and flushes on shutdown, replacing listeners from an earlier install; after an uncaught exception it exits only when no other listener keeps the process alive. */
export function installGlobalHandlers(options: GlobalHandlerOptions) {
  uninstallGlobalHandlers();

  if (options.captureUncaughtException) {
    process.on(
      "uncaughtException",
      tagListener((error: Error) => {
        void handleUncaughtException(error, options);
      }),
    );
  }

  if (options.captureUnhandledRejection) {
    process.on(
      "unhandledRejection",
      // oxlint-disable-next-line anti-slop/no-unknown-parameters -- Node rejection listeners receive arbitrary JavaScript values.
      tagListener((reason: unknown) => {
        void handleUnhandledRejection(reason, options);
      }),
    );
  }

  installShutdownFlush(options.flush);
}

export function uninstallGlobalHandlers() {
  removeInterfereListeners("uncaughtException");

  removeInterfereListeners("unhandledRejection");

  exiting = false;

  uninstallShutdownFlush();
}
