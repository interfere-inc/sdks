import react from "@vitejs/plugin-react";
import { defineConfig, mergeConfig } from "vitest/config";

import { browser } from "@interfere/test-config/browser";
import { node } from "@interfere/test-config/node";
import { base } from "@interfere/test-config/base";

const edgeTests = [
  "src/instrumentation.edge*.test.ts",
  "src/internal/server/exporters/edge-exporter.test.ts",
  "src/edge-bundle.qualification.ts",
];

export default defineConfig({
  root: import.meta.dirname,
  test: {
    projects: [
      node({ test: { name: "node", exclude: edgeTests } }),
      mergeConfig(base, {
        resolve: { conditions: ["edge-light"] },
        ssr: { resolve: { conditions: ["edge-light"] } },
        test: {
          name: "edge",
          environment: "edge-runtime",
          include: edgeTests,
        },
      }),
      browser({
        optimizeDeps: {
          exclude: [
            "fsevents",
            "@opentelemetry/otlp-transformer",
            "@opentelemetry/sdk-trace-node",
            "@opentelemetry/context-async-hooks",
          ],
          include: ["@opentelemetry/api"],
        },
        plugins: [react()],
        test: { name: "browser", setupFiles: ["./test/setup-browser.ts"] },
      }),
    ],
  },
});
