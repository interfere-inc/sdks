import { omitUndefined } from "@interfere/helpers/omit-undefined";

import { defineConfig, devices } from "@playwright/test";
import type { PlaywrightTestConfig } from "@playwright/test";

const policy = omitUndefined({
  fullyParallel: true,
  forbidOnly: Boolean(process.env["CI"]),
  retries: 0,
  reporter: (process.env["CI"]
    ? [
        ["github"],
        [
          "junit",
          {
            outputFile: "./test-results/e2e-junit.xml",
            includeProjectInTestName: true,
          },
        ],
      ]
    : [["html"]]) satisfies NonNullable<PlaywrightTestConfig["reporter"]>,
  use: omitUndefined({
    screenshot: "only-on-failure" as const,
    trace: "retain-on-failure" as const,
    actionTimeout: process.env["CI"] ? 10_000 : undefined,
    launchOptions: process.env["CHROMIUM_EXECUTABLE_PATH"]
      ? { executablePath: process.env["CHROMIUM_EXECUTABLE_PATH"] }
      : undefined,
  }),
  expect: process.env["CI"] ? { timeout: 10_000 } : undefined,
  projects: [
    {
      grep: /@desktop|@common/u,
      name: "desktop",
      use: { ...devices["Desktop Chrome"] },
    },
    {
      grep: /@mobile|@common/u,
      name: "mobile",
      use: {
        ...devices["Desktop Chrome"],
        hasTouch: true,
        isMobile: true,
        viewport: { height: 844, width: 390 },
      },
    },
  ],
}) satisfies PlaywrightTestConfig;

const port = Number(process.env["SDK_BUILD_TEST_PORT"] ?? 3037);

const config = defineConfig(policy, {
  testDir: "./test/e2e",
  outputDir: "./test-results/e2e",
  testMatch: "**/*.e2e.ts",
  workers: 1,
  use: { baseURL: `http://127.0.0.1:${port}` },
  webServer: {
    command: "bun test/serve-next.ts",
    env: { SDK_BUILD_TEST_PORT: String(port) },
    gracefulShutdown: { signal: "SIGTERM", timeout: 5000 },
    reuseExistingServer: false,
    timeout: 180_000,
    url: `http://127.0.0.1:${port}`,
  },
});

if (!process.env["CI"]) {
  config.reporter = [["list"]];
}

export default config;
