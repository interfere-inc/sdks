"use client";

import { MECHANISM_TYPE } from "@interfere/types/sdk/errors";

import { Component } from "react";
import type { ContextType, ErrorInfo, ReactNode } from "react";

import { captureReactError } from "./internal/capture/capture.js";
import { InterfereContext } from "./internal/react-context.js";

export interface ErrorBoundaryProps {
  children: ReactNode;
  // oxlint-disable-next-line anti-slop/no-unknown-parameters -- React forwards arbitrary thrown JavaScript values to boundary callbacks.
  fallback?: ReactNode | ((error: unknown, reset: () => void) => ReactNode);
  // oxlint-disable-next-line anti-slop/no-unknown-parameters -- React forwards arbitrary thrown JavaScript values to boundary callbacks.
  onError?: (error: unknown, info: ErrorInfo) => void;
}

interface ErrorBoundaryState {
  caught: { readonly error: unknown } | null;
}

/**
 * Catches render-phase React errors, reports them to the SDK, and renders a
 * fallback. Reads the kernel from `InterfereContext`; mounting outside the
 * provider is supported — capture is silently skipped, fallback + `onError`
 * still fire.
 *
 * Class component required — React has no hook-based error boundary API.
 */
export class ErrorBoundary extends Component<
  ErrorBoundaryProps,
  ErrorBoundaryState
> {
  static override contextType = InterfereContext;
  declare context: ContextType<typeof InterfereContext>;

  override state: ErrorBoundaryState = { caught: null };

  // oxlint-disable-next-line anti-slop/no-unknown-parameters -- React forwards arbitrary thrown JavaScript values to boundary lifecycle methods.
  static getDerivedStateFromError(error: unknown) {
    return { caught: { error } } satisfies ErrorBoundaryState;
  }

  // oxlint-disable-next-line anti-slop/no-unknown-parameters -- React forwards arbitrary thrown JavaScript values to boundary lifecycle methods.
  override componentDidCatch(error: unknown, info: ErrorInfo) {
    captureReactError(
      this.context?.kernel ?? null,
      error,
      info.componentStack,
      {
        handled: true,
        type: MECHANISM_TYPE.react.errorBoundary,
      },
    );

    this.props.onError?.(error, info);
  }

  private readonly reset = () => {
    // oxlint-disable-next-line react/no-set-state -- Resetting a class error boundary requires its native state API.
    this.setState({ caught: null });
  };

  // oxlint-disable-next-line typescript/promise-function-async
  override render() {
    const { caught } = this.state;

    if (caught) {
      const { fallback } = this.props;

      // oxlint-disable-next-line anti-slop/no-runtime-typeof -- React fallback content and an error/reset render callback are distinct supported prop variants.
      if (typeof fallback === "function") {
        return fallback(caught.error, this.reset);
      }

      return fallback ?? null;
    }

    return this.props.children;
  }
}
