import { configDefaults, defineProject, mergeConfig } from "vitest/config";
import type { ViteUserConfig } from "vitest/config";

import { base } from "./base.ts";

export const node = (overrides: ViteUserConfig = {}) =>
  mergeConfig(
    mergeConfig(
      base,
      defineProject({
        test: {
          env: {
            WRANGLER_BUILD_CONDITIONS: "monorepo,workerd,worker,browser",
          },
          fsModuleCache: true,
          exclude: [
            ...configDefaults.exclude,
            "**/*.browser.test.{ts,tsx}",
            "**/*.worker.test.ts",
            "**/*.contract.test.ts",
          ],
          include: ["**/*.test.{ts,tsx}"],
        },
      }),
    ),
    overrides,
  );
