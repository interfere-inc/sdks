// WARNING: beforeunload also fires for navigations the user cancels, so the leaving state lapses after this long instead of lasting until the page is shown again.
const LEAVING_GRACE_MS = 5000;

let unloading = false;

let leavingAt: number | null = null;

/** Records whether the page is inside a `pagehide`, until it is shown again. */
export function setPageUnloading(value: boolean) {
  unloading = value;
}

/** Whether the page has fired `pagehide` and not been shown since, so exports must fit a keepalive request. */
export function isPageUnloading() {
  return unloading;
}

/** Records an unloading signal (`beforeunload` or `pagehide`), or clears it when the page is shown again. */
export function setPageLeaving(value: boolean) {
  leavingAt = value ? performance.now() : null;
}

/** Whether an unloading signal fired recently and the page has not been shown since, so a request that is not keepalive may be cancelled by the navigation. */
export function isPageLeaving() {
  return (
    unloading ||
    (leavingAt !== null && performance.now() - leavingAt < LEAVING_GRACE_MS)
  );
}
