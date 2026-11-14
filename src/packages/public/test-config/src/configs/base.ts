import { defineConfig } from "vitest/config";

export const base = defineConfig({
  resolve: { conditions: ["monorepo"] },
  ssr: { resolve: { conditions: ["monorepo"] } },
  test: {
    globals: true,
    restoreMocks: true,
    unstubEnvs: true,
    unstubGlobals: true,
  },
});
