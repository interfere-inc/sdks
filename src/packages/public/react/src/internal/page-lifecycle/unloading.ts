let unloading = false;

/** Records whether the page is inside a `pagehide`, until it is shown again. */
export function setPageUnloading(value: boolean) {
  unloading = value;
}

/** Whether the page has fired `pagehide` and not been shown since, so exports must fit a keepalive request. */
export function isPageUnloading() {
  return unloading;
}
