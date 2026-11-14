"use client";

import { MECHANISM_TYPE } from "@interfere/types/sdk/errors";

import { Component } from "react";
import type { ContextType, ReactNode } from "react";

import { captureReactError } from "./capture.js";
import { errorFingerprint } from "./turn-dedupe.js";
import { InterfereContext } from "../react-context.js";
import { hasServerDigest } from "../server-digest/server-digest.js";

const capturing = new Set<unknown>();

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- React boundaries receive arbitrary thrown values, including null and strings.
function captureKey(thrown: unknown) {
  return thrown instanceof Error ? errorFingerprint(thrown) : thrown;
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- React boundaries receive arbitrary thrown values, including null and strings.
function markCapturing(key: unknown) {
  capturing.add(key);

  queueMicrotask(() => {
    capturing.delete(key);
  });
}

interface CaptureBoundaryProps {
  children: ReactNode;
}

interface CaptureBoundaryState {
  caught: { readonly error: Error } | null;
}

/**
 * Internal boundary used by `<InterfereProvider>` to capture render-phase
 * React errors without changing the app's UX.
 *
 * Unlike the public `ErrorBoundary`, this boundary always re-throws the
 * captured error in its `render()` so upstream boundaries — the customer's
 * own `ErrorBoundary`, Next.js's `error.tsx` / `global-error.tsx`, or React's
 * default unmount — keep control of what the user sees. The net effect:
 * zero visual change, full capture coverage for any render-phase error in
 * the subtree.
 *
 * Capture happens inside `getDerivedStateFromError` rather than
 * `componentDidCatch` because `componentDidCatch` does not fire on a boundary
 * that re-throws in render — React considers such a boundary to have failed
 * and skips its commit-phase lifecycle. The trade-off: we don't get
 * `errorInfo.componentStack` in this capture. Callers who want the component
 * tree should use the public `ErrorBoundary` (which renders a fallback and
 * therefore receives `componentDidCatch`), or pass
 * {@link reactErrorHandler} to `createRoot()`.
 *
 * React retries a failed render synchronously, so each thrown value is captured
 * once per task; non-Error values are boxed in state so `throw null` still
 * reaches the host boundary.
 *
 * `mechanism.handled` is `false` because from Interfere's perspective we
 * captured the error but did not render a fallback — propagation continues.
 */
export class CaptureBoundary extends Component<
  CaptureBoundaryProps,
  CaptureBoundaryState
> {
  static override contextType = InterfereContext;
  declare context: ContextType<typeof InterfereContext>;

  override state: CaptureBoundaryState = { caught: null };

  static getDerivedStateFromError(error: Error) {
    return { caught: { error } } satisfies CaptureBoundaryState;
  }

  // oxlint-disable-next-line typescript/promise-function-async
  override render() {
    const { caught } = this.state;

    if (caught) {
      const key = captureKey(caught.error);

      if (!capturing.has(key) && !hasServerDigest(caught.error)) {
        markCapturing(key);

        captureReactError(this.context?.kernel ?? null, caught.error, null, {
          handled: false,
          type: MECHANISM_TYPE.react.captureBoundary,
        });
      }

      throw caught.error;
    }

    return this.props.children;
  }
}
