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
      // WARNING: The edge group runs first because browser project transforms block Vite's main thread and stall cold edge module fetches for seconds on 4 vCPU runners.
      node({
        test: {
          name: "node",
          exclude: edgeTests,
          sequence: { groupOrder: 1 },
          // WARNING: vi.resetModules() reloads inlined workspace code, so the OpenTelemetry SDK bundled into server-core must load natively to keep one class identity.
          server: {
            deps: { external: [/\/server-core\/dist\/(?:otel|vendor)\//u] },
          },
        },
      }),
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
            "@opentelemetry/sdk-trace",
            "@opentelemetry/context-async-hooks",
          ],
          include: ["@opentelemetry/api"],
        },
        plugins: [react()],
        test: {
          name: "browser",
          setupFiles: ["./test/setup-browser.ts"],
          sequence: { groupOrder: 1 },
        },
      }),
    ],
  },
});
