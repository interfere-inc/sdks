import { isbot } from "isbot";

const AUTOMATION_UA_RE = /headless|electron|puppeteer|playwright/iu;

/**
 * `isbot` matches crawlers, fetchers, AI assistants, CLIs and HTTP
 * libraries. Returns `false` outside the browser (no UA to inspect).
 */
export function isBotUserAgent() {
  if (typeof navigator === "undefined") {
    return false;
  }

  return classifyBrowserAutomation(navigator);
}

export function classifyBrowserAutomation(
  browser: Pick<Navigator, "userAgent" | "webdriver">,
) {
  if (browser.webdriver) {
    return true;
  }

  if (AUTOMATION_UA_RE.test(browser.userAgent)) {
    return true;
  }

  return isbot(browser.userAgent);
}
