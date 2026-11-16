import { setPageLeaving, setPageUnloading } from "./unloading.js";

export { reserveKeepaliveBody } from "./keepalive-budget.js";

type Handler = () => void;

const hiddenHandlers = new Set<Handler>();

const shownHandlers = new Set<Handler>();

const unloadingHandlers = new Set<Handler>();

let installed = false;

let hidden = false;

function fire(set: Set<Handler>) {
  for (const handler of set) {
    try {
      handler();
    } catch {
      continue;
    }
  }
}

function documentHidden() {
  // oxlint-disable-next-line typescript/no-unnecessary-condition -- Server hosts have no document; they are never hidden.
  return globalThis.document?.visibilityState === "hidden";
}

// WARNING: pagehide and visibilitychange both fire on every navigation; hidden and shown only fire on a state transition so subscribers run once.
function fireHidden() {
  if (hidden) {
    return;
  }

  hidden = true;

  fire(hiddenHandlers);
}

function fireShown() {
  if (!hidden) {
    return;
  }

  hidden = false;

  setPageUnloading(false);

  setPageLeaving(false);

  fire(shownHandlers);
}

function fireUnloading() {
  setPageLeaving(true);

  fire(unloadingHandlers);
}

// WARNING: unloading subscribers must run before the page is marked hidden; exporters read isPageHidden() to choose between a keepalive send and carryover.
function onPageHide() {
  setPageUnloading(true);

  fireUnloading();

  fireHidden();
}

function onVisibilityChange() {
  if (documentHidden()) {
    fireHidden();

    return;
  }

  fireShown();
}

function onPageShow() {
  setPageLeaving(false);

  if (!documentHidden()) {
    fireShown();
  }
}

function install() {
  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Server and partial browser hosts may not provide event listeners; subscriptions remain inert until a callable host API exists.
  if (installed || typeof globalThis.addEventListener !== "function") {
    return;
  }

  installed = true;

  hidden = documentHidden();

  globalThis.addEventListener("visibilitychange", onVisibilityChange);

  globalThis.addEventListener("pagehide", onPageHide);

  globalThis.addEventListener("pageshow", onPageShow);

  // WARNING: beforeunload is an early flush whose ordinary fetches the navigation may cancel, so exports it starts must fit keepalive or be retained; it fires for cancelled navigations and is skipped by mobile Safari, so it must never mark the page hidden or replace the pagehide flush.
  globalThis.addEventListener("beforeunload", fireUnloading);
}

export function onPageHidden(handler: Handler) {
  install();

  hiddenHandlers.add(handler);

  return (() => {
    hiddenHandlers.delete(handler);
  }) satisfies () => void;
}

export function onPageShown(handler: Handler) {
  install();

  shownHandlers.add(handler);

  return (() => {
    shownHandlers.delete(handler);
  }) satisfies () => void;
}

/** Whether the page is hidden, as tracked by the same `visibilitychange`, `pagehide` and `pageshow` signals the hidden and shown handlers use. */
export function isPageHidden() {
  install();

  return hidden;
}

/** Fires on `beforeunload` while the navigation can still be cancelled, and again on every `pagehide`, including entry into the back-forward cache; the page may survive either, so subscribers flush and keep running. */
export function onPageUnloading(handler: Handler) {
  install();

  unloadingHandlers.add(handler);

  return (() => {
    unloadingHandlers.delete(handler);
  }) satisfies () => void;
}

/** Test hook — drops every handler and unbinds the listeners. */
export function resetPageLifecycleForTests() {
  hiddenHandlers.clear();

  shownHandlers.clear();

  unloadingHandlers.clear();

  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Cleanup must tolerate a host that removed its event API after installation.
  if (installed && typeof globalThis.removeEventListener === "function") {
    globalThis.removeEventListener("visibilitychange", onVisibilityChange);

    globalThis.removeEventListener("pagehide", onPageHide);

    globalThis.removeEventListener("pageshow", onPageShow);

    globalThis.removeEventListener("beforeunload", fireUnloading);
  }

  installed = false;

  hidden = false;

  setPageUnloading(false);

  setPageLeaving(false);
}
