/* oxlint-disable eslint/no-console */
import type { Page } from "@playwright/test";

export function reportChromiumFailures(page: Page) {
  page.on("pageerror", (error) => console.error(error));

  page.on("requestfailed", (request) =>
    console.error(
      `requestfailed: ${request.url()} ${request.failure()?.errorText ?? ""}`,
    ),
  );

  return page;
}
