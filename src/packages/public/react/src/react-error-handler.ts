"use client";

import { MECHANISM_TYPE, toException } from "@interfere/types/sdk/errors";
import type { ErrorMechanism } from "@interfere/types/sdk/plugins/payload/errors";

import { captureReactError } from "./internal/capture/capture.js";
import type { Kernel } from "./internal/kernel.js";
import { createRepeatGate, exceptionKey } from "./plugins/lib/repeat-gate.js";

/** React root error-handler metadata; only componentStack is consumed. */
interface ReactRootErrorInfo {
  componentStack?: string | null | undefined;
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- React forwards arbitrary thrown JavaScript values to root error callbacks.
type RootErrorCallback = (error: unknown, info: ReactRootErrorInfo) => void;

export interface ReactErrorHandlerCallbacks {
  /**
   * Called when a React error boundary caught and handled an error. Interfere
   * reports it with `mechanism.handled: true`. Your callback runs after the
   * capture.
   */
  onCaughtError?: RootErrorCallback;
  /**
   * Called when concurrent React auto-recovered from an error by retrying
   * the render. Reported with `mechanism.synthetic: true` so Interfere can
   * deprioritize — these often aren't user-facing bugs.
   */
  onRecoverableError?: RootErrorCallback;
  /**
   * Called when a render-phase error propagated past every boundary and
   * React unmounted (or fell back to its default UI). Reported with
   * `mechanism.handled: false` — this is the "real" uncaught case.
   */
  onUncaughtError?: RootErrorCallback;
}

export interface ReactErrorHandlerOptions extends ReactErrorHandlerCallbacks {
  /**
   * The kernel to capture errors against. Pass either the kernel directly
   * or a getter — framework wrappers expose `getKernelOrNull()` for the
   * latter pattern, which lets you wire the handler before `init()`
   * resolves. Pass `getKernelOrNull` (not `getKernel`) — the throwing
   * variant breaks this contract since the handler runs in error-recovery
   * paths where rethrowing isn't useful. Capture is silently skipped when
   * no kernel is available.
   */
  kernel?: Kernel | (() => Kernel | null) | null;
}

interface ReactRootErrorHandlers {
  onCaughtError: RootErrorCallback;
  onRecoverableError: RootErrorCallback;
  onUncaughtError: RootErrorCallback;
}

function resolveKernel(source: ReactErrorHandlerOptions["kernel"]) {
  if (!source) {
    return null;
  }

  return (
    // oxlint-disable-next-line anti-slop/no-runtime-typeof
    (typeof source === "function" ? source() : source) satisfies Kernel | null
  );
}

/**
 * Creates the three error callbacks React 19's `createRoot()` / `hydrateRoot()`
 * accept, wired into Interfere's capture pipeline.
 *
 * ```ts
 * import { createRoot } from "react-dom/client";
 * import { getKernelOrNull } from "@interfere/next";
 * import { reactErrorHandler } from "@interfere/react/react-error-handler";
 *
 * createRoot(
 *   document.getElementById("root")!,
 *   reactErrorHandler({ kernel: getKernelOrNull })
 * ).render(<App />);
 * ```
 *
 * Pass your own callbacks to observe errors without replacing the capture
 * behaviour — user callbacks run after Interfere has captured:
 *
 * ```ts
 * reactErrorHandler({
 *   kernel: getKernelOrNull,
 *   onUncaughtError: (err) => myLogger.fatal(err),
 * });
 * ```
 */
export function reactErrorHandler(options: ReactErrorHandlerOptions = {}) {
  const { kernel: kernelSource, ...callbacks } = options;
  const repeatGate = createRepeatGate();

  const capture = (
    // oxlint-disable-next-line anti-slop/no-unknown-parameters -- React forwards arbitrary thrown JavaScript values to root error callbacks.
    error: unknown,
    info: ReactRootErrorInfo,
    mechanism: ErrorMechanism,
  ) => {
    if (repeatGate(`${mechanism.type}|${exceptionKey(toException(error))}`)) {
      captureReactError(
        resolveKernel(kernelSource),
        error,
        info.componentStack,
        mechanism,
      );
    }
  };

  return {
    onCaughtError(error, info) {
      capture(error, info, {
        type: MECHANISM_TYPE.react.caughtError,
        handled: true,
      });

      callbacks.onCaughtError?.(error, info);
    },
    onRecoverableError(error, info) {
      capture(error, info, {
        type: MECHANISM_TYPE.react.recoverableError,
        handled: true,
        synthetic: true,
      });

      callbacks.onRecoverableError?.(error, info);
    },
    onUncaughtError(error, info) {
      capture(error, info, {
        type: MECHANISM_TYPE.react.uncaughtError,
        handled: false,
      });

      callbacks.onUncaughtError?.(error, info);
    },
  } satisfies ReactRootErrorHandlers;
}
