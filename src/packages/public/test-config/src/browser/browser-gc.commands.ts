import { defineBrowserCommand } from "@vitest/browser-playwright";

export const collectGarbage = defineBrowserCommand(
  async ({ context, page }) => {
    const session = await context.newCDPSession(page);

    try {
      await session.send("HeapProfiler.collectGarbage");
    } finally {
      await session.detach();
    }
  },
);
