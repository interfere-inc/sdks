import { isBot } from "@ua-parser-js/pro-enterprise/bot-detection";

const AUTOMATION_UA_RE = /headless|electron|puppeteer|playwright/iu;

/**
 * `isBot` covers the union of the Crawlers, Fetchers, CLIs, and Libraries
 * extensions, so `isAICrawler` / `isAIAssistant` are strict subsets that
 * don't need separate checks. Returns `false` outside the browser (no UA
 * to inspect).
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

  return isBot(browser.userAgent);
}
