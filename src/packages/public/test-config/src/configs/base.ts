import { defineConfig } from "vitest/config";

export const base = defineConfig({
  test: {
    globals: true,
    restoreMocks: true,
    unstubEnvs: true,
    unstubGlobals: true,
  },
});
