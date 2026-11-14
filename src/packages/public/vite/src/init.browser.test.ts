import { commands } from "@interfere/test-config/browser-platform";
import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { expect, onTestFinished, test } from "vitest";

import { close, init } from "./init";

test("returns null when the native browser user agent is a bot", async () => {
  setInjectedConfiguration("__INTERFERE_FORCE_ENABLE__", false);

  onTestFinished(async () => {
    await close();

    await commands.setBrowserMetadata("reset");
  });

  await commands.setBrowserMetadata({
    userAgent:
      "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
    language: "en-US",
    screen: { width: 1440, height: 900 },
  });

  expect(navigator.userAgent).toContain("Googlebot");

  await expect(init({ enabled: true })).resolves.toBeNull();
});
