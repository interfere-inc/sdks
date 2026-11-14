import type { Context } from "@opentelemetry/api";
import type { EventLoggerProvider } from "./event-logger.js";

import type { Kernel } from "../kernel.js";
import { installViewTracker } from "./navigation.js";
import { installUserActions } from "./user-actions.js";

export interface ViewCorrelationInput {
  /** Fresh stamped SSR context for the initial page view; null when nothing reads it. */
  documentContext: Context | null;
  kernel: Pick<
    Kernel,
    | "isInteractionCaptureEnabled"
    | "isPageEventsCaptureEnabled"
    | "privacyPolicy"
    | "redactUrl"
    | "views"
  >;
  loggerProvider: EventLoggerProvider;
  resolveRoute?: ((pathname: string) => string | undefined) | undefined;
}

/**
 * Runs the view tracker and user actions for one kernel: views rotate on
 * every real URL change and an action id is current while an admitted
 * click or submit dispatches, with or without a tracer. Page-view and
 * user-action events go to the given logger provider.
 */
export function installViewCorrelation(input: ViewCorrelationInput) {
  const { kernel } = input;

  const viewTracker = installViewTracker({
    documentContext: input.documentContext,
    isEnabled: () => kernel.isPageEventsCaptureEnabled(),
    loggerProvider: input.loggerProvider,
    redactUrl: kernel.redactUrl,
    resolveRoute: input.resolveRoute,
    views: kernel.views,
  });

  const disposeUserActions = installUserActions({
    isEnabled: () =>
      kernel.isPageEventsCaptureEnabled() &&
      kernel.isInteractionCaptureEnabled(),
    loggerProvider: input.loggerProvider,
    privacyPolicy: kernel.privacyPolicy,
    redactUrl: kernel.redactUrl,
    resolveRoute: input.resolveRoute,
    views: kernel.views,
  });

  return {
    dispose: () => {
      disposeUserActions();

      viewTracker.dispose();
    },
    recordCurrentView: viewTracker.recordCurrentView,
  };
}
