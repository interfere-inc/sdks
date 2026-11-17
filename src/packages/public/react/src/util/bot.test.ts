import { execFile } from "node:child_process";
import { promisify } from "node:util";
import { expect, test } from "vitest";

import { classifyBrowserAutomation } from "./bot.js";

const execute = promisify(execFile);

test("returns false in native Node without browser globals", async () => {
  const source = new URL("bot.ts", import.meta.url).href;

  const { stdout } = await execute(process.execPath, [
    "--no-experimental-global-navigator",
    "--input-type=module",
    "-e",
    `import { isBotUserAgent } from ${JSON.stringify(source)}; process.stdout.write(JSON.stringify(isBotUserAgent()));`,
  ]);

  expect(stdout).toBe("false");
});

test.each([
  [
    "a known bot UA",
    {
      userAgent:
        "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
    },
    true,
  ],
  [
    "an AI fetcher",
    {
      userAgent:
        "Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko); compatible; ChatGPT-User/1.0; +https://openai.com/bot",
    },
    true,
  ],
  [
    "headless Chrome",
    {
      userAgent:
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) HeadlessChrome/141.0.7390.0 Safari/537.36",
    },
    true,
  ],
  [
    "a driven browser that spoofs a human UA",
    {
      userAgent:
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/153.0.0.0 Safari/537.36",
      webdriver: true,
    },
    true,
  ],
  [
    "an AI assistant fetching on a user's behalf",
    {
      userAgent:
        "Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko; compatible; Claude-User/1.0; +Claude-User@anthropic.com)",
    },
    true,
  ],
  [
    "an in-app browser",
    {
      userAgent:
        "Mozilla/5.0 (iPhone; CPU iPhone OS 18_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 Instagram 350.0.0",
    },
    false,
  ],
  [
    "a normal Chrome UA",
    {
      userAgent:
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36",
    },
    false,
  ],
])("returns the expected verdict for %s", (_case, browser, expected) => {
  expect(classifyBrowserAutomation({ webdriver: false, ...browser })).toBe(
    expected,
  );
});
