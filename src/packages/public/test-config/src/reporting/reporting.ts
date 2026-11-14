import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

export const reporting = defineConfig({
  test: {
    fsModuleCachePath: resolve("node_modules/.vitest-cache"),
    reporters: [
      "default",
      [fileURLToPath(new URL("junit.ts", import.meta.url)), {}],
    ],
    outputFile: { junit: resolve("test-results/junit.xml") },
    coverage: {
      reportsDirectory: resolve("coverage"),
      exclude: [
        "**/dist/**",
        "**/*.d.{ts,mts,cts}",
        "**/*.test.{ts,tsx,mts,cts}",
        "**/test/**",
        "**/*.{fixture,mock}.{ts,tsx,mts,cts}",
        "**/{vitest,vite,tsdown}.config.{ts,mts,cts}",
        "**/*.gen.{ts,tsx,mts,cts}",
        "**/vitest-config/src/browser-{network,storage}.ts",
      ],
      include: ["src/**/*.{ts,tsx,mts,cts}"],
      provider: "istanbul",
      reportOnFailure: true,
      reporter: [
        "text-summary",
        [
          "lcov",
          {
            projectRoot: fileURLToPath(
              new URL("../../../../../..", import.meta.url),
            ),
          },
        ],
      ],
    },
  },
});
