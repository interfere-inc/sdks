import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { INTERFERE_BROWSER_EVENTS_LOGGER_NAME } from "@interfere/types/data/browser-event-loggers";
import { USER_ACTION_EVENT_NAME } from "@interfere/types/data/correlation-events";
import { ROOT_CONTEXT } from "@opentelemetry/api";
import type { Attributes } from "@opentelemetry/api";
import { SeverityNumber } from "@opentelemetry/api-logs";
import type { EventLoggerProvider } from "./event-logger.js";
import { getElementXPath } from "@opentelemetry/sdk-trace-web";
import { v7 as uuidv7 } from "uuid";

import type { ViewIds } from "../../tracking/view.js";
import { closestActionable, describeActionable } from "../dom/actionable.js";
import type { PrivacyPolicy } from "../dom/privacy.js";
import { createClickSpanGate } from "./instrumentations/click-span-gate.js";
import { currentUrlAttrs } from "./url-attrs.js";

const ACTION_EVENTS = ["click", "submit"] as const;

function isAction(type: string) {
  return type === "click" || type === "submit";
}

export interface UserActionsInput {
  /** Whether click and submit events are emitted now (pageEvents consent and interaction config). */
  isEnabled: () => boolean;
  loggerProvider: EventLoggerProvider;
  privacyPolicy: PrivacyPolicy;
  redactUrl?: ((url: string) => string) | undefined;
  resolveRoute?: ((pathname: string) => string | undefined) | undefined;
  views: ViewIds;
}

function interactionAttrs(
  eventType: string,
  element: Element,
  policy: PrivacyPolicy,
) {
  const desc = describeActionable(element, policy);

  const attributes: Attributes = omitUndefined({
    "ui.event_type": eventType,
    "ui.target.tag": desc.tag,
    "ui.target.label": desc.label || undefined,
    "ui.target.id": desc.id || undefined,
    "ui.target.name": desc.name || undefined,
    "ui.target.role": desc.role || undefined,
    "ui.target.aria_label": desc.ariaLabel || undefined,
    "ui.target.text": desc.text || undefined,
  });

  if (desc.privacy) {
    attributes["ui.target.privacy"] = desc.privacy;

    attributes["target_xpath"] = getElementXPath(element, false);
  }

  return attributes;
}

function actionTarget(event: Event) {
  if (event.type === "submit") {
    return event.target instanceof HTMLFormElement ? event.target : null;
  }

  return closestActionable(event.target);
}

/**
 * Emits one `interfere.user_action` event per admitted click or submit and
 * runs click and submit listeners registered after install inside its action
 * id, so only a handler's synchronous records carry it.
 */
export function installUserActions(input: UserActionsInput) {
  if (typeof window === "undefined") {
    // oxlint-disable-next-line eslint/no-empty-function
    return (() => {}) satisfies () => void;
  }

  const { views } = input;
  const redactUrl = input.redactUrl ?? ((url: string) => url);
  const admitClick = createClickSpanGate();

  const logger = input.loggerProvider.getLogger(
    INTERFERE_BROWSER_EVENTS_LOGGER_NAME,
  );

  const actions = new WeakMap<Event, string>();

  const scoped = new WeakMap<
    EventListenerOrEventListenerObject,
    EventListener
  >();

  let active = true;

  const scope = (listener: EventListenerOrEventListenerObject) => {
    function wrapped(this: EventTarget, event: Event) {
      views.runAction(actions.get(event) ?? views.actionId(), () => {
        if ("handleEvent" in listener) {
          listener.handleEvent(event);
        } else {
          listener.call(this, event);
        }
      });
    }

    const existing = scoped.get(listener) ?? wrapped;

    scoped.set(listener, existing);

    return existing;
  };

  const onCapture = (event: Event) => {
    if (actions.has(event) || !input.isEnabled()) {
      return;
    }

    const element = actionTarget(event);

    if (!element || (event.type === "click" && !admitClick(element))) {
      return;
    }

    const id = uuidv7();

    actions.set(event, id);

    views.runAction(id, () => {
      logger.emit({
        attributes: {
          ...interactionAttrs(event.type, element, input.privacyPolicy),
          ...currentUrlAttrs(input.resolveRoute, redactUrl),
        },
        context: ROOT_CONTEXT,
        eventName: USER_ACTION_EVENT_NAME,
        severityNumber: SeverityNumber.UNSPECIFIED,
      });
    });
  };

  const proto = EventTarget.prototype;
  // oxlint-disable-next-line typescript/unbound-method
  const originalAdd = proto.addEventListener;
  // oxlint-disable-next-line typescript/unbound-method
  const originalRemove = proto.removeEventListener;

  const addEventListener: EventTarget["addEventListener"] =
    function addEventListener(this: EventTarget, type, listener, options) {
      originalAdd.call(
        this,
        type,
        listener && active && isAction(type) ? scope(listener) : listener,
        options,
      );
    };

  // WARNING: Never restore removeEventListener; a listener scoped while active could no longer be removed and would keep firing.
  const removeEventListener: EventTarget["removeEventListener"] =
    function removeEventListener(this: EventTarget, type, listener, options) {
      originalRemove.call(this, type, listener, options);

      const wrapped = listener ? scoped.get(listener) : undefined;

      if (wrapped) {
        originalRemove.call(this, type, wrapped, options);
      }
    };

  proto.addEventListener = addEventListener;
  proto.removeEventListener = removeEventListener;

  for (const type of ACTION_EVENTS) {
    originalAdd.call(globalThis, type, onCapture, { capture: true });
  }

  return (() => {
    active = false;

    for (const type of ACTION_EVENTS) {
      originalRemove.call(globalThis, type, onCapture, { capture: true });
    }

    if (proto.addEventListener === addEventListener) {
      proto.addEventListener = originalAdd;
    }
  }) satisfies () => void;
}
