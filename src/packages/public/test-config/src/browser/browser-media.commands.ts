import { defineBrowserCommand } from "@vitest/browser-playwright";

export const setReducedMotion = defineBrowserCommand(
  async ({ page }, preference: "reduce" | "no-preference") => {
    await page.emulateMedia({ reducedMotion: preference });
  },
);

export const setColorScheme = defineBrowserCommand(
  async ({ page }, preference: "light" | "dark") => {
    await page.emulateMedia({ colorScheme: preference });
  },
);

export const setMediaType = defineBrowserCommand(
  async ({ page }, media: "print" | "screen") => {
    await page.emulateMedia({ media });
  },
);

export const printDocument = defineBrowserCommand(async ({ page }) => {
  await page.pdf();
});
