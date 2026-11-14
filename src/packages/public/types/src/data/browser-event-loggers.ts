/** Instrumentation scope the v12 browser SDK and the pinned upstream navigation instrumentation emit page-view log records under. */
export const BROWSER_NAVIGATION_LOGGER_NAME =
  "@opentelemetry/instrumentation-browser-navigation";

/** Instrumentation scope the v13 browser SDK emits its own page-view and user-action events under. */
export const INTERFERE_BROWSER_EVENTS_LOGGER_NAME = "@interfere/browser-events";

/** Instrumentation scopes a reader scans for page-view and user-action events, v12 and v13. */
export const BROWSER_EVENT_LOGGER_NAMES = [
  BROWSER_NAVIGATION_LOGGER_NAME,
  INTERFERE_BROWSER_EVENTS_LOGGER_NAME,
] as const;
