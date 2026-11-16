import {
  INTERFERE_ACTION_ID_KEY,
  INTERFERE_VIEW_ID_KEY,
} from "@interfere/types/data/correlation-events";
import { v7 as uuidv7 } from "uuid";

import { ATTR_SESSION_ID } from "./session.js";

const ATTR_DEVICE_ID = "device.id" as const;

/** Why a view started: the initial document load, a history change, or the max-age cut of a parked tab. */
export type ViewTrigger =
  | "max_age"
  | "pageload"
  | "popstate"
  | "push"
  | "replace";

export interface View {
  id: string;
  startedAtMs: number;
  trigger: ViewTrigger;
}

const VIEW_MAX_AGE_MS = 30 * 60 * 1000;

interface ViewIdsInput {
  maxAgeMs?: number;
  now?: () => number;
}

/**
 * Page-view and user-action identifiers for one kernel. The view id is
 * minted at construction (initial load) and by {@link ViewIds.start} on
 * each URL change; reading it after `maxAgeMs` starts a `max_age` view.
 * The action id is set only while {@link ViewIds.runAction} runs a click or
 * submit handler synchronously; nested actions restore the outer id.
 */
export class ViewIds {
  private readonly maxAgeMs: number;
  private readonly now: () => number;
  private readonly listeners = new Set<(view: View) => void>();
  private view: View;
  private action: string | null = null;

  constructor(input: ViewIdsInput = {}) {
    this.maxAgeMs = input.maxAgeMs ?? VIEW_MAX_AGE_MS;
    this.now = input.now ?? Date.now;
    this.view = this.mint("pageload");
  }

  private mint(trigger: ViewTrigger) {
    return { id: uuidv7(), startedAtMs: this.now(), trigger } satisfies View;
  }

  /** The current view, rotated to a `max_age` view when it has outlived the cap. */
  current() {
    if (this.now() - this.view.startedAtMs > this.maxAgeMs) {
      this.start("max_age");
    }

    return this.view;
  }

  viewId() {
    return this.current().id;
  }

  actionId() {
    return this.action;
  }

  /** Starts a new view and notifies listeners after the switch, so their records carry the new id. */
  start(trigger: ViewTrigger) {
    this.view = this.mint(trigger);

    for (const listener of this.listeners) {
      listener(this.view);
    }

    return this.view;
  }

  /** Subscribes to view starts after construction (soft navigations and max-age cuts). */
  onStart(listener: (view: View) => void) {
    this.listeners.add(listener);

    return () => {
      this.listeners.delete(listener);
    };
  }

  runAction<T>(id: string | null, fn: () => T) {
    const outer = this.action;

    this.action = id;

    try {
      return fn();
    } finally {
      this.action = outer;
    }
  }
}

export interface IdentityIds {
  getActionId?: (() => string | null) | undefined;
  getDeviceId?: (() => string | null) | undefined;
  getSessionId?: (() => string | null) | undefined;
  getViewId?: (() => string | null) | undefined;
}

/** The correlation stamps every Interfere span and log record carries; they never change parentage. */
export function identityAttributes(ids: IdentityIds) {
  const attributes: Record<string, string> = {};

  const stamps = [
    [ATTR_SESSION_ID, ids.getSessionId?.() ?? null],
    [INTERFERE_VIEW_ID_KEY, ids.getViewId?.() ?? null],
    [INTERFERE_ACTION_ID_KEY, ids.getActionId?.() ?? null],
    [ATTR_DEVICE_ID, ids.getDeviceId?.() ?? null],
  ] as const;

  for (const [key, value] of stamps) {
    if (value) {
      attributes[key] = value;
    }
  }

  return attributes;
}
