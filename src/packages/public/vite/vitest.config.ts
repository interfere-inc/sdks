import { defineConfig } from "vitest/config";
import { browser } from "@interfere/test-config/browser";
import { node } from "@interfere/test-config/node";

export default defineConfig({
  root: import.meta.dirname,
  test: {
    projects: [
      node({ test: { name: "node", exclude: ["**/*.native.test.ts"] } }),
      browser({
        optimizeDeps: {
          include: ["@opentelemetry/api"],
          exclude: ["fsevents", "@opentelemetry/otlp-transformer"],
        },
        test: { name: "browser" },
      }),
    ],
  },
});
