import { fileURLToPath } from "node:url";

import { playwright } from "@vitest/browser-playwright";
import { defineProject, mergeConfig } from "vitest/config";
import type { Plugin, ViteUserConfig } from "vitest/config";

import { base } from "./base.ts";
import { browserNetworkPlugin } from "../browser/browser-network-plugin.ts";
import { observePageEvents } from "../browser/browser-page-events.commands.ts";
import { parkPointer } from "../browser/browser-pointer.commands.ts";
import {
  closeBrowserPopups,
  printDocument,
  setBrowserMetadata,
  setClipboardAccess,
  setColorScheme,
  setMediaType,
  setReducedMotion,
  setVirtualAuthenticator,
} from "../browser/browser-platform.commands.ts";

const executablePath = process.env["CHROMIUM_EXECUTABLE_PATH"];

const packageDirectory = fileURLToPath(new URL("../../", import.meta.url));

// WARNING: Vite serves /@fs/ files only from the consumer's workspace root, so a project rooted outside this repository (vitest-config's own browser.test.ts) could not import the shared setup file below.
const serveSharedSetupFiles = {
  name: "interfere-vitest-config-setup-files",
  configResolved(config) {
    config.server.fs.allow.push(packageDirectory);
  },
} satisfies Plugin;

export const browser = (
  overrides: ViteUserConfig = {},
  launchOptions: NonNullable<
    Parameters<typeof playwright>[0]
  >["launchOptions"] = {},
) => {
  const options: NonNullable<Parameters<typeof playwright>[0]> = {
    launchOptions,
    contextOptions: { permissions: ["clipboard-read", "clipboard-write"] },
  };

  if (executablePath) {
    options.launchOptions = { ...launchOptions, executablePath };
  }

  const provider = playwright(options);

  delete provider.prewarm;

  return mergeConfig(
    mergeConfig(
      base,
      defineProject({
        plugins: [browserNetworkPlugin, serveSharedSetupFiles],
        test: {
          browser: {
            commands: {
              closeBrowserPopups,
              observePageEvents,
              parkPointer,
              printDocument,
              setBrowserMetadata,
              setClipboardAccess,
              setColorScheme,
              setMediaType,
              setReducedMotion,
              setVirtualAuthenticator,
            },
            enabled: true,
            headless: !process.env["UI"],
            instances: [{ browser: "chromium" }],
            locators: { exact: false },
            provider,
            screenshotFailures: false,
          },
          include: ["**/!(*.visual).browser.test.{ts,tsx}"],
          setupFiles: [
            fileURLToPath(
              new URL("../browser/browser-page-events.ts", import.meta.url),
            ),
            fileURLToPath(
              new URL("../browser/browser-pointer.ts", import.meta.url),
            ),
            fileURLToPath(
              new URL(
                "../browser/browser-storage-isolation.ts",
                import.meta.url,
              ),
            ),
          ],
        },
      }),
    ),
    overrides,
  );
};
