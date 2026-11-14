import { defineBrowserCommand } from "@vitest/browser-playwright";

export const parkPointer = defineBrowserCommand(async ({ page }) => {
  await page.mouse.move(-1, -1);
});
