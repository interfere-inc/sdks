import { playwright } from "@vitest/browser-playwright";
import { defineConfig } from "vitest/config";
import reactConfiguration from "./vitest.config.ts";

const project = reactConfiguration.test?.projects?.[1];

if (
  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Vitest's project union includes path strings and config functions; validate the imported configuration boundary.
  typeof project !== "object" ||
  !("test" in project) ||
  project.test?.name !== "browser"
) {
  throw new Error("React browser project is required for SDK qualification");
}

const provider = playwright();

delete provider.prewarm;

export default defineConfig({
  root: import.meta.dirname,
  test: {
    projects: [
      {
        ...project,
        test: {
          ...project.test,
          name: "sdk-browser",
          include: [
            "src/**/errors-*.browser.test.ts",
            "src/react-error-handler-lifecycle.browser.test.ts",
            "src/internal/page-lifecycle/*.browser.test.ts",
          ],
          browser: {
            ...project.test.browser,
            enabled: true,
            provider,
            instances: [{ browser: "firefox" }, { browser: "webkit" }],
          },
        },
      },
    ],
  },
});
