import { existsSync } from "node:fs";
import { relative, resolve } from "node:path";
import { defineConfig } from "vitest/config";

import { reporting } from "./src/packages/public/test-config/src/reporting/reporting.ts";

const projectSelected = process.argv.some(
  (argument) => argument === "--project" || argument.startsWith("--project="),
);

export default defineConfig({
  root: import.meta.dirname,
  test: {
    ...reporting.test,
    coverage: {
      ...reporting.test?.coverage,
      include: [
        relative(
          projectSelected ? process.cwd() : import.meta.dirname,
          resolve("src/**/*.{ts,tsx,mts,cts}"),
        ),
      ],
    },
    projects:
      process.cwd() === import.meta.dirname
        ? ["src/packages/public/*/vitest.config.ts"]
        : [
            resolve("vitest.config.{ts,mts}"),
            ...(existsSync(resolve("vitest.native.config.mts"))
              ? [resolve("vitest.native.config.mts")]
              : []),
          ],
  },
});
