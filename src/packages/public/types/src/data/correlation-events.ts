/** Span, log-record and span-event attribute carrying the page view a browser signal belongs to (UUIDv7, minted per view). */
export const INTERFERE_VIEW_ID_KEY = "interfere.view.id";

/** Span, log-record and span-event attribute carrying the user action a browser signal was produced inside (UUIDv7, minted per admitted click or submit). */
export const INTERFERE_ACTION_ID_KEY = "interfere.action.id";

/** Page-view event name: the pinned @opentelemetry/instrumentation-browser-navigation EVENT_NAME, which every v12 soft navigation already carries. */
export const PAGE_VIEW_EVENT_NAME = "browser.navigation";

/** User-action event name for admitted clicks and submits; semconv 1.43.0 defines no browser click event, only the mobile app.screen.click. */
export const USER_ACTION_EVENT_NAME = "interfere.user_action";

/** Attribute key ingest folds the OTLP log-record eventName into, so readers without an event-name column still identify events. */
export const EVENT_NAME_ATTRIBUTE_KEY = "event.name";

const uuidPattern =
  /^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/iu;

/** Whether a view or action id has the UUIDv7 shape the browser SDK mints; readers drop anything else. */
export function isCorrelationId(value: unknown): value is string {
  return typeof value === "string" && uuidPattern.test(value);
}
