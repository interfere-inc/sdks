import type { Page } from "playwright";
import { defineBrowserCommand } from "@vitest/browser-playwright";

const observed = new WeakMap<Page, { testPath: string | undefined }>();

function report(event: string, state: { testPath: string | undefined }) {
  if (state.testPath === undefined) {
    return;
  }

  process.stderr.write(
    `[vitest-config] browser page ${event} while ${state.testPath} was running; a VITEST_BROWSER_CONNECTION_CLOSED failure in that file has this cause\n`,
  );
}

// WARNING: Vitest reports a renderer crash or the orchestrator page navigating away (window.top.location from a test, a redirect) only as "Browser connection was closed"; Playwright's framenavigated arrives after the unload that closes the connection and loses to the pool teardown, so the CDP navigation request, which fires before unload, is logged with the file that was running. The test path is cleared in afterAll so events after a file stay silent.
export const observePageEvents = defineBrowserCommand(
  async ({ context, page, testPath }, running: boolean) => {
    const current = running ? testPath : undefined;
    const existing = observed.get(page);

    if (existing) {
      existing.testPath = current;

      return;
    }

    const state = { testPath: current };

    observed.set(page, state);

    page.on("crash", () => {
      report("crashed", state);
    });

    if (context.browser()?.browserType().name() !== "chromium") {
      page.on("framenavigated", (frame) => {
        if (frame === page.mainFrame()) {
          report(`navigated to ${frame.url()}`, state);
        }
      });

      return;
    }

    const session = await context.newCDPSession(page);

    await session.send("Page.enable");

    const { frameTree } = await session.send("Page.getFrameTree");

    session.on("Page.frameRequestedNavigation", (event) => {
      if (event.frameId === frameTree.frame.id) {
        report(`requested a ${event.reason} navigation to ${event.url}`, state);
      }
    });
  },
);
