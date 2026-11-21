import { INTERFERE_BROWSER_EVENTS_LOGGER_NAME } from "@interfere/types/data/browser-event-loggers";
import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { PAGE_VIEW_EVENT_NAME } from "@interfere/types/data/correlation-events";
import { ROOT_CONTEXT, SeverityNumber } from "@opentelemetry/api";
import type { Context } from "@opentelemetry/api";
import type { EventLoggerProvider } from "./event-logger.js";

import type { View, ViewIds, ViewTrigger } from "../../tracking/view.js";
import { currentUrlAttrs } from "./url-attrs.js";

const ATTR_NAVIGATION_TYPE = "browser.navigation.type" as const;

const ATTR_VIEW_TRIGGER = "interfere.view.trigger" as const;

const NAVIGATION_TYPES = {
  pageload: "pageload",
  popstate: "traverse",
  push: "push",
  replace: "replace",
} satisfies Partial<Record<ViewTrigger, string>>;

export interface ViewTrackerInput {
  /** Fresh stamped SSR context from the document meta, sampled or not; the initial page view's log record carries it. */
  documentContext: Context | null;
  /** Whether page-view events are emitted now (pageEvents consent and config). */
  isEnabled: () => boolean;
  loggerProvider: EventLoggerProvider;
  redactUrl?: ((url: string) => string) | undefined;
  resolveRoute?: ((pathname: string) => string | undefined) | undefined;
  views: ViewIds;
}

export interface ViewTracker {
  dispose: () => void;
  /** Emits the current view's page-view event if it has not been emitted yet; called when page events are enabled. */
  recordCurrentView: () => void;
}

function comparableHref(href: string) {
  const url = new URL(href);

  const pathname =
    url.pathname !== "/" && url.pathname.endsWith("/")
      ? url.pathname.slice(0, -1)
      : url.pathname;

  return `${url.origin}${pathname}${url.search}`;
}

/**
 * Starts a new view on every real URL change (history push/replace or
 * popstate; hash-only changes excluded) and emits one `browser.navigation`
 * page-view event per view. Views are correlation, not ancestry: nothing is
 * parented to a page or navigation span, and every Interfere record carries
 * the view id through the identity processor instead.
 */
export function installViewTracker(input: ViewTrackerInput) {
  const { views } = input;
  const redactUrl = input.redactUrl ?? ((url: string) => url);
  let lastEmittedId: string | undefined;

  const logger = input.loggerProvider.getLogger(
    INTERFERE_BROWSER_EVENTS_LOGGER_NAME,
  );

  let active = true;

  const emitView = (view: View) => {
    if (!active || lastEmittedId === view.id || !input.isEnabled()) {
      return;
    }

    lastEmittedId = view.id;

    const navigationType =
      view.trigger === "max_age" ? undefined : NAVIGATION_TYPES[view.trigger];

    logger.emit({
      attributes: omitUndefined({
        ...currentUrlAttrs(input.resolveRoute, redactUrl),
        [ATTR_NAVIGATION_TYPE]: navigationType,
        [ATTR_VIEW_TRIGGER]: view.trigger,
      }),
      context:
        view.trigger === "pageload" && input.documentContext
          ? input.documentContext
          : ROOT_CONTEXT,
      eventName: PAGE_VIEW_EVENT_NAME,
      severityNumber: SeverityNumber.UNSPECIFIED,
    });
  };

  const unsubscribe = views.onStart(emitView);

  const recordCurrentView = () => {
    emitView(views.current());
  };

  const disposeNavigation = installViewNavigation(views);

  return {
    dispose: () => {
      active = false;

      unsubscribe();

      disposeNavigation();
    },
    recordCurrentView,
  } satisfies ViewTracker;
}

/** Rotates view identity on history changes, including before event capture is ready. */
export function installViewNavigation(views: ViewIds) {
  let active = true;

  if (typeof window === "undefined") {
    return () => {
      active = false;
    };
  }

  let lastHref = comparableHref(globalThis.location.href);

  const onNavigate = (trigger: ViewTrigger) => {
    if (!active) {
      return;
    }

    const current = comparableHref(globalThis.location.href);

    if (current === lastHref) {
      return;
    }

    lastHref = current;

    views.start(trigger);
  };

  // oxlint-disable-next-line typescript/unbound-method
  const originalPushState = globalThis.history.pushState;
  // oxlint-disable-next-line typescript/unbound-method
  const originalReplaceState = globalThis.history.replaceState;

  const pushState: History["pushState"] = function pushState(
    this: History,
    ...args: Parameters<History["pushState"]>
  ) {
    originalPushState.apply(this, args);

    onNavigate("push");
  };

  const replaceState: History["replaceState"] = function replaceState(
    this: History,
    ...args: Parameters<History["replaceState"]>
  ) {
    originalReplaceState.apply(this, args);

    onNavigate("replace");
  };

  globalThis.history.pushState = pushState;
  globalThis.history.replaceState = replaceState;

  const onPopstate = () => {
    onNavigate("popstate");
  };

  globalThis.addEventListener("popstate", onPopstate);

  return () => {
    active = false;

    if (globalThis.history.pushState === pushState) {
      globalThis.history.pushState = originalPushState;
    }

    if (globalThis.history.replaceState === replaceState) {
      globalThis.history.replaceState = originalReplaceState;
    }

    globalThis.removeEventListener("popstate", onPopstate);
  };
}
